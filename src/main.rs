mod cli;

use clap::{error::ErrorKind, CommandFactory, Parser};
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

fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    if let Err(err) = term_saver(&cli.text, cli.moves_per_second) {
        cmd.error(ErrorKind::Io, err).exit()
    }
}

fn term_saver(text: &str, moves_per_second: u32) -> Result<(), io::Error> {
    let mut stdout = io::stdout();

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

    let _ = ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    });

    let sleep_duration = Duration::new(0, 1_000_000_000 / moves_per_second);

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
