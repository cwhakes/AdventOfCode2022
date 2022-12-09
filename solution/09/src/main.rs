use core::panic;
use std::{collections::HashSet, fmt::Display};

static INPUT: &str = include_str!("../../../input/09/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut rope = Rope::new(1);
	let mut set = HashSet::new();
	set.insert((0, 0));
	for line in input.lines() {
		set.extend(rope.process(line));
	}
	set.len()
}

fn get_answer2(input: &str) -> impl Display {
	let mut rope = Rope::new(9);
	let mut set = HashSet::new();
	set.insert((0, 0));
	for line in input.lines() {
		set.extend(rope.process(line));
	}
	set.len()
}

#[derive(Debug, Default)]
struct Rope {
	head: (i16, i16),
	tail: Vec<(i16, i16)>,
}

impl Rope {
	fn new(len: usize) -> Self {
		Self {
			head: (0, 0),
			tail: vec![(0, 0); len],
		}
	}

	fn process(&mut self, input: &str) -> Vec<(i16, i16)> {
		let (dir, num) = input.split_once(' ').unwrap();
		let f = match dir {
			"R" => |(x, _): &mut (i16, i16)| *x += 1,
			"L" => |(x, _): &mut (i16, i16)| *x -= 1,
			"U" => |(_, y): &mut (i16, i16)| *y += 1,
			"D" => |(_, y): &mut (i16, i16)| *y -= 1,
			_ => panic!(),
		};
		let mut out = Vec::new();
		for _ in 0..num.parse::<u8>().unwrap() {
			{
				f(&mut self.head);
			}
			self.relax();
			out.push(*self.tail.last().unwrap())
		}
		out
	}

	fn relax(&mut self) {
		let mut head = &mut self.head;
		for tail in self.tail.iter_mut() {
			let d_0 = head.0 - tail.0;
			let d_1 = head.1 - tail.1;
			if d_0.abs() > 1 || d_1.abs() > 1 {
				if d_0.abs() > 0 {
					tail.0 += d_0.signum();
				}
				if d_1.abs() > 0 {
					tail.1 += d_1.signum();
				}
			}
			head = tail;
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/09/test");
	static TEST2: &str = include_str!("../../../input/09/test2");

	#[test]
	fn part1() {
		assert_eq!("13", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("36", &get_answer2(TEST2).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}

	#[test]
	fn part2a() {
		let mut rope = Rope::new(9);
		rope.process("R 5");
		rope.process("U 2");
		assert_eq!((5, 1), rope.tail[0]);
		assert_eq!((4, 1), rope.tail[1]);
		assert_eq!((3, 1), rope.tail[2]);
	}
}
