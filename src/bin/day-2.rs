//! --- Day 2: Rock Paper Scissors ---
//! The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
//!
//! Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//!
//! Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
//!
//! The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
//!
//! The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
//!
//! Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
//!
//! For example, suppose you were given the following strategy guide:
//!
//! A Y
//! B X
//! C Z
//! This strategy guide predicts and recommends the following:
//!
//! In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
//! In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
//! The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
//! In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
//!
//! What would your total score be if everything goes exactly according to your strategy guide?
//!
//! --- Part Two ---
//! The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
//!
//! The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
//!
//! In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
//! In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
//! In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
//! Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
//!
//! Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
//!

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
#[clap(name = "Advent of Code", version, about)]
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
    println!("{}", env!("CARGO_PKG_NAME"));
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
