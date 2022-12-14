//! --- Day 13: Distress Signal ---
//! You climb the hill and again try contacting the Elves. However, you instead receive a signal you weren't expecting: a distress signal.
//!
//! Your handheld device must still not be working properly; the packets from the distress signal got decoded out of order. You'll need to re-order the list of received packets (your puzzle input) to decode the message.
//!
//! Your list consists of pairs of packets; pairs are separated by a blank line. You need to identify how many pairs of packets are in the right order.
//!
//! For example:
//!
//! [1,1,3,1,1]
//! [1,1,5,1,1]
//!
//! [[1],[2,3,4]]
//! [[1],4]
//!
//! [9]
//! [[8,7,6]]
//!
//! [[4,4],4,4]
//! [[4,4],4,4,4]
//!
//! [7,7,7,7]
//! [7,7,7]
//!
//! []
//! [3]
//!
//! [[[]]]
//! [[]]
//!
//! [1,[2,[3,[4,[5,6,7]]]],8,9]
//! [1,[2,[3,[4,[5,6,0]]]],8,9]
//! Packet data consists of lists and integers. Each list starts with [, ends with ], and contains zero or more comma-separated values (either integers or other lists). Each packet is always a list and appears on its own line.
//!
//! When comparing two values, the first value is called left and the second value is called right. Then:
//!
//! If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input.
//! If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
//! If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].
//! Using these rules, you can determine which of the pairs in the example are in the right order:
//!
//! == Pair 1 ==
//! - Compare [1,1,3,1,1] vs [1,1,5,1,1]
//! - Compare 1 vs 1
//! - Compare 1 vs 1
//! - Compare 3 vs 5
//! - Left side is smaller, so inputs are in the right order
//!
//! == Pair 2 ==
//! - Compare [[1],[2,3,4]] vs [[1],4]
//! - Compare [1] vs [1]
//! - Compare 1 vs 1
//! - Compare [2,3,4] vs 4
//! - Mixed types; convert right to [4] and retry comparison
//! - Compare [2,3,4] vs [4]
//! - Compare 2 vs 4
//! - Left side is smaller, so inputs are in the right order
//!
//! == Pair 3 ==
//! - Compare [9] vs [[8,7,6]]
//! - Compare 9 vs [8,7,6]
//! - Mixed types; convert left to [9] and retry comparison
//! - Compare [9] vs [8,7,6]
//! - Compare 9 vs 8
//! - Right side is smaller, so inputs are not in the right order
//!
//! == Pair 4 ==
//! - Compare [[4,4],4,4] vs [[4,4],4,4,4]
//! - Compare [4,4] vs [4,4]
//! - Compare 4 vs 4
//! - Compare 4 vs 4
//! - Compare 4 vs 4
//! - Compare 4 vs 4
//! - Left side ran out of items, so inputs are in the right order
//!
//! == Pair 5 ==
//! - Compare [7,7,7,7] vs [7,7,7]
//! - Compare 7 vs 7
//! - Compare 7 vs 7
//! - Compare 7 vs 7
//! - Right side ran out of items, so inputs are not in the right order
//!
//! == Pair 6 ==
//! - Compare [] vs [3]
//! - Left side ran out of items, so inputs are in the right order
//!
//! == Pair 7 ==
//! - Compare [[[]]] vs [[]]
//! - Compare [[]] vs []
//! - Right side ran out of items, so inputs are not in the right order
//!
//! == Pair 8 ==
//! - Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
//! - Compare 1 vs 1
//! - Compare [2,[3,[4,[5,6,7]]]] vs [2,[3,[4,[5,6,0]]]]
//! - Compare 2 vs 2
//! - Compare [3,[4,[5,6,7]]] vs [3,[4,[5,6,0]]]
//! - Compare 3 vs 3
//! - Compare [4,[5,6,7]] vs [4,[5,6,0]]
//! - Compare 4 vs 4
//! - Compare [5,6,7] vs [5,6,0]
//! - Compare 5 vs 5
//! - Compare 6 vs 6
//! - Compare 7 vs 0
//! - Right side is smaller, so inputs are not in the right order
//! What are the indices of the pairs that are already in the right order? (The first pair has index 1, the second pair has index 2, and so on.) In the above example, the pairs in the right order are 1, 2, 4, and 6; the sum of these indices is 13.
//!
//! Determine which pairs of packets are already in the right order. What is the sum of the indices of those pairs?
//!

use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;
use nom::sequence::delimited;
use nom::{branch::alt, bytes::complete::tag, character, IResult};

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    Literal(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn is_lesser(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Literal(this), Packet::Literal(that)) => this.cmp(that),
            (this @ Packet::Literal(_), other @ Packet::List(_list)) => {
                Packet::List(vec![this.clone()]).is_lesser(other)
            }
            (this @ Packet::List(_), other @ Packet::Literal(_)) => {
                this.is_lesser(&Packet::List(vec![other.clone()]))
            }
            (Packet::List(these), Packet::List(those)) => {
                let mut order = Ordering::Equal;
                for (idx, this) in these.iter().enumerate() {
                    if let Some(that) = those.get(idx) {
                        match this.is_lesser(that) {
                            Ordering::Greater => return Ordering::Greater,
                            ordering => order = ordering,
                        }
                    } else {
                        match order {
                            Ordering::Equal => return Ordering::Greater,
                            Ordering::Less => return Ordering::Less,
                            _ => panic!("Greater should have returned earlier"),
                        }
                    }
                }

                order
            }
        }
    }
}

fn parse_literal(input: &str) -> IResult<&str, Packet> {
    let (input, literal) = character::complete::u32(input)?;
    Ok((input, Packet::Literal(literal as usize)))
}

fn parse_set(input: &str) -> IResult<&str, Packet> {
    let (input, packet) = delimited(
        character::complete::char('['),
        nom::multi::separated_list0(tag(","), alt((parse_literal, parse_set))),
        character::complete::char(']'),
    )(input)?;

    let packet = Packet::List(packet);
    Ok((input, packet))
}

fn parse_packet(input: &str) -> Packet {
    parse_set(input).expect("bad parse").1
}

fn process_packets(reader: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut lines = reader.lines();

    let mut running_total = 0;
    let mut index = 0;
    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        index += 1;
        let packet1 = parse_packet(&line);
        let packet2 = parse_packet(&lines.next().expect("eof")?);

        if !packet1.is_lesser(&packet2).is_gt() {
            running_total += dbg!(index);
        }
    }

    Ok(running_total)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let reader = BufReader::new(File::open(filename)?);
    let answer = process_packets(reader)?;

    println!("part 1 sum of indexes: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    #[test]
    fn test_part1() {
        let sum_of_indexes = process_packets(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(sum_of_indexes, 13);
    }

    #[test]
    fn test_part1_each_packet() {
        let process_packets = |s: &str| process_packets(BufReader::new(s.as_bytes()));

        assert_eq!(
            process_packets(
                r#"
[1,1,3,1,1]
[1,1,5,1,1]
"#
            )
            .unwrap(),
            1
        );

        assert_eq!(
            process_packets(
                r#"
[[1],[2,3,4]]
[[1],4]
"#
            )
            .unwrap(),
            1
        );

        assert_eq!(
            process_packets(
                r#"
[9]
[[8,7,6]]
"#
            )
            .unwrap(),
            0
        );

        assert_eq!(
            process_packets(
                r#"
[[4,4],4,4]
[[4,4],4,4,4]
"#
            )
            .unwrap(),
            1
        );

        assert_eq!(
            process_packets(
                r#"
[7,7,7,7]
[7,7,7]
"#
            )
            .unwrap(),
            0
        );

        assert_eq!(
            process_packets(
                r#"
[]
[3]
"#
            )
            .unwrap(),
            1
        );

        assert_eq!(
            process_packets(
                r#"
[[[]]]
[[]]
"#
            )
            .unwrap(),
            0
        );

        assert_eq!(
            process_packets(
                r#"
[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#
            )
            .unwrap(),
            0
        );
    }
}
