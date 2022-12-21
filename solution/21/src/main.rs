use std::collections::HashMap;
use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/21/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut cache = HashMap::new();
	Troop::new(input).get("root", &mut cache).unwrap()
}

fn get_answer2(input: &str) -> impl Display {
	let mut cache = HashMap::new();
	let mut troop = Troop::new(input);
	troop.0.insert("humn", Monkey::Human);
	let Some(Monkey::Operation{ op, .. }) = troop.0.get_mut("root") else { panic!() };
	*op = Op::Eq;
	troop.set("root", 0 /* ignored */, &mut cache).unwrap();
	cache.get("humn").unwrap().unwrap()
}

enum Monkey<'a> {
	Number(i64),
	Operation { lhs: &'a str, rhs: &'a str, op: Op },
	Human,
}

enum Op {
	Add,
	Sub,
	Mul,
	Div,
	Eq,
}

impl<'a> Monkey<'a> {
	fn new(input: &'a str) -> (&'a str, Self) {
		let (name, input) = input.split_once(": ").unwrap();
		if input.len() == 11 {
			let mut iter = input.split(' ');
			let lhs = iter.next().unwrap();
			let op = match iter.next().unwrap() {
				"+" => Op::Add,
				"-" => Op::Sub,
				"*" => Op::Mul,
				"/" => Op::Div,
				_ => panic!(),
			};
			let rhs = iter.next().unwrap();
			(name, Self::Operation { lhs, rhs, op })
		} else {
			let num = input.parse().unwrap();
			(name, Self::Number(num))
		}
	}
}

struct Troop<'a>(HashMap<&'a str, Monkey<'a>>);

impl<'a> Troop<'a> {
	fn new(input: &'a str) -> Self {
		Self(input.lines().map(Monkey::new).collect())
	}

	fn get(&self, name: &'a str, cache: &mut HashMap<&'a str, Option<i64>>) -> Option<i64> {
		if let Some(num) = cache.get(name) {
			return *num;
		}

		let num = match self.0.get(name).unwrap() {
			Monkey::Number(num) => Some(*num),
			Monkey::Operation { lhs, rhs, op } => {
				let lhs = self.get(lhs, cache);
				let rhs = self.get(rhs, cache);
				if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
					match op {
						Op::Add => Some(lhs + rhs),
						Op::Sub => Some(lhs - rhs),
						Op::Mul => Some(lhs * rhs),
						Op::Div => Some(lhs / rhs),
						Op::Eq => None,
					}
				} else {
					None
				}
			},
			Monkey::Human => None,
		};

		cache.insert(name, num);
		num
	}

	fn set(
		&self,
		name: &'a str,
		num: i64,
		cache: &mut HashMap<&'a str, Option<i64>>,
	) -> Result<(), ()> {
		match self.0.get(name).unwrap() {
			Monkey::Human => {
				cache.insert(name, Some(num));
				Ok(())
			},
			Monkey::Number(_) => Err(()),
			Monkey::Operation { lhs, rhs, op } => {
				let lhs_x = self.get(lhs, cache);
				let rhs_x = self.get(rhs, cache);
				match (lhs_x, rhs_x) {
					(Some(lhs_x), None) => match op {
						Op::Add => self.set(rhs, num - lhs_x, cache),
						Op::Sub => self.set(rhs, lhs_x - num, cache),
						Op::Mul => self.set(rhs, num / lhs_x, cache),
						Op::Div => self.set(rhs, lhs_x / num, cache),
						Op::Eq => self.set(rhs, lhs_x, cache), // num ignored
					},
					(None, Some(rhs_x)) => match op {
						Op::Add => self.set(lhs, num - rhs_x, cache),
						Op::Sub => self.set(lhs, num + rhs_x, cache),
						Op::Mul => self.set(lhs, num / rhs_x, cache),
						Op::Div => self.set(lhs, num * rhs_x, cache),
						Op::Eq => self.set(lhs, rhs_x, cache), // num ignored
					},
					_ => Err(()),
				}
			},
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/21/test");

	#[test]
	fn part1() {
		assert_eq!("152", &get_answer1(TEST).to_string());
		assert_eq!("170237589447588", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("301", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
