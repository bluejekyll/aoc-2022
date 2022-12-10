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
//! --- Part Two ---
//!
//! A rope snaps! Suddenly, the river is getting a lot closer than you remember. The bridge is still there, but some of the ropes that broke are now whipping toward you as you fall through the air!
//!
//! The ropes are moving too quickly to grab; you only have a few seconds to choose how to arch your body to avoid being hit. Fortunately, your simulation can be extended to support longer ropes.
//!
//! Rather than two knots, you now must simulate a rope consisting of ten knots. One knot is still the head of the rope and moves according to the series of motions. Each knot further down the rope follows the knot in front of it using the same rules as before.
//!
//! Using the same series of motions as the above example, but with the knots marked H, 1, 2, ..., 9, the motions now occur as follows:
//!
//! == Initial State ==
//!
//! ......
//! ......
//! ......
//! ......
//! H.....  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)
//!
//! == R 4 ==
//!
//! ......
//! ......
//! ......
//! ......
//! 1H....  (1 covers 2, 3, 4, 5, 6, 7, 8, 9, s)
//!
//! ......
//! ......
//! ......
//! ......
//! 21H...  (2 covers 3, 4, 5, 6, 7, 8, 9, s)
//!
//! ......
//! ......
//! ......
//! ......
//! 321H..  (3 covers 4, 5, 6, 7, 8, 9, s)
//!
//! ......
//! ......
//! ......
//! ......
//! 4321H.  (4 covers 5, 6, 7, 8, 9, s)
//!
//! == U 4 ==
//!
//! ......
//! ......
//! ......
//! ....H.
//! 4321..  (4 covers 5, 6, 7, 8, 9, s)
//!
//! ......
//! ......
//! ....H.
//! .4321.
//! 5.....  (5 covers 6, 7, 8, 9, s)
//!
//! ......
//! ....H.
//! ....1.
//! .432..
//! 5.....  (5 covers 6, 7, 8, 9, s)
//!
//! ....H.
//! ....1.
//! ..432.
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! == L 3 ==
//!
//! ...H..
//! ....1.
//! ..432.
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ..H1..
//! ...2..
//! ..43..
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! .H1...
//! ...2..
//! ..43..
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! == D 1 ==
//!
//! ..1...
//! .H.2..
//! ..43..
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! == R 4 ==
//!
//! ..1...
//! ..H2..
//! ..43..
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ..1...
//! ...H..  (H covers 2)
//! ..43..
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ......
//! ...1H.  (1 covers 2)
//! ..43..
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ......
//! ...21H
//! ..43..
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! == D 1 ==
//!
//! ......
//! ...21.
//! ..43.H
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! == L 5 ==
//!
//! ......
//! ...21.
//! ..43H.
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ......
//! ...21.
//! ..4H..  (H covers 3)
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ......
//! ...2..
//! ..H1..  (H covers 4; 1 covers 3)
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ......
//! ...2..
//! .H13..  (1 covers 4)
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ......
//! ......
//! H123..  (2 covers 4)
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! == R 2 ==
//!
//! ......
//! ......
//! .H23..  (H covers 1; 2 covers 4)
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//!
//! ......
//! ......
//! .1H3..  (H covers 2, 4)
//! .5....
//! 6.....  (6 covers 7, 8, 9, s)
//! Now, you need to keep track of the positions the new tail, 9, visits. In this example, the tail never moves, and so it only visits 1 position. However, be careful: more types of motion are possible than before, so you might want to visually compare your simulated rope to the one above.
//!
//! Here's a larger example:
//!
//! R 5
//! U 8
//! L 8
//! D 3
//! R 17
//! D 10
//! L 25
//! U 20
//! These motions occur as follows (individual steps are not shown):
//!
//! == Initial State ==
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ...........H..............  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//!
//! == R 5 ==
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ...........54321H.........  (5 covers 6, 7, 8, 9, s)
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//!
//! == U 8 ==
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ................H.........
//! ................1.........
//! ................2.........
//! ................3.........
//! ...............54.........
//! ..............6...........
//! .............7............
//! ............8.............
//! ...........9..............  (9 covers s)
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//!
//! == L 8 ==
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ........H1234.............
//! ............5.............
//! ............6.............
//! ............7.............
//! ............8.............
//! ............9.............
//! ..........................
//! ..........................
//! ...........s..............
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//!
//! == D 3 ==
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! .........2345.............
//! ........1...6.............
//! ........H...7.............
//! ............8.............
//! ............9.............
//! ..........................
//! ..........................
//! ...........s..............
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//!
//! == R 17 ==
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ................987654321H
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ...........s..............
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//!
//! == D 10 ==
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ...........s.........98765
//! .........................4
//! .........................3
//! .........................2
//! .........................1
//! .........................H
//!
//! == L 25 ==
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ...........s..............
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! H123456789................
//!
//! == U 20 ==
//!
//! H.........................
//! 1.........................
//! 2.........................
//! 3.........................
//! 4.........................
//! 5.........................
//! 6.........................
//! 7.........................
//! 8.........................
//! 9.........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ...........s..............
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//!
//! Now, the tail (9) visits 36 positions (including s) at least once:
//!
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! ..........................
//! #.........................
//! #.............###.........
//! #............#...#........
//! .#..........#.....#.......
//! ..#..........#.....#......
//! ...#........#.......#.....
//! ....#......s.........#....
//! .....#..............#.....
//! ......#............#......
//! .......#..........#.......
//! ........#........#........
//! .........########.........
//! Simulate your complete series of motions on a larger rope with ten knots. How many positions does the tail of the rope visit at least once?

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

    fn catch_up(&mut self, other: &Position) -> bool {
        fn mov_and_diag(axis1: (&mut isize, isize), axis2: (&mut isize, isize)) -> bool {
            let a = axis1.0;
            let b = axis1.1;

            let c = axis2.0;
            let d = axis2.1;

            if *a > b && *a - b > 1 {
                *a -= 1;

                if *c > d {
                    *c -= 1;
                } else if *c < d {
                    *c += 1;
                }
                true
            } else if *a < b && b - *a > 1 {
                *a += 1;

                if *c > d {
                    *c -= 1;
                } else if *c < d {
                    *c += 1;
                }
                true
            } else {
                false
            }
        }

        mov_and_diag((&mut self.x, other.x), (&mut self.y, other.y))
            || mov_and_diag((&mut self.y, other.y), (&mut self.x, other.x))
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn calculate_moves(reader: impl BufRead, tail_len: usize) -> Result<usize, Box<dyn Error>> {
    let mut unique_tail_positions = HashSet::<Position>::new();

    let mut head = Position::default();
    let mut ropes = vec![Position::default(); tail_len];

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
            let mut prev = head.clone();

            for next in ropes.iter_mut() {
                next.catch_up(&prev);
                prev = next.clone();
            }
            unique_tail_positions.insert(ropes.last().unwrap().clone());
        }
    }

    Ok(unique_tail_positions.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let file = BufReader::new(File::open(filename)?);
    let score = calculate_moves(file, 1)?;

    println!("part1, unique places: {score}");

    let file = BufReader::new(File::open(filename)?);
    let score = calculate_moves(file, 9)?;

    println!("part2, unique places: {score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    const INPUT2: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn test_example_data_part_1() {
        assert_eq!(
            calculate_moves(BufReader::new(INPUT.as_bytes()), 1).unwrap(),
            13
        );
    }

    #[test]
    fn test_example_data_part_2() {
        assert_eq!(
            calculate_moves(BufReader::new(INPUT.as_bytes()), 9).unwrap(),
            1
        );
        assert_eq!(
            calculate_moves(BufReader::new(INPUT2.as_bytes()), 9).unwrap(),
            36
        );
    }
}