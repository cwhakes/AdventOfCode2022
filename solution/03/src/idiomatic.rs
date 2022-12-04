use std::fmt::Display;

static BUF: &'static str = include_str!("../../../input/03/input");

fn main() {
	let answer = get_answer1(&BUF);
	println!("{}", answer);

	let answer = get_answer2(&BUF);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut acc = 0;
	for rucksack in input.lines() {
		let (a, b) = rucksack.split_at(rucksack.len() / 2);
		let ruck_a: Rucksack = a.bytes().collect();
		let ruck_b: Rucksack = b.bytes().collect();
		acc += (ruck_a.0 & ruck_b.0).trailing_zeros();
	}
	acc
}

fn get_answer2(input: &str) -> impl Display {
	let mut acc = 0;
	let mut iter = input.lines();
	while let [Some(a), Some(b), Some(c)] = [iter.next(), iter.next(), iter.next()] {
		let ruck_a: Rucksack = a.bytes().collect();
		let ruck_b: Rucksack = b.bytes().collect();
		let ruck_c: Rucksack = c.bytes().collect();
		acc += (ruck_a.0 & ruck_b.0 & ruck_c.0).trailing_zeros();
	}
	acc
}

struct Rucksack(u64);

impl Rucksack {
	fn priority(item: u8) -> u8 {
		match item {
			b'a'..=b'z' => item - b'a' + 1,
			b'A'..=b'Z' => item - b'A' + 27,
			_ => panic!("Invalid item"),
		}
	}

	fn add_item(self, item: u8) -> Self {
		Self(self.0 | (1 << Self::priority(item)))
	}
}

impl FromIterator<u8> for Rucksack {
	fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
		iter.into_iter().fold(Rucksack(0), Rucksack::add_item)
	}
}
