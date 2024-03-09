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

const TEXT: &str = "Blazingly fast";

fn main() {
    let mut stdout = io::stdout();

    let text_length = TEXT.chars().count() as u16;
    let mut moving_right = true;
    let mut moving_top = false;

    let mut x = 20;
    let mut y = 20;

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
            .queue(style::Print(TEXT))
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
