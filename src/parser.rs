use std::io;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;

use toml::Value;

pub struct Command {
    cmd: String,
    args: Vec<String>
}

impl Command {
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
    file: String,
    args: Vec<String>
}

impl TOMLCommand {
    pub fn from(cmd: Command) -> Self {
        let arr = get_toml_arr(cmd.command());
        let file = String::from((*arr).get(1).expect("get(1)").as_str().expect("get(1) as_str"));
        Self {
            method: String::from((*arr).get(0).expect("get(0)").as_str().expect("get(0) as_str")),
            file: file.clone(),
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
        }
    }

    pub fn method(&self) -> String {
        self.method.clone()
    }

    pub fn args(&self) -> Vec<String> {
        self.args.clone()
    }
}

pub fn input(message: &String) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Could not read input.");
    line.pop();
    if line.chars().last().unwrap() == '\r' {
        line.pop();
    }
    line
}

pub fn parse(string: String) -> Command {
    let mut parts = string.split(" ").collect::<Vec<&str>>();
    Command {
        cmd: {
            let c = *parts.get_mut(0).expect("expected a command");
            parts.remove(0);
            String::from(c)
        },
        args: {
            let mut arg_vec = vec![];
            for item in parts {
                arg_vec.push(String::from(item))
            }
            arg_vec
        }
    }
}

pub fn open_file(fname: &str) -> String {
    let mut file = File::open(fname).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    contents
}

pub fn get_commands() -> Value {
    let contents = open_file("commands/commands.toml");
    contents.parse::<Value>().expect("couldn't parse TOML file")
}

pub fn get_toml_arr(name: String) -> Vec<toml::Value> {
    let cmd = get_commands().get(name.clone()).expect(&format!("could not find [{}] in toml", name)).clone();
    cmd.as_array().expect("improper TOML format").clone()
}

pub enum CommandType {
    CustomCommand(String),
    BuiltInCommand(String),
}

impl CommandType {
    pub fn from(src: String) -> Self {
        let cmd = src.split(" ").collect::<Vec<&str>>();
        if vec![].contains(cmd.get(0).unwrap()) {
            CommandType::BuiltInCommand(src)
        } else {
            CommandType::CustomCommand(src)
        }
    }
}