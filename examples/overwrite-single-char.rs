use tty_overwriter::prelude::*;

fn main() {
    println!("Printing 0 once !");
    println!("Next time it will be replaced by a 1");
    println!("Printing 0 once !");
    println!(
        "{}1",
        AnsiSeq::Move {
            up: 1,
            down: 0,
            left: 0,
            right: 9
        }
    )
}
