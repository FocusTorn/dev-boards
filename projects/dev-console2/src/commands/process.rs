use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub struct ProcessHandler {
    child: Child,
}

impl ProcessHandler {
    pub fn spawn(mut command: Command) -> Result<Self, std::io::Error> {
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        Ok(Self { child })
    }

    pub fn read_output<F>(mut self, cancel_signal: Arc<AtomicBool>, mut callback: F) -> Result<bool, std::io::Error>
    where
        F: FnMut(String) + Send + 'static,
    {
        let stdout = self.child.stdout.take().unwrap();
        let stderr = self.child.stderr.take().unwrap();

        let (tx, rx) = mpsc::channel();

        let stdout_tx = tx.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if stdout_tx.send(line.unwrap_or_default()).is_err() {
                    break;
                }
            }
        });

        let stderr_tx = tx.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if stderr_tx.send(line.unwrap_or_default()).is_err() {
                    break;
                }
            }
        });

        loop {
            // Check for cancellation signal
            if cancel_signal.load(Ordering::SeqCst) {
                let _ = self.child.kill();
                return Ok(false);
            }

            // Try to receive output without blocking too long to keep checking cancel_signal
            if let Ok(line) = rx.try_recv() {
                callback(line);
            } else {
                // Check if child has exited
                match self.child.try_wait()? {
                    Some(status) => {
                        // Process remaining messages in channel
                        while let Ok(line) = rx.try_recv() {
                            callback(line);
                        }
                        return Ok(status.success());
                    }
                    None => {
                        // Still running, sleep briefly
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        }
    }
}