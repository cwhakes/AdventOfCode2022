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
	let _input = input;
	0
}

fn get_answer2(input: &str) -> impl Display {
	let _input = input;
	0
}
