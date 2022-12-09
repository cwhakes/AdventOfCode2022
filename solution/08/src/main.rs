use std::{collections::HashMap, fmt::Display};

static INPUT: &str = include_str!("../../../input/08/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let map = TreeMap::new(input);
	map.map.keys().filter(|loc| map.is_visible(**loc)).count()
}

fn get_answer2(input: &str) -> impl Display {
	let map = TreeMap::new(input);
	map.map
		.keys()
		.map(|loc| map.scenic_score(*loc))
		.max()
		.unwrap()
}

struct TreeMap {
	map: HashMap<(i16, i16), i32>,
	size: (i16, i16),
}

impl TreeMap {
	fn new(input: &str) -> Self {
		let mut map = HashMap::new();
		for (m, line) in input.lines().enumerate() {
			for (n, c) in line.chars().enumerate() {
				map.insert((m as i16, n as i16), c.to_digit(10).unwrap() as i32);
			}
		}
		let m = input.lines().count() as i16;
		let n = input.lines().next().unwrap().chars().count() as i16;
		Self {
			map,
			size: (m as i16, n as i16),
		}
	}

	fn is_visible(&self, loc: (i16, i16)) -> bool {
		let height = self.map.get(&loc).unwrap();
		!(0..loc.0).any(|m| self.map.get(&(m, loc.1)).unwrap() >= height)
			|| !((loc.0 + 1)..self.size.0).any(|m| self.map.get(&(m, loc.1)).unwrap() >= height)
			|| !(0..loc.1).any(|n| self.map.get(&(loc.0, n)).unwrap() >= height)
			|| !((loc.1 + 1)..self.size.1).any(|n| self.map.get(&(loc.0, n)).unwrap() >= height)
	}

	fn scenic_score(&self, loc: (i16, i16)) -> usize {
		let height = self.map.get(&loc).unwrap();
		(0..loc.0)
			.rev()
			.enumerate()
			.find_map(|(i, m)| (self.map.get(&(m, loc.1))? >= height).then_some(i + 1))
			.unwrap_or(loc.0 as usize)
			* ((loc.0 + 1)..self.size.0)
				.enumerate()
				.find_map(|(i, m)| (self.map.get(&(m, loc.1))? >= height).then_some(i + 1))
				.unwrap_or((self.size.0 - loc.0 - 1) as usize)
			* (0..loc.1)
				.rev()
				.enumerate()
				.find_map(|(i, n)| (self.map.get(&(loc.0, n))? >= height).then_some(i + 1))
				.unwrap_or(loc.1 as usize)
			* ((loc.1 + 1)..self.size.1)
				.enumerate()
				.find_map(|(i, n)| (self.map.get(&(loc.0, n))? >= height).then_some(i + 1))
				.unwrap_or((self.size.1 - loc.1 - 1) as usize)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/08/test");

	#[test]
	fn part1() {
		assert_eq!("21", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("8", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
