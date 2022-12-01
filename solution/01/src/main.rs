use std::fmt::Display;
use std::fs::File;
use std::io::Read;

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
	let elves = input.split("\n\n");
	elves
		.map(|e| {
			e.split_whitespace()
				.map(|c| c.parse::<i64>().unwrap())
				.sum::<i64>()
		})
		.max()
		.unwrap()
}

fn get_answer2(input: &str) -> impl Display {
	let elves = input.split("\n\n");
	let mut vec: Vec<_> = elves
		.map(|e| {
			e.split_whitespace()
				.map(|c| c.parse::<i64>().unwrap())
				.sum::<i64>()
		})
		.collect();
	vec.sort_by_key(|c| -c);
	vec[0] + vec[1] + vec[2]
}
