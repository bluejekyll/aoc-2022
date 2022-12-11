//! --- Day 11: Monkey in the Middle ---
//! As you finally start making your way upriver, you realize your pack is much lighter than you remember. Just then, one of the items from your pack goes flying overhead. Monkeys are playing Keep Away with your missing things!
//!
//! To get your stuff back, you need to be able to predict where the monkeys will throw your items. After some careful observation, you realize the monkeys operate based on how worried you are about each item.
//!
//! You take some notes (your puzzle input) on the items each monkey currently has, how worried you are about those items, and how the monkey makes decisions based on your worry level. For example:
//!
//! Monkey 0:
//! Starting items: 79, 98
//! Operation: new = old * 19
//! Test: divisible by 23
//! If true: throw to monkey 2
//! If false: throw to monkey 3
//!
//! Monkey 1:
//! Starting items: 54, 65, 75, 74
//! Operation: new = old + 6
//! Test: divisible by 19
//! If true: throw to monkey 2
//! If false: throw to monkey 0
//!
//! Monkey 2:
//! Starting items: 79, 60, 97
//! Operation: new = old * old
//! Test: divisible by 13
//! If true: throw to monkey 1
//! If false: throw to monkey 3
//!
//! Monkey 3:
//! Starting items: 74
//! Operation: new = old + 3
//! Test: divisible by 17
//! If true: throw to monkey 0
//! If false: throw to monkey 1
//! Each monkey has several attributes:
//!
//! Starting items lists your worry level for each item the monkey is currently holding in the order they will be inspected.
//! Operation shows how your worry level changes as that monkey inspects an item. (An operation like new = old * 5 means that your worry level after the monkey inspected the item is five times whatever your worry level was before inspection.)
//! Test shows how the monkey uses your worry level to decide where to throw an item next.
//! If true shows what happens with an item if the Test was true.
//! If false shows what happens with an item if the Test was false.
//! After each monkey inspects an item but before it tests your worry level, your relief that the monkey's inspection didn't damage the item causes your worry level to be divided by three and rounded down to the nearest integer.
//!
//! The monkeys take turns inspecting and throwing items. On a single monkey's turn, it inspects and throws all of the items it is holding one at a time and in the order listed. Monkey 0 goes first, then monkey 1, and so on until each monkey has had one turn. The process of each monkey taking a single turn is called a round.
//!
//! When a monkey throws an item to another monkey, the item goes on the end of the recipient monkey's list. A monkey that starts a round with no items could end up inspecting and throwing many items by the time its turn comes around. If a monkey is holding no items at the start of its turn, its turn ends.
//!
//! In the above example, the first round proceeds as follows:
//!
//! Monkey 0:
//! Monkey inspects an item with a worry level of 79.
//! Worry level is multiplied by 19 to 1501.
//! Monkey gets bored with item. Worry level is divided by 3 to 500.
//! Current worry level is not divisible by 23.
//! Item with worry level 500 is thrown to monkey 3.
//! Monkey inspects an item with a worry level of 98.
//! Worry level is multiplied by 19 to 1862.
//! Monkey gets bored with item. Worry level is divided by 3 to 620.
//! Current worry level is not divisible by 23.
//! Item with worry level 620 is thrown to monkey 3.
//! Monkey 1:
//! Monkey inspects an item with a worry level of 54.
//! Worry level increases by 6 to 60.
//! Monkey gets bored with item. Worry level is divided by 3 to 20.
//! Current worry level is not divisible by 19.
//! Item with worry level 20 is thrown to monkey 0.
//! Monkey inspects an item with a worry level of 65.
//! Worry level increases by 6 to 71.
//! Monkey gets bored with item. Worry level is divided by 3 to 23.
//! Current worry level is not divisible by 19.
//! Item with worry level 23 is thrown to monkey 0.
//! Monkey inspects an item with a worry level of 75.
//! Worry level increases by 6 to 81.
//! Monkey gets bored with item. Worry level is divided by 3 to 27.
//! Current worry level is not divisible by 19.
//! Item with worry level 27 is thrown to monkey 0.
//! Monkey inspects an item with a worry level of 74.
//! Worry level increases by 6 to 80.
//! Monkey gets bored with item. Worry level is divided by 3 to 26.
//! Current worry level is not divisible by 19.
//! Item with worry level 26 is thrown to monkey 0.
//! Monkey 2:
//! Monkey inspects an item with a worry level of 79.
//! Worry level is multiplied by itself to 6241.
//! Monkey gets bored with item. Worry level is divided by 3 to 2080.
//! Current worry level is divisible by 13.
//! Item with worry level 2080 is thrown to monkey 1.
//! Monkey inspects an item with a worry level of 60.
//! Worry level is multiplied by itself to 3600.
//! Monkey gets bored with item. Worry level is divided by 3 to 1200.
//! Current worry level is not divisible by 13.
//! Item with worry level 1200 is thrown to monkey 3.
//! Monkey inspects an item with a worry level of 97.
//! Worry level is multiplied by itself to 9409.
//! Monkey gets bored with item. Worry level is divided by 3 to 3136.
//! Current worry level is not divisible by 13.
//! Item with worry level 3136 is thrown to monkey 3.
//! Monkey 3:
//! Monkey inspects an item with a worry level of 74.
//! Worry level increases by 3 to 77.
//! Monkey gets bored with item. Worry level is divided by 3 to 25.
//! Current worry level is not divisible by 17.
//! Item with worry level 25 is thrown to monkey 1.
//! Monkey inspects an item with a worry level of 500.
//! Worry level increases by 3 to 503.
//! Monkey gets bored with item. Worry level is divided by 3 to 167.
//! Current worry level is not divisible by 17.
//! Item with worry level 167 is thrown to monkey 1.
//! Monkey inspects an item with a worry level of 620.
//! Worry level increases by 3 to 623.
//! Monkey gets bored with item. Worry level is divided by 3 to 207.
//! Current worry level is not divisible by 17.
//! Item with worry level 207 is thrown to monkey 1.
//! Monkey inspects an item with a worry level of 1200.
//! Worry level increases by 3 to 1203.
//! Monkey gets bored with item. Worry level is divided by 3 to 401.
//! Current worry level is not divisible by 17.
//! Item with worry level 401 is thrown to monkey 1.
//! Monkey inspects an item with a worry level of 3136.
//! Worry level increases by 3 to 3139.
//! Monkey gets bored with item. Worry level is divided by 3 to 1046.
//! Current worry level is not divisible by 17.
//! Item with worry level 1046 is thrown to monkey 1.
//! After round 1, the monkeys are holding items with these worry levels:
//!
//! Monkey 0: 20, 23, 27, 26
//! Monkey 1: 2080, 25, 167, 207, 401, 1046
//! Monkey 2:
//! Monkey 3:
//! Monkeys 2 and 3 aren't holding any items at the end of the round; they both inspected items during the round and threw them all before the round ended.
//!
//! This process continues for a few more rounds:
//!
//! After round 2, the monkeys are holding items with these worry levels:
//! Monkey 0: 695, 10, 71, 135, 350
//! Monkey 1: 43, 49, 58, 55, 362
//! Monkey 2:
//! Monkey 3:
//!
//! After round 3, the monkeys are holding items with these worry levels:
//! Monkey 0: 16, 18, 21, 20, 122
//! Monkey 1: 1468, 22, 150, 286, 739
//! Monkey 2:
//! Monkey 3:
//!
//! After round 4, the monkeys are holding items with these worry levels:
//! Monkey 0: 491, 9, 52, 97, 248, 34
//! Monkey 1: 39, 45, 43, 258
//! Monkey 2:
//! Monkey 3:
//!
//! After round 5, the monkeys are holding items with these worry levels:
//! Monkey 0: 15, 17, 16, 88, 1037
//! Monkey 1: 20, 110, 205, 524, 72
//! Monkey 2:
//! Monkey 3:
//!
//! After round 6, the monkeys are holding items with these worry levels:
//! Monkey 0: 8, 70, 176, 26, 34
//! Monkey 1: 481, 32, 36, 186, 2190
//! Monkey 2:
//! Monkey 3:
//!
//! After round 7, the monkeys are holding items with these worry levels:
//! Monkey 0: 162, 12, 14, 64, 732, 17
//! Monkey 1: 148, 372, 55, 72
//! Monkey 2:
//! Monkey 3:
//!
//! After round 8, the monkeys are holding items with these worry levels:
//! Monkey 0: 51, 126, 20, 26, 136
//! Monkey 1: 343, 26, 30, 1546, 36
//! Monkey 2:
//! Monkey 3:
//!
//! After round 9, the monkeys are holding items with these worry levels:
//! Monkey 0: 116, 10, 12, 517, 14
//! Monkey 1: 108, 267, 43, 55, 288
//! Monkey 2:
//! Monkey 3:
//!
//! After round 10, the monkeys are holding items with these worry levels:
//! Monkey 0: 91, 16, 20, 98
//! Monkey 1: 481, 245, 22, 26, 1092, 30
//! Monkey 2:
//! Monkey 3:
//!
//! ...
//!
//! After round 15, the monkeys are holding items with these worry levels:
//! Monkey 0: 83, 44, 8, 184, 9, 20, 26, 102
//! Monkey 1: 110, 36
//! Monkey 2:
//! Monkey 3:
//!
//! ...
//!
//! After round 20, the monkeys are holding items with these worry levels:
//! Monkey 0: 10, 12, 14, 26, 34
//! Monkey 1: 245, 93, 53, 199, 115
//! Monkey 2:
//! Monkey 3:
//! Chasing all of the monkeys at once is impossible; you're going to have to focus on the two most active monkeys if you want any hope of getting your stuff back. Count the total number of times each monkey inspects items over 20 rounds:
//!
//! Monkey 0 inspected items 101 times.
//! Monkey 1 inspected items 95 times.
//! Monkey 2 inspected items 7 times.
//! Monkey 3 inspected items 105 times.
//! In this example, the two most active monkeys inspected items 101 and 105 times. The level of monkey business in this situation can be found by multiplying these together: 10605.
//!
//! Figure out which monkeys to chase by counting how many items they inspect over 20 rounds. What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
//!
//! --- Part Two ---
//! You're worried you might not ever get your items back. So worried, in fact, that your relief that a monkey's inspection didn't damage an item no longer causes your worry level to be divided by three.
//!
//! Unfortunately, that relief was all that was keeping your worry levels from reaching ridiculous levels. You'll need to find another way to keep your worry levels manageable.
//!
//! At this rate, you might be putting up with these monkeys for a very long time - possibly 10000 rounds!
//!
//! With these new rules, you can still figure out the monkey business after 10000 rounds. Using the same example above:
//!
//! == After round 1 ==
//! Monkey 0 inspected items 2 times.
//! Monkey 1 inspected items 4 times.
//! Monkey 2 inspected items 3 times.
//! Monkey 3 inspected items 6 times.
//!
//! == After round 20 ==
//! Monkey 0 inspected items 99 times.
//! Monkey 1 inspected items 97 times.
//! Monkey 2 inspected items 8 times.
//! Monkey 3 inspected items 103 times.
//!
//! == After round 1000 ==
//! Monkey 0 inspected items 5204 times.
//! Monkey 1 inspected items 4792 times.
//! Monkey 2 inspected items 199 times.
//! Monkey 3 inspected items 5192 times.
//!
//! == After round 2000 ==
//! Monkey 0 inspected items 10419 times.
//! Monkey 1 inspected items 9577 times.
//! Monkey 2 inspected items 392 times.
//! Monkey 3 inspected items 10391 times.
//!
//! == After round 3000 ==
//! Monkey 0 inspected items 15638 times.
//! Monkey 1 inspected items 14358 times.
//! Monkey 2 inspected items 587 times.
//! Monkey 3 inspected items 15593 times.
//!
//! == After round 4000 ==
//! Monkey 0 inspected items 20858 times.
//! Monkey 1 inspected items 19138 times.
//! Monkey 2 inspected items 780 times.
//! Monkey 3 inspected items 20797 times.
//!
//! == After round 5000 ==
//! Monkey 0 inspected items 26075 times.
//! Monkey 1 inspected items 23921 times.
//! Monkey 2 inspected items 974 times.
//! Monkey 3 inspected items 26000 times.
//!
//! == After round 6000 ==
//! Monkey 0 inspected items 31294 times.
//! Monkey 1 inspected items 28702 times.
//! Monkey 2 inspected items 1165 times.
//! Monkey 3 inspected items 31204 times.
//!
//! == After round 7000 ==
//! Monkey 0 inspected items 36508 times.
//! Monkey 1 inspected items 33488 times.
//! Monkey 2 inspected items 1360 times.
//! Monkey 3 inspected items 36400 times.
//!
//! == After round 8000 ==
//! Monkey 0 inspected items 41728 times.
//! Monkey 1 inspected items 38268 times.
//! Monkey 2 inspected items 1553 times.
//! Monkey 3 inspected items 41606 times.
//!
//! == After round 9000 ==
//! Monkey 0 inspected items 46945 times.
//! Monkey 1 inspected items 43051 times.
//! Monkey 2 inspected items 1746 times.
//! Monkey 3 inspected items 46807 times.
//!
//! == After round 10000 ==
//! Monkey 0 inspected items 52166 times.
//! Monkey 1 inspected items 47830 times.
//! Monkey 2 inspected items 1938 times.
//! Monkey 3 inspected items 52013 times.
//! After 10000 rounds, the two most active monkeys inspected items 52166 and 52013 times. Multiplying these together, the level of monkey business in this situation is now 2713310158.
//!
//! Worry levels are no longer divided by three after each item is inspected; you'll need to find another way to keep your worry levels manageable. Starting again from the initial state in your puzzle input, what is the level of monkey business after 10000 rounds?

use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

/// Cli
#[derive(Debug, Parser)]
#[clap(name = "Advent of Code", version, about)]
struct Cli {
    /// Disable INFO messages, WARN and ERROR will remain
    #[clap(short = 'f', long = "file")]
    pub(crate) file: String,
}

struct Item {
    starting_monkey_id: usize,
    initial_worry: u128,
    worry: u128,
}

impl Item {
    fn calculate_new_worry(
        &mut self,
        current_monkey_id: usize,
        anxiety: &Instruction,
        reduce_anxiety: bool,
    ) {
        if self.starting_monkey_id == current_monkey_id && !reduce_anxiety {
            // reset to the original worry value
            self.worry = self.initial_worry;
        }

        // raise the anxiety
        let mut new_worry = anxiety.inspection_score(self.worry);
        // wow, it's still ok, divide by 3
        if reduce_anxiety {
            new_worry /= 3;
        }

        self.worry = new_worry;
    }
}

struct Monkey {
    id: usize,
    inspected_items_count: usize,
    items: Vec<Item>,
    anxiety: Instruction,
    test: Test,
}

//   Starting items: 79, 98
fn parse_items(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = space1(input)?;
    let (input, _) = tag("Starting items: ")(input)?;

    let (input, items) = fold_many0(
        pair(
            character::complete::u32,
            opt(pair(character::complete::char(','), space1)),
        ),
        Vec::new,
        |mut items, (item, _)| {
            items.push(item as usize);
            items
        },
    )(input)?;

    let (input, _) = opt(newline)(input)?;
    Ok((input, items))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Multiply,
    Sum,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    arg1: Literal,
    op: Operation,
    arg2: Literal,
}

impl Instruction {
    fn inspection_score(&self, worry: u128) -> u128 {
        let arg1 = match self.arg1 {
            Literal::Num(val) => val as u128,
            Literal::Old => worry.clone(),
        };
        let arg2 = match self.arg2 {
            Literal::Num(val) => val as u128,
            Literal::Old => worry,
        };

        match self.op {
            Operation::Multiply => arg1 * arg2,
            Operation::Sum => arg1 + arg2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Literal {
    Old,
    Num(usize),
}

// old|{number}
fn parse_literal(input: &str) -> IResult<&str, Literal> {
    if let Some((input, _)) = tag::<_, _, nom::error::Error<&str>>("old")(input).ok() {
        return Ok((input, Literal::Old));
    }

    character::complete::u32(input).map(|(input, val)| (input, Literal::Num(val as usize)))
}

//   Operation: new = old * 19
fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = space1(input)?;
    let (input, _) = tag("Operation:")(input)?;

    let (input, _) = preceded(space1, tag("new"))(input)?;
    let (input, _) = tuple((space1, character::complete::char('='), space1))(input)?;

    let (input, var1) = alphanumeric1(input)?;
    let (input, operation) = preceded(space1, anychar)(input)?;
    let (input, var2) = preceded(space1, alphanumeric1)(input)?;
    let (input, _) = opt(newline)(input)?;

    let arg1 = parse_literal(var1)?.1;
    let arg2 = parse_literal(var2)?.1;

    let op = match operation {
        '+' => Operation::Sum,
        '*' => Operation::Multiply,
        _ => panic!("unrecognized operation: {operation}"),
    };

    Ok((input, Instruction { arg1, op, arg2 }))
}

struct Test {
    divisor: usize,
    true_monkey: usize,
    false_monkey: usize,
}

//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, _) = preceded(space1, tag("Test:"))(input)?;
    let (input, _) = preceded(space1, tag("divisible"))(input)?;
    let (input, _) = preceded(space1, tag("by"))(input)?;
    let (input, _) = space1(input)?;
    let (input, divisor) = map(character::complete::u32, |val| val as usize)(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = preceded(space1, tag("If"))(input)?;
    let (input, _) = preceded(space1, tag("true:"))(input)?;
    let (input, _) = preceded(space1, tag("throw"))(input)?;
    let (input, _) = preceded(space1, tag("to"))(input)?;
    let (input, _) = preceded(space1, tag("monkey"))(input)?;
    let (input, _) = space1(input)?;
    let (input, true_monkey) = map(character::complete::u32, |val| val as usize)(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = preceded(space1, tag("If"))(input)?;
    let (input, _) = preceded(space1, tag("false:"))(input)?;
    let (input, _) = preceded(space1, tag("throw"))(input)?;
    let (input, _) = preceded(space1, tag("to"))(input)?;
    let (input, _) = preceded(space1, tag("monkey"))(input)?;
    let (input, _) = space1(input)?;
    let (input, false_monkey) = map(character::complete::u32, |val| val as usize)(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        Test {
            divisor,
            true_monkey,
            false_monkey,
        },
    ))
}

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("Monkey")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = character::complete::u32(input)?;
    let (input, _) = character::complete::char(':')(input)?;
    let (input, _) = newline(input)?;

    let (input, items) = parse_items(input)?;
    let (input, anxiety) = parse_instruction(input)?;
    let (input, test) = parse_test(input)?;

    let id = id as usize;
    let items = items
        .into_iter()
        .map(|worry| Item {
            starting_monkey_id: id,
            initial_worry: worry as u128,
            worry: worry as u128,
        })
        .collect();

    Ok((
        input,
        Monkey {
            id,
            inspected_items_count: 0,
            items,
            anxiety,
            test,
        },
    ))
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let (_input, monkeys) = many1(parse_monkey)(input).expect("failed to parse monkeys");

    monkeys
}

fn monkey_business(monkeys: &mut [Monkey], rounds: usize, reduce_anxiety: bool) {
    // rounds
    for _ in 0..rounds {
        // (target index, item worry score)
        let mut items_thrown_to = Vec::<(usize, Item)>::new();

        for m in 0..monkeys.len() {
            // mut area
            {
                let monkey = monkeys.get_mut(m).unwrap();
                let anxiety = &monkey.anxiety;
                let test = &monkey.test;

                // inspect all items
                for mut item in monkey.items.drain(..) {
                    monkey.inspected_items_count += 1;

                    item.calculate_new_worry(monkey.id, anxiety, reduce_anxiety);

                    if &item.worry % test.divisor as u128 == 0 {
                        items_thrown_to.push((monkey.test.true_monkey, item));
                    } else {
                        items_thrown_to.push((monkey.test.false_monkey, item));
                    }
                }
            }

            // complete toss to the other monkeys
            for (to_monkey, item) in items_thrown_to.drain(..) {
                monkeys[to_monkey].items.push(item);
            }
        }
    }
}

fn calculate_top_monkey_bussiness(monkeys: &[Monkey]) -> usize {
    let mut scores = monkeys
        .iter()
        .map(|monkey| monkey.inspected_items_count)
        .collect::<Vec<_>>();

    scores.sort_unstable();
    scores.iter().rev().take(2).product()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let reader = BufReader::new(File::open(filename)?);
    let mut input = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        writeln!(input, "{line}")?;
    }

    // got all the monkeys
    let mut monkeys = parse_monkeys(&input);
    monkey_business(&mut monkeys, 20, true);

    let total_mb = calculate_top_monkey_bussiness(&monkeys);
    println!("part 1, monkey business product: {total_mb}");

    let mut monkeys = parse_monkeys(&input);
    monkey_business(&mut monkeys, 10000, false);

    let total_mb = calculate_top_monkey_bussiness(&monkeys);
    println!("part 2, monkey business product: {total_mb}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
  
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
  
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
  
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    fn test_parse_items() {
        assert_eq!(parse_items("  Starting items: 74").unwrap().1, &[74usize]);
        assert_eq!(
            parse_items("  Starting items: 79, 60, 97").unwrap().1,
            &[79usize, 60usize, 97usize]
        );
    }

    #[test]
    fn test_parse_instruction() {
        let instruction = parse_instruction("  Operation: new = old * 19").unwrap().1;

        assert_eq!(
            instruction,
            Instruction {
                arg1: Literal::Old,
                op: Operation::Multiply,
                arg2: Literal::Num(19),
            }
        )
    }

    #[test]
    fn test_parse_test() {
        parse_test(
            r#"   Test: divisible by 23
             If true: throw to monkey 2
             If false: throw to monkey 3
"#,
        )
        .unwrap();
    }

    #[test]
    fn test_part1_input() {
        let mut monkeys = parse_monkeys(INPUT);

        monkey_business(&mut monkeys, 20, true);

        assert_eq!(monkeys[0].inspected_items_count, 101);
        assert_eq!(monkeys[1].inspected_items_count, 95);
        assert_eq!(monkeys[2].inspected_items_count, 7);
        assert_eq!(monkeys[3].inspected_items_count, 105);

        assert_eq!(calculate_top_monkey_bussiness(&monkeys), 10605);
    }

    #[test]
    fn test_part2_input() {
        let mut monkeys = parse_monkeys(INPUT);

        monkey_business(&mut monkeys, 10000, false);

        assert_eq!(monkeys[0].inspected_items_count, 52166);
        assert_eq!(monkeys[1].inspected_items_count, 47830);
        assert_eq!(monkeys[2].inspected_items_count, 1938);
        assert_eq!(monkeys[3].inspected_items_count, 52013);

        assert_eq!(calculate_top_monkey_bussiness(&monkeys), 2713310158);
    }
}
