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
                    match TOMLCommand::from( parse(s) ) {
                        Ok(c) => c,
                        Err(()) => return
                    }
                ).unwrap();
            }
            CommandType::BuiltInCommand(s) => {
                self.run_builtin_command(s)
            }
            CommandType::CMDCall(s) => {
                // override everything and send straight to normal cmd
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
        cmd.current_dir(&self.cwd);
        cmd.status()
    }

    fn run_builtin_command(& mut self, command: String) {
        match command.as_str() {
            "cd" => {

            }
            "dir" => {

            }
            "md" | "mkdir" => {

            }
            "del" | "delete" | "erase" => {

            }
            "attrib" | "attributes" => {

            }
            "cls" | "clear" => {

            }
            "exit" | "quit" => {
                std::process::exit(0)
            }
            "color" => {

            }
            "comp" | "compare" => {

            }
            "copy" => {

            }
            "echo" => {

            }
            "find" => {

            }
            "print" => {

            }
            "rename" => {

            }
            "rmdir" => {

            }
            "ip" | "ipconfig" => {
                
            }
            _ => unreachable!()
        }
    }
}