use std::io::BufRead;
use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = std::fs::File::open(args.path)?;
    let reader = std::io::BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
