use std::path::PathBuf;
use std::{io, io::ErrorKind};

use super::{ForegroundJob, Signal};

/// Unsupported platform stub.
pub fn foreground_job(_child_pid: u32) -> Option<ForegroundJob> {
    None
}

/// Unsupported platform stub.
pub fn process_cwd(_pid: u32) -> Option<PathBuf> {
    None
}

/// Unsupported platform stub.
pub fn session_processes(_child_pid: u32) -> Vec<u32> {
    Vec::new()
}

/// Unsupported platform stub.
pub fn signal_processes(_pids: &[u32], _signal: Signal) {}

/// Unsupported platform stub.
pub fn process_exists(_pid: u32) -> bool {
    false
}

/// Unsupported platform stub.
pub fn write_clipboard(_bytes: &[u8]) -> bool {
    false
}

/// Unsupported platform stub.
pub fn open_url(_url: &str) -> io::Result<()> {
    Err(io::Error::new(
        ErrorKind::Unsupported,
        "opening URLs is not supported on this platform",
    ))
}
