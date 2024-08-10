use crossterm::event;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone)]
struct Alert<'a> {
    msg: &'a str,
    points: u32,
}

impl std::fmt::Display for Alert<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} +{}pts", self.msg, self.points)
    }
}

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();

    println!("[a]: Add a random message to queue. [q]: Quit.");

    let alerts = [
        Alert {
            msg: "Yard sign obliterated!",
            points: 20,
        },
        Alert {
            msg: "Mailbox flattened!",
            points: 50,
        },
        Alert {
            msg: "Thirteen car pileup!",
            points: 2000,
        },
        Alert {
            msg: "Pedestrian eliminated!",
            points: 50,
        },
        Alert {
            msg: "Stop sign demolished!",
            points: 70,
        },
        Alert {
            msg: "MILLIONS DEAD!",
            points: 10000,
        },
        Alert {
            msg: "HUNDREDS INJURED!",
            points: 5000,
        },
        Alert {
            msg: "White-owned business destroyed!",
            points: 700,
        },
        Alert {
            msg: "T-boned a school bus!",
            points: 800,
        },
        Alert {
            msg: "Bicyclist hospitalized!",
            points: 600,
        },
    ];

    let (print_tx, print_rx) = std::sync::mpsc::channel::<Alert>();

    // Creates a printer manager thread.
    // This is in charge of the cancel mechanics
    thread::spawn(move || {
        // A `for` loop makes peeking difficult.
        // We'll use a `while` loop instead, taking ownership of each item
        // individually rather than the entire iterator.
        let mut print_rx_iter = print_rx.iter().peekable();

        loop {
            while let Some(alert) = print_rx_iter.next() {
                animated_print(&alert);

                // Transmitter/receiver for optionally cancelling the
                // fade out animation.
                let (fade_tx, fade_rx) = std::sync::mpsc::channel::<bool>();

                // Linger on the completed message before fading out.
                thread::sleep(Duration::from_millis(460));

                let fade_out = thread::spawn(move || animated_unprint(alert, fade_rx));

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

    let mut allocated_points = 0;

    loop {
        if event::poll(Duration::from_millis(30)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == event::KeyEventKind::Press && key.code == event::KeyCode::Char('a') {
                    let alert = alerts.choose(&mut rand::thread_rng()).unwrap();
                    allocated_points += alert.points;
                    print_tx.send(*alert).unwrap();
                }
                if key.kind == event::KeyEventKind::Press && key.code == event::KeyCode::Char('q')
                    || key.code == event::KeyCode::Char('Q')
                {
                    break;
                }
            }
        }
    }

    println!("You earned a total of {allocated_points} points!");

    crossterm::execute!(std::io::stdout(), crossterm::cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}

fn animated_print(alert: &Alert) {
    print!("\r");
    io::stdout().flush().unwrap();
    // '\r' doesn't always reset the cursor properly, I guess.
    // position() resets it reliably!
    crossterm::cursor::position().unwrap();

    for char in alert.to_string().chars() {
        // print single character and display it
        print!("{char}");
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(15));
    }
}

// Repeatedly replaces the last character with a space, "erasing"
// the entire message.
fn animated_unprint<T>(alert: Alert, sigint_rx: Receiver<T>) {
    for i in (0..alert.to_string().len()).rev() {
        // Cancel the [remaining] animation if the receiver detects a signal.
        if sigint_rx.try_recv().is_ok() {
            print!("\r{}\r", " ".repeat(alert.to_string().len()));
            io::stdout().flush().unwrap();
            return;
        }

        let slice = &alert.to_string()[..i];
        print!("\r{slice} ");
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(15));
    }
}
