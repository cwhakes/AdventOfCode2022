use std::fmt::{self, Display, Write};
use std::iter::Sum;

static INPUT: &str = include_str!("../../../input/25/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	input.lines().map(Snafu::new).sum::<Snafu>()
}

fn get_answer2(input: &str) -> impl Display {
	let _ = input;
	0
}

struct Snafu(i64);

impl Snafu {
	fn new(input: &str) -> Self {
		let mut acc = 0;
		for char in input.chars() {
			acc *= 5;
			acc += match char {
				'2' => 2,
				'1' => 1,
				'0' => 0,
				'-' => -1,
				'=' => -2,
				_ => panic!(),
			}
		}
		Self(acc)
	}
}

impl Sum for Snafu {
	fn sum<I>(iter: I) -> Self
	where
		I: Iterator<Item = Self>,
	{
		Self(iter.into_iter().map(|s| s.0).sum())
	}
}

impl Display for Snafu {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let &Self(mut acc) = self;
		let digits = (acc as f64).log(5.0) as u32;

		for k in (0..=digits).rev() {
			acc += 2 * 5_i64.pow(k);
		}

		for k in (0..=digits).rev() {
			match acc / 5_i64.pow(k) {
				0 => f.write_char('=')?,
				1 => f.write_char('-')?,
				2 => f.write_char('0')?,
				3 => f.write_char('1')?,
				4 => f.write_char('2')?,
				_ => panic!("{}", acc),
			}
			acc %= 5_i64.pow(k);
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/25/test");

	#[test]
	fn part1() {
		assert_eq!("2=-1=0", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	// 	#[test]
	// 	fn part2() {
	// 		assert_eq!("0", &get_answer2(TEST).to_string());
	// 		// assert_eq!("0", &get_answer2(INPUT).to_string());
	// 	}
}
