// Monitor-MQTT command execution (Rust-based, direct MQTT connection)

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::process_manager::ProcessManager;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use std::io::{Read, Write};
use mqttrs::{encode_slice, decode_slice, Packet, Pid, QoS, Connect, Subscribe, SubscribeTopic};

/// Execute monitor-mqtt command using Rust (direct MQTT connection)
pub fn execute_monitor_mqtt_rust(
    dashboard: Arc<Mutex<DashboardState>>,
    _settings: Settings,
    _process_manager: Arc<ProcessManager>,
) {
    // MQTT configuration (can be made configurable later)
    let mqtt_host = "localhost";
    let mqtt_port = 1883u16;
    let mqtt_topic = "sensors/sht21/readings";
    let client_id = "dev-console-monitor";
    
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
        state.add_output_line(format!("Connecting to MQTT broker at {}:{}...", mqtt_host, mqtt_port));
        state.add_output_line(format!("Subscribing to topic: {}", mqtt_topic));
        state.set_progress_stage("Connecting");
    }
    
    // Create TCP stream
    let mut stream = match TcpStream::connect(format!("{}:{}", mqtt_host, mqtt_port)) {
        Ok(stream) => stream,
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_status_text(&format!("Error: Failed to connect to MQTT broker: {}", e));
            state.add_output_line(format!("Error: Failed to connect to MQTT broker: {}", e));
            return;
        }
    };
    
    // Send CONNECT packet
    let connect = Connect {
        protocol: mqttrs::Protocol::MQTT311,
        keep_alive: 60,
        client_id: client_id,
        clean_session: true,
        last_will: None,
        username: None,
        password: None,
    };
    let connect_packet = Packet::Connect(connect);
    
    let mut buf = vec![0u8; 1024];
    match encode_slice(&connect_packet, &mut buf) {
        Ok(len) => {
            if let Err(e) = stream.write_all(&buf[..len]) {
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text(&format!("Error: Failed to send CONNECT: {}", e));
                state.add_output_line(format!("Error: Failed to send CONNECT: {}", e));
                return;
            }
        }
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_status_text(&format!("Error: Failed to encode CONNECT: {}", e));
            state.add_output_line(format!("Error: Failed to encode CONNECT: {}", e));
            return;
        }
    }
    
    // Read CONNACK response
    let mut read_buf = vec![0u8; 1024];
    match stream.read(&mut read_buf) {
        Ok(n) if n > 0 => {
            match decode_slice(&read_buf[..n]) {
                Ok(Some(Packet::Connack { .. })) => {
                    let mut state = dashboard.lock().unwrap();
                    state.set_progress_stage("Monitoring");
                    state.add_output_line(format!("✅ Connected to MQTT broker at {}:{}", mqtt_host, mqtt_port));
                }
                Ok(Some(_)) => {
                    let mut state = dashboard.lock().unwrap();
                    state.is_running = false;
                    state.set_status_text("Error: Unexpected packet type in response");
                    state.add_output_line("Error: Unexpected packet type in response".to_string());
                    return;
                }
                Ok(None) => {
                    let mut state = dashboard.lock().unwrap();
                    state.is_running = false;
                    state.set_status_text("Error: Incomplete CONNACK packet");
                    state.add_output_line("Error: Incomplete CONNACK packet".to_string());
                    return;
                }
                Err(e) => {
                    let mut state = dashboard.lock().unwrap();
                    state.is_running = false;
                    state.set_status_text(&format!("Error: Failed to decode CONNACK: {}", e));
                    state.add_output_line(format!("Error: Failed to decode CONNACK: {}", e));
                    return;
                }
            }
        }
        Ok(_) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_status_text("Error: No response from broker");
            state.add_output_line("Error: No response from broker".to_string());
            return;
        }
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_status_text(&format!("Error: Failed to read CONNACK: {}", e));
            state.add_output_line(format!("Error: Failed to read CONNACK: {}", e));
            return;
        }
    }
    
    // Subscribe to topic
    let subscribe = Subscribe {
        pid: Pid::new(),
        topics: vec![SubscribeTopic {
            topic_path: mqtt_topic.to_string(),
            qos: QoS::AtLeastOnce,
        }],
    };
    let subscribe_packet = Packet::Subscribe(subscribe);
    
    match encode_slice(&subscribe_packet, &mut buf) {
        Ok(len) => {
            if let Err(e) = stream.write_all(&buf[..len]) {
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text(&format!("Error: Failed to send SUBSCRIBE: {}", e));
                state.add_output_line(format!("Error: Failed to send SUBSCRIBE: {}", e));
                return;
            }
        }
        Err(e) => {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_status_text(&format!("Error: Failed to encode SUBSCRIBE: {}", e));
            state.add_output_line(format!("Error: Failed to encode SUBSCRIBE: {}", e));
            return;
        }
    }
    
    // Read SUBACK response
    match stream.read(&mut read_buf) {
        Ok(n) if n > 0 => {
            match decode_slice(&read_buf[..n]) {
                Ok(Some(Packet::Suback { .. })) => {
                    let mut state = dashboard.lock().unwrap();
                    state.add_output_line(format!("✅ Subscribed to topic: {}", mqtt_topic));
                }
                Ok(Some(_)) | Ok(None) => {
                    // Continue anyway
                }
                Err(e) => {
                    let mut state = dashboard.lock().unwrap();
                    state.add_output_line(format!("Warning: Failed to decode SUBACK: {}", e));
                }
            }
        }
        _ => {
            // Continue anyway - subscription might still work
        }
    }
    
    // Configure stream for non-blocking reads and writes
    stream.set_read_timeout(Some(Duration::from_millis(100))).ok();
    stream.set_write_timeout(Some(Duration::from_millis(1000))).ok();
    
    // Main loop - poll for messages
    let mut pending_lines: Vec<String> = Vec::new();
    let mut packet_buf = vec![0u8; 4096];
    let mut read_pos = 0;
    let keep_alive_seconds = 60u64;
    let ping_interval = Duration::from_secs(keep_alive_seconds / 2); // Send ping at 50% of keep-alive interval
    let mut last_ping_time = std::time::Instant::now();
    let mut last_activity_time = std::time::Instant::now();
    
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
        
        // Send PINGREQ if keep-alive interval has elapsed
        let now = std::time::Instant::now();
        if now.duration_since(last_ping_time) >= ping_interval {
            // Send PINGREQ to keep connection alive
            let pingreq_packet = Packet::Pingreq;
            let mut ping_buf = vec![0u8; 16];
            match encode_slice(&pingreq_packet, &mut ping_buf) {
                Ok(len) => {
                    if let Err(e) = stream.write_all(&ping_buf[..len]) {
                        let mut state = dashboard.lock().unwrap();
                        state.is_running = false;
                        state.set_status_text(&format!("Error: Failed to send PINGREQ: {}", e));
                        state.add_output_line(format!("Error: Failed to send PINGREQ: {}", e));
                        break;
                    }
                    last_ping_time = now;
                    last_activity_time = now; // Sending ping counts as activity
                }
                Err(e) => {
                    let mut state = dashboard.lock().unwrap();
                    state.is_running = false;
                    state.set_status_text(&format!("Error: Failed to encode PINGREQ: {}", e));
                    state.add_output_line(format!("Error: Failed to encode PINGREQ: {}", e));
                    break;
                }
            }
        }
        
        // Check if connection is dead (no activity for full keep-alive period)
        if now.duration_since(last_activity_time) >= Duration::from_secs(keep_alive_seconds) {
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_status_text("MQTT connection timeout - no activity");
            state.add_output_line("MQTT connection timeout - no activity".to_string());
            break;
        }
        
        // Try to read data from stream
        match stream.read(&mut packet_buf[read_pos..]) {
            Ok(0) => {
                // Connection closed
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text("MQTT broker disconnected");
                state.add_output_line("MQTT broker disconnected".to_string());
                break;
            }
            Ok(n) => {
                read_pos += n;
                last_activity_time = std::time::Instant::now(); // Update activity time
                
                // Try to decode packets
                loop {
                    match decode_slice(&packet_buf[..read_pos]) {
                        Ok(Some(Packet::Publish(publish))) => {
                            // Parse message payload
                            let payload_str = match String::from_utf8(publish.payload.to_vec()) {
                                Ok(p) => p.trim().to_string(),
                                Err(_) => {
                                    format!("[Binary data: {} bytes]", publish.payload.len())
                                }
                            };
                            
                            // Format message with topic
                            let message = format!("[{}] {}", publish.topic_name, payload_str);
                            
                            // Try to get lock, but don't block - queue if busy
                            if let Ok(mut state) = dashboard.try_lock() {
                                // Got the lock - add pending lines first, then this one
                                for pending_line in pending_lines.drain(..) {
                                    state.add_output_line(pending_line);
                                }
                                state.add_output_line(message);
                                read_pos = 0; // Reset buffer
                            } else {
                                // Lock is busy (UI thread is rendering) - queue this line for later
                                pending_lines.push(message);
                                // Keep the data in buffer for next iteration
                                break;
                            }
                        }
                        Ok(Some(Packet::Pingresp)) => {
                            // Ping response - update activity time
                            last_activity_time = std::time::Instant::now();
                            read_pos = 0;
                        }
                        Ok(Some(_)) => {
                            // Other packet types - update activity time
                            last_activity_time = std::time::Instant::now();
                            read_pos = 0;
                        }
                        Ok(None) => {
                            // Incomplete packet - need more data
                            break;
                        }
                        Err(_) => {
                            // Decode error - reset buffer
                            read_pos = 0;
                            break;
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                // Timeout is expected - continue
            }
            Err(e) => {
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text(&format!("MQTT read error: {}", e));
                state.add_output_line(format!("MQTT read error: {}", e));
                break;
            }
        }
        
        // Flush any pending lines if we can get the lock
        if !pending_lines.is_empty() {
            if let Ok(mut state) = dashboard.try_lock() {
                for pending_line in pending_lines.drain(..) {
                    state.add_output_line(pending_line);
                }
            }
        }
        
        thread::sleep(Duration::from_millis(10));
    }
    
    // Flush any remaining pending lines before exiting
    if !pending_lines.is_empty() {
        if let Ok(mut state) = dashboard.lock() {
            for pending_line in pending_lines.drain(..) {
                state.add_output_line(pending_line);
            }
        }
    }
    
    // Update state
    {
        let mut state = dashboard.lock().unwrap();
        state.is_running = false;
        state.set_status_text("MQTT monitor closed");
        state.add_output_line("MQTT monitor closed".to_string());
    }
}
