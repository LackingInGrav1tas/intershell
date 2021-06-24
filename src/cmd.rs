use std::process::Command;
use std::io;
use std::env;

use crate::parser::{input, parse, TOMLCommand, CommandType};

pub struct Shell {
    cwd: String,
    history: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            cwd: env::current_dir().expect("").into_os_string().into_string().expect(""),
            history: vec![]
        }
    }

    pub fn handle_command(&mut self, command: String) {
        match CommandType::from(command) {
            CommandType::CustomCommand(s) => {
                self.run_custom_command(
                    TOMLCommand::from(
                        parse(
                            s
                        )
                    )
                ).unwrap();
            }
            CommandType::BuiltInCommand(s) => {
                self.run_builtin_command(s)
            }
        }
    }

    fn run_custom_command(&mut self, command: TOMLCommand) -> io::Result<std::process::ExitStatus> {
        let mut cmd = Command::new(command.method());
        self.history.push(format!("{:?}", cmd));
        for a in command.args() {
            cmd.arg(a);
        }
        println!("running {:?}", cmd);
        cmd.status()    
    }

    fn run_builtin_command(& mut self, command: String) {

    }
}