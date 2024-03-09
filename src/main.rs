use crossterm::{
    cursor,
    style::{self},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

const TEXT: &str = "Blazingly fast";

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let text_length = TEXT.chars().count() as u16;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let (width, height) = crossterm::terminal::size()?;

    stdout
        .queue(cursor::MoveTo(width - text_length, height))?
        .queue(style::Print(TEXT))?;

    stdout.flush()?;
    Ok(())
}
