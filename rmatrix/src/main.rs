use rand::Rng;

fn main() {
    loop {
        let rand_num = rand::thread_rng().gen_range(0..=9);

        print!("{rand_num} ");
    }
}
