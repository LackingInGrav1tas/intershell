use std::io;
use std::io::Write;

use crossterm::QueueableCommand;
use crossterm::Result;

use crossterm::{execute, queue};
use crossterm::{terminal, cursor};
use crossterm::event::{self, Event};
use crossterm::style::{self, Color};

pub fn test() -> Result<()> {
    let mut renderer = Renderer::new();
    renderer.test()?;

    Ok(())
}

pub struct Renderer {

    stdout: io::Stdout

}

impl Renderer {

    pub fn new() -> Renderer {
        let stdout = io::stdout();
        Renderer { stdout }
    }


    pub fn test(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(&self.stdout,
            terminal::EnterAlternateScreen,
            terminal::SetTitle("Intershell"),
            cursor::Hide
        )?;
    
        let mut text = String::from("> ");
        execute!(&self.stdout, style::Print(&text))?;

        loop {
            match event::read()? {
                Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Char(c) => {
                            text.push(c);
                            execute!(&self.stdout, cursor::MoveTo(0, 0), style::Print(&text))?;
                        },
                        _ => ()
                    }
                },
                Event::Resize(width, height) => self.draw(width, height)?,
                _ => ()
            }
        }
    }
    
    fn draw(&mut self, width: u16, height: u16) -> Result<()> {
        for x in 0..=width {
            queue!(&self.stdout,
                cursor::MoveTo(x, height),
                style::SetBackgroundColor(Color::Blue),
                style::Print(" ")
            )?;
        }
        self.stdout.queue(style::ResetColor)?;

        self.stdout.flush()?;
        Ok(())
    }

}
