/// Management of external child processes and their output streams.
///>
/// This module provides a thread-safe wrapper for spawning system commands 
/// and capturing their output in real-time. It ensures that output from both 
/// stdout and stderr is interleaved and processed as it arrives, while 
/// maintaining responsiveness to user cancellation signals.
///< 
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

/// Managed wrapper for a running system process.
pub struct ProcessHandler {
    child: Child,
}

impl ProcessHandler {
    /// Spawns a new command with piped output streams.
///>
    /// Captures stdout and stderr to allow for real-time monitoring of 
    /// toolchain progress.
    ///< 
    pub fn spawn(mut command: Command) -> Result<Self, std::io::Error> {
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        Ok(Self { child })
    }

    /// Monitors the process output and status until completion or cancellation.
///>
    /// This method spawns background threads to read raw bytes from the process 
    /// streams, ensuring that even slow or partial output is captured without 
    /// blocking the main UI. It returns `Ok(true)` if the process exited 
    /// successfully, and `Ok(false)` if it was killed or failed.
    ///< 
    pub fn read_output<F>(mut self, cancel_signal: Arc<AtomicBool>, mut callback: F) -> Result<bool, std::io::Error>
    where
        F: FnMut(String) + Send + 'static,
    {
        let stdout = self.child.stdout.take().unwrap();
        let stderr = self.child.stderr.take().unwrap();

        let (tx, rx) = mpsc::channel();

        // Internal helper to read a stream byte-by-byte and aggregate into lines.
        // Byte-level reading is necessary for tools that emit progress percentages 
        // without newlines (e.g., esptool).
        fn spawn_byte_reader<R: Read + Send + 'static>(stream: R, tx: mpsc::Sender<String>) {
            thread::spawn(move || {
                let mut reader = stream;
                let mut buffer = Vec::new();
                let mut byte = [0u8; 1];
                
                while reader.read_exact(&mut byte).is_ok() {
                    let b = byte[0];
                    if b == b'\n' || b == b'\r' {
                        if !buffer.is_empty() {
                            let s = String::from_utf8_lossy(&buffer).to_string();
                            if tx.send(s).is_err() { break; }
                            buffer.clear();
                        }
                    } else {
                        buffer.push(b);
                    }
                }
            });
        }

        spawn_byte_reader(stdout, tx.clone());
        spawn_byte_reader(stderr, tx.clone());

        loop {
            // Respect the global cancellation signal (e.g., user pressed Esc)
            if cancel_signal.load(Ordering::SeqCst) {
                let _ = self.child.kill();
                return Ok(false);
            }

            // Ingest lines from the reader threads and pass them to the provided callback
            if let Ok(line) = rx.try_recv() {
                callback(line);
            } else {
                // Check if the process has terminated naturally
                match self.child.try_wait()? {
                    Some(status) => {
                        // Flush remaining messages from the channel before exiting
                        while let Ok(line) = rx.try_recv() {
                            callback(line);
                        }
                        return Ok(status.success());
                    }
                    None => {
                        // Snipe CPU usage during busy-wait
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        }
    }
}
