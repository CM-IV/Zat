use std::io::{Write, BufRead, BufReader};
use clap::{Parser};
use color_eyre::eyre::Result;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Filepath argument
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = std::fs::File::open(args.path)?;
    let reader = BufReader::new(f);

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);

        if lines.len() == 100 {
            
            for line in &lines {
                writeln!(stdout, "{}", line)?;
            }
            lines.clear();
        }
    }

    if !lines.is_empty() {
        for line in &lines {
            writeln!(stdout, "{}", line)?;
        }
    }

    Ok(())
}
