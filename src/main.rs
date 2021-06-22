mod parser;

use parser::{input, parse};

fn main() {
    println!("{}", parse(input(&String::from("enter some code... "))).print());
}
