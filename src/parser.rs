use std::fs::File;
use std::io::prelude::*;

use toml::Value;

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