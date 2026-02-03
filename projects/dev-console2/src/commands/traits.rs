use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

#[cfg(test)]
use mockall::automock;

/// Trait for executing external commands.
#[cfg_attr(test, automock)]
pub trait CommandRunner: Send + Sync {
    fn spawn(&self, command: Command) -> io::Result<Box<dyn ChildProcess>>;
}

/// Trait for interacting with a spawned child process.
#[cfg_attr(test, automock)]
pub trait ChildProcess: Send {
    fn stdout(&mut self) -> Option<Box<dyn io::Read + Send>>;
    fn stderr(&mut self) -> Option<Box<dyn io::Read + Send>>;
    fn wait(&mut self) -> io::Result<std::process::ExitStatus>;
    fn try_wait(&mut self) -> io::Result<Option<std::process::ExitStatus>>;
    fn kill(&mut self) -> io::Result<()>;
}

/// Trait for file system operations.
#[cfg_attr(test, automock)]
pub trait FileSystem: Send + Sync {
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn remove_dir_all(&self, path: &Path) -> io::Result<()>;
    fn copy(&self, from: &Path, to: &Path) -> io::Result<u64>;
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
}

/// Trait for serial port operations.
#[cfg_attr(test, automock)]
pub trait SerialProvider: Send + Sync {
    fn open(
        &self,
        port_name: &str,
        baud_rate: u32,
    ) -> Result<Box<dyn SerialPort>, serialport::Error>;
}

/// Rich metadata for a discovered serial port.
#[derive(Debug, Clone, PartialEq)]
pub struct PortInfo {
    pub port_name: String,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub product: Option<String>,
}

/// Trait for discovering available serial ports.
#[cfg_attr(test, automock)]
pub trait PortScanner: Send + Sync {
    fn list_ports(&self) -> Result<Vec<PortInfo>, serialport::Error>;
}

/// Trait for a serial port.
pub trait SerialPort: io::Read + io::Write + Send {
    fn try_clone(&self) -> Result<Box<dyn SerialPort>, serialport::Error>;
}

#[cfg(test)]
mockall::mock! {
    pub SerialPort {}
    impl io::Read for SerialPort {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    }
    impl io::Write for SerialPort {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
        fn flush(&mut self) -> io::Result<()>;
    }
    impl SerialPort for SerialPort {
        fn try_clone(&self) -> Result<Box<dyn SerialPort>, serialport::Error>;
    }
}

/// Real implementation of CommandRunner.
pub struct RealCommandRunner;
impl CommandRunner for RealCommandRunner {
    fn spawn(&self, mut command: Command) -> io::Result<Box<dyn ChildProcess>> {
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        Ok(Box::new(RealChildProcess { child }))
    }
}

pub struct RealChildProcess {
    child: Child,
}

impl ChildProcess for RealChildProcess {
    fn stdout(&mut self) -> Option<Box<dyn io::Read + Send>> {
        self.child
            .stdout
            .take()
            .map(|s| Box::new(s) as Box<dyn io::Read + Send>)
    }
    fn stderr(&mut self) -> Option<Box<dyn io::Read + Send>> {
        self.child
            .stderr
            .take()
            .map(|s| Box::new(s) as Box<dyn io::Read + Send>)
    }
    fn wait(&mut self) -> io::Result<std::process::ExitStatus> {
        self.child.wait()
    }
    fn try_wait(&mut self) -> io::Result<Option<std::process::ExitStatus>> {
        self.child.try_wait()
    }
    fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }
}

/// Real implementation of FileSystem.
pub struct RealFileSystem;
impl FileSystem for RealFileSystem {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }
    fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_dir_all(path)
    }
    fn copy(&self, from: &Path, to: &Path) -> io::Result<u64> {
        std::fs::copy(from, to)
    }
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        std::fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect()
    }
}

/// Real implementation of SerialProvider.
pub struct RealSerialProvider;
impl SerialProvider for RealSerialProvider {
    fn open(
        &self,
        port_name: &str,
        baud_rate: u32,
    ) -> Result<Box<dyn SerialPort>, serialport::Error> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(std::time::Duration::from_millis(10))
            .open()?;
        Ok(Box::new(RealSerialPort { port }))
    }
}

pub struct RealSerialPort {
    port: Box<dyn serialport::SerialPort>,
}

impl io::Read for RealSerialPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.port.read(buf)
    }
}

impl io::Write for RealSerialPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.port.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.port.flush()
    }
}

impl SerialPort for RealSerialPort {
    fn try_clone(&self) -> Result<Box<dyn SerialPort>, serialport::Error> {
        let port = self.port.try_clone()?;
        Ok(Box::new(RealSerialPort { port }))
    }
}

/// Real implementation of PortScanner.
pub struct RealPortScanner;
impl PortScanner for RealPortScanner {
    fn list_ports(&self) -> Result<Vec<PortInfo>, serialport::Error> {
        let ports = serialport::available_ports()?;
        let mut result = Vec::new();

        for port in ports {
            let mut info = PortInfo {
                port_name: port.port_name,
                vid: None,
                pid: None,
                manufacturer: None,
                serial_number: None,
                product: None,
            };

            if let serialport::SerialPortType::UsbPort(usb) = port.port_type {
                info.vid = Some(usb.vid);
                info.pid = Some(usb.pid);
                info.manufacturer = usb.manufacturer;
                info.serial_number = usb.serial_number;
                info.product = usb.product;
            }

            result.push(info);
        }

        Ok(result)
    }
}
