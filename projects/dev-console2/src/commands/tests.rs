use super::compile::*;
use super::upload::*;
use super::serial_v2::*;
use super::traits::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::path::PathBuf;
use crate::commands::history::StageStats;

#[cfg(windows)]
fn create_exit_status(code: i32) -> std::process::ExitStatus {
    use std::os::windows::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(code as u32)
}

#[cfg(unix)]
fn create_exit_status(code: i32) -> std::process::ExitStatus {
    use std::os::unix::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(code << 8)
}

#[test]
fn test_run_compile_success() {
    std::env::set_var("WORKSPACE_ROOT", ".");
    let mut mock_runner = MockCommandRunner::new();
    let mut mock_fs = MockFileSystem::new();

    let settings = Settings {
        sketch_directory: "test_sketch".to_string(),
        sketch_name: "test_sketch".to_string(),
        fqbn: "esp32:esp32:esp32s3".to_string(),
        port: "COM3".to_string(),
        baudrate: 115200,
        board_model: "esp32s3".to_string(),
        env: "arduino".to_string(),
    };

    let sketch_dir = PathBuf::from("test_sketch");
    let sketch_file = sketch_dir.join("test_sketch.ino");
    
    // Mock FS calls
    mock_fs.expect_exists()
        .with(mockall::predicate::eq(sketch_file.clone()))
        .return_const(true);
    
    // find_arduino_cli calls
    mock_fs.expect_exists()
        .return_const(false); // arduino-cli.exe in workspace not found

    let mut mock_child = MockChildProcess::new();
    mock_child.expect_stdout()
        .return_once(|| {
            let data = "Compiling sketch...\nLinking everything...\nDone!\n";
            Some(Box::new(std::io::Cursor::new(data)))
        });
    mock_child.expect_stderr()
        .return_once(|| Some(Box::new(std::io::Cursor::new(""))));
    
    // First try_wait returns None (still running), second returns Some(success)
    let mut wait_count = 0;
    mock_child.expect_try_wait()
        .returning(move || {
            wait_count += 1;
            if wait_count > 1 {
                Ok(Some(create_exit_status(0)))
            } else {
                Ok(None)
            }
        });

    mock_runner.expect_spawn()
        .return_once(|_| Ok(Box::new(mock_child)));

    let cancel_signal = Arc::new(AtomicBool::new(false));
    let updates = Arc::new(Mutex::new(Vec::new()));
    let updates_clone = updates.clone();

    run_compile_with_runners(
        &mock_runner,
        &mock_fs,
        &settings,
        StageStats::default(),
        cancel_signal,
        move |update| {
            updates_clone.lock().unwrap().push(update);
        },
    );

    let updates = updates.lock().unwrap();
    assert!(updates.contains(&ProgressUpdate::Stage("Compiling".to_string())));
    
    // Check if we got a CompletedWithMetrics update
    let has_completed = updates.iter().any(|u| matches!(u, ProgressUpdate::CompletedWithMetrics { .. }));
    assert!(has_completed, "Expected CompletedWithMetrics update, got {:?}", updates);
}

#[test]
fn test_run_upload_success() {
    std::env::set_var("WORKSPACE_ROOT", ".");
    let mut mock_runner = MockCommandRunner::new();
    let mut mock_fs = MockFileSystem::new();

    let settings = Settings {
        sketch_directory: "test_sketch".to_string(),
        sketch_name: "test_sketch".to_string(),
        fqbn: "esp32:esp32:esp32s3".to_string(),
        port: "COM3".to_string(),
        baudrate: 115200,
        board_model: "esp32s3".to_string(),
        env: "arduino".to_string(),
    };

    // find_arduino_cli calls
    mock_fs.expect_exists()
        .return_const(false); // arduino-cli.exe in workspace not found

    let mut mock_child = MockChildProcess::new();
    mock_child.expect_stdout()
        .return_once(|| {
            let data = "Resetting...\nWriting at 0x00001000... (10 %)\nWriting at 0x00002000... (100 %)\nLeaving...\nHard resetting via RTS pin...\n";
            Some(Box::new(std::io::Cursor::new(data)))
        });
    mock_child.expect_stderr()
        .return_once(|| Some(Box::new(std::io::Cursor::new(""))));
    
    let mut wait_count = 0;
    mock_child.expect_try_wait()
        .returning(move || {
            wait_count += 1;
            if wait_count > 5 {
                Ok(Some(create_exit_status(0)))
            } else {
                Ok(None)
            }
        });

    mock_runner.expect_spawn()
        .return_once(|_| Ok(Box::new(mock_child)));

    let cancel_signal = Arc::new(AtomicBool::new(false));
    let updates = Arc::new(Mutex::new(Vec::new()));
    let updates_clone = updates.clone();

    run_upload_with_runner(
        &mock_runner,
        &mock_fs,
        &settings,
        StageStats::default(),
        cancel_signal,
        move |update| {
            updates_clone.lock().unwrap().push(update);
        },
    );

    let updates = updates.lock().unwrap();
    assert!(updates.contains(&ProgressUpdate::Stage("Resetting".to_string())));
    assert!(updates.contains(&ProgressUpdate::Stage("Uploading".to_string())));
    assert!(updates.contains(&ProgressUpdate::Stage("Complete".to_string())));
    
    let has_completed = updates.iter().any(|u| matches!(u, ProgressUpdate::CompletedWithMetrics { .. }));
    assert!(has_completed);
}

#[test]
fn test_run_serial_monitor_success() {
    let mut mock_provider = MockSerialProvider::new();
    let mut mock_port = MockSerialPort::new();

    // Mock reading from the port
    let data = b"Hello from ESP32!\n";
    let mut read_count = 0;
    mock_port.expect_read()
        .returning(move |buf| {
            read_count += 1;
            if read_count == 1 {
                let n = data.len();
                buf[..n].copy_from_slice(data);
                Ok(n)
            } else {
                // Return timeout to simulate no data
                Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout"))
            }
        });

    mock_provider.expect_open()
        .return_once(|_, _| Ok(Box::new(mock_port)));

    let cancel_signal = Arc::new(AtomicBool::new(false));
    let (_tx, rx) = std::sync::mpsc::channel();
    let updates = Arc::new(Mutex::new(Vec::new()));
    let updates_clone = updates.clone();

    let cancel_signal_clone = cancel_signal.clone();
    std::thread::spawn(move || {
        run_serial_monitor_with_provider(
            &mock_provider,
            "COM3".to_string(),
            115200,
            cancel_signal_clone,
            rx,
            move |update| {
                updates_clone.lock().unwrap().push(update);
            },
        );
    });

    // Wait for the monitor to pick up the data
    std::thread::sleep(std::time::Duration::from_millis(100));
    cancel_signal.store(true, std::sync::atomic::Ordering::SeqCst);
    std::thread::sleep(std::time::Duration::from_millis(50));

    let updates = updates.lock().unwrap();
    assert!(updates.contains(&ProgressUpdate::OutputLine("⇄ Connected to COM3 at 115200 baud.".to_string())));
    assert!(updates.contains(&ProgressUpdate::OutputLine("Hello from ESP32!".to_string())));
    assert!(updates.contains(&ProgressUpdate::OutputLine("⬒ Serial connection closed.".to_string())));
}

#[test]
fn test_run_compile_sketch_not_found() {
    std::env::set_var("WORKSPACE_ROOT", ".");
    let mock_runner = MockCommandRunner::new();
    let mut mock_fs = MockFileSystem::new();

    let settings = Settings {
        sketch_directory: "test_dir".to_string(),
        sketch_name: "test_sketch".to_string(),
        fqbn: "esp32:esp32:esp32s3".to_string(),
        port: "COM3".to_string(),
        baudrate: 115200,
        board_model: "esp32s3".to_string(),
        env: "dev".to_string(),
    };

    let sketch_file = PathBuf::from("test_dir").join("test_sketch.ino");
    mock_fs.expect_exists()
        .with(mockall::predicate::eq(sketch_file))
        .times(1)
        .return_const(false);

    let cancel_signal = Arc::new(AtomicBool::new(false));
    let updates = Arc::new(Mutex::new(Vec::new()));
    let updates_clone = updates.clone();

    run_compile_with_runners(
        &mock_runner,
        &mock_fs,
        &settings,
        StageStats::default(),
        cancel_signal,
        move |update| {
            updates_clone.lock().unwrap().push(update);
        },
    );

    let updates = updates.lock().unwrap();
    assert!(updates.contains(&ProgressUpdate::Stage("Initializing".to_string())));
    match &updates[1] {
        ProgressUpdate::Failed(msg) => assert!(msg.contains("Sketch file not found")),
        _ => panic!("Expected Failed update, got {:?}", updates[1]),
    }
}
