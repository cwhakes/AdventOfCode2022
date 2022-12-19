use std::{collections::BTreeSet, fmt::Display};

static INPUT: &str = include_str!("../../../input/17/input");
static ROCKS: &str = include_str!("../../../input/17/rocks");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT, 346);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let rocks: Vec<_> = ROCKS.split("\n\n").map(Rock::new).collect();
	let mut gusts = input.trim().bytes().cycle();
	let mut shaft = Shaft::default();
	for i in 0..2022 {
		shaft.drop_rock(&rocks[i % rocks.len()], &mut gusts);
	}
	shaft.highest()
}

fn get_answer2(input: &str, magic_number: u128) -> impl Display {
	let rocks: Vec<_> = ROCKS.split("\n\n").map(Rock::new).collect();
	let mut gusts = input.trim().bytes().cycle();
	let modulus = rocks.len() as u128 * magic_number;
	let target = 1000000000000;
	let remainder = target % modulus;

	let mut shaft = Shaft::default();
	for i in 0..modulus as usize {
		shaft.drop_rock(&rocks[i % rocks.len()], &mut gusts);
	}
	let beginning = shaft.highest() as u128;

	for i in 0..modulus as usize {
		shaft.drop_rock(&rocks[i % rocks.len()], &mut gusts);
	}
	let middle = shaft.highest() as u128 - beginning;

	for i in 0..remainder as usize {
		shaft.drop_rock(&rocks[i % rocks.len()], &mut gusts);
	}
	let end = shaft.highest() as u128 - middle - beginning;

	beginning + middle * (target / modulus - 1) + end
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
	y: i128,
	x: i8,
}

impl Point {
	fn offset(&self, offset: &Self) -> Self {
		Self {
			y: self.y + offset.y,
			x: self.x + offset.x,
		}
	}

	fn down(&self) -> Self {
		Self {
			y: self.y - 1,
			x: self.x,
		}
	}

	fn left(&self) -> Self {
		Self {
			y: self.y,
			x: self.x - 1,
		}
	}

	fn right(&self) -> Self {
		Self {
			y: self.y,
			x: self.x + 1,
		}
	}
}

struct Rock(Vec<Point>);

impl Rock {
	fn new(input: &str) -> Self {
		let mut rock = Vec::new();
		for (y, line) in input.lines().rev().enumerate() {
			for (x, c) in line.chars().enumerate() {
				if c == '#' {
					rock.push(Point {
						y: y as i128,
						x: x as i8,
					});
				}
			}
		}
		Self(rock)
	}
}

#[derive(Default, Debug)]
struct Shaft(BTreeSet<Point>);

impl Shaft {
	fn highest(&self) -> i128 {
		self.0.last().map(|p| p.y + 1).unwrap_or(0)
	}

	fn starting_point(&self) -> Point {
		Point {
			x: 2,
			y: self.highest() + 3,
		}
	}

	fn can_insert_rock(&self, rock: &Rock, offset: &Point) -> bool {
		rock.0
			.iter()
			.map(|p| p.offset(offset))
			.all(|p| 0 <= p.x && p.x < 7 && 0 <= p.y && !self.0.contains(&p))
	}

	fn insert_rock(&mut self, rock: &Rock, offset: &Point) {
		self.0.extend(rock.0.iter().map(|p| p.offset(offset)))
	}

	fn drop_rock(&mut self, rock: &Rock, gusts: &mut impl Iterator<Item = u8>) {
		let mut loc = self.starting_point();
		loop {
			match gusts.next() {
				Some(b'<') => {
					if self.can_insert_rock(rock, &loc.left()) {
						loc = loc.left();
					}
				},
				Some(b'>') => {
					if self.can_insert_rock(rock, &loc.right()) {
						loc = loc.right();
					}
				},
				_ => panic!(),
			}
			if self.can_insert_rock(rock, &loc.down()) {
				loc = loc.down();
			} else {
				self.insert_rock(rock, &loc);
				break;
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/17/test");

	#[test]
	fn part1() {
		assert_eq!("3068", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("1514285714288", &get_answer2(TEST, 7).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
