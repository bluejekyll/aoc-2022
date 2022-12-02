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

#[derive(Clone, Copy)]
#[repr(usize)]
enum End {
    Win,
    Lose,
    Draw,
}

impl FromStr for End {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s
            .chars()
            .next()
            .expect("expected a character in the string")
        {
            'X' => End::Lose,
            'Y' => End::Draw,
            'Z' => End::Win,
            r => panic!("Not an End: {}", r),
        };

        Ok(r)
    }
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

    fn choose_for_end(self, end: End) -> Self {
        match (self, end) {
            (Rochambeau::Rock, End::Draw) => Rochambeau::Rock,
            (Rochambeau::Rock, End::Lose) => Rochambeau::Scissors,
            (Rochambeau::Rock, End::Win) => Rochambeau::Paper,
            (Rochambeau::Paper, End::Win) => Rochambeau::Scissors,
            (Rochambeau::Paper, End::Draw) => Rochambeau::Paper,
            (Rochambeau::Paper, End::Lose) => Rochambeau::Rock,
            (Rochambeau::Scissors, End::Lose) => Rochambeau::Paper,
            (Rochambeau::Scissors, End::Win) => Rochambeau::Rock,
            (Rochambeau::Scissors, End::Draw) => Rochambeau::Scissors,
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

fn score_round_for_end(opponent: Rochambeau, end: End) -> usize {
    let player = opponent.choose_for_end(end);
    score_round(opponent, player)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 2");
    let args = Cli::parse();

    let filename = &args.file;

    let file = BufReader::new(File::open(filename)?);

    let score = file
        .lines()
        .map(|s| s.expect("i/o error"))
        .filter(|s| !s.is_empty())
        .map(|s| {
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
            score
        });

    println!("Total score, part 1: {score}");

    // part 2
    let file = BufReader::new(File::open(filename)?);

    let score = file
        .lines()
        .map(|s| s.expect("i/o error"))
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut split = s.split(' ');
            (
                Rochambeau::from_str(split.next().expect("no opponent play"))
                    .expect("not a Rochambeau"),
                End::from_str(split.next().expect("no player play")).expect("not a Rochambeau"),
            )
        })
        .fold(0usize, |score, (opponent, end)| {
            score + score_round_for_end(opponent, end)
        });

    println!("Total score, part 2: {score}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data_part_1() {
        assert_eq!(score_round(Rochambeau::Rock, Rochambeau::Paper), 8);
        assert_eq!(score_round(Rochambeau::Paper, Rochambeau::Rock), 1);
        assert_eq!(score_round(Rochambeau::Scissors, Rochambeau::Scissors), 6);
    }

    #[test]
    fn test_example_data_part_2() {
        assert_eq!(score_round_for_end(Rochambeau::Rock, End::Draw), 4);
        assert_eq!(score_round_for_end(Rochambeau::Paper, End::Lose), 1);
        assert_eq!(score_round_for_end(Rochambeau::Scissors, End::Win), 7);
    }
}
