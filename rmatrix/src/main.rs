use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};
use colored::Colorize;

fn main() {
    let mut rng = rand::rng();

    loop {
        let rand_num = rng.random_range(0..10);

        let mut word = String::new();

        for _ in 0..1 {

            let is_upper = rng.random_bool(0.5);

            let letter = if is_upper {
                rng.random_range(b'A'..=b'Z') as char
            } else {
                rng.random_range(b'a'..=b'z') as char
            };

            word.push(letter);
        }

        let rand_num = rand_num.to_string(); 

        print!("{} {}", rand_num.green(), word.green());
        io::stdout().flush().unwrap();
        
        sleep(Duration::from_micros(200));
    }
}
