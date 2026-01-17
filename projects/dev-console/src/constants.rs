// Application constants

/// Minimum terminal width in pixels
pub const MIN_WIDTH_PIXELS: u16 = 80;

/// Minimum terminal height in pixels
pub const MIN_HEIGHT_PIXELS: u16 = 21;

/// Field height in lines (for settings fields)
pub const FIELD_HEIGHT: u16 = 3;

/// Spacing between fields
pub const FIELD_SPACING: u16 = 0;

/// Content area width percentage (50% of available space)
pub const CONTENT_WIDTH_PERCENT: u16 = 50;

/// Content area height percentage (50% of available space)
pub const CONTENT_HEIGHT_PERCENT: u16 = 50;

/// Maximum output lines to keep in memory
pub const MAX_OUTPUT_LINES: usize = 1000;

/// Toast display duration in seconds
#[allow(dead_code)] // For future use
pub const TOAST_DURATION_SECS: f64 = 1.5;

/// Main content box handle name
pub const HWND_MAIN_CONTENT_BOX: &str = "hwndMainContentBox";

/// Main content tab bar handle name
pub const HWND_MAIN_CONTENT_TAB_BAR: &str = "hwndMainContentTabBar";

/// Settings field HWND constants
pub const HWND_SETTINGS_FIELD_SKETCH_DIR: &str = "hwndSettingsFieldSketchDir";
pub const HWND_SETTINGS_FIELD_SKETCH_NAME: &str = "hwndSettingsFieldSketchName";
pub const HWND_SETTINGS_FIELD_ENV: &str = "hwndSettingsFieldEnv";
pub const HWND_SETTINGS_FIELD_BOARD_MODEL: &str = "hwndSettingsFieldBoardModel";
pub const HWND_SETTINGS_FIELD_FQBN: &str = "hwndSettingsFieldFQBN";
pub const HWND_SETTINGS_FIELD_PORT: &str = "hwndSettingsFieldPort";
pub const HWND_SETTINGS_FIELD_BAUDRATE: &str = "hwndSettingsFieldBaudrate";

/// Settings section HWND constants
pub const HWND_SETTINGS_SECTION_DEVICE: &str = "hwndSettingsSectionDevice";
pub const HWND_SETTINGS_SECTION_CONNECTION: &str = "hwndSettingsSectionConnection";
pub const HWND_SETTINGS_SECTION_MQTT: &str = "hwndSettingsSectionMQTT";

/// MQTT field HWND constants
pub const HWND_SETTINGS_FIELD_MQTT_HOST: &str = "hwndSettingsFieldMqttHost";
pub const HWND_SETTINGS_FIELD_MQTT_PORT: &str = "hwndSettingsFieldMqttPort";
pub const HWND_SETTINGS_FIELD_MQTT_USERNAME: &str = "hwndSettingsFieldMqttUsername";
pub const HWND_SETTINGS_FIELD_MQTT_PASSWORD: &str = "hwndSettingsFieldMqttPassword";
pub const HWND_SETTINGS_FIELD_MQTT_TOPIC_COMMAND: &str = "hwndSettingsFieldMqttTopicCommand";
pub const HWND_SETTINGS_FIELD_MQTT_TOPIC_STATE: &str = "hwndSettingsFieldMqttTopicState";
pub const HWND_SETTINGS_FIELD_MQTT_TOPIC_STATUS: &str = "hwndSettingsFieldMqttTopicStatus";

/// Profile box HWND constants
pub const HWND_PROFILE_BOX: &str = "hwndProfileBox";
pub const HWND_PROFILE_LIST: &str = "hwndProfileList";

/// Profile selector HWND constant (for dashboard)
pub const HWND_PROFILE_SELECTOR: &str = "hwndProfileSelector";