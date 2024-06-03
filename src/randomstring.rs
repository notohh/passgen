use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};
use std::iter;

use crate::Args;

#[derive(Debug)]
pub struct RandomString;

impl RandomString {
    pub fn generate_string(&self) -> String {
        const CHARSET: &[u8] =
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";
        let args = Args::parse();
        let mut rng = rand::thread_rng();
        let char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;

        if args.symbols {
            let s: String = iter::repeat_with(char).take(args.length).collect();
            s
        } else {
            let s: String = rng
                .sample_iter(&Alphanumeric)
                .take(args.length)
                .map(char::from)
                .collect();
            s
        }
    }
}
