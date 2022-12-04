use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Range, RangeInclusive};

use clap::Parser;

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

fn either_contains_full(clean1: RangeInclusive<usize>, clean2: RangeInclusive<usize>) -> bool {
    (clean1.contains(clean2.start()) && clean1.contains(clean2.end()))
        || (clean2.contains(clean1.start()) && clean2.contains(clean1.end()))
}

fn parse_line(line: String) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    fn parse_range(range: &str) -> RangeInclusive<usize> {
        let mut range = range.split('-');
        let start = range.next().expect("missig range start");
        let end = range.next().expect("missing range end");

        let start: usize = start.parse().expect("bad number for start");
        let end: usize = end.parse().expect("bad number for end");

        start..=end
    }

    let mut ranges = line.split(',');
    let r1 = ranges.next().expect("missing first range");
    let r2 = ranges.next().expect("missing second range");

    (parse_range(r1), parse_range(r2))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let file = BufReader::new(File::open(filename)?);

    let part1_score: usize = file
        .lines()
        .map(Result::unwrap)
        .map(parse_line)
        .map(|(r1, r2)| either_contains_full(r1, r2))
        .map(usize::from)
        .sum();

    println!("part 1 score: {part1_score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert!(!either_contains_full(2..=4, 6..=8));
        assert!(!either_contains_full(2..=3, 4..=5));
        assert!(!either_contains_full(5..=7, 7..=9));
        assert!(either_contains_full(2..=8, 3..=7));
        assert!(either_contains_full(6..=6, 4..=6));
        assert!(!either_contains_full(2..=6, 4..=8));

        // made up ones
        assert!(either_contains_full(6..=6, 6..=6));
    }

    #[test]
    fn test_example_part2() {}
}
