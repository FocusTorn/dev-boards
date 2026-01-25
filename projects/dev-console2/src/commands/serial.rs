use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::mpsc;
use crate::commands::compile::ProgressUpdate;
use serialport;

pub enum SerialCommand {
    SendData(String),
}

pub fn run_serial_monitor(
    port_name: String,
    baud_rate: u32,
    cancel_signal: Arc<AtomicBool>,
    command_rx: mpsc::Receiver<SerialCommand>,
    progress_callback: impl FnMut(ProgressUpdate) + Send + 'static,
) {
    let mut callback = progress_callback;
    
    let port = serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open();

    let mut port = match port {
        Ok(p) => {
            callback(ProgressUpdate::OutputLine(format!("[SERIAL] Connected to {} at {} baud.", port_name, baud_rate)));
            p
        }
        Err(e) => {
            callback(ProgressUpdate::Failed(format!("[SERIAL] Failed to open port {}: {}", port_name, e)));
            return;
        }
    };

    let mut buffer = [0u8; 1024];
    
    while !cancel_signal.load(Ordering::SeqCst) {
        // 1. Check for commands to send
        if let Ok(cmd) = command_rx.try_recv() {
            match cmd {
                SerialCommand::SendData(data) => {
                    let to_send = format!("{}\n", data);
                    if let Err(e) = port.write_all(to_send.as_bytes()) {
                        callback(ProgressUpdate::OutputLine(format!("[SERIAL] Write Error: {}", e)));
                    } else {
                        callback(ProgressUpdate::OutputLine(format!("[TX] {}", data)));
                    }
                }
            }
        }

        // 2. Read incoming data
        match port.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                // Simple line-based reporting
                for line in data.lines() {
                    callback(ProgressUpdate::OutputLine(line.to_string()));
                }
            }
            Ok(_) => {} // Ignore empty reads
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {} // Ignore timeouts
            Err(e) => {
                callback(ProgressUpdate::OutputLine(format!("[SERIAL] Read Error: {}", e)));
                break;
            }
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
    
    callback(ProgressUpdate::OutputLine("[SERIAL] Connection closed.".to_string()));
}
