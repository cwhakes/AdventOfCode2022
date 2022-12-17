#![allow(clippy::unnecessary_cast)]

use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/05/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let (stacks, moves) = input.split_once("\n\n").unwrap();
	let mut stacks = Stacks::new(stacks);
	for line in moves.lines() {
		if let Some(shift) = Move::new(line) {
			stacks.shift(shift);
		}
	}
	stacks.tops()
}

fn get_answer2(input: &str) -> impl Display {
	let (stacks, moves) = input.split_once("\n\n").unwrap();
	let mut stacks = Stacks::new(stacks);
	for line in moves.lines() {
		if let Some(shift) = Move::new(line) {
			stacks.shift2(shift);
		}
	}
	stacks.tops()
}

#[derive(Debug)]
struct Crate(char);

impl Crate {
	fn new(input: &[u8]) -> Option<Self> {
		assert_eq!(3, input.len());

		if input == b"   " {
			None
		} else {
			Some(Crate(input[1] as char))
		}
	}
}

impl Display for Crate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Debug)]
struct Stacks(Vec<Vec<Crate>>);

impl Stacks {
	fn new(input: &str) -> Self {
		let mut stacks = Self(Vec::new());
		for line in input.lines().rev().skip(1) {
			let mut line = AsRef::<[u8]>::as_ref(line);
			let mut index = 0;
			loop {
				let (next, remainder) = line.split_at(3);
				if let Some(c) = Crate::new(next) {
					if stacks.0.len() < index + 1 {
						stacks.0.resize_with(index + 1, Vec::new);
					}
					stacks.0[index].push(c);
				}
				if let Some(remainder) = remainder.strip_prefix(b" ") {
					line = remainder;
				} else {
					break;
				}

				index += 1;
			}
		}
		stacks
	}

	fn shift(&mut self, shift: Move) {
		for _ in 0..shift.count {
			let c = self.0[shift.from - 1].pop().unwrap();
			self.0[shift.to - 1].push(c)
		}
	}

	fn shift2(&mut self, shift: Move) {
		let count = self.0[shift.from - 1].len() - shift.count;
		let mut c = self.0[shift.from - 1].split_off(count);
		self.0[shift.to - 1].append(&mut c);
	}

	fn tops(&self) -> String {
		let mut out = String::new();
		for stack in self.0.iter() {
			if let Some(last) = stack.last() {
				out.push(last.0 as char);
			}
		}
		out
	}
}

struct Move {
	count: usize,
	from: usize,
	to: usize,
}

impl Move {
	fn new(input: &str) -> Option<Self> {
		let input = input.strip_prefix("move ")?;
		let (count, input) = input.split_once(" from ")?;
		let (from, to) = input.split_once(" to ")?;
		let count = count.parse::<usize>().ok()?;
		let from = from.parse::<usize>().ok()?;
		let to = to.parse::<usize>().ok()?;
		Some(Self { count, from, to })
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/05/test");

	#[test]
	fn part1() {
		assert_eq!("CMZ", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("MCD", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
