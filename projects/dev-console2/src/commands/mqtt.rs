use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::mpsc;
use crate::commands::compile::ProgressUpdate;
use rumqttc::{Client, MqttOptions, QoS, Event, Packet};

/// Commands sent from the TUI to the background MQTT thread.
pub enum MqttCommand {
    Publish { topic: String, payload: String },
}

/// Spawns a background task to monitor and interact with an MQTT broker.
///>
/// Handles connection lifecycle, subscription to all topics ("#"), and 
/// bidirectional message passing between the TUI and the broker. 
/// Translates incoming MQTT packets into TUI output lines.
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

    // Apply authentication if provided
    if let (Some(u), Some(p)) = (username, password) { //>
        mqtt_options.set_credentials(u, p);
    } //<

    let (client, mut connection) = Client::new(mqtt_options, 10);
    
    callback(ProgressUpdate::OutputLine(format!("⮻ Connecting to {}:{}...", host, port)));

    // Subscribe to a default topic or everything if permitted
    if let Err(e) = client.subscribe("#", QoS::AtMostOnce) { //>
        callback(ProgressUpdate::Failed(format!("✗ Subscription failed: {}", e)));
        return;
    } //<

    while !cancel_signal.load(Ordering::SeqCst) { //>
        // 1. Check for commands to send
        if let Ok(cmd) = command_rx.try_recv() { //>
            match cmd { //>
                MqttCommand::Publish { topic, payload } => { //>
                    if let Err(e) = client.publish(&topic, QoS::AtMostOnce, false, payload.as_bytes()) { //>
                        callback(ProgressUpdate::OutputLine(format!("✗ Publish Error: {}", e)));
                    } else {
                        callback(ProgressUpdate::OutputLine(format!("ｉ{} -> {}", topic, payload)));
                    } //<
                } //<
            } //<
        } //<

        // 2. Poll for incoming messages
        match connection.recv_timeout(Duration::from_millis(50)) { //>
            Ok(notification) => { //>
                match notification { //>
                    Ok(Event::Incoming(Packet::Publish(publish))) => { //>
                        let payload = String::from_utf8_lossy(&publish.payload).to_string();
                        callback(ProgressUpdate::OutputLine(format!("[{}] {}", publish.topic, payload)));
                    } //<
                    Ok(_) => {}
                    Err(e) => { //>
                        callback(ProgressUpdate::OutputLine(format!("✗ Connection Error: {}", e)));
                        break;
                    } //<
                } //<
            } //<
            Err(_) => {} // Timeout
        } //<
    } //<
    
    callback(ProgressUpdate::OutputLine("⮻ Connection closed.".to_string()));
}