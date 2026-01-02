// Monitor command execution (Rust-based, direct serial port connection)

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::process_manager::ProcessManager;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Execute monitor-serial command using Rust (direct serial port connection)
pub fn execute_monitor_serial_rust(
    dashboard: Arc<Mutex<DashboardState>>,
    settings: Settings,
    _process_manager: Arc<ProcessManager>,
) {
    // Clear status and output panels before starting monitor
    {
        let mut state = dashboard.lock().unwrap();
        // Clear output lines
        state.output_lines.clear();
        // Reset progress
        state.progress_percent = 0.0;
        state.set_progress_stage("");
        state.set_current_file("");
        // Reset scroll position
        state.output_scroll = 0;
        state.auto_scroll_enabled = true;
        // Set initial status
        use crate::string_intern::common;
        state.status_text = common::RUNNING.clone();
        state.is_running = true;
    }
    
    // Add initial message
    {
        let mut state = dashboard.lock().unwrap();
        state.add_output_line(format!("Opening serial monitor on {} at {} baud...", settings.port, settings.baudrate));
        state.add_output_line(format!("Connecting directly to serial port: {}", settings.port));
        state.set_progress_stage("Monitoring");
    }
    
    // Open serial port directly
    let mut port = match serialport::new(&settings.port, settings.baudrate as u32)
        .timeout(Duration::from_millis(100))
        .open()
    {
        Ok(port) => {
            // Port opened successfully - add success message
            let mut state = dashboard.lock().unwrap();
            state.add_output_line(format!("✅ Serial monitor connected successfully on {} at {} baud", settings.port, settings.baudrate));
            state.add_output_line("Monitor is live - waiting for data...".to_string());
            state.add_output_line("".to_string());
            state.add_output_line("Note: If you don't see any output:".to_string());
            state.add_output_line("  • Press the RESET button on your ESP32 to restart the sketch".to_string());
            state.add_output_line("  • Check that the sketch is uploaded and running".to_string());
            state.add_output_line("  • Verify the baud rate matches your sketch (usually 115200)".to_string());
            state.add_output_line("".to_string());
            state.set_status_text("Monitor active - waiting for data");
            drop(state); // Release lock before continuing
            port
        },
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            let error_msg = format!("Error: Failed to open serial port {}: {}", settings.port, e);
            state.set_status_text(&error_msg);
            state.add_output_line(error_msg.clone());
            
            // Provide helpful suggestions for common errors
            let error_str = e.to_string().to_lowercase();
            if error_str.contains("access is denied") || error_str.contains("permission denied") {
                state.add_output_line("".to_string());
                state.add_output_line("Troubleshooting tips:".to_string());
                state.add_output_line("  • Check if another program is using the serial port".to_string());
                state.add_output_line("  • Close Arduino IDE serial monitor if it's open".to_string());
                state.add_output_line("  • Close any other terminal programs using this port".to_string());
                state.add_output_line("  • Try disconnecting and reconnecting the device".to_string());
            } else if error_str.contains("no such file") || error_str.contains("not found") {
                state.add_output_line("".to_string());
                state.add_output_line("Troubleshooting tips:".to_string());
                state.add_output_line("  • Verify the port name is correct (e.g., COM9, /dev/ttyUSB0)".to_string());
                state.add_output_line("  • Check if the device is connected".to_string());
                state.add_output_line("  • Try unplugging and replugging the USB cable".to_string());
            }
            return;
        }
    };
    
    // Read from serial port with non-blocking lock
    let mut pending_lines: Vec<String> = Vec::new();
    let mut buffer = vec![0u8; 1024];
    let mut line_buffer = String::new();
    let mut last_data_time = std::time::Instant::now();
    let mut last_heartbeat_time = std::time::Instant::now();
    let heartbeat_interval = Duration::from_secs(30); // Show heartbeat every 30 seconds if no data
    let partial_flush_interval = Duration::from_secs(2); // Flush partial lines every 2 seconds
    
    loop {
        // Check if we should stop (command cancelled)
        {
            if let Ok(state) = dashboard.try_lock() {
                if !state.is_running {
                    // Command was cancelled
                    break;
                }
            }
        }
        
        let now = std::time::Instant::now();
        
        // Periodic heartbeat if no data received for a while
        if now.duration_since(last_heartbeat_time) >= heartbeat_interval {
            if let Ok(mut state) = dashboard.try_lock() {
                let time_since_data = now.duration_since(last_data_time);
                if time_since_data >= heartbeat_interval {
                    state.add_output_line(format!("[Monitor] Still waiting for data... (no data for {}s)", time_since_data.as_secs()));
                }
            }
            last_heartbeat_time = now;
        }
        
        // Flush partial lines periodically (in case data comes without newlines)
        if !line_buffer.trim().is_empty() && now.duration_since(last_data_time) >= partial_flush_interval {
            let line = line_buffer.trim().to_string();
            if !line.is_empty() {
                if let Ok(mut state) = dashboard.try_lock() {
                    for pending_line in pending_lines.drain(..) {
                        state.add_output_line(pending_line);
                    }
                    state.add_output_line(line.clone());
                } else {
                    pending_lines.push(line);
                }
            }
            line_buffer.clear();
        }
        
        // Read from serial port
        match port.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    // No data available, sleep briefly
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                
                // Update last data time
                last_data_time = now;
                
                // Convert bytes to string, handling partial UTF-8 sequences
                let text = match String::from_utf8(buffer[..n].to_vec()) {
                    Ok(t) => t,
                    Err(_) => {
                        // Handle UTF-8 error - try to recover by skipping invalid bytes
                        String::from_utf8_lossy(&buffer[..n]).to_string()
                    }
                };
                
                // Process characters to build lines
                for ch in text.chars() {
                    if ch == '\n' {
                        // End of line - add to output
                        let line = line_buffer.trim().to_string();
                        if !line.is_empty() {
                            // Try to get lock, but don't block - queue if busy
                            if let Ok(mut state) = dashboard.try_lock() {
                                // Got the lock - add pending lines first, then this one
                                for pending_line in pending_lines.drain(..) {
                                    state.add_output_line(pending_line);
                                }
                                state.add_output_line(line.clone());
                            } else {
                                // Lock is busy (UI thread is rendering) - queue this line for later
                                pending_lines.push(line);
                            }
                        }
                        line_buffer.clear();
                    } else if ch != '\r' {
                        // Add character to line buffer (skip carriage returns)
                        line_buffer.push(ch);
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // Timeout is expected - continue reading
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(e) => {
                // Error reading from port
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text(&format!("Error reading from serial port: {}", e));
                state.add_output_line(format!("Error reading from serial port: {}", e));
                return;
            }
        }
    }
    
    // Flush any remaining pending lines before exiting
    if !pending_lines.is_empty() || !line_buffer.trim().is_empty() {
        if let Ok(mut state) = dashboard.lock() {
            for pending_line in pending_lines.drain(..) {
                state.add_output_line(pending_line);
            }
            let remaining = line_buffer.trim().to_string();
            if !remaining.is_empty() {
                state.add_output_line(remaining);
            }
        }
    }
    
    // Close port and update state
    drop(port); // Close the port
    
    {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        state.set_status_text("Monitor closed");
        state.add_output_line("Monitor closed".to_string());
    }
}
