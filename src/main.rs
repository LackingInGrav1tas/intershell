mod parser;
mod cmd;
// mod tests;

use std::env;

use parser::{input, parse, get_commands};
use cmd::run_command;

fn main() -> crossterm::Result<()> {
    // tests::test()?; // This enables the fancy rendering

    let dir = env::current_dir().expect("").into_os_string().into_string().expect("");
    let inp = input( &(dir + "::>") );

    let cmd = get_commands().get(inp.clone()).expect(&format!("could not find [{}] in toml", inp)).clone();
    let cmd_info = cmd.as_array().expect("improper TOML format");

    let method = (*cmd_info).get(0).expect("get(0)").as_str().expect("get(0) as_str");
    let file = (*cmd_info).get(1).expect("get(1)").as_str().expect("get(1) as_str");

    let mut args: Vec<String> = vec![];
    let mut i = 1;
    loop {
        i += 1;
        if i == cmd_info.len() {
            break;
        }
        args.push(String::from(cmd_info.get(i).unwrap().as_str().expect("additional arg should be a string")));
    }
    args.push(String::from("commands\\") + file);

    run_command(method, args).unwrap();
    Ok(())
}
