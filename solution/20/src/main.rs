use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/20/input");

const KEY: i64 = 811589153;

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut message = Message::new(input);
	message.mix();
	message.eval()
}

fn get_answer2(input: &str) -> impl Display {
	let mut message = Message::new(input);
	for m in &mut message.0 {
		m.1 *= KEY;
	}
	for _ in 0..10 {
		message.mix();
		// dbg!(&message);
	}
	message.eval()
}

#[derive(Debug)]
struct Message(Vec<(usize, i64)>);

impl Message {
	fn new(input: &str) -> Self {
		Self(
			input
				.lines()
				.enumerate()
				.map(|(n, s)| (n, s.parse().unwrap()))
				.collect(),
		)
	}

	fn mix(&mut self) {
		let len = self.0.len();
		for n in 0..len {
			let i = self
				.0
				.iter()
				.enumerate()
				.find(|(_i, (nn, _x))| *nn == n)
				.unwrap()
				.0;
			let (nn, x) = self.0.remove(i);
			let new_i = (x + i as i64 + 2 * KEY * (len - 1) as i64) as usize % (len - 1);
			self.0.insert(new_i, (nn, x));
		}
	}

	fn eval(&self) -> i64 {
		let idx = self
			.0
			.iter()
			.enumerate()
			.find(|(_i, (_nn, x))| *x == 0)
			.unwrap()
			.0;
		let len = self.0.len();
		self.0[(idx + 1000) % len].1 + self.0[(idx + 2000) % len].1 + self.0[(idx + 3000) % len].1
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/20/test");

	#[test]
	fn part1() {
		assert_eq!("3", &get_answer1(TEST).to_string());
		assert_eq!("2827", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("1623178306", &get_answer2(TEST).to_string());
		assert_eq!("7834270093909", &get_answer2(INPUT).to_string());
	}
}
