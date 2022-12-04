use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;

fn main() {
	let mut buf = String::new();
	let mut file = File::open("input/04/input").unwrap();
	file.read_to_string(&mut buf).unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	input
		.lines()
		.map(Pair::new)
		.filter(Pair::fully_contains)
		.count()
}

fn get_answer2(input: &str) -> impl Display {
	input
		.lines()
		.map(Pair::new)
		.filter(Pair::partially_contains)
		.count()
}

struct Pair(RangeInclusive<i32>, RangeInclusive<i32>);

impl Pair {
	fn new(s: &str) -> Self {
		let (a, b) = s.split_once(',').unwrap();
		let (a0, a1) = a.split_once('-').unwrap();
		let a = a0.parse().unwrap()..=a1.parse().unwrap();
		let (b0, b1) = b.split_once('-').unwrap();
		let b = b0.parse().unwrap()..=b1.parse().unwrap();
		Self(a, b)
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
