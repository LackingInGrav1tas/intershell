mod parser;

use parser::{input, parse, get_commands, open_file};

fn main() {
    println!("{}", parse(
        input( &String::from("enter some code... ") )
    ).print());

    let cmd = get_commands()["m"].clone();
    let file = cmd.as_str().expect("improper TOML format");

    println!("{}", open_file(&(String::from("commands/") + file)))
}
