use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

fn find_signal(buf: &[u8]) -> usize {
    'outer: for i in 0..buf.len() {
        let slice = &buf[i..i + 4];
        assert_eq!(slice.len(), 4);

        for ch in slice.iter() {
            if slice.iter().filter(|search| ch == *search).count() > 1 {
                continue 'outer;
            }
        }

        // count looks good
        return i + 4;
    }

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;
    let mut file = BufReader::new(File::open(filename)?);

    let mut buf = String::new();

    file.read_line(&mut buf).expect("failed to read line");
    let buf = buf.into_bytes();

    let signal = find_signal(&buf);
    println!("part1, signal location {signal}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_signal() {
        assert_eq!(find_signal(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_signal(b"nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_signal(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_signal(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
