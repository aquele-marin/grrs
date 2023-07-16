#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    // Reads file handling errors with a custom message using anyhow package
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;
    println!("file content:\n {}", content);

    let lines = content.lines().count() as u64;
    let pb = indicatif::ProgressBar::new(lines);

    for (i,line) in content.lines().enumerate() {
        if line.contains(&args.pattern) {
            // println!("{}", line);
            pb.println(format!("[+] finished #{} || FOUND TEXT: {}", i, line));
        } else {
            pb.println(format!("[+] finished #{}", i));
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(())
}