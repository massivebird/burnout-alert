use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();

    let phrases = vec![
        "Nice airtime! +200pts",
        "BIG DAMAGE! +1,000pts",
        "Utter destruction! +300pts",
        "Pedestrian eliminated! +50pts",
        "Stop sign demolished! +70pts",
    ];

    let (tx, rx) = std::sync::mpsc::channel::<&str>();

    thread::spawn(move || {
        'outer: for msg in &rx {
            animated_print(msg);
            let start = std::time::Instant::now();
            let (tx, rx) = std::sync::mpsc::channel::<bool>();
            thread::sleep(Duration::from_millis(400));
            animated_unprint(msg, rx);
            // while start.elapsed() < Duration::from_millis(900) {
            //     if let Ok(a) = rx.recv_timeout(Duration::from_millis(900)) {
            //         print!("Found {a}\r");
            //         continue 'outer;
            //     }
            // }
        }
    });

    loop {
        let print_this = phrases.choose(&mut rand::thread_rng()).unwrap();
        tx.send(print_this).unwrap();

        if crossterm::event::poll(Duration::from_millis(30)).unwrap() {
            if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
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
    for char in str.chars() {
        print!("{char}");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(27))
    }
}

fn animated_unprint<T>(str: &str, rx: Receiver<T>) {
    for i in (1..str.len()).rev() {
        io::stdout().flush().unwrap();
        let slice = &str[..i];
        print!("\r{slice} ");
        thread::sleep(Duration::from_millis(27))
    }

    print!("\r \r"); // clear last character and place cursor at pos 0
}
