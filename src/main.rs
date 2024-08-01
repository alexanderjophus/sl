mod train;

use std::io::stdout;
use std::{thread, time};

use clap::{arg, command, Parser};
use crossterm::terminal::{DisableLineWrap, EnableLineWrap};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{size, Clear, ClearType},
};

use train::{COAL_COLLECTION, LOCO_COLLECTION, WHEEL_COLLECTION};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of carriages to display
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let (width, height) = size().expect("Unable to get terminal size");
    execute!(stdout(), Hide, DisableLineWrap).expect("Unable to hide cursor");
    for x in (0..width).rev() {
        execute!(stdout(), Clear(ClearType::All)).expect("Unable to clear screen");
        for i in 0..7 {
            execute!(stdout(), MoveTo(x, height - 10 + i)).expect("Unable to move cursor");
            execute!(stdout(), Print(LOCO_COLLECTION[i as usize]),).expect("Unable to print");
            for _ in 0..args.count {
                execute!(stdout(), Print(COAL_COLLECTION[i as usize]),).expect("Unable to print");
            }
        }
        for i in 0..3 {
            execute!(stdout(), MoveTo(x, height - 3 + i)).expect("Unable to move cursor");
            execute!(
                stdout(),
                Print(WHEEL_COLLECTION[i as usize][(x % 3) as usize]),
            )
            .expect("Unable to print");
            for _ in 0..args.count {
                execute!(stdout(), Print(COAL_COLLECTION[(i + 7) as usize]))
                    .expect("Unable to print");
            }
        }
        thread::sleep(time::Duration::from_millis(60));
    }
    execute!(stdout(), Clear(ClearType::All)).expect("Unable to clear screen");
    execute!(stdout(), Show, EnableLineWrap).expect("Unable to show cursor");
    Ok(())
}
