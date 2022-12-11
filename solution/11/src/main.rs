use std::{collections::VecDeque, fmt::Display};

static INPUT: &str = include_str!("../../../input/11/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut monkeys: Vec<_> = input.split("\n\n").filter_map(Monkey::new).collect();
	let mut handles = process(&mut monkeys, 20, |i| i / 3);
	handles.sort_by_key(|&i| std::cmp::Reverse(i));
	handles[0] * handles[1]
}

fn get_answer2(input: &str) -> impl Display {
	let mut monkeys: Vec<_> = input.split("\n\n").filter_map(Monkey::new).collect();
	let mut handles = process(&mut monkeys, 10000, |i| i);
	handles.sort_by_key(|&i| std::cmp::Reverse(i));
	handles[0] * handles[1]
}

fn process(monkeys: &mut [Monkey], rounds: usize, worry: impl Fn(i64) -> i64) -> Vec<usize> {
	let modulo = monkeys.iter().map(|m| m.divider).product::<i64>();
	let mut handles = vec![0; monkeys.len()];
	for _round in 0..rounds {
		for n in 0..monkeys.len() {
			while let Some(item) = monkeys[n].items.pop_front() {
				let item = (monkeys[n].op)(item) % modulo;
				let item = worry(item);
				if item % monkeys[n].divider == 0 {
					let if_true = monkeys[n].if_true;
					monkeys[if_true].items.push_back(item);
				} else {
					let if_false = monkeys[n].if_false;
					monkeys[if_false].items.push_back(item);
				}
				handles[n] += 1;
			}
		}
	}
	handles
}

struct Monkey {
	items: VecDeque<i64>,
	op: Box<dyn Fn(i64) -> i64>,
	divider: i64,
	if_true: usize,
	if_false: usize,
}

impl Monkey {
	fn new(input: &str) -> Option<Self> {
		let mut iter = input.lines();
		let _monkey = iter.next()?;
		let items = iter
			.next()?
			.trim_start_matches("  Starting items: ")
			.split(", ")
			.filter_map(|n| n.parse::<i64>().ok())
			.collect();
		let (symbol, num) = iter
			.next()?
			.trim_start_matches("  Operation: new = old ")
			.split_once(' ')?;
		let op = if num == "old" {
			match symbol {
				"+" => Box::new(move |i| i + i) as Box<dyn Fn(i64) -> i64>,
				"*" => Box::new(move |i| i * i) as Box<dyn Fn(i64) -> i64>,
				_ => return None,
			}
		} else {
			let num = num.parse::<i64>().ok()?;
			match symbol {
				"+" => Box::new(move |i| i + num) as Box<dyn Fn(i64) -> i64>,
				"*" => Box::new(move |i| i * num) as Box<dyn Fn(i64) -> i64>,
				_ => return None,
			}
		};
		let divider = iter
			.next()?
			.trim_start_matches("  Test: divisible by ")
			.parse::<i64>()
			.ok()?;
		let if_true = iter
			.next()?
			.trim_start_matches("    If true: throw to monkey ")
			.parse::<usize>()
			.ok()?;
		let if_false = iter
			.next()?
			.trim_start_matches("    If false: throw to monkey ")
			.parse::<usize>()
			.ok()?;
		Some(Self {
			items,
			op,
			divider,
			if_true,
			if_false,
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/11/test");

	#[test]
	fn part1() {
		assert_eq!("10605", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("2713310158", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
