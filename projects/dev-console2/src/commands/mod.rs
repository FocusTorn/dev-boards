pub mod compile;
mod compile_parser;
mod compile_state;
pub mod discovery;
pub mod history;
pub mod mqtt;
mod path_utils;
pub mod predictor;
mod process;
pub mod serial_v2;
pub mod traits;
pub mod upload;
mod utils;

pub use compile::{run_compile, ProgressUpdate, Settings};
pub use discovery::scan_ports;
pub use history::HistoryManager;
pub use mqtt::{run_mqtt_monitor, MqttCommand};
pub use predictor::ProgressPredictor;
pub use serial_v2::{run_serial_monitor, SerialCommand};
pub use traits::{PortInfo, PortScanner, RealPortScanner};
pub use upload::run_upload;

#[cfg(test)]
mod tests;
