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
}

fn get_answer1(input: &str) -> impl Display {
	let mut score = 0u32;
	for strat in input.lines() {
		let (op, me) = strat.split_once(' ').expect("Invalid strategy");
		let op = match op {
			"A" => Shape::Rock,
			"B" => Shape::Paper,
			"C" => Shape::Scissors,
			_ => panic!("Invalid opponent move"),
		};
		let me = match me {
			"X" => Shape::Rock,
			"Y" => Shape::Paper,
			"Z" => Shape::Scissors,
			_ => panic!("Invalid move"),
		};
		score += Shape::get_outcome(op, me) + me as u32;
	}
	score
}

fn get_answer2(input: &str) -> impl Display {
	let mut score = 0u32;
	for strat in input.lines() {
		let (op, me) = strat.split_once(' ').expect("Invalid strategy");
		let op = match op {
			"A" => Shape::Rock,
			"B" => Shape::Paper,
			"C" => Shape::Scissors,
			_ => panic!("Invalid opponent move"),
		};
		let me = match me {
			"X" => op.get_next().get_next(),
			"Y" => op,
			"Z" => op.get_next(),
			_ => panic!("Invalid move"),
		};
		score += Shape::get_outcome(op, me) + me as u32;
	}
	score
}

#[repr(u32)]
#[derive(Clone, Copy)]
enum Shape {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl Shape {
	fn get_outcome(op: Self, me: Self) -> u32 {
		match (3 + me as u32 - op as u32) % 3 {
			2 => 0, // Loser is 2 forward
			0 => 3, // Tier is equal
			1 => 6, // Winner is 1 forward
			_ => unreachable!(),
		}
	}

	fn get_next(self) -> Self {
		match self {
			Self::Rock => Self::Paper,
			Self::Paper => Self::Scissors,
			Self::Scissors => Self::Rock,
		}
	}
}
