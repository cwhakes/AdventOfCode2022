use std::fmt::Display;
use std::fs::File;
use std::io::Read;

fn main() {
	let mut buf = String::new();
	let mut file = File::open("input/02/input").unwrap();
	file.read_to_string(&mut buf).unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);

	println!("{}", Shape::get_outcome(Shape::Scissors, Shape::Rock));
}

fn get_answer1(input: &str) -> impl Display {
	let mut score = 0i32;
	for strat in input.lines() {
		let (op, me) = strat.split_once(' ').unwrap();
		let op = match op {
			"A" => Shape::Rock,
			"B" => Shape::Paper,
			"C" => Shape::Scissors,
			_ => panic!(),
		};
		let me = match me {
			"X" => Shape::Rock,
			"Y" => Shape::Paper,
			"Z" => Shape::Scissors,
			_ => panic!(),
		};
		score += Shape::get_outcome(op, me) + me as i32;
	}
	score
}

fn get_answer2(input: &str) -> impl Display {
	let mut score = 0i32;
	for strat in input.lines() {
		let (op, me) = strat.split_once(' ').unwrap();
		let op = match op {
			"A" => Shape::Rock,
			"B" => Shape::Paper,
			"C" => Shape::Scissors,
			_ => panic!(),
		};
		let me = match me {
			"X" => op.get_loser(),
			"Y" => op.get_tier(),
			"Z" => op.get_winner(),
			_ => panic!(),
		};
		score += Shape::get_outcome(op, me) + me as i32;
	}
	score
}

#[repr(i32)]
#[derive(Clone, Copy)]
enum Shape {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl Shape {
	fn get_outcome(op: Self, me: Self) -> i32 {
		match (3 + me as i32 - op as i32) % 3 {
			2 => 0,
			0 => 3,
			1 => 6,
			_ => unreachable!(),
		}
	}

	fn get_loser(self) -> Self {
		((self as i32 + 1) % 3 + 1).try_into().unwrap()
	}

	fn get_tier(self) -> Self {
		((self as i32 - 1) % 3 + 1).try_into().unwrap()
	}

	fn get_winner(self) -> Self {
		(self as i32 % 3 + 1).try_into().unwrap()
	}
}

impl TryFrom<i32> for Shape {
	type Error = ();
	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::Rock),
			2 => Ok(Self::Paper),
			3 => Ok(Self::Scissors),
			_ => Err(()),
		}
	}
}
