use std::collections::{HashMap, HashSet};
use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/24/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let valley = Valley::new(input);
	dijkstra::find_any_cost(
		Caravan {
			loc: valley.start,
			..Caravan::default()
		},
		Caravan {
			visited_end: true,
			loc: valley.end,
			..Caravan::default()
		},
		|loc, cost| valley.adjacencies(loc, cost),
	)
	.unwrap()
}

fn get_answer2(input: &str) -> impl Display {
	let valley = Valley::new(input);
	dijkstra::find_any_cost(
		Caravan {
			loc: valley.start,
			..Caravan::default()
		},
		Caravan {
			visited_end: true,
			visited_start: true,
			loc: valley.end,
		},
		|loc, cost| valley.adjacencies(loc, cost),
	)
	.unwrap()
}

struct Valley {
	map: HashSet<(i32, i32)>,
	blizzards_h: HashMap<i32, Vec<(i32, i8)>>,
	blizzards_v: HashMap<i32, Vec<(i32, i8)>>,
	min_x: i32,
	max_x: i32,
	min_y: i32,
	max_y: i32,
	start: (i32, i32),
	end: (i32, i32),
}

impl Valley {
	fn new(input: &str) -> Self {
		let mut map = HashSet::new();
		let mut blizzards_h: HashMap<_, Vec<_>> = HashMap::new();
		let mut blizzards_v: HashMap<_, Vec<_>> = HashMap::new();

		for (y, line) in input.lines().enumerate() {
			for (x, char) in line.chars().enumerate() {
				let (x, y) = (x as i32, y as i32);
				match char {
					'.' => {
						map.insert((x, y));
					},
					'<' => {
						blizzards_h.entry(y).or_default().push((x, -1));
						map.insert((x, y));
					},
					'>' => {
						blizzards_h.entry(y).or_default().push((x, 1));
						map.insert((x, y));
					},
					'^' => {
						blizzards_v.entry(x).or_default().push((y, -1));
						map.insert((x, y));
					},
					'v' => {
						blizzards_v.entry(x).or_default().push((y, 1));
						map.insert((x, y));
					},
					_ => {},
				}
			}
		}

		let min_x = map.iter().map(|(x, _y)| *x).min().unwrap();
		let max_x = map.iter().map(|(x, _y)| *x).max().unwrap();
		let min_y = map.iter().map(|(_x, y)| *y).min().unwrap() + 1; // Entrance
		let max_y = map.iter().map(|(_x, y)| *y).max().unwrap() - 1; // Exit

		let start = (min_x, min_y - 1);
		let end = (max_x, max_y + 1);

		assert!(map.contains(&start));
		assert!(map.contains(&end));

		Self {
			map,
			blizzards_v,
			blizzards_h,
			min_x,
			max_x,
			min_y,
			max_y,
			start,
			end,
		}
	}

	fn contains_blizzard(&self, (x, y): (i32, i32), time: u32) -> bool {
		for &(start_x, flow) in self.blizzards_h.get(&y).unwrap_or(&Vec::new()) {
			let b_x = (start_x + time as i32 * flow as i32 - self.min_x)
				.rem_euclid(self.max_x - self.min_x + 1)
				+ self.min_x;
			if x == b_x {
				return true;
			}
		}

		for &(start_y, flow) in self.blizzards_v.get(&x).unwrap_or(&Vec::new()) {
			let b_y = (start_y + time as i32 * flow as i32 - self.min_y)
				.rem_euclid(self.max_y - self.min_y + 1)
				+ self.min_y;
			if y == b_y {
				return true;
			}
		}

		false
	}

	fn adjacencies(
		&self,
		caravan: Caravan,
		time: u32,
	) -> impl Iterator<Item = (Caravan, u32)> + '_ {
		[(0, 1), (1, 0), (0, -1), (-1, 0)]
			.into_iter()
			.filter_map(move |offset| {
				let new_loc = (caravan.loc.0 + offset.0, caravan.loc.1 + offset.1);
				if self.map.contains(&new_loc) {
					for add_time in 1.. {
						if !self.contains_blizzard(new_loc, time + add_time) {
							return Some((
								Caravan {
									visited_start: caravan.visited_start
										|| caravan.visited_end && new_loc == self.start,
									visited_end: caravan.visited_end || new_loc == self.end,
									loc: new_loc,
								},
								add_time,
							));
						}
						if self.contains_blizzard(caravan.loc, time + add_time) {
							return None;
						}
					}
					unreachable!();
				} else {
					None
				}
			})
	}
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Caravan {
	visited_start: bool,
	visited_end: bool,
	loc: (i32, i32),
}

mod dijkstra {
	use std::cmp::Reverse;
	use std::collections::{BinaryHeap, HashSet};
	use std::hash::Hash;

	pub fn find_any_cost<Idx, F, I>(start: Idx, end: Idx, adjacencies: F) -> Option<u32>
	where
		Idx: Eq + Hash + Ord + Copy,
		F: Fn(Idx, u32) -> I,
		I: IntoIterator<Item = (Idx, u32)>,
	{
		let mut visited = HashSet::<(Idx, u32)>::new();
		let mut frontier = BinaryHeap::<(Reverse<u32>, Idx)>::new();
		frontier.push((Reverse(0), start));

		while let Some((Reverse(cost), loc)) = frontier.pop() {
			if loc == end {
				return Some(cost);
			}

			if !visited.contains(&(loc, cost)) {
				for (new_loc, add_cost) in adjacencies(loc, cost) {
					let new_cost = cost + add_cost;
					frontier.push((Reverse(new_cost), new_loc));
				}
				visited.insert((loc, cost));
			}
		}
		None
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/24/test");

	#[test]
	fn part1() {
		assert_eq!("18", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("54", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
