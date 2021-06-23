mod parser;
mod cmd;
mod tests;

use std::env;

use parser::{input, parse, get_commands};
use cmd::run_command;

fn main() -> crossterm::Result<()> {
    // tests::test()?; // This enables the fancy rendering

    let dir = env::current_dir().expect("").into_os_string().into_string().expect("");

    println!("{}> {}", dir, parse(
        input( &String::from("enter some code... ") )
    ).print());

    let cmd = get_commands()["m"].clone();
    let cmd_info = cmd.as_array().expect("improper TOML format");

    let method = (*cmd_info).get(0).unwrap().as_str().unwrap();
    let file = (*cmd_info).get(1).unwrap().as_str().unwrap();

    run_command(method, vec![String::from("commands\\") + file]).unwrap();
    Ok(())
}
