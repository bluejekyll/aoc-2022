//! --- Day 15: Beacon Exclusion Zone ---
//! You feel the ground rumble again as the distress signal leads you to a large network of subterranean tunnels. You don't have time to search them all, but you don't need to: your pack contains a set of deployable sensors that you imagine were originally built to locate lost Elves.
//!
//! The sensors aren't very powerful, but that's okay; your handheld device indicates that you're close enough to the source of the distress signal to use them. You pull the emergency sensor system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.
//!
//! Once a sensor finds a spot it thinks will give it a good reading, it attaches itself to a hard surface and begins monitoring for the nearest signal source beacon. Sensors and beacons always exist at integer coordinates. Each sensor knows its own position and can determine the position of a beacon precisely; however, sensors can only lock on to the one beacon closest to the sensor as measured by the Manhattan distance. (There is never a tie where two beacons are the same distance to a sensor.)
//!
//! It doesn't take long for the sensors to report back their positions and closest beacons (your puzzle input). For example:
//!
//! Sensor at x=2, y=18: closest beacon is at x=-2, y=15
//! Sensor at x=9, y=16: closest beacon is at x=10, y=16
//! Sensor at x=13, y=2: closest beacon is at x=15, y=3
//! Sensor at x=12, y=14: closest beacon is at x=10, y=16
//! Sensor at x=10, y=20: closest beacon is at x=10, y=16
//! Sensor at x=14, y=17: closest beacon is at x=10, y=16
//! Sensor at x=8, y=7: closest beacon is at x=2, y=10
//! Sensor at x=2, y=0: closest beacon is at x=2, y=10
//! Sensor at x=0, y=11: closest beacon is at x=2, y=10
//! Sensor at x=20, y=14: closest beacon is at x=25, y=17
//! Sensor at x=17, y=20: closest beacon is at x=21, y=22
//! Sensor at x=16, y=7: closest beacon is at x=15, y=3
//! Sensor at x=14, y=3: closest beacon is at x=15, y=3
//! Sensor at x=20, y=1: closest beacon is at x=15, y=3
//! So, consider the sensor at 2,18; the closest beacon to it is at -2,15. For the sensor at 9,16, the closest beacon to it is at 10,16.
//!
//! Drawing sensors as S and beacons as B, the above arrangement of sensors and beacons looks like this:
//!
//!                1    1    2    2
//!      0    5    0    5    0    5
//!  0 ....S.......................
//!  1 ......................S.....
//!  2 ...............S............
//!  3 ................SB..........
//!  4 ............................
//!  5 ............................
//!  6 ............................
//!  7 ..........S.......S.........
//!  8 ............................
//!  9 ............................
//! 10 ....B.......................
//! 11 ..S.........................
//! 12 ............................
//! 13 ............................
//! 14 ..............S.......S.....
//! 15 B...........................
//! 16 ...........SB...............
//! 17 ................S..........B
//! 18 ....S.......................
//! 19 ............................
//! 20 ............S......S........
//! 21 ............................
//! 22 .......................B....
//! This isn't necessarily a comprehensive map of all beacons in the area, though. Because each sensor only identifies its closest beacon, if a sensor detects a beacon, you know there are no other beacons that close or closer to that sensor. There could still be beacons that just happen to not be the closest beacon to any sensor. Consider the sensor at 8,7:
//!
//!                1    1    2    2
//!      0    5    0    5    0    5
//! -2 ..........#.................
//! -1 .........###................
//!  0 ....S...#####...............
//!  1 .......#######........S.....
//!  2 ......#########S............
//!  3 .....###########SB..........
//!  4 ....#############...........
//!  5 ...###############..........
//!  6 ..#################.........
//!  7 .#########S#######S#........
//!  8 ..#################.........
//!  9 ...###############..........
//! 10 ....B############...........
//! 11 ..S..###########............
//! 12 ......#########.............
//! 13 .......#######..............
//! 14 ........#####.S.......S.....
//! 15 B........###................
//! 16 ..........#SB...............
//! 17 ................S..........B
//! 18 ....S.......................
//! 19 ............................
//! 20 ............S......S........
//! 21 ............................
//! 22 .......................B....
//! This sensor's closest beacon is at 2,10, and so you know there are no beacons that close or closer (in any positions marked #).
//!
//! None of the detected beacons seem to be producing the distress signal, so you'll need to work out where the distress beacon is by working out where it isn't. For now, keep things simple by counting the positions where a beacon cannot possibly be along just a single row.
//!
//! So, suppose you have an arrangement of beacons and sensors like in the example above and, just in the row where y=10, you'd like to count the number of positions a beacon cannot possibly exist. The coverage from all sensors near that row looks like this:
//!
//!                  1    1    2    2
//!        0    5    0    5    0    5
//!  9 ...#########################...
//! 10 ..####B######################..
//! 11 .###S#############.###########.
//! In this example, in the row where y=10, there are 26 positions where a beacon cannot be present.
//!
//! Consult the report from the sensors you just deployed. In the row where y=2000000, how many positions cannot contain a beacon?
//!
//! --- Part Two ---
//! Your handheld device indicates that the distress signal is coming from a beacon nearby. The distress beacon is not detected by any sensor, but the distress beacon must have x and y coordinates each no lower than 0 and no larger than 4000000.
//!
//! To isolate the distress beacon's signal, you need to determine its tuning frequency, which can be found by multiplying its x coordinate by 4000000 and then adding its y coordinate.
//!
//! In the example above, the search space is smaller: instead, the x and y coordinates can each be at most 20. With this reduced search area, there is only a single position that could have a beacon: x=14, y=11. The tuning frequency for this distress beacon is 56000011.
//!
//! Find the only possible position for the distress beacon. What is its tuning frequency?

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

use clap::Parser;
use nom::sequence::preceded;
use nom::{bytes::complete::tag, character, IResult};
use rayon::prelude::*;

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
    x: isize,
    y: isize,
}

impl Point {
    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    location: Point,
    closest_beacon: Beacon,
}

impl Sensor {
    fn range(&self) -> usize {
        self.location.distance(&self.closest_beacon.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Beacon(Point);

impl Beacon {
    fn frequency(&self) -> usize {
        (self.0.x as usize * 4000000) + (self.0.y as usize)
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = preceded(tag("x="), character::complete::i32)(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, y) = preceded(tag("y="), character::complete::i32)(input)?;

    Ok((
        input,
        Point {
            x: x as isize,
            y: y as isize,
        },
    ))
}

fn parse_sensor_and_beacon(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = parse_point(input)?;

    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon) = parse_point(input)?;
    Ok((
        input,
        Sensor {
            location: sensor,
            closest_beacon: Beacon(beacon),
        },
    ))
}

fn parse_sensors(reader: impl BufRead) -> Vec<Sensor> {
    reader
        .lines()
        .filter_map(|s| s.ok())
        .filter(|s| !s.is_empty())
        .map(|s| parse_sensor_and_beacon(&s).expect("bad data").1)
        .collect()
}

fn get_beacons(sensors: &[Sensor]) -> Vec<Beacon> {
    sensors
        .iter()
        .map(|sensor| sensor.closest_beacon.clone())
        .collect::<Vec<_>>()
}

fn is_excluded(sensors: &[Sensor], beacons: &[Beacon], point: &Point) -> bool {
    if beacons.contains(&Beacon(point.clone())) {
        return false;
    }

    sensors
        .iter()
        .filter(|sensor| sensor.closest_beacon.0 != *point)
        .any(|sensor| sensor.location.distance(&point) <= sensor.range())
}

fn is_excluded2(sensors: &[Sensor], point: &Point) -> Option<Sensor> {
    sensors
        .iter()
        .filter(|sensor| sensor.closest_beacon.0 != *point)
        .find(|sensor| sensor.location.distance(&point) <= sensor.range())
        .cloned()
}

fn count_spaces_in_range(sensors: &[Sensor], beacons: &[Beacon], y: isize) -> usize {
    // get min and max of x to get grid size...
    let x_min = sensors
        .iter()
        .flat_map(|sensor| {
            [
                sensor.location.x - sensor.range() as isize,
                sensor.closest_beacon.0.x,
            ]
        })
        .min()
        .expect("no x coords");
    let x_max = sensors
        .iter()
        .flat_map(|sensor| {
            [
                sensor.location.x + sensor.range() as isize,
                sensor.closest_beacon.0.x,
            ]
        })
        .max()
        .expect("no x coords");

    let mut excluded_spaces_count = 0;
    for x in x_min..=x_max {
        let point = Point { x, y };
        if is_excluded(sensors, beacons, &point) {
            excluded_spaces_count += 1;
        };
    }

    excluded_spaces_count
}

fn locate_distress_beacon2(
    sensors: &[Sensor],
    beacons: &[Beacon],
    x_and_y_range: Range<isize>,
) -> Option<Beacon> {
    x_and_y_range
        .clone()
        .into_par_iter()
        .find_map_first(|y| {
            if y as usize % 1000 == 0 {
                dbg!(y);
            }
            let mut x = x_and_y_range.clone().min().expect("no min");

            while x < x_and_y_range.clone().max().expect("no max") {
                let point = Point { x, y };
                if let Some(sensor) = is_excluded2(sensors, &point) {
                    if sensor.location.x > x {
                        // skip to the furthest stop on x
                        x += sensor.range() as isize - sensor.location.y.abs_diff(y) as isize;
                    } else {
                        x += 1;
                    }
                } else {
                    if !beacons.contains(&Beacon(point.clone())) {
                        return Some(point);
                    } else {
                        x += 1;
                    }
                }
            }
            None

            // for x in x_and_y_range
            // .clone()

            // x_and_y_range
            //     .clone()
            //     .into_iter()
            //     .map(|x| Point { x, y })
            //     .find(|point| {
            //         !beacons.contains(&Beacon((point).clone()))
            //             && !is_excluded(sensors, beacons, point)
            //     })
        })
        .map(|point| Beacon(point))

    // x_and_y_range
    //     .clone()
    //     .flat_map(|x| x_and_y_range.clone().map(move |y| Point { x, y }))
    //     .into_iter()
    //     .find(|point| {
    //         !beacons.contains(&Beacon((*point).clone())) && !is_excluded(sensors, beacons, point)
    //     })
    //     .map(|point| Beacon(point.clone()))
}

#[allow(unused)]
fn locate_distress_beacon(
    sensors: &[Sensor],
    beacons: &[Beacon],
    x_and_y_range: Range<isize>,
) -> Option<Beacon> {
    x_and_y_range
        .clone()
        .into_par_iter()
        .find_map_first(|y| {
            x_and_y_range
                .clone()
                .into_iter()
                .map(|x| Point { x, y })
                .find(|point| {
                    !beacons.contains(&Beacon((point).clone()))
                        && !is_excluded(sensors, beacons, point)
                })
        })
        .map(|point| Beacon(point))

    // x_and_y_range
    //     .clone()
    //     .flat_map(|x| x_and_y_range.clone().map(move |y| Point { x, y }))
    //     .into_iter()
    //     .find(|point| {
    //         !beacons.contains(&Beacon((*point).clone())) && !is_excluded(sensors, beacons, point)
    //     })
    //     .map(|point| Beacon(point.clone()))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", env!("CARGO_PKG_NAME"));
    let args = Cli::parse();

    let filename = &args.file;

    let reader = BufReader::new(File::open(filename)?);
    let sensors = parse_sensors(reader);

    for sensor in &sensors {
        println!("Sensor {:?} has range {}", sensor.location, sensor.range());
    }

    let beacons = get_beacons(&sensors);
    let empty_spaces = count_spaces_in_range(&sensors, &beacons, 2000000);

    println!("part1, spaces without beacon: {empty_spaces}");

    let reader = BufReader::new(File::open(filename)?);
    let sensors = parse_sensors(reader);
    let beacons = get_beacons(&sensors);
    //let beacon = locate_distress_beacon(&sensors, &beacons, 0..4000000).expect("no beacon");
    let beacon = locate_distress_beacon2(&sensors, &beacons, 0..4000000).expect("no beacon");
    let frequency = beacon.frequency();

    println!("part2, distress beacon frequency: {frequency}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn test_parse() {
        parse_sensors(BufReader::new(INPUT.as_bytes()));
    }

    #[test]
    fn test_distance() {
        assert_eq!(
            Sensor {
                location: Point { x: 8, y: 7 },
                closest_beacon: Beacon(Point { x: 2, y: 10 }),
            }
            .range(),
            9
        );
    }

    #[test]
    fn test_part1() {
        let sensors = parse_sensors(BufReader::new(INPUT.as_bytes()));
        let beacons = get_beacons(&sensors);
        assert_eq!(count_spaces_in_range(&sensors, &beacons, 10), 26);
    }

    #[test]
    fn test_part2() {
        let sensors = parse_sensors(BufReader::new(INPUT.as_bytes()));
        let beacons = get_beacons(&sensors);
        let beacon = locate_distress_beacon(&sensors, &beacons, 0..20).expect("no beacon");
        assert_eq!(beacon.0, Point { x: 14, y: 11 });
        assert_eq!(beacon.frequency(), 56000011);
    }

    #[test]
    fn test_part2_optimized() {
        let sensors = parse_sensors(BufReader::new(INPUT.as_bytes()));
        let beacons = get_beacons(&sensors);
        let beacon = locate_distress_beacon2(&sensors, &beacons, 0..20).expect("no beacon");
        assert_eq!(beacon.0, Point { x: 14, y: 11 });
        assert_eq!(beacon.frequency(), 56000011);
    }
}
