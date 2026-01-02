// Process execution and output handling

use crate::dashboard::DashboardState;
use crate::process_manager::ProcessManager;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;

/// Handle process execution with stdout/stderr reading
pub struct ProcessHandler {
    child: Child,
    pid: u32,
}

impl ProcessHandler {
    /// Spawn a process and return a handler
    pub fn spawn(
        mut cmd: Command,
        process_manager: Arc<ProcessManager>,
    ) -> Result<Self, String> {
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let child = cmd.spawn()
            .map_err(|e| format!("Failed to start process: {}", e))?;
        
        // Register process with process manager for cleanup tracking
        process_manager.register(&child);
        
        let pid = child.id();
        
        Ok(Self {
            child,
            pid,
        })
    }
    
    /// Start reading stderr in a separate thread
    pub fn start_stderr_reader(
        &mut self,
        dashboard: Arc<Mutex<DashboardState>>,
        log_file: Arc<Mutex<File>>,
    ) {
        let dashboard_stderr = dashboard.clone();
        let log_file_stderr = log_file.clone();
        
        if let Some(stderr) = self.child.stderr.take() {
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        // Preserve ANSI codes for colorization - only trim whitespace
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            let mut state = dashboard_stderr.lock().unwrap();
                            state.add_output_line(trimmed.to_string());
                            // Log to file
                            if let Ok(mut log) = log_file_stderr.lock() {
                                let _ = writeln!(log, "{}", trimmed);
                            }
                            // Auto-scroll is handled during rendering with correct visible_height
                        }
                    }
                }
            });
        }
    }
    
    /// Take stdout for reading (consumes the handler's stdout)
    pub fn take_stdout(&mut self) -> Option<std::process::ChildStdout> {
        self.child.stdout.take()
    }
    
    /// Get the process ID
    #[allow(dead_code)] // May be useful for external callers
    pub fn pid(&self) -> u32 {
        self.pid
    }
    
    /// Wait for the process to finish and return the exit status
    pub fn wait(mut self, process_manager: Arc<ProcessManager>) -> std::io::Result<std::process::ExitStatus> {
        let exit_status = self.child.wait()?;
        // Unregister process from process manager (completed normally)
        process_manager.unregister(self.pid);
        Ok(exit_status)
    }
}
