//! --- Day 4: Camp Cleanup ---
//! Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.
//!
//! However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).
//!
//! For example, consider the following list of section assignment pairs:
//!
//! 2-4,6-8
//! 2-3,4-5
//! 5-7,7-9
//! 2-8,3-7
//! 6-6,4-6
//! 2-6,4-8
//! For the first few pairs, this list means:
//!
//! Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
//! The Elves in the second pair were each assigned two sections.
//! The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
//! This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:
//!
//! .234.....  2-4
//! .....678.  6-8
//!
//! .23......  2-3
//! ...45....  4-5
//!
//! ....567..  5-7
//! ......789  7-9
//!
//! .2345678.  2-8
//! ..34567..  3-7
//!
//! .....6...  6-6
//! ...456...  4-6
//!
//! .23456...  2-6
//! ...45678.  4-8
//! Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.
//!
//! In how many assignment pairs does one range fully contain the other?
//!
//! Your puzzle answer was 556.
//!
//! --- Part Two ---
//! It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.
//!
//! In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:
//!
//! 5-7,7-9 overlaps in a single section, 7.
//! 2-8,3-7 overlaps all of the sections 3 through 7.
//! 6-6,4-6 overlaps in a single section, 6.
//! 2-6,4-8 overlaps in sections 4, 5, and 6.
//! So, in this example, the number of overlapping assignment pairs is 4.
//!
//! In how many assignment pairs do the ranges overlap?
//!
//! Your puzzle answer was 876.

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

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

fn either_contains_any(clean1: RangeInclusive<usize>, clean2: RangeInclusive<usize>) -> bool {
    (clean1.contains(clean2.start()) || clean1.contains(clean2.end()))
        || (clean2.contains(clean1.start()) || clean2.contains(clean1.end()))
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

    // part 2
    let file = BufReader::new(File::open(filename)?);

    let part2_score: usize = file
        .lines()
        .map(Result::unwrap)
        .map(parse_line)
        .map(|(r1, r2)| either_contains_any(r1, r2))
        .map(usize::from)
        .sum();

    println!("part 2 score: {part2_score}");

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
    fn test_example_part2() {
        assert!(!either_contains_any(2..=4, 6..=8));
        assert!(!either_contains_any(2..=3, 4..=5));
        assert!(either_contains_any(5..=7, 7..=9));
        assert!(either_contains_any(2..=8, 3..=7));
        assert!(either_contains_any(6..=6, 4..=6));
        assert!(either_contains_any(2..=6, 4..=8));

        // made up ones
        assert!(either_contains_full(6..=6, 6..=6));
    }
}
