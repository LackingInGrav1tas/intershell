use std::env;
use std::io;

use crossterm::Result;
use crossterm::ExecutableCommand;

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

    cursor: usize,

    path: String,
    content: String

}

impl Renderer {

    pub fn new() -> Renderer {
        let stdout = io::stdout();

        let dir = env::current_dir().expect("");
        let path = dir.display().to_string();

        Renderer { stdout, cursor: 0, path, content: String::from("") }
    }


    pub fn test(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(&mut self.stdout,
            terminal::EnterAlternateScreen,
            terminal::SetTitle("Intershell")
        )?;

        loop {
            match event::read()? {
                Event::Key(event) => self.handle_key(event)?,
                Event::Resize(width, height) => {
                    self.draw(width, height)?;

                    self.stdout.execute(cursor::MoveToRow(0))?;
                    self.render_content()?;
                },
                _ => ()
            }
        }
    }

    fn handle_key(&mut self, event: event::KeyEvent) -> Result<()> {
        match event.code {
            event::KeyCode::Char(c) => {
                self.content.insert(self.cursor, c);
                self.cursor += 1;
                
                self.render_content()?;
            },
            event::KeyCode::Left => {
                if self.cursor > 0 {
                    self.stdout.execute(cursor::MoveLeft(1))?;
                    self.cursor -= 1;
                }
            },
            event::KeyCode::Right => {
                if self.cursor < self.content.len() {
                    self.stdout.execute(cursor::MoveRight(1))?;
                    self.cursor += 1;
                }
            },
            event::KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.content.remove(self.cursor);
                    
                    self.render_content()?;
                }
            },
            event::KeyCode::Enter => {
                print!("\n{}", self.content);

                self.cursor = 0;
                self.content.clear();

                self.stdout.execute(cursor::MoveDown(1))?;
                self.render_content()?;
            },
            _ => ()
        }
        Ok(())
    }

    fn render_content(&self) -> Result<()> {
        let col = self.path.len() + 3;

        execute!(io::stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print(format!("{}> {}", self.path, self.content)),
            cursor::MoveToColumn((col + self.cursor) as u16)
        )?;
        Ok(())
    }

    fn draw(&mut self, width: u16, height: u16) -> Result<()> {
        let mut footer = String::from("[intershell]");
        for _ in footer.len() as u16..=width {
            footer.push(' ');
        }

        execute!(io::stdout(),
            cursor::MoveTo(0, height),
            style::SetBackgroundColor(Color::Blue),
            style::SetForegroundColor(Color::White),
            style::Print(footer),
            style::ResetColor
        )?;
        Ok(())
    }

}
