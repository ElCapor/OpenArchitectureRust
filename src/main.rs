use std::path::PathBuf;

mod BlockParser;
mod TypeExtensions;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    path: std::path::PathBuf,
}


fn main() {
    let mut args = Cli::parse();
    println!("Passed file as argument, {:?}", args.path);
    let current_path = std::env::current_dir().unwrap();
    println!("Current directory : {}", current_path.display());
    let relative_path = current_path.to_str().unwrap().to_string() + args.path.to_str().unwrap();
    args.path = PathBuf::from(relative_path);
    let mut parser = BlockParser::BlockParser {
        blocks : Vec::new()
    };
    parser.new(args.path);
}
