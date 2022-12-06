use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

const SINGAL_LEN: usize = 4;
const MESSAGE_LEN: usize = 14;

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

fn find_signal(buf: &[u8], unique_len: usize) -> usize {
    'outer: for i in 0..buf.len() {
        let slice = &buf[i..i + unique_len];
        assert_eq!(slice.len(), unique_len);

        for ch in slice.iter() {
            if slice.iter().filter(|search| ch == *search).count() > 1 {
                continue 'outer;
            }
        }

        // count looks good
        return i + unique_len;
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

    let signal = find_signal(&buf, SINGAL_LEN);
    println!("part1, signal location {signal}");

    let message = find_signal(&buf, MESSAGE_LEN);
    println!("part2, message location {message}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_signal() {
        assert_eq!(
            find_signal(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", SINGAL_LEN),
            7
        );
        assert_eq!(find_signal(b"bvwbjplbgvbhsrlpgdmjqwftvncz", SINGAL_LEN), 5);
        assert_eq!(find_signal(b"nppdvjthqldpwncqszvftbrmjlhg", SINGAL_LEN), 6);
        assert_eq!(
            find_signal(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", SINGAL_LEN),
            10
        );
        assert_eq!(
            find_signal(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", SINGAL_LEN),
            11
        );
    }

    #[test]
    fn test_find_message() {
        assert_eq!(
            find_signal(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", MESSAGE_LEN),
            19
        );
        assert_eq!(
            find_signal(b"bvwbjplbgvbhsrlpgdmjqwftvncz", MESSAGE_LEN),
            23
        );
        assert_eq!(
            find_signal(b"nppdvjthqldpwncqszvftbrmjlhg", MESSAGE_LEN),
            23
        );
        assert_eq!(
            find_signal(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", MESSAGE_LEN),
            29
        );
        assert_eq!(
            find_signal(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", MESSAGE_LEN),
            26
        );
    }
}
