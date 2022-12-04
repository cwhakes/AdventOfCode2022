use std::ops::RangeInclusive;

static BUF: &str = include_str!("../../../input/04/input");

fn main() {
	let answer = get_answer1(BUF);
	println!("{}", answer);

	let answer = get_answer2(BUF);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl std::fmt::Display {
	input
		.lines()
		.filter_map(Pair::new)
		.filter(Pair::fully_contains)
		.count()
}

fn get_answer2(input: &str) -> impl std::fmt::Display {
	input
		.lines()
		.filter_map(Pair::new)
		.filter(Pair::partially_contains)
		.count()
}

struct Pair(RangeInclusive<i32>, RangeInclusive<i32>);

impl Pair {
	fn new(s: &str) -> Option<Self> {
		let (a, b) = s.split_once(',')?;
		let (a0, a1) = a.split_once('-')?;
		let a = a0.parse().ok()?..=a1.parse().ok()?;
		let (b0, b1) = b.split_once('-')?;
		let b = b0.parse().ok()?..=b1.parse().ok()?;
		Some(Self(a, b))
	}

	fn fully_contains(&self) -> bool {
		self.0.contains(self.1.start()) && self.0.contains(self.1.end())
			|| self.1.contains(self.0.start()) && self.1.contains(self.0.end())
	}

	fn partially_contains(&self) -> bool {
		self.0.contains(self.1.start())
			|| self.0.contains(self.1.end())
			|| self.1.contains(self.0.start())
			|| self.1.contains(self.0.end())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST_BUF: &str = include_str!("../../../input/04/test");

	#[test]
	fn part1() {
		assert_eq!("2", &get_answer1(TEST_BUF).to_string());
		assert_eq!("605", &get_answer1(BUF).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("4", &get_answer2(TEST_BUF).to_string());
		assert_eq!("914", &get_answer2(BUF).to_string());
	}
}
