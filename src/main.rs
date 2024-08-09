use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use rand::seq::SliceRandom;

fn main() {
    let phrases = vec![
        "Nice airtime! +200pts",
        "BIG DAMAGE! +1,000pts",
        "Utter destruction! +300pts",
    ];

    // println!("{}", phrases.choose(&mut rand::thread_rng()).unwrap());
    animated_print(phrases.choose(&mut rand::thread_rng()).unwrap());
}

fn animated_print(str: &str) {
    for char in str.chars() {
        print!("{char}");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(30))
    }
}
