use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Day 1", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,

    #[clap(short = 't', long = "top-three")]
    pub(crate) top_three: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let filename = &args.file;

    let file = BufReader::new(File::open(filename)?);
    let mut counts = Vec::<usize>::new();
    let mut current_count = Option::<usize>::None;

    for line in file.lines() {
        let line = line?;

        if line.is_empty() || line.starts_with('\n') || line.starts_with("\r\n") {
            let current_count = current_count.take();
            if let Some(current_count) = current_count {
                counts.push(current_count);
            }
            continue;
        }

        let value = usize::from_str_radix(&line, 10)?;
        current_count.get_or_insert(0);
        if let Some(count) = current_count.as_mut() {
            *count += value;
        }
    }

    if let Some(current_count) = current_count.take() {
        counts.push(current_count);
    }

    let max_count = counts.iter().max().cloned().unwrap_or_default();
    println!("Max Calories: {max_count}");

    if args.top_three {
        let top_three: [usize; 3] =
            counts
                .iter()
                .cloned()
                .fold([0_usize; 3], |mut tops, mut next| {
                    if next > tops[0] {
                        let t = tops[0];
                        tops[0] = next;
                        next = t;
                    }

                    if next > tops[1] {
                        let t = tops[1];
                        tops[1] = next;
                        next = t;
                    }

                    if next > tops[2] {
                        let t = tops[2];
                        tops[2] = next;
                        next = t;
                    }

                    tops
                });

        let top_three = top_three[0] + top_three[1] + top_three[2];
        println!("Top Three Calories: {top_three}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
