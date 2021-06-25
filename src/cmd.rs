use std::process::Command;
use std::io;
use std::env;
use std::path::Path;
use std::str;

use crate::parser::{parse, TOMLCommand, CommandType};

pub struct Shell {
    cwd: String,
    history: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        // initializes a Shell object
        Self {
            cwd: env::current_dir().expect("").into_os_string().into_string().expect(""),
            history: vec![]
        }
    }

    pub fn get_cwd(&self) -> String {
        // returns current working directory
        self.cwd.clone()
    }

    pub fn handle_command(&mut self, command: String) {
        // wrapper function which takes a command String as input and executes it
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
        // runs a custom (toml -> file) command
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
        // runs a built in command
        let args = command.split(" ").collect::<Vec<&str>>();
        match args.get(0).expect("no command given.").clone() {
            "cd" => {
                let newdir: String;
                self.cwd = String::from(
                    str::replace(
                        if Path::new(&(self.cwd.clone() + args.get(1).expect("expected more args for CD command"))).exists() {
                            newdir = self.cwd.clone() + args[1];
                            &newdir
                        } else if Path::new(args.get(1).expect("expected more args for CD command")).exists() {
                            args[1]
                        } else {
                            println!("directory {} does not exist.", args.get(1).expect("expected more args for CD command"));
                            return
                        },
                        "/", "\\"
                    )
                )
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
            _ => {
                println!("should not be reachable, value {:?}", command.to_lowercase().as_str());
                std::process::exit(0)
            }
        }
    }
}