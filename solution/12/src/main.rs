#![allow(clippy::needless_collect)]

use std::{
	cmp::Reverse,
	collections::{BinaryHeap, HashMap},
	fmt::Display,
};

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
	let mins: Vec<_> = map
		.map
		.iter()
		.filter_map(|(&loc, &ele)| (ele == 0).then_some(loc))
		.collect();
	mins.into_iter()
		.filter_map(|min| {
			map.start = min;
			map.pathfind()
		})
		.min()
		.unwrap()
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
				match byte {
					b'S' => {
						start = Some((x as i16, y as i16));
						map.insert((x as i16, y as i16), 0);
					},
					b'E' => {
						end = Some((x as i16, y as i16));
						map.insert((x as i16, y as i16), 25);
					},
					b => {
						map.insert((x as i16, y as i16), b - b'a');
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

	fn pathfind(&self) -> Option<usize> {
		let mut costs: HashMap<(i16, i16), Option<usize>> =
			self.map.keys().map(|loc| (*loc, None)).collect();
		let mut frontier = BinaryHeap::<Reverse<(usize, (i16, i16))>>::new();
		frontier.push(Reverse((0, self.start)));

		while let Some(Reverse((cost, loc))) = frontier.pop() {
			if loc == self.end {
				return Some(cost);
			}

			let elvevation = self.map.get(&loc).unwrap();
			let new_locs =
				[(1, 0), (0, 1), (-1, 0), (0, -1)].map(|off| (loc.0 + off.0, loc.1 + off.1));
			for new_loc in new_locs {
				let Some(&new_elevation) = self.map.get(&new_loc) else { continue };
				if new_elevation <= elvevation + 1 {
					if let Some(curr_cost) = costs.get_mut(&new_loc) {
						if curr_cost.is_none() || curr_cost.unwrap() > cost + 1 {
							*curr_cost = Some(cost + 1);
							frontier.push(Reverse((cost + 1, new_loc)));
						}
					}
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
