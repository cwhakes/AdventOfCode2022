use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/01/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let _ = input;
	0
}

fn get_answer2(input: &str) -> impl Display {
	let _ = input;
	0
}

// #[cfg(test)]
// mod test {
// 	use super::*;
// 	static TEST: &str = include_str!("../../../input/01/test");

// 	#[test]
// 	fn part1() {
// 		assert_eq!("0", &get_answer1(TEST).to_string());
// 		// assert_eq!("0", &get_answer1(INPUT).to_string());
// 	}

// 	#[test]
// 	fn part2() {
// 		assert_eq!("0", &get_answer2(TEST).to_string());
// 		// assert_eq!("0", &get_answer2(INPUT).to_string());
// 	}
// }
