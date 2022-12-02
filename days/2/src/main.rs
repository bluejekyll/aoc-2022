use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use clap::Parser;

const WIN: usize = 6;
const DRAW: usize = 3;
const LOSS: usize = 0;

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Day 1", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

#[derive(Clone, Copy)]
#[repr(usize)]
enum Rochambeau {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Rochambeau {
    fn play(self, opponent: Rochambeau) -> usize {
        match (self, opponent) {
            (Rochambeau::Rock, Rochambeau::Rock) => DRAW,
            (Rochambeau::Rock, Rochambeau::Paper) => LOSS,
            (Rochambeau::Rock, Rochambeau::Scissors) => WIN,
            (Rochambeau::Paper, Rochambeau::Rock) => WIN,
            (Rochambeau::Paper, Rochambeau::Paper) => DRAW,
            (Rochambeau::Paper, Rochambeau::Scissors) => LOSS,
            (Rochambeau::Scissors, Rochambeau::Rock) => LOSS,
            (Rochambeau::Scissors, Rochambeau::Paper) => WIN,
            (Rochambeau::Scissors, Rochambeau::Scissors) => DRAW,
        }
    }
}

impl FromStr for Rochambeau {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s
            .chars()
            .next()
            .expect("expected a character in the string")
        {
            'A' | 'X' => Rochambeau::Rock,
            'B' | 'Y' => Rochambeau::Paper,
            'C' | 'Z' => Rochambeau::Scissors,
            r => panic!("Not a Rochambeau: {}", r),
        };

        Ok(r)
    }
}

fn score_round(opponent: Rochambeau, player: Rochambeau) -> usize {
    let mut score = 0;
    score += player.play(opponent);
    score += player as usize;
    score
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 2");
    let args = Cli::parse();

    let filename = &args.file;

    let file = BufReader::new(File::open(filename)?);

    let score = file
        .lines()
        .map(|s| s.expect("i/o error"))
        .enumerate()
        .filter(|(_, s)| !s.is_empty())
        .map(|(line, s)| {
            print!("iter: {line} play: {s} ");
            let mut split = s.split(' ');
            (
                Rochambeau::from_str(split.next().expect("no opponent play"))
                    .expect("not a Rochambeau"),
                Rochambeau::from_str(split.next().expect("no player play"))
                    .expect("not a Rochambeau"),
            )
        })
        .fold(0usize, |mut score, (opponent, player)| {
            let play = score_round(opponent, player);
            score += play;
            println!("play: {play} tally: {score}");
            score
        });

    println!("Total score: {score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(score_round(Rochambeau::Rock, Rochambeau::Paper), 8);
        assert_eq!(score_round(Rochambeau::Paper, Rochambeau::Rock), 1);
        assert_eq!(score_round(Rochambeau::Scissors, Rochambeau::Scissors), 6);
    }
}
