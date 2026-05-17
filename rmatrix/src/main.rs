use rand::Rng;

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

        print!("{} {}", rand_num, word);
    }
}
