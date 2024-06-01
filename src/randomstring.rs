use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};

use crate::Args;

#[derive(Debug)]
pub struct RandomString;

impl RandomString {
    pub fn generate_string(&self) -> String {
        let args = Args::parse();
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(args.length)
            .map(char::from)
            .collect();
        s
    }
}
