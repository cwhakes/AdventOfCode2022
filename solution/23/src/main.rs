use std::collections::{HashMap, HashSet};
use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/23/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut elves = Elves::new(input);
	for i in 0..10 {
		elves.step(i);
	}
	elves.score()
}

fn get_answer2(input: &str) -> impl Display {
	let mut elves = Elves::new(input);
	let mut i = 0;
	while elves.step(i) {
		i += 1;
		if i % 100 == 0 {
			println!("Inter: {}", i);
		}
	}
	i + 1
}

#[derive(Debug, PartialEq, Eq)]
struct Elves(HashSet<(i32, i32)>);

impl Elves {
	fn new(input: &str) -> Self {
		Self(
			input
				.lines()
				.enumerate()
				.flat_map(|(y, line)| {
					line.bytes()
						.enumerate()
						.filter_map(move |(x, b)| (b == b'#').then_some((x as i32, y as i32)))
				})
				.collect(),
		)
	}

	fn need_move(&self, (x, y): (i32, i32)) -> bool {
		self.0.contains(&(x - 1, y - 1))
			|| self.0.contains(&(x - 1, y))
			|| self.0.contains(&(x - 1, y + 1))
			|| self.0.contains(&(x, y - 1))
			|| self.0.contains(&(x, y + 1))
			|| self.0.contains(&(x + 1, y - 1))
			|| self.0.contains(&(x + 1, y))
			|| self.0.contains(&(x + 1, y + 1))
	}

	fn try_north(&self, (x, y): (i32, i32)) -> Option<(i32, i32)> {
		(!self.0.contains(&(x - 1, y - 1))
			&& !self.0.contains(&(x, y - 1))
			&& !self.0.contains(&(x + 1, y - 1)))
		.then_some((x, y - 1))
	}

	fn try_south(&self, (x, y): (i32, i32)) -> Option<(i32, i32)> {
		(!self.0.contains(&(x - 1, y + 1))
			&& !self.0.contains(&(x, y + 1))
			&& !self.0.contains(&(x + 1, y + 1)))
		.then_some((x, y + 1))
	}

	fn try_west(&self, (x, y): (i32, i32)) -> Option<(i32, i32)> {
		(!self.0.contains(&(x - 1, y - 1))
			&& !self.0.contains(&(x - 1, y))
			&& !self.0.contains(&(x - 1, y + 1)))
		.then_some((x - 1, y))
	}

	fn try_east(&self, (x, y): (i32, i32)) -> Option<(i32, i32)> {
		(!self.0.contains(&(x + 1, y - 1))
			&& !self.0.contains(&(x + 1, y))
			&& !self.0.contains(&(x + 1, y + 1)))
		.then_some((x + 1, y))
	}

	fn get_declaration(&self, i: usize, (x, y): (i32, i32)) -> Option<(i32, i32)> {
		type Cyc = dyn Fn(&Elves, (i32, i32)) -> Option<(i32, i32)>;
		let cycle: [&Cyc; 4] = [
			&Elves::try_north as &Cyc,
			&Elves::try_south as &Cyc,
			&Elves::try_west as &Cyc,
			&Elves::try_east as &Cyc,
		];

		cycle[i % 4](self, (x, y))
			.or_else(|| cycle[(i + 1) % 4](self, (x, y)))
			.or_else(|| cycle[(i + 2) % 4](self, (x, y)))
			.or_else(|| cycle[(i + 3) % 4](self, (x, y)))
	}

	fn step(&mut self, i: usize) -> bool {
		let mut declarations: HashMap<(i32, i32), u8> = HashMap::new();
		for &elf in &self.0 {
			if self.need_move(elf) {
				if let Some(dec) = self.get_declaration(i, elf) {
					*declarations.entry(dec).or_default() += 1;
				}
			}
		}

		let mut elf_moved = false;
		let mut new = HashSet::new();
		for &elf in &self.0 {
			if self.need_move(elf) {
				if let Some(dec) = self.get_declaration(i, elf) {
					if *declarations.get(&dec).unwrap() <= 1 {
						new.insert(dec);
						elf_moved = true;
						continue;
					}
				}
			}
			new.insert(elf);
		}

		self.0 = new;
		elf_moved
	}

	fn score(&self) -> i32 {
		let min_x = self.0.iter().map(|(x, _)| x).min().unwrap();
		let max_x = self.0.iter().map(|(x, _)| x).max().unwrap();
		let min_y = self.0.iter().map(|(_, y)| y).min().unwrap();
		let max_y = self.0.iter().map(|(_, y)| y).max().unwrap();

		(max_x + 1 - min_x) * (max_y + 1 - min_y) - self.0.len() as i32
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/23/test");
	static TEST_SMOL: &str = include_str!("../../../input/23/test_smol");
	static TEST_SMOL1: &str = include_str!("../../../input/23/test_smol1");
	static TEST_SMOL2: &str = include_str!("../../../input/23/test_smol2");
	static TEST_SMOL3: &str = include_str!("../../../input/23/test_smol3");

	#[test]
	fn smol() {
		let mut elves = Elves::new(TEST_SMOL);
		elves.step(0);
		assert_eq!(Elves::new(TEST_SMOL1), elves);
		elves.step(1);
		assert_eq!(Elves::new(TEST_SMOL2), elves);
		elves.step(2);
		assert_eq!(Elves::new(TEST_SMOL3), elves);
	}

	#[test]
	fn part1() {
		assert_eq!("110", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("20", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
