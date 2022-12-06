use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/06/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: impl AsRef<[u8]>) -> impl Display {
	for (index, window) in input.as_ref().windows(4).enumerate() {
		if MarkerHash::new(window).validate(4) {
			return index + 4;
		}
	}
	panic!("No signal")
}

fn get_answer2(input: impl AsRef<[u8]>) -> impl Display {
	for (index, window) in input.as_ref().windows(14).enumerate() {
		if MarkerHash::new(window).validate(14) {
			return index + 14;
		}
	}
	panic!("No signal")
}

struct MarkerHash(u128); // Ascii

impl MarkerHash {
	fn new<'a>(iter: impl IntoIterator<Item = &'a u8>) -> Self {
		Self(iter.into_iter().fold(0, |acc, i| acc | (1 << i)))
	}

	fn validate(&self, count: u32) -> bool {
		self.0.count_ones() == count
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/06/test");

	#[test]
	fn part1() {
		assert_eq!("7", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("19", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
