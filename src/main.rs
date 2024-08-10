use crossterm::event;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();

    println!("[a]: Add a random message to queue. [q]: Quit.");

    let alerts = [
        "Yard sign obliterated! +20pts",
        "Mailbox flattened! +50pts",
        "Thirteen car pileup! +2,000pts",
        "Pedestrian eliminated! +50pts",
        "Stop sign demolished! +70pts",
        "MILLIONS DEAD! +10,000pts",
        "HUNDREDS INJURED! +5,000pts",
        "Skateboarder injured! +400pts",
        "Speed limit ignored! +10pts",
        "White-owned business destroyed! +700pts",
        "T-boned a school bus! +800pts",
        "Bicyclist hospitalized! +600pts",
    ];

    let (print_tx, print_rx) = std::sync::mpsc::channel::<&str>();

    // Creates a printer manager thread.
    // This is in charge of the cancel mechanics
    thread::spawn(move || {
        // A `for` loop makes peeking difficult.
        // We'll use a `while` loop instead, taking ownership of each item
        // individually rather than the entire iterator.
        let mut print_rx_iter = print_rx.iter().peekable();

        loop {
            while let Some(msg) = print_rx_iter.next() {
                animated_print(msg);

                // Transmitter/receiver for optionally cancelling the
                // fade out animation.
                let (fade_tx, fade_rx) = std::sync::mpsc::channel::<bool>();

                // Linger on the completed message before fading out.
                thread::sleep(Duration::from_millis(560));

                let fade_out = thread::spawn(|| animated_unprint(msg, fade_rx));

                while !fade_out.is_finished() {
                    // If there is a message in the queue, cancel the current
                    // fade out animation.
                    if print_rx_iter.peek().is_some() {
                        let _ = fade_tx.send(true); // eat the Result
                        break;
                    }
                }
            }
        }
    });

    loop {
        if event::poll(Duration::from_millis(30)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == event::KeyEventKind::Press && key.code == event::KeyCode::Char('a') {
                    let alert = alerts.choose(&mut rand::thread_rng()).unwrap();
                    print_tx.send(alert).unwrap();
                }
                if key.kind == event::KeyEventKind::Press && key.code == event::KeyCode::Char('q')
                    || key.code == event::KeyCode::Char('Q')
                {
                    break;
                }
            }
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}

fn animated_print(str: &str) {
    print!("\r");
    io::stdout().flush().unwrap();
    // '\r' doesn't always reset the cursor properly, I guess.
    // position() resets it reliably!
    crossterm::cursor::position().unwrap();

    for char in str.chars() {
        // print single character and display it
        print!("{char}");
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(15));
    }
}

// Repeatedly replaces the last character with a space, "erasing"
// the entire message.
fn animated_unprint<T>(str: &str, sigint_rx: Receiver<T>) {
    for i in (0..str.len()).rev() {
        // Cancel the [remaining] animation if the receiver detects a signal.
        if sigint_rx.try_recv().is_ok() {
            print!("\r{}\r", " ".repeat(str.len()));
            io::stdout().flush().unwrap();
            return;
        }

        let slice = &str[..i];
        print!("\r{slice} ");
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(15));
    }
}
