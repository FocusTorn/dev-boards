use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Connection {
    pub id: String,
    pub compiler: String,
    pub port: String,
    pub baudrate: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Device {
    pub id: String,
    pub board_model: String,
    pub fbqn: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mqtt {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sketch {
    pub id: String,
    pub path: String,
    pub connection: String,
    pub device: String,
    pub mqtt: String,
}
