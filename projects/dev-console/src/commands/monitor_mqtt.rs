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
    settings: Settings,
    _process_manager: Arc<ProcessManager>,
) {
    // MQTT configuration from settings or defaults
    let mqtt_host = settings.mqtt_host.as_deref().unwrap_or("localhost");
    let mqtt_port = settings.mqtt_port.unwrap_or(1883u16);
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
    
    // Use default credentials "mqtt"/"mqtt" if not configured
    let mqtt_username = settings.mqtt_username.as_deref().unwrap_or("mqtt");
    let mqtt_password = settings.mqtt_password.as_deref().unwrap_or("mqtt");
    
    // Send CONNECT packet with authentication
    let connect = Connect {
        protocol: mqttrs::Protocol::MQTT311,
        keep_alive: 60,
        client_id: client_id,
        clean_session: true,
        last_will: None,
        username: Some(mqtt_username),
        password: Some(mqtt_password.as_bytes()),
    };
    
    // Log authentication status
    {
        let mut state = dashboard.lock().unwrap();
        state.add_output_line(format!("Using MQTT authentication (username: {})", mqtt_username));
    }
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
    
    // Read SUBACK response (with timeout to avoid hanging)
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    match stream.read(&mut read_buf) {
        Ok(0) => {
            // Connection closed by broker
            let mut state = dashboard.lock().unwrap();
            state.is_running = false;
            state.set_status_text("MQTT broker closed connection during subscription");
            state.add_output_line("MQTT broker closed connection during subscription".to_string());
            return;
        }
        Ok(n) => {
            // Received data - try to decode packet
            match decode_slice(&read_buf[..n]) {
                Ok(Some(Packet::Suback { .. })) => {
                    let mut state = dashboard.lock().unwrap();
                    state.add_output_line(format!("✅ Subscribed to topic: {}", mqtt_topic));
                }
                Ok(Some(Packet::Pingresp)) => {
                    // Ping response - unexpected here but handle gracefully
                    let mut state = dashboard.lock().unwrap();
                    state.add_output_line("Warning: Received PINGRESP instead of SUBACK".to_string());
                }
                Ok(Some(_)) => {
                    // Other packet types - log and continue
                    let mut state = dashboard.lock().unwrap();
                    state.add_output_line("Warning: Received unexpected packet type, but continuing".to_string());
                }
                Ok(None) => {
                    // Incomplete packet - log and continue
                    let mut state = dashboard.lock().unwrap();
                    state.add_output_line("Warning: Incomplete SUBACK packet, but continuing".to_string());
                }
                Err(e) => {
                    let mut state = dashboard.lock().unwrap();
                    state.add_output_line(format!("Warning: Failed to decode SUBACK: {} - continuing anyway", e));
                }
            }
        }
        Err(e) => {
            // Check if it's a connection error
            if e.kind() == std::io::ErrorKind::ConnectionReset || 
               e.kind() == std::io::ErrorKind::BrokenPipe ||
               e.raw_os_error() == Some(10053) {
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text("MQTT broker closed connection during subscription");
                state.add_output_line("MQTT broker closed connection during subscription".to_string());
                return;
            }
            // Timeout - try one more read with shorter timeout
            if e.kind() == std::io::ErrorKind::TimedOut {
                stream.set_read_timeout(Some(Duration::from_millis(500))).ok();
                match stream.read(&mut read_buf) {
                    Ok(0) => {
                        let mut state = dashboard.lock().unwrap();
                        state.is_running = false;
                        state.set_status_text("MQTT broker closed connection during subscription");
                        state.add_output_line("MQTT broker closed connection during subscription".to_string());
                        return;
                    }
                    Ok(n) => {
                        match decode_slice(&read_buf[..n]) {
                            Ok(Some(Packet::Suback { .. })) => {
                                let mut state = dashboard.lock().unwrap();
                                state.add_output_line(format!("✅ Subscribed to topic: {}", mqtt_topic));
                            }
                            _ => {
                                let mut state = dashboard.lock().unwrap();
                                state.add_output_line("Warning: No SUBACK received, but continuing anyway".to_string());
                            }
                        }
                    }
                    Err(_) => {
                        let mut state = dashboard.lock().unwrap();
                        state.add_output_line("Warning: SUBACK timeout, but continuing anyway - subscription may have succeeded".to_string());
                    }
                }
            } else {
                // Other error - log and continue
                let mut state = dashboard.lock().unwrap();
                state.add_output_line(format!("Warning: Error reading SUBACK: {} - continuing anyway", e));
            }
        }
    }
    
    // Small delay to ensure broker has processed subscription
    thread::sleep(Duration::from_millis(100));
    
    // Configure stream for non-blocking reads and writes (for main loop)
    // Use a longer read timeout to avoid premature timeouts
    stream.set_read_timeout(Some(Duration::from_millis(500))).ok();
    stream.set_write_timeout(Some(Duration::from_millis(1000))).ok();
    
    // Enable TCP nodelay to reduce latency
    if let Err(e) = stream.set_nodelay(true) {
        let mut state = dashboard.lock().unwrap();
        state.add_output_line(format!("Warning: Failed to set TCP_NODELAY: {}", e));
    }
    
    // Main loop - poll for messages
    let mut pending_lines: Vec<String> = Vec::new();
    let mut packet_buf = vec![0u8; 4096];
    let mut read_pos = 0;
    let keep_alive_seconds = 60u64;
    let ping_interval = Duration::from_secs(keep_alive_seconds / 2); // Send ping at 50% of keep-alive interval
    // Start ping timer from now (not from connection time) to send first ping sooner
    let mut last_ping_time = std::time::Instant::now();
    let mut last_activity_time = std::time::Instant::now();
    
    // Send initial PINGREQ immediately to show we're alive
    // This helps prevent brokers from closing idle connections
    {
        let pingreq_packet = Packet::Pingreq;
        let mut ping_buf = vec![0u8; 16];
        match encode_slice(&pingreq_packet, &mut ping_buf) {
            Ok(len) => {
                match stream.write_all(&ping_buf[..len]) {
                    Ok(_) => {
                        last_ping_time = std::time::Instant::now();
                        last_activity_time = std::time::Instant::now();
                    }
                    Err(e) => {
                        let mut state = dashboard.lock().unwrap();
                        state.add_output_line(format!("Warning: Failed to send initial PINGREQ: {}", e));
                    }
                }
            }
            Err(e) => {
                let mut state = dashboard.lock().unwrap();
                state.add_output_line(format!("Warning: Failed to encode initial PINGREQ: {}", e));
            }
        }
    }
    
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
        
        // Check for buffer overflow before reading
        if read_pos >= packet_buf.len() {
            // Buffer is full - this shouldn't happen with normal MQTT packets
            // Reset buffer and log warning
            let mut state = dashboard.lock().unwrap();
            state.add_output_line("Warning: MQTT packet buffer overflow - resetting".to_string());
            read_pos = 0;
        }
        
        // Try to read data from stream
        match stream.read(&mut packet_buf[read_pos..]) {
            Ok(0) => {
                // Connection closed gracefully by broker
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text("MQTT broker disconnected");
                state.add_output_line("MQTT broker disconnected".to_string());
                break;
            }
            Ok(n) => {
                read_pos += n;
                last_activity_time = std::time::Instant::now(); // Update activity time
                
                // Check for buffer overflow after reading
                if read_pos > packet_buf.len() {
                    // This shouldn't happen, but handle it gracefully
                    let mut state = dashboard.lock().unwrap();
                    state.add_output_line("Warning: MQTT packet buffer overflow - resetting".to_string());
                    read_pos = 0;
                    continue;
                }
                
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
                            // Decode error - reset buffer to prevent infinite loop
                            let mut state = dashboard.lock().unwrap();
                            state.add_output_line("Warning: MQTT packet decode error - resetting buffer".to_string());
                            read_pos = 0;
                            break;
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                // Timeout is expected with non-blocking reads - continue
                // But check if we've been idle too long
                let now = std::time::Instant::now();
                if now.duration_since(last_activity_time) >= Duration::from_secs(keep_alive_seconds) {
                    let mut state = dashboard.lock().unwrap();
                    state.is_running = false;
                    state.set_status_text("MQTT connection timeout - no activity");
                    state.add_output_line("MQTT connection timeout - no activity".to_string());
                    break;
                }
            }
            Err(e) => {
                // Connection error - provide more informative error message
                let error_msg = if e.raw_os_error() == Some(10053) {
                    "MQTT connection closed by broker (connection aborted). This may indicate:\n  - Broker rejected the connection\n  - Network/firewall issue\n  - Protocol mismatch".to_string()
                } else if e.kind() == std::io::ErrorKind::ConnectionReset {
                    "MQTT connection reset by broker".to_string()
                } else if e.kind() == std::io::ErrorKind::BrokenPipe {
                    "MQTT connection broken (pipe closed)".to_string()
                } else {
                    format!("MQTT read error: {}", e)
                };
                
                let mut state = dashboard.lock().unwrap();
                state.is_running = false;
                state.set_status_text(&error_msg);
                state.add_output_line(error_msg.clone());
                // Add troubleshooting info
                state.add_output_line("Troubleshooting:".to_string());
                state.add_output_line("  1. Verify Mosquitto is running: netstat -an | findstr 1883".to_string());
                state.add_output_line("  2. Check Mosquitto logs for connection errors".to_string());
                state.add_output_line("  3. Verify firewall allows connections on port 1883".to_string());
                state.add_output_line("  4. Try connecting with another MQTT client to verify broker".to_string());
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
