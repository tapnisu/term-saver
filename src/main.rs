mod cli;

use clap::Parser;
use cli::Cli;
use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

fn main() -> Result<(), clap::Error> {
    let cli = Cli::parse();

    let mut stdout = io::stdout();

    let text = cli.text.as_str();
    let text_length = text.chars().count() as u16;

    let mut moving_right = true;
    let mut moving_top = false;

    let mut x = 1;
    let mut y = 1;

    stdout
        .execute(cursor::Hide)?
        .execute(terminal::EnterAlternateScreen)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let sleep_duration = Duration::new(0, 1_000_000_000 / cli.moves_per_second);

    while running.load(Ordering::SeqCst) {
        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;

        let (width, height) = crossterm::terminal::size()?;

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
            .queue(style::Print(text))?
            .flush()?;

        sleep(sleep_duration);
    }

    stdout
        .execute(cursor::Show)?
        .execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}
