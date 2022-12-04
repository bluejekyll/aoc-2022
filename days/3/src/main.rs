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
}

/// Lowercase item types a through z have priorities 1 through 26.
/// Uppercase item types A through Z have priorities 27 through 52.
fn priority(ch: u8) -> usize {
    if ch.is_ascii_lowercase() {
        (ch - b'a' + 1) as usize
    } else {
        (ch - b'A' + 27) as usize
    }
}

fn score(rucksack: &[u8]) -> usize {
    let (c1, c2) = rucksack.split_at(rucksack.len() / 2);
    assert_eq!(c1.len(), c2.len());

    for c1ch in c1 {
        if c2.contains(c1ch) {
            return priority(*c1ch);
        }
    }

    0
}

fn badge_score(r1: &[u8], r2: &[u8], r3: &[u8]) -> usize {
    for r1ch in r1 {
        if r2.contains(r1ch) && r3.contains(r1ch) {
            return priority(*r1ch);
        }
    }

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let file = BufReader::new(File::open(filename)?);

    let part1_score: usize = file
        .lines()
        .map(Result::unwrap)
        .map(|rucksack| score(rucksack.as_bytes()))
        .sum();

    println!("part1 score: {part1_score}");

    // part 2
    let file = BufReader::new(File::open(filename)?);
    let mut lines = file.lines().peekable();
    let mut part2_score = 0usize;

    while let Some(_) = lines.peek() {
        let r1 = lines.next().unwrap()?;
        let r2 = lines.next().unwrap()?;
        let r3 = lines.next().unwrap()?;

        part2_score += badge_score(r1.as_bytes(), r2.as_bytes(), r3.as_bytes());
    }

    println!("part2 score: {part2_score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority(b'a'), 1);
        assert_eq!(priority(b'b'), 2);
        assert_eq!(priority(b'z'), 26);

        assert_eq!(priority(b'A'), 27);
        assert_eq!(priority(b'B'), 28);
        assert_eq!(priority(b'Z'), 52);
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(score(b"vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(score(b"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
        assert_eq!(score(b"PmmdzqPrVvPwwTWBwg"), 42);
        assert_eq!(score(b"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
        assert_eq!(score(b"ttgJtRGJQctTZtZT"), 20);
        assert_eq!(score(b"CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            badge_score(
                b"vJrwpWtwJgWrhcsFMMfFFhFp",
                b"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                b"PmmdzqPrVvPwwTWBwg"
            ),
            18
        );
        assert_eq!(
            badge_score(
                b"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                b"ttgJtRGJQctTZtZT",
                b"CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            52
        );
    }
}
