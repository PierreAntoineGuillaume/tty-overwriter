use std::thread::sleep;
use std::time::{Duration, SystemTime};
use tty_overwriter::prelude::*;

const AWAIT_TIME: Duration = Duration::from_millis(5);

fn do_print(spinner: &Spinner) {
    let char = spinner.current();
    println!("fmt             cargo fmt {char}");
    println!("clippy          cargo clippy {char}");
    println!("node modules    npm install {char}");
    println!("analyse         long process {char}");
}

fn redraw(spinner: &Spinner) {
    let looper = spinner.current();
    let first = Movement::new().up(4).right(26);
    let second = Movement::new().down(1).right(2);
    let third = Movement::new().down(1).left(2);
    let fourth = Movement::new().down(1);
    let last = AnsiSeq::MoveLines { up: 0, down: 1 };
    print!("{first}{looper}{second}{looper}{third}{looper}{fourth}{looper}{last}");
}

fn main() {
    let mut last_occurence = SystemTime::now();
    let mut elapsed = 0 as usize;
    let mut spinner = Spinner::default();
    do_print(&spinner);
    let hide = AnsiSeq::ShowAndHideCursor { show: false };
    let show = AnsiSeq::ShowAndHideCursor { show: true };
    print!("{hide}");
    loop {
        redraw(&spinner);

        let should_sleep_duration = AWAIT_TIME
            .checked_sub(last_occurence.elapsed().unwrap())
            .unwrap_or_default();

        elapsed = should_sleep_duration.as_millis() as usize;
        if elapsed != 0 {
            sleep(should_sleep_duration);
        }
        last_occurence = SystemTime::now();
        spinner.tick(elapsed)
    }
    print!("{show}");
}

const FRAMES: [&str; 8] = ["⡏", "⠟", "⠻", "⢹", "⣸", "⣴", "⣦", "⣇"];

#[derive(Clone, Default)]
pub struct Spinner {
    ticks: usize,
    current_frame: usize,
}

impl Spinner {
    pub fn current(&self) -> &'static str {
        &FRAMES[self.ticks]
    }
    pub fn tick(&mut self, millis: usize) {
        let up = self.current_frame + millis >= 40;
        self.ticks = if up {
            (self.ticks + 1) % FRAMES.len()
        } else {
            self.ticks
        };
        self.current_frame = if up { 0 } else { self.current_frame + millis };
    }
}
