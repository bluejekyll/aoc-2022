//! --- Day 9: Rope Bridge ---
//! This rope bridge creaks as you walk along it. You aren't sure how old it is, or whether it can even support your weight.
//!
//! It seems to support the Elves just fine, though. The bridge spans a gorge which was carved out by the massive river far below you.
//!
//! You step carefully; as you do, the ropes stretch and twist. You decide to distract yourself by modeling rope physics; maybe you can even figure out where not to step.
//!
//! Consider a rope with a knot at each end; these knots mark the head and the tail of the rope. If the head moves far enough away from the tail, the tail is pulled toward the head.
//!
//! Due to nebulous reasoning involving Planck lengths, you should be able to model the positions of the knots on a two-dimensional grid. Then, by following a hypothetical series of motions (your puzzle input) for the head, you can determine how the tail will move.
//!
//! Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching):
//!
//! ....
//! .TH.
//! ....
//!
//! ....
//! .H..
//! ..T.
//! ....
//!
//! ...
//! .H. (H covers T)
//! ...
//! If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:
//!
//! .....    .....    .....
//! .TH.. -> .T.H. -> ..TH.
//! .....    .....    .....
//!
//! ...    ...    ...
//! .T.    .T.    ...
//! .H. -> ... -> .T.
//! ...    .H.    .H.
//! ...    ...    ...
//! Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:
//!
//! .....    .....    .....
//! .....    ..H..    ..H..
//! ..H.. -> ..... -> ..T..
//! .T...    .T...    .....
//! .....    .....    .....
//!
//! .....    .....    .....
//! .....    .....    .....
//! ..H.. -> ...H. -> ..TH.
//! .T...    .T...    .....
//! .....    .....    .....
//! You just need to work out where the tail goes as the head follows a series of motions. Assume the head and the tail both start at the same position, overlapping.
//!
//! For example:
//!
//! R 4
//! U 4
//! L 3
//! D 1
//! R 4
//! D 1
//! L 5
//! R 2
//! This series of motions moves the head right four steps, then up four steps, then left three steps, then down one step, and so on. After each step, you'll need to update the position of the tail if the step means the head is no longer adjacent to the tail. Visually, these motions occur as follows (s marks the starting position as a reference point):
//!
//! == Initial State ==
//!
//! ......
//! ......
//! ......
//! ......
//! H.....  (H covers T, s)
//!
//! == R 4 ==
//!
//! ......
//! ......
//! ......
//! ......
//! TH....  (T covers s)
//!
//! ......
//! ......
//! ......
//! ......
//! sTH...
//!
//! ......
//! ......
//! ......
//! ......
//! s.TH..
//!
//! ......
//! ......
//! ......
//! ......
//! s..TH.
//!
//! == U 4 ==
//!
//! ......
//! ......
//! ......
//! ....H.
//! s..T..
//!
//! ......
//! ......
//! ....H.
//! ....T.
//! s.....
//!
//! ......
//! ....H.
//! ....T.
//! ......
//! s.....
//!
//! ....H.
//! ....T.
//! ......
//! ......
//! s.....
//!
//! == L 3 ==
//!
//! ...H..
//! ....T.
//! ......
//! ......
//! s.....
//!
//! ..HT..
//! ......
//! ......
//! ......
//! s.....
//!
//! .HT...
//! ......
//! ......
//! ......
//! s.....
//!
//! == D 1 ==
//!
//! ..T...
//! .H....
//! ......
//! ......
//! s.....
//!
//! == R 4 ==
//!
//! ..T...
//! ..H...
//! ......
//! ......
//! s.....
//!
//! ..T...
//! ...H..
//! ......
//! ......
//! s.....
//!
//! ......
//! ...TH.
//! ......
//! ......
//! s.....
//!
//! ......
//! ....TH
//! ......
//! ......
//! s.....
//!
//! == D 1 ==
//!
//! ......
//! ....T.
//! .....H
//! ......
//! s.....
//!
//! == L 5 ==
//!
//! ......
//! ....T.
//! ....H.
//! ......
//! s.....
//!
//! ......
//! ....T.
//! ...H..
//! ......
//! s.....
//!
//! ......
//! ......
//! ..HT..
//! ......
//! s.....
//!
//! ......
//! ......
//! .HT...
//! ......
//! s.....
//!
//! ......
//! ......
//! HT....
//! ......
//! s.....
//!
//! == R 2 ==
//!
//! ......
//! ......
//! .H....  (H covers T)
//! ......
//! s.....
//!
//! ......
//! ......
//! .TH...
//! ......
//! s.....
//! After simulating the rope, you can count up all of the positions the tail visited at least once. In this diagram, s again marks the starting position (which the tail also visited) and # marks other positions the tail visited:
//!
//! ..##..
//! ...##.
//! .####.
//! ....#.
//! s###..
//! So, there are 13 positions the tail visited at least once.
//!
//! Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at least once?
//!
//! Your puzzle answer was 6642.
//!

use std::collections::HashSet;
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

#[derive(Debug)]
#[repr(u8)]
enum Direction {
    Up = b'U',
    Down = b'D',
    Left = b'L',
    Right = b'R',
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        assert!(ch.is_ascii_alphabetic());
        match u8::try_from(ch).expect("not ascii") {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!("not a direction {ch}"),
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn mov(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn catch_up(&mut self, other: &Position) {
        if other.x > self.x && (other.x - self.x) > 1 {
            self.x += 1;
            if self.y != other.y {
                self.y = other.y;
            }
        } else if other.x < self.x && (self.x - other.x) > 1 {
            self.x -= 1;
            if self.y != other.y {
                self.y = other.y;
            }
        } else if other.y > self.y && (other.y - self.y) > 1 {
            self.y += 1;
            if self.x != other.x {
                self.x = other.x;
            }
        } else if other.y < self.y && (self.y - other.y) > 1 {
            self.y -= 1;
            if self.x != other.x {
                self.x = other.x;
            }
        }
    }
}

fn calculate_moves(reader: impl BufRead) -> Result<usize, Box<dyn Error>> {
    let mut unique_tail_positions = HashSet::<Position>::new();

    let mut head = Position::default();
    let mut tail = Position::default();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let mut split = line.split(' ');
        let direction = split
            .next()
            .map(|s| s.chars().next().expect("no move"))
            .map(Direction::from)
            .expect("line missing move");
        let count = split
            .next()
            .map(|s| usize::from_str_radix(s, 10).expect("bad count"))
            .expect("line missing count");

        for _ in 0..count {
            head.mov(&direction);
            tail.catch_up(&head);
            unique_tail_positions.insert(tail.clone());
        }
    }

    Ok(unique_tail_positions.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let file = BufReader::new(File::open(filename)?);
    let score = calculate_moves(file)?;

    println!("part1, unique places: {score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    #[test]
    fn test_example_data_part_1() {
        assert_eq!(
            calculate_moves(BufReader::new(input.as_bytes())).unwrap(),
            13
        );
    }
}
