use std::fmt::{self, Display};

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
	Crt::new(Clock::new(input.lines()))
}

struct Clock<'a, I: Iterator<Item = &'a str>> {
	iter: I,
	x: i32,
	addx: Option<i32>,
}

impl<'a, I: Iterator<Item = &'a str>> Clock<'a, I> {
	fn new<II: 'a + IntoIterator<IntoIter = I>>(iter: II) -> Self {
		Self {
			iter: iter.into_iter(),
			x: 1,
			addx: None,
		}
	}
}

impl<'a, I: Iterator<Item = &'a str>> Iterator for Clock<'a, I> {
	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {
		let old_x = self.x;
		if let Some(addx) = self.addx.take() {
			self.x += addx;
		} else if let Some(inst) = self.iter.next() {
			if inst != "noop" {
				self.addx = inst.trim_start_matches("addx ").parse().ok(); // Converts errors to noops
			}
		}
		Some(old_x)
	}
}

struct Crt(Vec<i32>);

impl Crt {
	fn new(iter: impl IntoIterator<Item = i32>) -> Self {
		Self(iter.into_iter().take(240).collect())
	}
}

impl Display for Crt {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (i, &x) in self.0.iter().enumerate() {
			if i > 0 && i % 40 == 0 {
				writeln!(f)?;
			}
			if ((i as i32 % 40) - x).abs() <= 1 {
				write!(f, "#")?;
			} else {
				write!(f, ".")?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/10/test");
	static TEST_OUT: &str = include_str!("../../../input/10/test_out");
	static INPUT_OUT: &str = include_str!("../../../input/10/input_out");

	#[test]
	fn part1() {
		assert_eq!("13140", &get_answer1(TEST).to_string());
		assert_eq!("13680", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!(TEST_OUT, &get_answer2(TEST).to_string());
		assert_eq!(INPUT_OUT, &get_answer2(INPUT).to_string());
	}
}
