#![allow(clippy::needless_collect)]
use std::{collections::HashMap, fmt::Display};

static INPUT: &str = include_str!("../../../input/12/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	ElevationMap::new(input).unwrap().pathfind().unwrap()
}

fn get_answer2(input: &str) -> impl Display {
	let mut map = ElevationMap::new(input).unwrap();
	map.start = (-1, -1);
	map.pathfind().unwrap()
}

struct ElevationMap {
	map: HashMap<(i16, i16), u8>,
	start: (i16, i16),
	end: (i16, i16),
}

impl ElevationMap {
	fn new(input: &str) -> Option<Self> {
		let mut map = HashMap::new();
		let mut start = None;
		let mut end = None;
		for (y, line) in input.lines().enumerate() {
			for (x, byte) in line.bytes().enumerate() {
				let loc = (x as i16, y as i16);
				match byte {
					b'S' => {
						start = Some(loc);
						map.insert(loc, b'a');
					},
					b'E' => {
						end = Some(loc);
						map.insert(loc, b'z');
					},
					b => {
						map.insert(loc, b);
					},
				}
			}
		}
		Some(Self {
			map,
			start: start?,
			end: end?,
		})
	}

	fn adjacencies<'a>(
		&'a self,
		loc: (i16, i16),
	) -> Box<dyn Iterator<Item = ((i16, i16), u32)> + 'a> {
		// Fake point
		if loc == (-1, -1) {
			Box::new(
				self.map
					.iter()
					.filter_map(|(&loc, &ele)| (ele == b'a').then_some((loc, 0))),
			)
		} else {
			let ele = self.map.get(&loc).unwrap();
			Box::new(
				[(1, 0), (0, 1), (-1, 0), (0, -1)]
					.into_iter()
					.map(move |off| (loc.0 + off.0, loc.1 + off.1))
					.filter(move |loc| {
						self.map
							.get(loc)
							.map_or(false, |new_ele| *new_ele <= ele + 1)
					})
					.map(|loc| (loc, 1)),
			)
		}
	}

	fn pathfind(&self) -> Option<u32> {
		dijkstra::find_lowest_cost(self.start, self.end, |loc| self.adjacencies(loc))
	}
}

mod dijkstra {
	use std::cmp::Reverse;
	use std::collections::{BinaryHeap, HashMap};
	use std::hash::Hash;

	pub fn find_lowest_cost<Idx, F, I>(start: Idx, end: Idx, adjacencies: F) -> Option<u32>
	where
		Idx: Eq + Hash + Ord + Copy,
		F: Fn(Idx) -> I,
		I: IntoIterator<Item = (Idx, u32)>,
	{
		let mut costs = HashMap::<Idx, u32>::new();
		let mut frontier = BinaryHeap::<Reverse<(u32, Idx)>>::new();
		frontier.push(Reverse((0, start)));

		while let Some(Reverse((cost, loc))) = frontier.pop() {
			if let Some(&old_cost) = costs.get(&end) {
				if old_cost <= cost {
					return Some(old_cost);
				}
			}

			for (new_loc, add_cost) in adjacencies(loc) {
				let new_cost = cost + add_cost;
				let old_cost = costs.get(&new_loc);
				if old_cost.is_none() || *old_cost.unwrap() > new_cost {
					costs.insert(new_loc, new_cost);
					frontier.push(Reverse((new_cost, new_loc)));
				}
			}
		}
		None
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/12/test");

	#[test]
	fn part1() {
		assert_eq!("31", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("29", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
