use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();

    println!("[a]: Add message to queue. [q]: Quit.");

    let phrases = [
        "Nice airtime! +200pts",
        "MASSIVE DAMAGE! +1,000pts",
        "Utter destruction! +300pts",
        "Pedestrian eliminated! +50pts",
        "Stop sign demolished! +70pts",
        "Bicyclist hospitalized! +600pts",
    ];

    let (print_tx, print_rx) = std::sync::mpsc::channel::<&str>();

    // Creates a printer manager thread.
    // This is in charge of the cancel mechanics
    thread::spawn(move || {
        let mut print_rx_iter = print_rx.iter().peekable();

        loop {
            while let Some(msg) = print_rx_iter.next() {
                animated_print(msg);

                let start = std::time::Instant::now();

                let (fade_tx, fade_rx) = std::sync::mpsc::channel::<bool>();
                thread::sleep(Duration::from_millis(400));
                thread::spawn(|| animated_unprint(msg, fade_rx));

                while start.elapsed() < Duration::from_millis(900) {
                    if print_rx_iter.peek().is_some() {
                        let _ = fade_tx.send(true);
                        break;
                    }
                }
            }
        }
    });

    loop {
        if crossterm::event::poll(Duration::from_millis(30)).unwrap() {
            if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                if key.kind == crossterm::event::KeyEventKind::Press
                    && key.code == crossterm::event::KeyCode::Char('a')
                {
                    let phrase = phrases.choose(&mut rand::thread_rng()).unwrap();
                    print_tx.send(phrase).unwrap();
                }
                if key.kind == crossterm::event::KeyEventKind::Press
                    && key.code == crossterm::event::KeyCode::Char('q')
                    || key.code == crossterm::event::KeyCode::Char('Q')
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

        thread::sleep(Duration::from_millis(27));
    }
}

fn animated_unprint<T>(str: &str, sigint_rx: Receiver<T>) {
    for i in (0..str.len()).rev() {
        if sigint_rx.try_recv().is_ok() {
            print!("\r{}\r", " ".repeat(str.len()));
            io::stdout().flush().unwrap();
            return;
        }

        let slice = &str[..i];
        print!("\r{slice} ");
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(27))
    }
}
