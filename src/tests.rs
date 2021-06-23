use std::env;
use std::io;

use crossterm::Result;

use crossterm::execute;
use crossterm::{terminal, cursor};
use crossterm::event::{self, Event};
use crossterm::style::{self, Color};

pub fn test() -> Result<()> {
    let mut renderer = Renderer::new();
    renderer.test()?;

    Ok(())
}

pub struct Renderer {

    stdout: io::Stdout,
    content: String

}

impl Renderer {

    pub fn new() -> Renderer {
        let stdout = io::stdout();
        let content = Renderer::path();

        Renderer { stdout, content }
    }

    fn path() -> String {
        let dir = env::current_dir().expect("");
        String::from(format!("{}> ", dir.display()))
    }


    pub fn test(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(&self.stdout,
            terminal::EnterAlternateScreen,
            terminal::SetTitle("Intershell")
        )?;

        loop {
            match event::read()? {
                Event::Key(event) => self.handle_key(event)?,
                Event::Resize(width, height) => {
                    self.draw(width, height)?;
                    execute!(&self.stdout, cursor::MoveTo(0, 0), style::Print(&self.content))?;
                },
                _ => ()
            }
        }
    }

    fn handle_key(&mut self, event: event::KeyEvent) -> Result<()> {
        match event.code {
            event::KeyCode::Char(c) => {
                self.content.push(c);
                execute!(&self.stdout, cursor::MoveTo(0, 0), style::Print(&self.content))?;
            },
            event::KeyCode::Enter => {
                self.content = Renderer::path();
                
                execute!(&self.stdout,
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    cursor::MoveTo(0, 0),
                    style::Print(&self.content)
                )?;
            },
            _ => ()
        }
        Ok(())
    }
    
    fn draw(&mut self, width: u16, height: u16) -> Result<()> {
        execute!(&self.stdout,
            cursor::MoveTo(0, height),
            style::SetBackgroundColor(Color::Blue),
            style::Print((0..=width).map(|_| " ").collect::<String>()),
            style::ResetColor
        )?;
        Ok(())
    }

}
