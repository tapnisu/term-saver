use crossterm::{
    cursor::{self, RestorePosition, SavePosition},
    style, terminal, ExecutableCommand, QueueableCommand,
};
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

const TEXT: &str = "Blazingly fast";

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let text_length = TEXT.chars().count() as u16;
    let mut moving_right = true;
    let mut moving_top = false;

    let mut x = 20;
    let mut y = 20;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(SavePosition)?;

    loop {
        let (width, height) = crossterm::terminal::size()?;

        stdout.queue(RestorePosition)?;
        stdout.queue(SavePosition)?;

        if moving_right {
            x += 1;
        } else {
            x -= 1;
        }

        if moving_top {
            y -= 1;
        } else {
            y += 1;
        }

        if x == width - text_length || x == 1 {
            moving_right = !moving_right;
        }

        if y == height || y == 1 {
            moving_top = !moving_top;
        }

        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::Print(TEXT))?;

        stdout.flush()?;

        sleep(Duration::new(0, 25000000));
    }

    Ok(())
}
