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

use std::collections::BTreeSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, character, IResult};

const SAND_START: Point = Point { x: 500, y: 0 };

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

impl Point {
    fn down(&self) -> Point {
        let mut down = self.clone();
        down.y += 1;
        down
    }

    fn down_and_left(&self) -> Point {
        let mut left = self.clone();
        left.y += 1;
        left.x -= 1;
        left
    }

    fn down_and_right(&self) -> Point {
        let mut right = self.clone();
        right.y += 1;
        right.x += 1;
        right
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Next {
    Free(Point),
    Blocked,
    EndlessVoid,
}

impl Next {
    fn or_else<F: FnOnce() -> Next>(self, f: F) -> Next {
        match self {
            Next::Blocked => f(),
            next => next,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Rock {
    line: Vec<Point>,
    max_x: usize,
    min_x: usize,
    max_y: usize,
}

impl Rock {
    fn new(line: Vec<Point>) -> Self {
        let max_x = line.iter().map(|point| point.x).max().expect("no x coords");
        let min_x = line.iter().map(|point| point.x).min().expect("no x coords");
        let max_y = line.iter().map(|point| point.y).max().expect("no y coords");

        Self {
            line,
            max_x,
            min_x,
            max_y,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        let cmp_axis = |a1: usize, a2: usize, p: usize| -> bool {
            (a1 <= a2 && p >= a1 && p <= a2) || (a2 <= a1 && p >= a2 && p <= a1)
        };

        if point.x >= self.min_x && point.x <= self.max_x && point.y <= self.max_y {
            for i in 1..self.line.len() {
                let vertex1 = &self.line[i - 1];
                let vertex2 = &self.line[i];

                if cmp_axis(vertex1.x, vertex2.x, point.x)
                    && cmp_axis(vertex1.y, vertex2.y, point.y)
                {
                    return true;
                }
            }
        }

        false
    }
}

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
fn parse_rock(input: &str) -> IResult<&str, Rock> {
    let (input, points) = nom::multi::separated_list1(tag(" -> "), parse_point)(input)?;

    Ok((input, Rock::new(points)))
}

fn parse_rocks(reader: impl BufRead) -> Vec<Rock> {
    reader
        .lines()
        .filter_map(|s| s.ok())
        .filter(|s| !s.is_empty())
        .map(|s| parse_rock(&s).expect("invalid data").1)
        .collect()
}

struct Cave {
    max_depth: usize,
    min_x: usize,
    max_x: usize,
    rocks: Vec<Rock>,
    sand: BTreeSet<Point>,
}

impl Cave {
    fn new(rocks: Vec<Rock>) -> Self {
        let max_x = rocks
            .iter()
            .map(|rock| rock.max_x)
            .max()
            .expect("no x coords");
        let min_x = rocks
            .iter()
            .map(|rock| rock.min_x)
            .min()
            .expect("no x coords");

        let max_depth = rocks
            .iter()
            .map(|rock| rock.max_y)
            .max()
            .expect("no y coords");

        let sand = BTreeSet::new();

        Cave {
            max_depth,
            max_x,
            min_x,
            rocks,
            sand,
        }
    }

    // true if the space is free, false if not, None if off the board (past the maximal values)
    fn maybe_free(&self, point: &Point) -> Option<bool> {
        if point.x > self.max_x || point.x < self.min_x || point.y > self.max_depth {
            return None;
        }

        if self.sand.contains(point) {
            return Some(false);
        }

        for rock in &self.rocks {
            if rock.contains(point) {
                return Some(false);
            }
        }

        Some(true)
    }

    fn can_drop_inner(&self, to: Point) -> Next {
        if let Some(free) = self.maybe_free(&to) {
            if free {
                Next::Free(to)
            } else {
                Next::Blocked
            }
        } else {
            Next::EndlessVoid
        }
    }

    fn can_drop_down(&self, from: &Point) -> Next {
        let to = from.down();
        self.can_drop_inner(to)
    }

    fn can_drop_left(&self, from: &Point) -> Next {
        let to = from.down_and_left();
        self.can_drop_inner(to)
    }

    fn can_drop_right(&self, from: &Point) -> Next {
        let to = from.down_and_right();
        self.can_drop_inner(to)
    }

    fn can_drop(&self, from: &Point) -> Next {
        self.can_drop_down(from)
            .or_else(|| self.can_drop_left(from))
            .or_else(|| self.can_drop_right(from))
    }

    fn drop_sand(&mut self) {
        loop {
            let mut point = SAND_START;

            loop {
                match self.can_drop(&point) {
                    Next::Free(next) => point = next,
                    Next::Blocked => break,
                    Next::EndlessVoid => return,
                }
            }

            assert!(self.sand.insert(point));
        }
    }

    fn sand_count(&self) -> usize {
        self.sand.len()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let reader = BufReader::new(File::open(filename)?);
    let rocks = parse_rocks(BufReader::new(reader));
    let mut cave = Cave::new(rocks);
    cave.drop_sand();

    let amount_of_sand = cave.sand_count();
    println!("part1, how much sand: {amount_of_sand}");

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
    fn test_rock_contains() {
        let rock = Rock::new(vec![Point { x: 502, y: 9 }, Point { x: 494, y: 9 }]);
        assert!(rock.contains(&Point { x: 500, y: 9 }));
    }

    #[test]
    fn test_parse() {
        let lines = parse_rocks(BufReader::new(INPUT.as_bytes()));
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].line.len(), 3);
        assert_eq!(lines[1].line.len(), 4);
    }

    #[test]
    fn test_part1() {
        let rocks = parse_rocks(BufReader::new(INPUT.as_bytes()));
        let mut cave = Cave::new(rocks);

        cave.drop_sand();
        assert_eq!(cave.sand_count(), 24);
    }
}
