//! --- Day 12: Hill Climbing Algorithm ---
//! You try contacting the Elves using your handheld device, but the river you're following must be too low to get a decent signal.
//!
//! You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap shows the local area from above broken into a grid; the elevation of each square of the grid is given by a single lowercase letter, where a is the lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.
//!
//! Also included on the heightmap are marks for your current position (S) and the location that should get the best signal (E). Your current position (S) has elevation a, and the location that should get the best signal (E) has elevation z.
//!
//! You'd like to reach E, but to save energy, you should do it in as few steps as possible. During each step, you can move exactly one square up, down, left, or right. To avoid needing to get out your climbing gear, the elevation of the destination square can be at most one higher than the elevation of your current square; that is, if your current elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation of the destination square can be much lower than the elevation of your current square.)
//!
//! For example:
//!
//! Sabqponm
//! abcryxxl
//! accszExk
//! acctuvwj
//! abdefghi
//! Here, you start in the top-left corner; your goal is near the middle. You could start by moving down or right, but eventually you'll need to head toward the e at the bottom. From there, you can spiral around to the goal:
//!
//! v..v<<<<
//! >v.vv<<^
//! .>vv>E^^
//! ..v>>>^^
//! ..>>>>>^
//! In the above diagram, the symbols indicate whether the path exits each square moving up (^), down (v), left (<), or right (>). The location that should get the best signal is still E, and . marks unvisited squares.
//!
//! This path reaches the goal in 31 steps, the fewest possible.
//!
//! What is the fewest steps required to move from your current position to the location that should get the best signal?

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::sync::Arc;

use clap::Parser;
use nom::character::complete::{alphanumeric1, newline};
use nom::character::complete::{anychar, multispace0};
use nom::multi::{fold_many0, many1};
use nom::sequence::{pair, preceded};
use nom::{
    bytes::complete::tag,
    character::{self, complete::space1},
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    fn up(&self) -> Option<Point> {
        Some(Point {
            x: self.x,
            y: self.y + 1,
        })
    }

    fn down(&self) -> Option<Point> {
        Some(Point {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }

    fn left(&self) -> Option<Point> {
        Some(Point {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }

    fn right(&self) -> Option<Point> {
        Some(Point {
            x: self.x + 1,
            y: self.y,
        })
    }
}

struct Grid {
    start: Point,
    end: Point,
    data: Vec<Vec<u8>>,
}

fn get_height(ch: u8) -> u8 {
    match ch {
        b'S' => b'a',
        b'E' => b'z',
        _ => ch,
    }
}

impl Grid {
    // warning, this only returns the heigh, S and E will be replaces with a and z respectively
    fn get(&self, point: Point) -> Option<u8> {
        self.data
            .get(point.x)
            .and_then(|row| row.get(point.y))
            .cloned()
            .map(get_height)
    }

    fn find_shortest_path(&self) -> Vec<Point> {
        let to_search = Vec::new();
        let bad_path: Rc<RefCell<HashSet<Point>>> = Rc::new(RefCell::new(HashSet::new()));
        let shortest_path: Rc<RefCell<usize>> = Rc::new(RefCell::new(usize::MAX));

        self.search_from(self.start, to_search, bad_path, shortest_path)
            .expect("no paths found")
    }

    fn search_from(
        &self,
        current: Point,
        mut path: Vec<Point>,
        visited: Rc<RefCell<HashSet<Point>>>,
        shortest_path: Rc<RefCell<usize>>,
    ) -> Option<Vec<Point>> {
        path.push(current);
        if self.end == current {
            path.iter().for_each(|point| print!("{point}"));
            println!(" FOUND {}", path.len());
            return Some(path);
        }

        // are we already too long?
        let shortest = *shortest_path.borrow();
        if path.len() > shortest {
            path.iter().for_each(|point| print!("{point}"));
            println!(" TOO LONG {} > {shortest} ", path.len());
            return None;
        }

        let next = &[
            current.up(),
            current.down(),
            current.left(),
            current.right(),
        ];

        let mut next: Vec<Point> = next.into_iter().filter_map(|next| next.clone()).collect();

        // we'll order our search to go towards the end
        next.sort_unstable_by(|n1, n2| {
            if visited.borrow().contains(&n1) && !visited.borrow().contains(&n2) {
                return Ordering::Greater;
            } else if !visited.borrow().contains(&n1) && visited.borrow().contains(&n2) {
                return Ordering::Less;
            }

            if current.x < self.end.x {
                if n1.x > current.x {
                    Ordering::Less
                } else if n2.x > current.x {
                    Ordering::Greater
                } else {
                    n1.x.cmp(&n2.x)
                }
            } else if current.x > self.end.x {
                if n1.x < current.x {
                    Ordering::Less
                } else if n2.x < current.x {
                    Ordering::Greater
                } else {
                    n1.x.cmp(&n2.x)
                }
            } else if current.y < self.end.y {
                if n1.y > current.y {
                    Ordering::Less
                } else if n2.y > current.y {
                    Ordering::Greater
                } else {
                    n1.y.cmp(&n2.y)
                }
            } else if current.y > self.end.y {
                if n1.y < current.y {
                    Ordering::Less
                } else if n2.y < current.y {
                    Ordering::Greater
                } else {
                    n1.y.cmp(&n2.y)
                }
            } else {
                Ordering::Equal
            }
        });

        visited.borrow_mut().insert(current);

        let mut found_path: Option<Vec<Point>> = None;
        for next in next.into_iter() {
            if let Some(ch) = self.get(next) {
                // height difference of 1 or less
                let current_ch = self.get(current).expect("current point doesn't exist");
                if ch.abs_diff(current_ch) > 1 {
                    // too high, continue
                    continue;
                }
            } else {
                // off the edge, continue search elsewhere
                continue;
            }

            // check if this path already has this point
            if path.contains(&next) {
                continue; // skip spaces already in this path
            }

            if let Some(next_path) = self.search_from(
                next,
                path.clone(),
                Rc::clone(&visited),
                Rc::clone(&shortest_path),
            ) {
                // if this path doesn't have the end, continue searching
                if !next_path
                    .last()
                    .map(|end| *end == self.end)
                    .unwrap_or_default()
                {
                    continue;
                }

                if next_path.len()
                    < found_path
                        .as_ref()
                        .map(|path| path.len())
                        .unwrap_or(usize::MAX)
                {
                    found_path = Some(next_path);
                }
            }
        }

        if let Some(found_path) = &found_path {
            let shortest = *shortest_path.borrow();
            if found_path.len() < shortest {
                *shortest_path.borrow_mut() = found_path.len()
            }
        } else {
            // path.iter().for_each(|point| print!("{point}"));
            // println!(" REJECTED");
        }
        found_path
    }
}

fn parse_grid(reader: impl BufRead) -> Result<Grid, Box<dyn Error>> {
    let mut rows: Vec<Vec<u8>> = reader
        .lines()
        .map(|s| s.expect("bad_data"))
        .filter(|s| !s.is_empty())
        .map(|s| s.into_bytes())
        .collect();

    // organize such that the bottom left of the grid is 0,0
    rows.reverse();

    // find the start and end
    let start_and_end = rows
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(move |(y, ch)| (Point { x, y }, *ch))
        })
        .filter(|(_, ch)| *ch == b'S' || *ch == b'E');

    let mut start = None;
    let mut end = None;

    for (point, ch) in start_and_end {
        match ch {
            b'S' => start = Some(point),
            b'E' => end = Some(point),
            _ => panic!("unexpected character: {ch}"),
        }
    }

    Ok(Grid {
        start: start.expect("start not found"),
        end: end.expect("end not found"),
        data: rows,
    })
}

fn path_len(path: Vec<Point>) -> usize {
    path.len() - 1
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let reader = BufReader::new(File::open(filename)?);
    let grid = parse_grid(reader)?;
    let path = grid.find_shortest_path();

    println!("part1, shortest path: {}", path_len(path));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn test_part1() {
        let grid = parse_grid(BufReader::new(INPUT.as_bytes())).unwrap();

        assert_eq!(path_len(grid.find_shortest_path()), 31);
    }
}
