use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::mpsc;
use crate::commands::compile::ProgressUpdate;
use serialport;

/// Commands sent from the TUI to the background serial thread.
pub enum SerialCommand {
    SendData(String),
}

/// A high-performance Serial Monitor implementation with byte-level line buffering.
///>
/// Operates in a background thread to prevent blocking the TUI. Handles 
/// bidirectional communication with hardware, split-line reassembly for 
/// UTF-8 data, and semantic tagging for different message types to support 
/// consistent UI theming.
///<
pub fn run_serial_monitor(
    port_name: String,
    baud_rate: u32,
    cancel_signal: Arc<AtomicBool>,
    command_rx: mpsc::Receiver<SerialCommand>,
    mut callback: impl FnMut(ProgressUpdate) + Send + 'static,
) {
    // 1. Open the port with a short timeout for responsive polling
    let port_result = serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    let mut port = match port_result {
        Ok(p) => {
            callback(ProgressUpdate::OutputLine(format!("⇄ Connected to {} at {} baud.", port_name, baud_rate)));
            p
        }
        Err(e) => {
            // Signal error using the requested icon (will be themed by App)
            callback(ProgressUpdate::Failed(format!("✗ Failed to open port {}: {}", port_name, e)));
            return;
        }
    };

    let mut read_buffer = [0u8; 1024];
    let mut line_buffer = Vec::new();
    
    while !cancel_signal.load(Ordering::SeqCst) {
        // 2. Process Outgoing Data (TX)
        // We drain the channel to handle multiple commands between reads
        while let Ok(cmd) = command_rx.try_recv() {
            match cmd {
                SerialCommand::SendData(data) => {
                    let to_send = format!("{}\n", data);
                    if let Err(e) = port.write_all(to_send.as_bytes()) {
                        callback(ProgressUpdate::OutputLine(format!("✗ Write Error: {}", e)));
                    } else {
                        // Semantic tag for TX info (Info icon, bold, no space)
                        callback(ProgressUpdate::OutputLine(format!("ｉ{}", data)));
                    }
                }
            }
        }

        // 3. Process Incoming Data (RX)
        // We read raw bytes and only flush complete lines to the UI
        match port.read(&mut read_buffer) {
            Ok(n) if n > 0 => {
                for &byte in &read_buffer[..n] {
                    // Split on newline or carriage return
                    if byte == b'\n' || byte == b'\r' {
                        if !line_buffer.is_empty() {
                            if let Ok(s) = String::from_utf8(line_buffer.clone()) {
                                // Send raw serial data (no prefix here, let board speak)
                                callback(ProgressUpdate::OutputLine(s));
                            } else {
                                // If UTF-8 fails, try lossy conversion
                                let s = String::from_utf8_lossy(&line_buffer).to_string();
                                callback(ProgressUpdate::OutputLine(s));
                            }
                            line_buffer.clear();
                        }
                    } else {
                        line_buffer.push(byte);
                    }
                }
            }
            Ok(_) => {} // Nothing to read yet
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {} // Expected timeout
            Err(e) => {
                callback(ProgressUpdate::OutputLine(format!("✗ Serial Read Error: {}", e)));
                break;
            }
        }
        
        // Snipe CPU usage while maintaining high throughput
        std::thread::sleep(Duration::from_millis(1));
    }
    
    callback(ProgressUpdate::OutputLine("⬒ Serial connection closed.".to_string()));
}