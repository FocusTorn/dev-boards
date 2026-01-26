/// Background operation orchestrators and hardware interaction logic.
/// 
/// The `commands` module provides the "How" for all long-running tasks and external 
/// system interactions. It encapsulates the logic for:
/// - **Compiling:** Parsing MCU build outputs and tracking progress.
/// - **Flashing:** Managing hardware upload processes via toolchains like `arduino-cli`.
/// - **Monitoring:** Providing asynchronous serial and MQTT data streams.
/// - **History & Prediction:** Tracking past performance to provide accurate ETAs for builds.
///
/// All commands communicate back to the main application through a `ProgressUpdate` 
/// stream to ensure the UI remains responsive and informed.
pub mod compile;
pub mod upload;
pub mod predictor;
pub mod history;
pub mod serial_v2;
pub mod mqtt;
mod compile_state;
mod compile_parser;
mod utils;
mod path_utils;
mod process;

pub use compile::{run_compile, ProgressUpdate, Settings};
pub use upload::{run_upload};
pub use predictor::{ProgressPredictor};
pub use history::{HistoryManager};
pub use serial_v2::{run_serial_monitor, SerialCommand};
pub use mqtt::{run_mqtt_monitor, MqttCommand};
