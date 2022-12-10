use std::fmt::{Display, Write};

static INPUT: &str = include_str!("../../../input/10/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let signal: Vec<_> = Clock::new(input.lines()).take(240).collect();

	[20, 60, 100, 140, 180, 220]
		.into_iter()
		.map(|i| signal[i - 1] * i as i32)
		.sum::<i32>()
}

fn get_answer2(input: &str) -> impl Display {
	let signal: Vec<_> = Clock::new(input.lines()).take(240).collect();

	let mut s = String::new();
	for (i, x) in signal.into_iter().enumerate() {
		if i > 0 && i % 40 == 0 {
			writeln!(s).unwrap();
		}
		if ((i as i32 % 40) - x).abs() <= 1 {
			write!(s, "#").unwrap();
		} else {
			write!(s, ".").unwrap();
		}
	}
	s
}

struct Clock<'a, T: Iterator<Item = &'a str>> {
	iter: T,
	x: i32,
	addx: Option<i32>,
}

impl<'a, T: Iterator<Item = &'a str>> Clock<'a, T> {
	fn new(iter: T) -> Self {
		Self {
			iter,
			x: 1,
			addx: None,
		}
	}
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Clock<'a, T> {
	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {
		let old_x = self.x;
		if let Some(addx) = self.addx.take() {
			self.x += addx;
		} else if let Some(inst) = self.iter.next() {
			if inst != "noop" {
				self.addx = Some(inst.trim_start_matches("addx ").parse().unwrap())
			}
		}
		Some(old_x)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/10/test");
	static TEST_OUT: &str = include_str!("../../../input/10/test_out");

	#[test]
	fn part1() {
		assert_eq!("13140", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!(TEST_OUT, &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
