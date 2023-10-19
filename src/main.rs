use std::{path::PathBuf, fs, process::Command};

use clap::Parser;

#[derive(Parser)]
struct Args {
    input: PathBuf,
    output: Option<PathBuf>,
    #[arg(short, long, default_value_t = false)]
    sign: bool
}

pub fn patch(mut data: Vec<u8>, injected_code_block: &str) -> anyhow::Result<Vec<u8>> {
    let regex = regex::bytes::Regex::new(r#"\{window\.cdc.+;\}"#)?;
    let code_block = regex.find(&data).ok_or(anyhow::Error::msg("code block not found"))?;

    if injected_code_block.len() > code_block.len() {
        return Err(anyhow::Error::msg("injected code block length is too big"));
    }

    let injected_code_block = injected_code_block
        .bytes()
        .chain(
            std::iter::repeat(b' ')
                .take(code_block.len() - injected_code_block.len())
        );

    for (i, b) in code_block.range().zip(injected_code_block) {
        data[i] = b;
    }

    Ok(data)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let data = fs::read(&args.input)?;
    let output = args.output.unwrap_or(args.input);
    
    fs::write(&output, patch(data, "{}")?)?;

    if args.sign {
        if cfg!(target_os = "macos") {
            let output = output.as_os_str().to_str().unwrap();

            Command::new("codesign")
                .args(["--remove-signature", output])
                .status()
                .expect("unable to remove signature from patched chromedriver");
            Command::new("codesign")
                .args(["--force", "--deep", "-s", "-", output])
                .status()
                .expect("unable to sign patched chromedriver");
        } else {
            eprintln!("code-signing is not supported on operating systems other than macOS yet");
        }
    }

    Ok(())
}
