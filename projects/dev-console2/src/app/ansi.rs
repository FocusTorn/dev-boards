use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // Match ANSI escape sequences: \x1b[ followed by codes and ending with m
    static ref ANSI_REGEX: Regex = Regex::new(r"\x1b\[([0-9;]*)([a-zA-Z])").unwrap();
}

/// Parse a line with ANSI color codes and convert to ratatui Line
pub fn parse_ansi_line(line: &str) -> Line<'static> {
    if line.contains('\x1b') || line.contains('\u{001b}') {
        parse_ansi_to_spans(line)
    } else {
        Line::from(Span::raw(line.to_string()))
    }
}

/// Parse ANSI escape sequences and convert to ratatui Spans
fn parse_ansi_to_spans(text: &str) -> Line<'static> {
    let mut spans = Vec::new();
    let mut last_end = 0;
    let mut current_style = Style::default();
    
    for cap in ANSI_REGEX.captures_iter(text) {
        let full_match = cap.get(0).unwrap();
        let codes = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let command = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        
        // Add text before this ANSI code
        if full_match.start() > last_end {
            let text_before = &text[last_end..full_match.start()];
            if !text_before.is_empty() {
                spans.push(Span::styled(text_before.to_string(), current_style));
            }
        }
        
        // Parse ANSI codes and update style
        if command == "m" {
            current_style = parse_ansi_codes_to_style(codes, current_style);
        }
        
        last_end = full_match.end();
    }
    
    // Add remaining text
    if last_end < text.len() {
        let remaining = &text[last_end..];
        if !remaining.is_empty() {
            spans.push(Span::styled(remaining.to_string(), current_style));
        }
    }
    
    if spans.is_empty() {
        Line::from(Span::raw(text.to_string()))
    } else {
        Line::from(spans)
    }
}

fn parse_ansi_codes_to_style(codes: &str, mut current_style: Style) -> Style {
    if codes.is_empty() {
        return Style::default();
    }
    
    let code_parts: Vec<&str> = codes.split(';').collect();
    let mut i = 0;
    
    while i < code_parts.len() {
        if let Ok(code) = code_parts[i].parse::<u8>() {
            match code {
                0 => { current_style = Style::default(); }
                1 => { current_style = current_style.add_modifier(Modifier::BOLD); }
                30..=37 => { current_style = current_style.fg(parse_ansi_color(code - 30)); }
                38 => {
                    if i + 1 < code_parts.len() {
                        let color_type = code_parts[i + 1].parse::<u8>().unwrap_or(0);
                        if color_type == 5 && i + 2 < code_parts.len() {
                            let color_code = code_parts[i + 2].parse::<u16>().unwrap_or(0);
                            current_style = current_style.fg(parse_256_color(color_code));
                            i += 2;
                        } else if color_type == 2 && i + 4 < code_parts.len() {
                            let r = code_parts[i + 2].parse::<u8>().unwrap_or(0);
                            let g = code_parts[i + 3].parse::<u8>().unwrap_or(0);
                            let b = code_parts[i + 4].parse::<u8>().unwrap_or(0);
                            current_style = current_style.fg(Color::Rgb(r, g, b));
                            i += 4;
                        }
                    }
                }
                39 => { current_style = current_style.fg(Color::Reset); }
                90..=97 => { current_style = current_style.fg(parse_ansi_color(code - 90 + 8)); }
                _ => {}
            }
        }
        i += 1;
    }
    current_style
}

fn parse_ansi_color(code: u8) -> Color {
    match code {
        0 => Color::Black,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::White,
        8 => Color::DarkGray,
        9 => Color::LightRed,
        10 => Color::LightGreen,
        11 => Color::LightYellow,
        12 => Color::LightBlue,
        13 => Color::LightMagenta,
        14 => Color::LightCyan,
        15 => Color::White,
        _ => Color::Reset,
    }
}

fn parse_256_color(code: u16) -> Color {
    if code < 16 {
        parse_ansi_color(code as u8)
    } else if code < 232 {
        let r = ((code - 16) / 36) * 51;
        let g = (((code - 16) / 6) % 6) * 51;
        let b = ((code - 16) % 6) * 51;
        Color::Rgb(r as u8, g as u8, b as u8)
    } else {
        let gray = ((code - 232) * 10 + 8) as u8;
        Color::Rgb(gray, gray, gray)
    }
}
