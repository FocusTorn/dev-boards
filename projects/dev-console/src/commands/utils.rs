// Command execution utility functions

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref ANSI_RE: Regex = Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
    static ref PERCENT_RE: Regex = Regex::new(r"(\d+\.?\d*)%").unwrap();
    static ref FILE_RE: Regex = Regex::new(r"(?:-\s+)?([^\s\[\]()]+\.(cpp|c|ino|S))").unwrap();
}

/// Remove ANSI escape sequences from a string
pub fn remove_ansi_escapes(s: &str) -> String {
    ANSI_RE.replace_all(s, "").to_string()
}

/// Extract percentage from a line
pub fn extract_percentage(line: &str) -> Option<f64> {
    if let Some(captures) = PERCENT_RE.captures(line) {
        if let Ok(percent) = captures[1].parse::<f64>() {
            return Some(percent.min(100.0));
        }
    }
    None
}

/// Extract current file from a line
pub fn extract_current_file(line: &str) -> Option<String> {
    if let Some(captures) = FILE_RE.captures(line) {
        return Some(captures[1].to_string());
    }
    None
}
