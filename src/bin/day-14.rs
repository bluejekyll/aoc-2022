//! --- Day 14: Regolith Reservoir ---
//! The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like it's coming from the waterfall itself, and that doesn't make any sense. However, you do notice a little path that leads behind the waterfall.
//!
//! Correction: the distress signal leads you behind a giant waterfall! There seems to be a large cave system here, and the signal definitely leads further inside.
//!
//! As you begin to make your way deeper underground, you feel the ground rumble for a moment. Sand begins pouring into the cave! If you don't quickly figure out where the sand is going, you could quickly become trapped!
//!
//! Fortunately, your familiarity with analyzing the path of falling material will come in handy here. You scan a two-dimensional vertical slice of the cave above you (your puzzle input) and discover that it is mostly air with structures made of rock.
//!
//! Your scan traces the path of each solid rock structure and reports the x,y coordinates that form the shape of the path, where x represents distance to the right and y represents distance down. Each path appears as a single line of text in your scan. After the first point of each path, each point indicates the end of a straight horizontal or vertical line to be drawn from the previous point. For example:
//!
//! 498,4 -> 498,6 -> 496,6
//! 503,4 -> 502,4 -> 502,9 -> 494,9
//! This scan means that there are two paths of rock; the first path consists of two straight lines, and the second path consists of three straight lines. (Specifically, the first path consists of a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)
//!
//! The sand is pouring into the cave from point 500,0.
//!
//! Drawing rock as #, air as ., and the source of the sand as +, this becomes:
//!
//!
//!   4     5  5
//!   9     0  0
//!   4     0  3
//! 0 ......+...
//! 1 ..........
//! 2 ..........
//! 3 ..........
//! 4 ....#...##
//! 5 ....#...#.
//! 6 ..###...#.
//! 7 ........#.
//! 8 ........#.
//! 9 #########.
//! Sand is produced one unit at a time, and the next unit of sand is not produced until the previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in your scan.
//!
//! A unit of sand always falls down one step if possible. If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step down and to the left. If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right. Sand keeps moving as long as it is able to do so, at each step trying to move down, then down-left, then down-right. If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves, at which point the next unit of sand is created back at the source.
//!
//! So, drawing sand that has come to rest as o, the first unit of sand simply falls straight down and then stops:
//!
//! ......+...
//! ..........
//! ..........
//! ..........
//! ....#...##
//! ....#...#.
//! ..###...#.
//! ........#.
//! ......o.#.
//! #########.
//! The second unit of sand then falls straight down, lands on the first one, and then comes to rest to its left:
//!
//! ......+...
//! ..........
//! ..........
//! ..........
//! ....#...##
//! ....#...#.
//! ..###...#.
//! ........#.
//! .....oo.#.
//! #########.
//! After a total of five units of sand have come to rest, they form this pattern:
//!
//! ......+...
//! ..........
//! ..........
//! ..........
//! ....#...##
//! ....#...#.
//! ..###...#.
//! ......o.#.
//! ....oooo#.
//! #########.
//! After a total of 22 units of sand:
//!
//! ......+...
//! ..........
//! ......o...
//! .....ooo..
//! ....#ooo##
//! ....#ooo#.
//! ..###ooo#.
//! ....oooo#.
//! ...ooooo#.
//! #########.
//! Finally, only two more units of sand can possibly come to rest:
//!
//! ......+...
//! ..........
//! ......o...
//! .....ooo..
//! ....#ooo##
//! ...o#ooo#.
//! ..###ooo#.
//! ....oooo#.
//! .o.ooooo#.
//! #########.
//! Once all 24 units of sand shown above have come to rest, all further sand flows out the bottom, falling into the endless void. Just for fun, the path any new sand takes before falling forever is shown here with ~:
//!
//! .......+...
//! .......~...
//! ......~o...
//! .....~ooo..
//! ....~#ooo##
//! ...~o#ooo#.
//! ..~###ooo#.
//! ..~..oooo#.
//! .~o.ooooo#.
//! ~#########.
//! ~..........
//! ~..........
//! ~..........
//! Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?

use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;
use nom::sequence::{delimited, tuple};
use nom::{branch::alt, bytes::complete::tag, character, IResult};

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Line(Vec<Point>);

// 498,4
fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, _, y)) = tuple((
        character::complete::u32,
        character::complete::char(','),
        character::complete::u32,
    ))(input)?;

    Ok((
        input,
        Point {
            x: x as usize,
            y: y as usize,
        },
    ))
}

// 498,4 -> 498,6 -> 496,6
fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, points) = nom::multi::separated_list1(tag(" -> "), parse_point)(input)?;

    Ok((input, Line(points)))
}

fn parse_lines(reader: impl BufRead) -> Vec<Line> {
    reader
        .lines()
        .filter_map(|s| s.ok())
        .filter(|s| !s.is_empty())
        .map(|s| parse_line(&s).expect("invalid data").1)
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let reader = BufReader::new(File::open(filename)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn test_parse() {
        let lines = parse_lines(BufReader::new(INPUT.as_bytes()));
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].0.len(), 3);
        assert_eq!(lines[1].0.len(), 4);
    }
}
