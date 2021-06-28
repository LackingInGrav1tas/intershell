use std::process::Command;
use std::io;
use std::env;
use std::path::Path;
use std::str;
use std::fs;

macro_rules! needs_rendering {
    () => {
        println!("needs re-rendering")
    };
}

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
                let mut c = s.chars();
                c.next();
                self.run_builtin_command(String::from(c.as_str()))
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
        cmd.current_dir(&self.get_cwd());
        cmd.status()
    }

    fn run_cmd_command(&mut self, args: &mut Vec<&str>) {
        // runs a cmd command
        let mut cmd = Command::new(args[0]);
        args.remove(0);
        self.history.push(format!("{:?}", cmd));
        for a in args {
            cmd.arg(a);
        }
        cmd.current_dir(&self.get_cwd());
        println!("running {:?}", cmd);
        cmd.status().unwrap();
    }

    fn run_builtin_command(& mut self, command: String) {
        // runs a built in command
        let mut args = command.split(" ").collect::<Vec<&str>>();
        // args.remove(0);
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
                println!("CONTENTS OF {}", self.get_cwd());
                let paths = fs::read_dir(self.get_cwd()).unwrap();
                for path in paths {
                    let file_path = path.unwrap().path();
                    let metadata = match fs::metadata(file_path.clone()) {
                        Ok(data) => data.len(),
                        Err(_) => 0,
                    };
                    println!("{} - {} bytes", file_path.display().to_string().replace(&self.get_cwd(), "."), metadata)
                }
                println!();
            }
            "md" | "mkdir" => {
                fs::create_dir_all(self.get_cwd() + args.get(1).unwrap()).unwrap();
            }
            "del" | "delete" | "erase" => {
                match fs::remove_file(self.get_cwd() + args.get(1).unwrap()) {
                    Ok(_) => (),
                    Err(_) => {
                        fs::remove_dir(self.get_cwd() + args.get(1).unwrap()).unwrap()
                    }
                }
            }
            "cls" | "clear" => {
                // std::process::Command::new("cls").status().unwrap();
                print!("{}[2J", 27 as char);
                needs_rendering!();
            }
            "exit" | "quit" => {
                std::process::exit(0)
            }
            "color" => {

            }
            "comp" | "compare" => {
                args[0] = "comp";
                self.run_cmd_command(& mut args)
            }
            "copy" => {
                self.run_cmd_command(& mut args)
            }
            "echo" => {
                let mut c = std::process::Command::new("cmd");
                c.arg("/c");
                c.arg("echo");
                c.arg(args.get(1).expect("expected an additional arg")); 
                println!("{:?}", c);
                c.status().unwrap();
            }
            "find" => {
                self.run_cmd_command(& mut args)
            }
            "print" => {
                self.run_cmd_command(& mut args)
            }
            "rename" => {
                self.run_cmd_command(& mut args)
            }
            "rmdir" => {
                self.run_cmd_command(& mut args)
            }
            "ip" | "ipconfig" => {
                args[0] = "ipconfig";
                self.run_cmd_command(& mut args)
            }
            com => {
                println!("should not be reachable, value {:?}", com);
                std::process::exit(0)
            }
        }
    }
}