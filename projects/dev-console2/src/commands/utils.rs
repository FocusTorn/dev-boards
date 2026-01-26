/// General utility functions for command output processing.
///>
/// This module provides shared string manipulation and data extraction 
/// helpers used by various background command parsers.
///<
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    /// Matches standard ANSI SGR escape sequences used for terminal colors.
    static ref ANSI_RE: Regex = Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
    /// Matches simple percentage patterns (e.g., "45.5%").
    static ref PERCENT_RE: Regex = Regex::new(r"(\d+\.?\d*)%").unwrap();
    /// Matches file paths with common C/C++ and Arduino extensions.
    static ref FILE_RE: Regex = Regex::new(r"(?:-\s+)?([^\s\[\]()]+\.(cpp|c|ino|S))").unwrap();
}

/// Strip ANSI escape sequences from a string to allow for clean text analysis.
///>
/// This is essential for parsers that rely on string matching, as raw 
/// terminal output often contains invisible color and formatting codes.
///<
pub fn remove_ansi_escapes(s: &str) -> String {
    ANSI_RE.replace_all(s, "").to_string()
}

/// Extracts a numeric percentage value from a line of text.
///>
/// Returns the value as a float between 0.0 and 100.0 if a match is found.
///<
#[allow(dead_code)]
pub fn extract_percentage(line: &str) -> Option<f64> {
    if let Some(captures) = PERCENT_RE.captures(line) {
        if let Ok(percent) = captures[1].parse::<f64>() {
            return Some(percent.min(100.0));
        }
    }
    None
}

/// Extracts a source file path from a compiler log line.
///>
/// Used to identify which component is currently being processed by the 
/// toolchain.
///<
#[allow(dead_code)]
pub fn extract_current_file(line: &str) -> Option<String> {
    if let Some(captures) = FILE_RE.captures(line) {
        return Some(captures[1].to_string());
    }
    None
}