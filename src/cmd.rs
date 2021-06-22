use std::process::Command;
use std::io;

pub fn run_command(base_command: &str, args: Vec<String>) -> io::Result<std::process::ExitStatus> {
    let mut cmd = Command::new(base_command);
    for a in args {
        cmd.arg(a);
    }
    cmd.status()
}