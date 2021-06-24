mod parser;
mod cmd;
mod renderer;

use renderer::Renderer;

fn main() -> crossterm::Result<()> {
    let mut renderer = Renderer::new();
    renderer.start()?;

    Ok(())
}
