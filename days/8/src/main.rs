//! --- Day 8: Treetop Tree House ---
//! The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.
//!
//! First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.
//!
//! The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:
//!
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
//!
//! A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
//!
//! All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:
//!
//! The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
//! The top-middle 5 is visible from the top and right.
//! The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
//! The left-middle 5 is visible, but only from the right.
//! The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
//! The right-middle 3 is visible from the right.
//! In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
//! With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.
//!
//! Consider your map; how many trees are visible from outside the grid?
//!
//! Your puzzle answer was 1809.
//!
//! --- Part Two ---
//! Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.
//!
//! To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)
//!
//! The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.
//!
//! In the example above, consider the middle 5 in the second row:
//!
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! Looking up, its view is not blocked; it can see 1 tree (of height 3).
//! Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
//! Looking right, its view is not blocked; it can see 2 trees.
//! Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).
//! A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
//!
//! However, you can do even better: consider the tree of height 5 in the middle of the fourth row:
//!
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
//! Looking left, its view is not blocked; it can see 2 trees.
//! Looking down, its view is also not blocked; it can see 1 tree.
//! Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
//! This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.
//!
//! Consider each tree on your map. What is the highest scenic score possible for any tree?
//!
//! Your puzzle answer was 479400.

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

#[derive(PartialEq, Eq, Hash)]
struct Tree {
    height: usize,
    row: usize,
    col: usize,
}

fn build_grid(reader: impl BufRead) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let mut grid = Vec::<Vec<usize>>::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let row = line
            .chars()
            .map(|ch| ch.to_digit(10).expect("not a digit") as usize)
            .collect();
        grid.push(row);
    }

    Ok(grid)
}

fn get_visible_trees(grid: &Vec<Vec<usize>>) -> HashSet<Tree> {
    let columns = grid[0].len();
    let rows = grid.len();
    let mut visible_trees = HashSet::<Tree>::new();

    let mut current_max_height: usize;

    let compare_and_set = |current_max_height: &mut usize,
                           visible_trees: &mut HashSet<Tree>,
                           row: usize,
                           col: usize| {
        // println!(
        //     "grid[({row})][({col})]) == {} > *{current_max_height}",
        //     grid[(row)][(col)]
        // );
        if (grid[(row)][(col)]) > *current_max_height {
            let height = grid[row][col];
            *current_max_height = height;
            visible_trees.insert(Tree { height, row, col });
        }
    };

    // insert all the edges, they are all visible
    for row in 0..rows {
        visible_trees.insert(Tree {
            height: grid[row][0],
            row,
            col: 0,
        });
        visible_trees.insert(Tree {
            height: grid[row][columns - 1],
            row,
            col: columns - 1,
        });
    }

    for col in 0..columns {
        visible_trees.insert(Tree {
            height: grid[0][col],
            row: 0,
            col,
        });
        visible_trees.insert(Tree {
            height: grid[rows - 1][col],
            row: rows - 1,
            col,
        });
    }

    // process rows
    for row in 0..rows {
        // left to right
        //println!("left to right");
        current_max_height = 0;
        for col in 0..columns {
            compare_and_set(&mut current_max_height, &mut visible_trees, row, col);
        }

        // right to left
        //println!("right to left");
        current_max_height = 0;
        for col in (0..columns).rev() {
            compare_and_set(&mut current_max_height, &mut visible_trees, row, col);
        }
    }

    // process columns
    for col in 0..columns {
        // top to bottom
        //println!("top to bottom");
        current_max_height = 0;
        for row in 0..rows {
            compare_and_set(&mut current_max_height, &mut visible_trees, row, col);
        }

        // bottom to top
        //println!("bottom to top");
        current_max_height = 0;
        for row in (0..rows).rev() {
            compare_and_set(&mut current_max_height, &mut visible_trees, row, col);
        }
    }

    // print visible trees
    // let mut grid = Vec::<Vec<u8>>::new();
    // for _ in 0..rows {
    //     let row = vec!['*' as u8; columns];

    //     grid.push(row);
    // }

    // for tree in visible_trees.iter() {
    //     grid[tree.row][tree.col] = '0' as u8 + tree.height as u8;
    // }

    // for row in grid.iter() {
    //     for i in row.iter() {
    //         print!("{}", char::from(*i));
    //     }
    //     println!("");
    // }

    visible_trees
}

fn calculate_tree_visibility(grid: &Vec<Vec<usize>>, tree: Tree) -> usize {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut left_view = 0;
    let mut right_view = 0;
    let mut up_view = 0;
    let mut down_view = 0;

    let compare = |row: usize, col: usize| grid[row][col] >= grid[tree.row][tree.col];

    // look left
    for col in (0..tree.col).rev() {
        left_view += 1;
        if compare(tree.row, col) {
            break;
        };
    }

    // look right
    for col in tree.col + 1..columns {
        right_view += 1;
        if compare(tree.row, col) {
            break;
        };
    }

    // look up
    for row in (0..tree.row).rev() {
        up_view += 1;
        if compare(row, tree.col) {
            break;
        };
    }

    // look down
    for row in tree.row + 1..rows {
        down_view += 1;
        if compare(row, tree.col) {
            break;
        };
    }

    left_view * right_view * up_view * down_view
}

fn calculate_max_view_score(grid: &Vec<Vec<usize>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(row, row_vec)| {
            row_vec.iter().enumerate().map(move |(col, height)| Tree {
                height: *height,
                row,
                col,
            })
        })
        .flatten()
        .map(|tree| calculate_tree_visibility(&grid, tree))
        .max()
        .expect("no Tree view values found")
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;
    let file = BufReader::new(File::open(filename)?);

    let grid = build_grid(file)?;
    let visible_trees = get_visible_trees(&grid);

    let part1_count = visible_trees.len();
    println!("part1, count of visible trees: {part1_count}");

    let max_view_score = calculate_max_view_score(&grid);
    println!("part2, max view score: {max_view_score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"
30373
25512
65332
33549
35390
"#;

    #[test]
    fn test_example_part1() {
        let data = BufReader::new(DATA.as_bytes());

        let grid = build_grid(data).unwrap();

        // assert row and column lengths
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 5);
        assert_eq!(get_visible_trees(&grid).len(), 21);
    }

    #[test]
    fn test_example_part2() {
        let data = BufReader::new(DATA.as_bytes());

        let grid = build_grid(data).unwrap();

        let max_view_score = calculate_max_view_score(&grid);
        assert_eq!(max_view_score, 8)
    }
}
