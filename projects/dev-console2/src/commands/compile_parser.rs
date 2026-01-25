// Compilation output parsing and stage detection

use crate::commands::compile_state::CompileState;
use crate::commands::utils::remove_ansi_escapes;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_COMPILE_COMMAND: Regex = Regex::new(
        r"@([^\s]+\.(cpp|c|ino|S))|([^\s/\\]+\.(cpp|c|ino|S))"
    ).unwrap();
    static ref RE_COMPILE_LINE: Regex = Regex::new(
        r"(?i)compiling\s+([^\s]+\.(cpp|c|ino|S))"
    ).unwrap();
    static ref RE_COMPILED_FILE: Regex = Regex::new(
        r"(?i)\.(cpp|c|ino|S)\.o|gcc-ar|compiled\s+[^\s]+\.(cpp|c|ino|S)|using previously compiled file"
    ).unwrap();
}

/// Parse a line and detect compilation stage changes
/// Returns (stage_changed, should_continue)
pub fn detect_stage_change(line: &str, compile_state: &mut CompileState, current_progress: f64, callback: &mut impl FnMut(String)) -> (bool, bool) {
    let cleaned = remove_ansi_escapes(line);
    let line_lower = cleaned.to_lowercase();
    let trimmed = cleaned.trim();
    
    if trimmed.is_empty() {
        return (false, true); // Continue processing
    }
    
    // Detect errors - skip further processing
    if line_lower.contains("error") || line_lower.contains("fatal") {
        return (false, false); // Don't continue processing
    }
    
    let mut next_stage = None;

    // Detect markers
    if line_lower.contains("detecting libraries") || line_lower.contains("detecting library") {
        next_stage = Some(crate::commands::compile_state::CompileStage::DetectingLibraries);
    } else if line_lower.contains("generating function prototypes") || line_lower.contains("generating prototypes") {
        next_stage = Some(crate::commands::compile_state::CompileStage::Compiling);
    } else if line_lower.contains("linking everything together") 
           || (line_lower.contains("linking") && line_lower.contains(".elf"))
           || line_lower.contains("archive") 
           || line_lower.contains("gcc-ar")
    {
        next_stage = Some(crate::commands::compile_state::CompileStage::Linking);
    } else if line_lower.contains("esptool") && line_lower.contains("elf2image") && 
              line_lower.contains(".ino.elf") && line_lower.contains(".ino.bin") &&
              !line_lower.contains("bootloader") {
        if compile_state.stage == crate::commands::compile_state::CompileStage::Linking || compile_state.link_stage_start.is_some() {
            next_stage = Some(crate::commands::compile_state::CompileStage::Generating);
        }
    } else if line_lower.contains("sketch uses") || line_lower.contains("global variables use") {
        next_stage = Some(crate::commands::compile_state::CompileStage::Complete);
    }

    if let Some(stage) = next_stage {
        // Monotonic transition: only move forward
        if stage.rank() > compile_state.stage.rank() {
            compile_state.previous_stage_progress = current_progress;
            let skipped = compile_state.transition_to(stage);
            
            // Notify about skips
            for s in skipped {
                callback(format!("[WARNING] Skipped stage marker: {:?}", s));
            }

            compile_state.last_marker_time = std::time::Instant::now();
            
            // Setup timers
            match stage {
                crate::commands::compile_state::CompileStage::DetectingLibraries => {
                    if compile_state.detect_libs_stage_start.is_none() {
                        compile_state.detect_libs_stage_start = Some(std::time::Instant::now());
                    }
                }
                crate::commands::compile_state::CompileStage::Compiling => {
                    if compile_state.compile_stage_start.is_none() {
                        compile_state.compile_stage_start = Some(std::time::Instant::now());
                    }
                }
                crate::commands::compile_state::CompileStage::Linking => {
                    compile_state.current_file.clear();
                    if compile_state.link_stage_start.is_none() {
                        compile_state.link_stage_start = Some(std::time::Instant::now());
                    }
                }
                crate::commands::compile_state::CompileStage::Generating => {
                    compile_state.current_file.clear();
                    if compile_state.generate_stage_start.is_none() {
                        compile_state.generate_stage_start = Some(std::time::Instant::now());
                    }
                }
                crate::commands::compile_state::CompileStage::Complete => {
                    compile_state.current_file.clear();
                }
                _ => {}
            }
            return (true, true);
        }
    }
    
    (false, true) // Continue processing
}

/// Parse compilation commands and files from a line
pub fn parse_compilation_info(line: &str, compile_state: &mut CompileState) {
    let cleaned = remove_ansi_escapes(line);
    let line_lower = cleaned.to_lowercase();
    let trimmed = cleaned.trim();
    
    // Detect compilation commands/files
    if line.contains("xtensa-esp32s3-elf-g++") || line.contains("xtensa-esp32s3-elf-gcc") {
        if line.contains("-c") {
            // Only move to Compiling if we are currently in an earlier stage
            if compile_state.stage.rank() < crate::commands::compile_state::CompileStage::Compiling.rank() {
                compile_state.stage = crate::commands::compile_state::CompileStage::Compiling;
            }
            
            if compile_state.compile_stage_start.is_none() {
                compile_state.compile_stage_start = Some(std::time::Instant::now());
            }
            
            if let Some(captures) = RE_COMPILE_COMMAND.captures(line) {
                if let Some(file_match) = captures.get(1).or_else(|| captures.get(3)) {
                    let file_path = file_match.as_str();
                    compile_state.current_file = file_path.to_string();
                    if !compile_state.compile_lines_seen.contains(trimmed) {
                        compile_state.compile_lines_seen.insert(trimmed.to_string());
                        compile_state.total_files = compile_state.compile_lines_seen.len();
                    }
                }
            }
        }
    } else if let Some(captures) = RE_COMPILE_LINE.captures(&line_lower) {
        if let Some(file_match) = captures.get(1) {
            let file_path = file_match.as_str();
            compile_state.current_file = file_path.to_string();
            compile_state.stage = crate::commands::compile_state::CompileStage::Compiling;
            if compile_state.compile_stage_start.is_none() {
                compile_state.compile_stage_start = Some(std::time::Instant::now());
            }
            if !compile_state.compile_lines_seen.contains(trimmed) {
                compile_state.compile_lines_seen.insert(trimmed.to_string());
                compile_state.total_files = compile_state.compile_lines_seen.len();
            }
        }
    } else if RE_COMPILED_FILE.is_match(&line_lower) {
        if !compile_state.compiled_lines_seen.contains(trimmed) {
            compile_state.compiled_lines_seen.insert(trimmed.to_string());
            compile_state.files_compiled = compile_state.compiled_lines_seen.len();
        }
    }
}
