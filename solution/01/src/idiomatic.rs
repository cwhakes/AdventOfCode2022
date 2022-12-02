//! Not exactly idiomatic, just not my first attempt.
//! This implementation doesn't allocate (mostly).

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
	let mut buf = String::new();
	let mut file = File::open("input/01/input").unwrap();
	file.read_to_string(&mut buf).unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	input
		.split("\n\n")
		.map(|e| e.parse::<Elf>().unwrap())
		.max()
		.unwrap()
		.calories
}

fn get_answer2(input: &str) -> impl Display {
	get_max_3(input.split("\n\n").map(|e| e.parse::<Elf>().unwrap()))
		.unwrap()
		.into_iter()
		.map(|e| e.calories)
		.sum::<i64>()
}

// `generic_const_exprs`, plz
fn get_max_3<T: Ord>(mut iter: impl Iterator<Item = T>) -> Option<[T; 3]> {
	// If we can't get 3, bail with None
	let [mut b, mut c, mut d] = [iter.next()?, iter.next()?, iter.next()?];
	for a in iter {
		let mut array = [a, b, c, d];
		array.sort(); // ascending, so max is in back
		[_, b, c, d] = array;
	}
	Some([b, c, d])
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
	calories: i64,
}

impl FromStr for Elf {
	type Err = Box<dyn Error>;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut calories = 0;
		for food in input.lines() {
			calories += food.parse::<i64>()?;
		}
		Ok(Self { calories })
	}
}
