//! --- Day 6: Tuning Trouble ---
//! The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.
//!
//! As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the most important one to set up right now is the communication system.
//!
//! However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.
//!
//! As if inspired by comedic timing, the device emits a few colorful sparks.
//!
//! To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that the device receives one at a time.
//!
//! To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.
//!
//! The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.
//!
//! For example, suppose you receive the following datastream buffer:
//!
//! mjqjpqmgbljsphdztnvjfqwrcgsmlb
//! After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.
//!
//! The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.
//!
//! Here are a few more examples:
//!
//! bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
//! nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
//! nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
//! zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
//! How many characters need to be processed before the first start-of-packet marker is detected?
//!
//!
//! --- Part Two ---
//! Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.
//!
//! A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.
//!
//! Here are the first positions of start-of-message markers for all of the above examples:
//!
//! mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
//! bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
//! nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
//! nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
//! zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26
//! How many characters need to be processed before the first start-of-message marker is detected?
//!

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

const SINGAL_LEN: usize = 4;
const MESSAGE_LEN: usize = 14;

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

fn find_signal(buf: &[u8], unique_len: usize) -> usize {
    'outer: for i in 0..buf.len() {
        let slice = &buf[i..i + unique_len];
        assert_eq!(slice.len(), unique_len);

        for ch in slice.iter() {
            if slice.iter().filter(|search| ch == *search).count() > 1 {
                continue 'outer;
            }
        }

        // count looks good
        return i + unique_len;
    }

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;
    let mut file = BufReader::new(File::open(filename)?);

    let mut buf = String::new();

    file.read_line(&mut buf).expect("failed to read line");
    let buf = buf.into_bytes();

    let signal = find_signal(&buf, SINGAL_LEN);
    println!("part1, signal location {signal}");

    let message = find_signal(&buf, MESSAGE_LEN);
    println!("part2, message location {message}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_signal() {
        assert_eq!(
            find_signal(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", SINGAL_LEN),
            7
        );
        assert_eq!(find_signal(b"bvwbjplbgvbhsrlpgdmjqwftvncz", SINGAL_LEN), 5);
        assert_eq!(find_signal(b"nppdvjthqldpwncqszvftbrmjlhg", SINGAL_LEN), 6);
        assert_eq!(
            find_signal(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", SINGAL_LEN),
            10
        );
        assert_eq!(
            find_signal(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", SINGAL_LEN),
            11
        );
    }

    #[test]
    fn test_find_message() {
        assert_eq!(
            find_signal(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", MESSAGE_LEN),
            19
        );
        assert_eq!(
            find_signal(b"bvwbjplbgvbhsrlpgdmjqwftvncz", MESSAGE_LEN),
            23
        );
        assert_eq!(
            find_signal(b"nppdvjthqldpwncqszvftbrmjlhg", MESSAGE_LEN),
            23
        );
        assert_eq!(
            find_signal(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", MESSAGE_LEN),
            29
        );
        assert_eq!(
            find_signal(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", MESSAGE_LEN),
            26
        );
    }
}
