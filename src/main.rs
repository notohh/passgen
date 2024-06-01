use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short)]
    length: usize,
}

#[derive(Debug)]
struct RandomString;

impl RandomString {
    fn generate_string(&self) -> String {
        let args = Args::parse();
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(args.length)
            .map(char::from)
            .collect();
        s
    }
}

fn main() {
    let random_string = RandomString;
    let string_output = random_string.generate_string();

    println!("{}", string_output);
}
