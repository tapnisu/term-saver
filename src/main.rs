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

fn main() {
    let cli = Cli::parse();

    let mut stdout = io::stdout();

    let text = cli.text.as_str();
    let text_length = text.chars().count() as u16;

    let mut moving_right = true;
    let mut moving_top = false;

    let mut x = 1;
    let mut y = 1;

    stdout
        .execute(cursor::Hide)
        .unwrap()
        .execute(terminal::EnterAlternateScreen)
        .unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        stdout
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();

        let (width, height) = crossterm::terminal::size().unwrap();

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
            .queue(cursor::MoveTo(x, y))
            .unwrap()
            .queue(style::Print(text))
            .unwrap()
            .flush()
            .unwrap();

        sleep(Duration::new(0, 25000000));
    }

    stdout
        .execute(cursor::Show)
        .unwrap()
        .execute(terminal::LeaveAlternateScreen)
        .unwrap();
}
