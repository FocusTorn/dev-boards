use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;

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

    pub fn read_output<F>(mut self, mut callback: F) -> Result<bool, std::io::Error>
    where
        F: FnMut(String) + Send + 'static,
    {
        let stdout = self.child.stdout.take().unwrap();
        let stderr = self.child.stderr.take().unwrap();

        let (tx, rx) = mpsc::channel();

        let stdout_tx = tx.clone();
        let stdout_thread = thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if stdout_tx.send(line.unwrap_or_default()).is_err() {
                    break;
                }
            }
        });

        let stderr_thread = thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if tx.send(line.unwrap_or_default()).is_err() {
                    break;
                }
            }
        });

        for line in rx {
            callback(line);
        }

        stdout_thread.join().unwrap();
        stderr_thread.join().unwrap();

        let status = self.child.wait()?;
        Ok(status.success())
    }
}
