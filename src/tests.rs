use std::io;
use std::io::Write;

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, event::{self, Event}, style::{self, Color}, Result
};

pub fn test() -> Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(cursor::Hide)?;

    let mut col = 1;
    draw(&mut stdout)?;

    loop {
        match event::read()? {
            Event::Key(event) => {
                match event.code {
                    event::KeyCode::Char(c) => {
                        stdout
                            .execute(cursor::MoveTo(col, 1))?
                            .execute(style::SetBackgroundColor(Color::Reset))?
                            .execute(style::Print(c))?;
                        col += 1;
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

fn draw(stdout: &mut io::Stdout) -> Result<()> {
    let (cols, rows) = terminal::size()?;

    for y in 0..rows {
        for x in 0..cols {
            stdout.queue(cursor::MoveTo(x, y))?;

            let color;
            if (y == 0 || y == rows - 1) || (x == 0 || x == cols - 1) {
                color = Color::Rgb { r: 255, g: 0, b: 0 };
            } else {
                color = Color::Reset;
            }

            stdout
                .queue(style::SetBackgroundColor(color))?
                .queue(style::Print(" "))?;
        }
    }

    stdout.flush()?;
    Ok(())
}
