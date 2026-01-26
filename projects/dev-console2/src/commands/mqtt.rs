/// Real-time MQTT monitoring and interaction.
///>
/// This module provides the bridge between the TUI and an MQTT broker. It allows 
/// for asynchronous monitoring of hardware telemetry and enables the UI to 
/// publish control commands back to connected ESP32 devices.
///<
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::mpsc;
use crate::commands::compile::ProgressUpdate;
use rumqttc::{Client, MqttOptions, QoS, Event, Packet};

/// Semantic commands that can be sent to the MQTT background thread.
///>
/// Represents the actions the UI can request from the MQTT client, 
/// such as publishing data to a specific topic.
///<
pub enum MqttCommand {
    /// Publishes a text payload to a specific MQTT topic.
    Publish { topic: String, payload: String },
}

/// Executes the MQTT monitor loop in a background thread.
///>
/// This function:
/// 1. Establishes a persistent connection to the broker.
/// 2. Subscribes to all topics (`#`) by default to capture broad device telemetry.
/// 3. Ingests incoming MQTT messages and translates them into `ProgressUpdate::OutputLine`.
/// 4. Processes outgoing `MqttCommand` requests from the UI.
///<
pub fn run_mqtt_monitor(
    host: String,
    port: u16,
    client_id: String,
    username: Option<String>,
    password: Option<String>,
    cancel_signal: Arc<AtomicBool>,
    command_rx: mpsc::Receiver<MqttCommand>,
    progress_callback: impl FnMut(ProgressUpdate) + Send + 'static,
) {
    let mut callback = progress_callback;
    
    let mut mqtt_options = MqttOptions::new(client_id, host.clone(), port);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    // Apply authentication credentials if they were defined in config.yaml
    if let (Some(u), Some(p)) = (username, password) {
        mqtt_options.set_credentials(u, p);
    }

    let (client, mut connection) = Client::new(mqtt_options, 10);
    
    callback(ProgressUpdate::OutputLine(format!("⮻ Connecting to {}:{}...", host, port)));

    // Subscribe to everything to ensure the user sees all relevant hardware chatter
    if let Err(e) = client.subscribe("#", QoS::AtMostOnce) {
        callback(ProgressUpdate::Failed(format!("✗ Subscription failed: {}", e)));
        return;
    }

    while !cancel_signal.load(Ordering::SeqCst) {
        // 1. Process outgoing commands (e.g., user typing into the MQTT input field)
        if let Ok(cmd) = command_rx.try_recv() {
            match cmd {
                MqttCommand::Publish { topic, payload } => {
                    if let Err(e) = client.publish(&topic, QoS::AtMostOnce, false, payload.as_bytes()) {
                        callback(ProgressUpdate::OutputLine(format!("✗ Publish Error: {}", e)));
                    } else {
                        // Echo the outgoing message to the local terminal for feedback
                        callback(ProgressUpdate::OutputLine(format!("ｉ{} -> {}", topic, payload)));
                    }
                }
            }
        }

        // 2. Poll for incoming telemetry from the broker
        match connection.recv_timeout(Duration::from_millis(50)) {
            Ok(notification) => {
                match notification {
                    Ok(Event::Incoming(Packet::Publish(publish))) => {
                        let payload = String::from_utf8_lossy(&publish.payload).to_string();
                        callback(ProgressUpdate::OutputLine(format!("[{}] {}", publish.topic, payload)));
                    }
                    Ok(_) => {}
                    Err(e) => {
                        callback(ProgressUpdate::OutputLine(format!("✗ Connection Error: {}", e)));
                        break;
                    }
                }
            }
            Err(_) => {} // Timeout - continue the loop to check cancel_signal
        }
    }
    
    callback(ProgressUpdate::OutputLine("⮻ Connection closed.".to_string()));
}
