// Command execution trait for unified command interface

use crate::dashboard::DashboardState;
use crate::settings::Settings;
use crate::process_manager::ProcessManager;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::process::Command;

/// Trait for command executors
pub trait CommandExecutor {
    /// Execute the command
    fn execute(
        &self,
        dashboard: Arc<Mutex<DashboardState>>,
        settings: Settings,
        process_manager: Arc<ProcessManager>,
    );
    
    /// Get command name
    fn name(&self) -> &str;
    
    /// Check if command supports progress tracking
    fn supports_progress(&self) -> bool {
        false
    }
}

/// Command configuration builder
pub struct CommandConfig {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<PathBuf>,
    pub env_vars: Vec<(String, String)>,
    pub stdout_piped: bool,
    pub stderr_piped: bool,
}

impl CommandConfig {
    /// Create a new command configuration builder
    pub fn new<S: Into<String>>(command: S) -> Self {
        Self {
            command: command.into(),
            args: Vec::new(),
            working_dir: None,
            env_vars: Vec::new(),
            stdout_piped: true,
            stderr_piped: true,
        }
    }
    
    /// Add an argument
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }
    
    /// Add multiple arguments
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(|s| s.into()));
        self
    }
    
    /// Set working directory
    pub fn working_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.into());
        self
    }
    
    /// Add environment variable
    pub fn env<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.env_vars.push((key.into(), value.into()));
        self
    }
    
    /// Set whether stdout should be piped
    pub fn stdout_piped(mut self, piped: bool) -> Self {
        self.stdout_piped = piped;
        self
    }
    
    /// Set whether stderr should be piped
    pub fn stderr_piped(mut self, piped: bool) -> Self {
        self.stderr_piped = piped;
        self
    }
    
    /// Build the Command from this configuration
    pub fn build(&self) -> Command {
        let mut cmd = Command::new(&self.command);
        
        // Add arguments
        for arg in &self.args {
            cmd.arg(arg);
        }
        
        // Set working directory
        if let Some(ref dir) = self.working_dir {
            cmd.current_dir(dir);
        }
        
        // Add environment variables
        for (key, value) in &self.env_vars {
            cmd.env(key, value);
        }
        
        // Configure stdio
        if self.stdout_piped {
            use std::process::Stdio;
            cmd.stdout(Stdio::piped());
        }
        if self.stderr_piped {
            use std::process::Stdio;
            cmd.stderr(Stdio::piped());
        }
        
        cmd
    }
}
