use std::{path::PathBuf, fs};

use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Args {
    input: PathBuf,
    output: Option<PathBuf>,
}

pub fn patch(mut data: Vec<u8>) -> Result<Vec<u8>, regex::Error> {
    let regex = regex::bytes::Regex::new("cdc_[a-zA-Z0-9]*")?;
    let matches = regex.find_iter(&data).peekable();
    let len = matches.peek().expect("signature not found").len();
    let ranges: Vec<_> = matches.map(|m| m.range()).collect();
    let signature: Vec<_> = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(len).collect();

    for r in ranges {
        let s = r.start;
        data[s..s + signature.len()].iter_mut().enumerate().for_each(|(i, b)| *b = signature[i]);
    }

    Ok(data)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let data = fs::read(&args.input)?;
    let output = args.output.unwrap_or(args.input);
    
    fs::write(output, patch(data)?)?;

    Ok(())
}
