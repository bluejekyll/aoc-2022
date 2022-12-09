//! --- Day 5: Supply Stacks ---
//! The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
//!
//! The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:
//!
//!     [D]    
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//! In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.
//!
//! Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//!
//! [D]        
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:
//!
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//! Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:
//!
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//! Finally, one crate is moved from stack 1 to stack 2:
//!
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//! The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.
//!
//! After the rearrangement procedure completes, what crate ends up on top of each stack?
//!
//! Your puzzle answer was TBVFVDZPN.
//!
//! --- Part Two ---
//! As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.
//!
//! Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
//!
//! The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.
//!
//! Again considering the example above, the crates begin in the same configuration:
//!
//!     [D]    
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! Moving a single crate from stack 2 to stack 1 behaves the same as before:
//!
//! [D]        
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:
//!
//!         [D]
//!         [N]
//!     [C] [Z]
//!     [M] [P]
//!  1   2   3
//! Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:
//!
//!         [D]
//!         [N]
//! [C]     [Z]
//! [M]     [P]
//!  1   2   3
//! Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:
//!
//!         [D]
//!         [N]
//!         [Z]
//! [M] [C] [P]
//!  1   2   3
//! In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.
//!
//! Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
//!
//! Your puzzle answer was VLCWHTDSZ.

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

const ROW_WIDTH: usize = 9;

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

fn parse_column_offsets(s: &str) -> [usize; ROW_WIDTH] {
    let mut columns = [0usize; ROW_WIDTH];

    for (offset, ch) in s.char_indices().filter(|(_, ch)| !ch.is_whitespace()) {
        // we don't actuall need this...
        let col = ch.to_digit(10).expect("bad number for column") as usize - 1;
        columns[col] = offset;
    }

    columns
}

fn parse_stack_row(line: &str, column_offsets: &[usize; ROW_WIDTH]) -> [u8; ROW_WIDTH] {
    let mut row = [0u8; ROW_WIDTH];

    let line = line.as_bytes();

    // assuming the data is all clean :)
    for (idx, offset) in column_offsets.iter().enumerate() {
        let package = line[*offset];
        if package.is_ascii_alphabetic() {
            row[idx] = package;
        }
    }

    row
}

#[derive(Debug, Copy, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_move(line: &str) -> Move {
    let mut split = line.split_whitespace();

    assert_eq!(split.next().expect("missing 'move'"), "move");
    let count = usize::from_str_radix(split.next().expect("missing count"), 10).expect("bad count");
    assert_eq!(split.next().expect("missing 'from'"), "from");
    // our indexes are 0 indexed
    let from =
        usize::from_str_radix(split.next().expect("missing from"), 10).expect("bad from") - 1;
    assert_eq!(split.next().expect("missing 'to'"), "to");
    let to = usize::from_str_radix(split.next().expect("missing to"), 10).expect("bad to") - 1;

    Move { count, from, to }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;
    let file = BufReader::new(File::open(filename)?);

    // read all the stacks into a vec, we want to start processing on the final line.
    let mut stack_lines = Vec::<String>::new();
    let mut lines = file.lines();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            // there is a new line separator from the stacks to the next
            break;
        }

        stack_lines.push(line);
    }

    // now pull all the stacks, pop()ing will be bottom up.
    // first the column count
    let column_offsets = parse_column_offsets(&stack_lines.pop().expect("no column numbers"));
    let mut columns = Vec::<Vec<u8>>::with_capacity(column_offsets.len());
    columns.resize_with(ROW_WIDTH, || Vec::new());

    // initialize the stacks
    while let Some(line) = stack_lines.pop() {
        let row = parse_stack_row(&line, &column_offsets);

        for (idx, value) in row.iter().enumerate() {
            if *value != 0 {
                columns[idx].push(*value);
            }
        }
    }

    let mut columns_part2 = columns.clone();

    // collect all the moves
    let moves = lines
        .map(|line| line.unwrap())
        .map(|line| parse_move(&line))
        .collect::<Vec<Move>>();

    // now do all the swaps
    for mov in &moves {
        let split_at_idx = columns[mov.from].len() - mov.count;
        let taken = columns[mov.from].split_off(split_at_idx);

        for took in taken.into_iter().rev() {
            columns[mov.to].push(took);
        }
    }

    let top_of_stacks = columns
        .iter()
        .map(|stack| *stack.last().expect("nothing in this stack"))
        .collect::<Vec<_>>();

    let top_of_stacks = String::from_utf8(top_of_stacks).expect("bad characters");
    println!("part 1, top of stacks: {top_of_stacks}");

    // part 2
    for mov in &moves {
        let split_at_idx = columns_part2[mov.from].len() - mov.count;
        let taken = columns_part2[mov.from].split_off(split_at_idx);

        for took in taken.into_iter() {
            columns_part2[mov.to].push(took);
        }
    }

    let top_of_stacks = columns_part2
        .iter()
        .map(|stack| *stack.last().expect("nothing in this stack"))
        .collect::<Vec<_>>();

    let top_of_stacks = String::from_utf8(top_of_stacks).expect("bad characters");
    println!("part 2, top of stacks: {top_of_stacks}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_columns() {
        let columns = parse_column_offsets("1 2 3 4 5 6 7 8 9");

        assert_eq!(&columns, &[0, 2, 4, 6, 8, 10, 12, 14, 16]);
    }

    #[test]
    fn test_parse_stack_row() {
        let columns = parse_column_offsets("1 2 3 4 5 6 7 8 9");
        let row = parse_stack_row("D E A D B E E F Y", &columns);

        assert_eq!(&row, b"DEADBEEFY");
    }

    #[test]
    fn test_parse_move() {
        let mov = parse_move("move 1 from 2 to 3");

        assert_eq!(mov.count, 1);
        assert_eq!(mov.from, 1);
        assert_eq!(mov.to, 2);
    }
}
