use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/13/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	input
		.split("\n\n")
		.filter_map(|i| i.split_once('\n'))
		.filter_map(|(left, right)| Some((Packet::new(left)?, Packet::new(right)?)))
		.enumerate()
		.filter_map(|(i, (l, r))| (l <= r).then_some(i + 1))
		.sum::<usize>()
}

fn get_answer2(input: &str) -> impl Display {
	let mut packets: Vec<_> = input.lines().filter_map(Packet::new).collect();
	packets.push(Packet::new("[[2]]").unwrap());
	packets.push(Packet::new("[[6]]").unwrap());
	packets.sort();

	let a = packets
		.binary_search(&Packet::new("[[2]]").unwrap())
		.unwrap()
		+ 1;
	let b = packets
		.binary_search(&Packet::new("[[6]]").unwrap())
		.unwrap()
		+ 1;
	a * b
}

#[derive(Clone, PartialEq, Eq)]
enum Packet {
	Int(i32),
	List(Vec<Packet>),
}

impl Packet {
	fn new(input: &str) -> Option<Self> {
		Some(parse::packet(input).ok()?.1)
	}
}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(&Self::Int(left), &Self::Int(right)) => left.cmp(&right),
			(&Self::List(ref left), &Self::List(ref right)) => left.cmp(right),
			(left @ Self::Int(_), &Self::List(ref right)) => [left.clone()][..].cmp(&**right),
			(&Self::List(ref left), right @ Self::Int(_)) => (**left).cmp(&[right.clone()]),
		}
	}
}

mod parse {
	use super::*;

	use nom::branch::alt;
	use nom::bytes::complete::tag;
	use nom::character::complete::i32 as i32_parse;
	use nom::combinator::{cut, map};
	use nom::multi::separated_list0;
	use nom::sequence::{preceded, terminated};
	use nom::IResult;

	pub(crate) fn packet(s: &str) -> IResult<&str, Packet> {
		alt((
			map(
				preceded(
					tag("["),
					cut(terminated(separated_list0(tag(","), packet), tag("]"))),
				),
				Packet::List,
			),
			map(i32_parse, Packet::Int),
		))(s)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/13/test");

	#[test]
	fn part1() {
		assert_eq!("13", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("140", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
