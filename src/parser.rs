use std::fs::File;
use std::io::prelude::*;

use toml::Value;

pub struct Command {
    cmd: String,
    args: Vec<String>
}

#[allow(dead_code)]
impl Command {
    pub fn from(string: String) -> Self {
        // formats a string into a Command object, essentially seperating it into args and the base command
        let mut parts = parse(&string);
        Self {
            cmd: parts.remove(0),
            args: parts
        }
    }
    pub fn command(&self) -> String {
        self.cmd.clone()
    }

    pub fn arguements(&self) -> Vec<String> {
        self.args.clone()
    } 

    pub fn print(&mut self) -> String {
        let mut s = self.cmd.clone();
        for arg in &self.args {
            s += &(String::from(" ") + arg)
        }
        s
    }
}

pub struct TOMLCommand {
    method: String,
    args: Vec<String>
}

impl TOMLCommand {
    pub fn from(cmd: Command) -> Result<Self, ()> {
        // parses a command into a TOML command
        let arr = match get_toml_arr(cmd.command()) {
            Ok(a) => a,
            Err(()) => return Err(())
        };
        let file = String::from((*arr).get(1).expect("get(1)").as_str().expect("get(1) as_str"));
        Ok(Self {
            method: String::from((*arr).get(0).expect("get(0)").as_str().expect("get(0) as_str")),
            args: {
                let mut args: Vec<String> = vec![];
                let mut i = 1;
                loop {
                    i += 1;
                    if i == arr.len() {
                        break;
                    }
                    args.push(String::from(arr.get(i).unwrap().as_str().expect("additional arg should be a string")));
                }
                args.push(String::from("commands\\") + &file);
                args.append(& mut cmd.arguements());
                args
            }
        })
    }

    pub fn method(&self) -> String {
        self.method.clone()
    }

    pub fn args(&self) -> Vec<String> {
        self.args.clone()
    }
}

pub enum CommandType {
    CustomCommand(String),
    BuiltInCommand(String),
    CMDCall(String),
}

impl CommandType {
    pub fn from(src: String) -> Self {
        // parses string into a command type enum
        let cmd = parse(&src);
        if cmd.get(0).unwrap() == &"$" {
            CommandType::CMDCall(src)
        } else if vec![
            "cd", "dir", "mkdir", "md", "help", "exit", "quit", "attrib", "attribute",
            "cls", "clear", "del", "delete", "color", "comp", "compare", "copy",
            "echo", "erase", "find", "print", "rename", "rmdir", "ip", "ipconfig",
            "save", "saveenv", "savestate", "load", "loadenv", "loadstate", "history"
            ].contains(&cmd.get(0).unwrap().as_str()) {
            CommandType::BuiltInCommand(src)
        } else {
            CommandType::CustomCommand(src)
        }
    }
}

pub fn parse(command: &String) -> Vec<String> {
    let mut args = vec![];
    let mut current = String::new();
    let mut nested = false;
    for c in command.chars() {
        if ['$'].contains(&c) {
            // symbols
            if current.len() > 0 && !nested {
                args.push(current);
                current = String::new();
            }
            args.push(c.to_string());
        } else if c == '"' {
            nested = !nested;
            if !nested {
                // end quote
                args.push(current);
                current = String::new();
            }
        } else if c == ' ' {
            if nested {
                current.push(' ');
            } else if current.len() > 0 {
                args.push(current);
                current = String::new();
            }
        } else {
            current.push(c)
        }
    }
    if current.len() > 0 {
        args.push(current);
    }
    args
}


pub fn open_file(fname: &str) -> String {
    // gets file contents as a String
    let mut file = File::open(fname).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    contents
}

pub fn get_commands() -> Value {
    // returns the TOML table
    let contents = open_file("commands/commands.toml");
    contents.parse::<Value>().expect("couldn't parse TOML file")
}

pub fn get_toml_arr(name: String) -> Result<Vec<toml::Value>, ()> {
    // finds command array from toml file
    let cmd = match get_commands().get(name.clone()) {
        Some(v) => v,
        None => return Err(())
    }.clone();
    Ok(cmd.as_array().expect("improper TOML format").clone())
}