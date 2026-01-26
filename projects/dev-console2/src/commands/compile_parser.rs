/// Parsing logic for MCU compiler output streams.
/// 
/// This module provides the regex-based analysis needed to extract semantic meaning 
/// from the raw text emitted by tools like `arduino-cli`. It detects stage 
/// transitions (e.g., from Compiling to Linking) and tracks file-level progress.
use crate::commands::compile_state::CompileState;
use crate::commands::utils::remove_ansi_escapes;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    /// Matches file paths in compilation commands to identify which file is being processed.
    static ref RE_COMPILE_COMMAND: Regex = Regex::new(
        r"@([^\s]+\.(cpp|c|ino|S))|([^\s/\\]+\.(cpp|c|ino|S))"
    ).unwrap();
    /// Matches explicit "compiling" log lines often found in higher-level build tools.
    static ref RE_COMPILE_LINE: Regex = Regex::new(
        r"(?i)compiling\s+([^\s]+\.(cpp|c|ino|S))"
    ).unwrap();
    /// Matches successful compilation markers or archive creation to increment the file count.
    static ref RE_COMPILED_FILE: Regex = Regex::new(
        r"(?i)\.(cpp|c|ino|S)\.o|gcc-ar|compiled\s+[^\s]+\.(cpp|c|ino|S)|using previously compiled file"
    ).unwrap();
}

/// Analyzes a single line of output to detect transitions between build stages.
/// 
/// This function returns a tuple of `(stage_changed, should_continue)`.
/// - `stage_changed`: True if a new stage marker (e.g., "Linking") was found.
/// - `should_continue`: False if a fatal error was detected, signaling the process should stop.
pub fn detect_stage_change(line: &str, compile_state: &mut CompileState, current_progress: f64, callback: &mut impl FnMut(String)) -> (bool, bool) {
    let cleaned = remove_ansi_escapes(line);
    let line_lower = cleaned.to_lowercase();
    let trimmed = cleaned.trim();
    
    if trimmed.is_empty() {
        return (false, true); // Ignore empty lines but keep processing
    }
    
    // Stop processing if we encounter known failure markers
    if line_lower.contains("error") || line_lower.contains("fatal") {
        return (false, false); 
    }
    
    let mut next_stage = None;

    // Detect keyword markers that signal a change in the build phase
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
    } else if line_lower.contains("leaving...") || line_lower.contains("hard resetting...") {
        next_stage = Some(crate::commands::compile_state::CompileStage::Complete);
    }

    if let Some(stage) = next_stage {
        // Enforce monotonic transitions: we should never go 'backwards' in the build sequence
        if stage.rank() > compile_state.stage.rank() {
            compile_state.previous_stage_progress = current_progress;
            let skipped = compile_state.transition_to(stage);
            
            // Log if we missed a marker (e.g., build tool skipped library detection)
            for s in skipped {
                callback(format!("[WARNING] Skipped stage marker: {:?}", s));
            }

            compile_state.last_marker_time = std::time::Instant::now();
            
            // Record timestamps for duration-based progress calculation
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
    
    (false, true)
}

/// Extracts specific file and command information from a compiler output line.
/// 
/// This populates the `CompileState` with the name of the file currently being 
/// processed and increments the total file count when new compilation commands 
/// are encountered.
pub fn parse_compilation_info(line: &str, compile_state: &mut CompileState) {
    let cleaned = remove_ansi_escapes(line);
    let line_lower = cleaned.to_lowercase();
    let trimmed = cleaned.trim();
    
    // Detect toolchain-specific compilation commands (gcc/g++)
    if line.contains("xtensa-esp32s3-elf-g++") || line.contains("xtensa-esp32s3-elf-gcc") {
        if line.contains("-c") {
            // Ensure we are in the 'Compiling' stage if we see compiler flags
            if compile_state.stage.rank() < crate::commands::compile_state::CompileStage::Compiling.rank() {
                compile_state.stage = crate::commands::compile_state::CompileStage::Compiling;
            }
            
            if compile_state.compile_stage_start.is_none() {
                compile_state.compile_stage_start = Some(std::time::Instant::now());
            }
            
            // Extract the source file path being processed
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
        // Generic "Compiling <file>" pattern
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
        // Increment the count of successfully processed files
        if !compile_state.compiled_lines_seen.contains(trimmed) {
            compile_state.compiled_lines_seen.insert(trimmed.to_string());
            compile_state.files_compiled = compile_state.compiled_lines_seen.len();
        }
    }
}
