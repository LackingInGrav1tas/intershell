mod parser;
mod cmd;
mod tests;

use cmd::{Shell};

fn main() -> crossterm::Result<()> {
    // tests::test()?; // This enables the fancy rendering

    let mut shell = Shell::new();
    loop {
        shell.handle_command()
    }
    
    Ok(())
}
