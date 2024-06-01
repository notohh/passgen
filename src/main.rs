use args::Args;
use clap::Parser;
use core::panic;
use randomstring::RandomString;
use std::fs::File;
use std::io::{Error, Write};

mod args;
mod randomstring;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let random_string = RandomString;
    let string_output = random_string.generate_string();

    if args.write_to_file {
        let f = &mut File::create(args.file);
        let file = match f {
            Ok(file) => file.write_all(string_output.as_bytes()),
            Err(e) => panic!("cannot create file: {:?}", e),
        };
    } else {
        println!("{}", string_output);
    };

    Ok(())
}
