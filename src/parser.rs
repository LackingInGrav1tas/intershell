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
    pub fn print(&mut self) -> String {
        let mut s = self.cmd.clone();
        for arg in &self.args {
            s += &(String::from(" ") + arg)
        }
        s
    }
}

pub fn input(message: &String) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Could not read input.");
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