use rand::seq::SliceRandom;

fn main() {
    let phrases = vec![
        "Nice airtime! +200pts",
        "BIG DAMAGE! +1,000pts",
        "Utter destruction! +300pts",
    ];

    println!("{}", phrases.choose(&mut rand::thread_rng()).unwrap());
}
