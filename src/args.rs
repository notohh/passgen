use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub(crate) length: usize,
    #[arg(short)]
    pub write_to_file: bool,
    #[arg(short, default_value_t = String::from("generated_pwd.txt"))]
    pub file: String,
}
