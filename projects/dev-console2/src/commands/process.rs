use super::traits::{ChildProcess, CommandRunner};
use std::io::Read;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Managed wrapper for spawning and monitoring external child processes.
///>
/// Provides real-time line-based output capture from both stdout and stderr,
/// with integrated support for external cancellation signals.
///<
pub struct ProcessHandler {
    child: Box<dyn ChildProcess>,
}

impl ProcessHandler {
    /// Spawns the command with piped IO using the provided runner.
    pub fn spawn(runner: &dyn CommandRunner, command: Command) -> Result<Self, std::io::Error> {
        let child = runner.spawn(command)?;
        Ok(Self { child })
    }

    /// Monitors the process output and handles lifecycle events.
    ///>
    /// Spawns background threads to read stdout/stderr byte-by-byte,
    /// reassembling them into lines for the provided callback. Blocks until
    /// the process terminates or is killed via the `cancel_signal`.
    ///<
    pub fn read_output<F>(
        mut self,
        cancel_signal: Arc<AtomicBool>,
        mut callback: F,
    ) -> Result<bool, std::io::Error>
    where
        F: FnMut(String) + Send + 'static,
    {
        let stdout = self.child.stdout().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stdout")
        })?;
        let stderr = self.child.stderr().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stderr")
        })?;

        let (tx, rx) = mpsc::channel();

        // Helper to spawn a real-time byte-reader for a stream
        fn spawn_byte_reader<R: Read + Send + 'static>(stream: R, tx: mpsc::Sender<String>) {
            thread::spawn(move || {
                let mut reader = stream;
                let mut buffer = Vec::new();
                let mut byte = [0u8; 1];

                while reader.read_exact(&mut byte).is_ok() {
                    //>
                    let b = byte[0];
                    if b == b'\n' || b == b'\r' {
                        //>
                        if !buffer.is_empty() {
                            //>
                            let s = String::from_utf8_lossy(&buffer).to_string();
                            if tx.send(s).is_err() {
                                break;
                            }
                            buffer.clear();
                        } //<
                    } else {
                        buffer.push(b);
                    } //<
                } //<
            });
        }

        spawn_byte_reader(stdout, tx.clone());
        spawn_byte_reader(stderr, tx.clone());

        loop {
            //>
            // Check for cancellation signal
            if cancel_signal.load(Ordering::SeqCst) {
                //>
                let _ = self.child.kill();
                return Ok(false);
            } //<

            // Try to receive output without blocking too long to keep checking cancel_signal
            if let Ok(line) = rx.try_recv() {
                //>
                callback(line);
            } else {
                // Check if child has exited
                match self.child.try_wait()? {
                    //>
                    Some(status) => {
                        //>
                        // Process remaining messages in channel
                        while let Ok(line) = rx.try_recv() {
                            //>
                            callback(line);
                        } //<
                        return Ok(status.success());
                    } //<
                    None => {
                        // Still running, sleep briefly
                        thread::sleep(Duration::from_millis(10));
                    }
                } //<
            } //<
        } //<
    }
}
