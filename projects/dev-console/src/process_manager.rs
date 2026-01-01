// Process manager for tracking and cleaning up child processes

use std::process::Child;
use std::sync::{Arc, Mutex};

/// Process manager that tracks running child processes
pub struct ProcessManager {
    processes: Arc<Mutex<Vec<u32>>>, // Store process IDs
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Register a child process to be tracked (extracts PID)
    pub fn register(&self, child: &Child) {
        if let Ok(mut processes) = self.processes.lock() {
            // Get the process ID (id() returns u32 directly)
            let pid = child.id();
            processes.push(pid);
        }
    }
    
    /// Kill all registered processes and clean up
    pub fn cleanup(&self) {
        let pids = match self.processes.lock() {
            Ok(p) => p.clone(), // Clone the list so we can release the lock
            Err(_) => return, // Poisoned lock, can't clean up
        };
        
        for pid in pids {
            // Kill the process by PID
            #[cfg(unix)]
            {
                use std::process::Command;
                // On Unix, use kill command
                let _ = Command::new("kill")
                    .arg("-TERM")
                    .arg(pid.to_string())
                    .output();
            }
            
            #[cfg(windows)]
            {
                use std::process::Command;
                // On Windows, use taskkill
                let _ = Command::new("taskkill")
                    .args(&["/F", "/PID", &pid.to_string()])
                    .output();
            }
        }
        
        // Clear the list
        if let Ok(mut processes) = self.processes.lock() {
            processes.clear();
        }
    }
    
    /// Remove a process from tracking (called when process completes normally)
    pub fn unregister(&self, pid: u32) {
        if let Ok(mut processes) = self.processes.lock() {
            processes.retain(|&p| p != pid);
        }
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
