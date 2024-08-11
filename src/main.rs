use std::{error::Error, fs::read_to_string, path::PathBuf};

use clap::Parser;

mod align_comments;

#[derive(Parser)]
struct Args {
    /// Path to file
    #[arg(short, long)]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = args.path;
    let file = read_to_string(path)?;

    let result = align_comments::align_comments(file);

    for line in result {
        println!("{}", line);
    }

    Ok(())
}
