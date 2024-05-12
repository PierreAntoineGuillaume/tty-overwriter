use std::thread::sleep;
use std::time::{Duration, SystemTime};
use tty_overwriter::prelude::*;

const AWAIT_TIME: Duration = Duration::from_millis(5);

fn do_print(spinner: &Spinner) -> String {
    let char = spinner.current();
    format!(
        "fmt             cargo fmt {char}
clippy          cargo clippy {char}
node modules    npm install {char}
analyse         long string {} {char}",
        " ".repeat(80)
    )
}

fn main() -> std::io::Result<()> {
    let mut last_occurence = SystemTime::now();
    let mut spinner = Spinner::default();
    let hide = AnsiSeq::ShowAndHideCursor { show: false };
    print!("{hide}");
    let _ = ctrlc::set_handler(|| {
        print!(
            "{}{}",
            AnsiSeq::ShowAndHideCursor { show: true },
            AnsiSeq::MoveLines { down: 1, up: 0 }
        );
        std::process::exit(0);
    });

    let mut body = Body::default();

    loop {
        let (width, _) = term_size::dimensions().expect("it has dimensions");
        body.overwrite(&do_print(&spinner), &mut std::io::stdout(), width)?;

        let should_sleep_duration = AWAIT_TIME
            .checked_sub(last_occurence.elapsed().unwrap())
            .unwrap_or_default();

        let elapsed = should_sleep_duration.as_millis() as usize;
        if elapsed != 0 {
            sleep(should_sleep_duration);
        }
        last_occurence = SystemTime::now();
        spinner.tick(elapsed)
    }
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
