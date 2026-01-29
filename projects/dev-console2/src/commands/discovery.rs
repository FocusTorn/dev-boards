use super::traits::{PortScanner, RealPortScanner, PortInfo};

/// Discovers available serial ports and retrieves rich metadata.
pub fn scan_ports() -> Result<Vec<PortInfo>, serialport::Error> {
    scan_ports_with_scanner(&RealPortScanner)
}

/// Discovers available serial ports using the provided scanner.
pub fn scan_ports_with_scanner(scanner: &dyn PortScanner) -> Result<Vec<PortInfo>, serialport::Error> {
    scanner.list_ports()
}
