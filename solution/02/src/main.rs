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
	let mut score = 0i64;
	for strat in input.lines() {
		let (op, me) = strat.split_once(" ").unwrap();
		match op {
			"A" => match me {
				"X" => score += 3 + 1,
				"Y" => score += 6 + 2,
				"Z" => score += 0 + 3,
				_ => {},
			},
			"B" => match me {
				"X" => score += 0 + 1,
				"Y" => score += 3 + 2,
				"Z" => score += 6 + 3,
				_ => {},
			},
			"C" => match me {
				"X" => score += 6 + 1,
				"Y" => score += 0 + 2,
				"Z" => score += 3 + 3,
				_ => {},
			},
			_ => {},
		}
	}
	score
}

fn get_answer2(input: &str) -> impl Display {
	let mut score = 0i64;
	for strat in input.lines() {
		let (op, me) = strat.split_once(" ").unwrap();
		match op {
			"A" => match me {
				"X" => score += 0 + 3,
				"Y" => score += 3 + 1,
				"Z" => score += 6 + 2,
				_ => {},
			},
			"B" => match me {
				"X" => score += 0 + 1,
				"Y" => score += 3 + 2,
				"Z" => score += 6 + 3,
				_ => {},
			},
			"C" => match me {
				"X" => score += 0 + 2,
				"Y" => score += 3 + 3,
				"Z" => score += 6 + 1,
				_ => {},
			},
			_ => {},
		}
	}
	score
}
