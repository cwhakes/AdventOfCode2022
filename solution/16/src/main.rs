use std::{
	collections::{BinaryHeap, HashMap},
	fmt::Display,
};

static INPUT: &str = include_str!("../../../input/16/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let map = Map::new(input);
	let mut frontier = BinaryHeap::new();
	frontier.push((0, State::new()));
	let mut visited = HashMap::new();
	let mut best = 0;
	while let Some((current_pressure, state)) = frontier.pop() {
		if visited
			.get(&state)
			.map_or(false, |&p| p >= current_pressure)
		{
			continue;
		}
		best = best.max(current_pressure);
		frontier.extend(
			state
				.adjacencies(&map)
				.map(|(state, new_pressure)| (current_pressure + new_pressure, state)),
		);
		visited.insert(state, current_pressure);
	}
	best
}

fn get_answer2(input: &str) -> impl Display {
	let map = Map::new(input);
	let mut frontier = BinaryHeap::new();
	frontier.push((0, State2::new()));
	let mut visited = HashMap::new();
	let mut best = 0;
	while let Some((current_pressure, state)) = frontier.pop() {
		if visited
			.get(&state)
			.map_or(false, |&p| p >= current_pressure)
		{
			continue;
		}
		if current_pressure > best {
			best = current_pressure;
			dbg!(best);
		}
		frontier.extend(
			state
				.adjacencies(&map)
				.map(|(state, new_pressure)| (current_pressure + new_pressure, state)),
		);
		visited.insert(state, current_pressure);
	}
	best
}

#[derive(Clone, PartialEq, Eq)]
struct Valve {
	flow_rate: i32,
	adjacencies: Vec<[u8; 2]>,
}

impl Valve {
	fn new(input: &str) -> ([u8; 2], Self) {
		let (name, input) = input
			.trim_start_matches("Valve ")
			.split_once(" has flow rate=")
			.unwrap();
		let (flow_rate, adjacencies) = input
			.split_once("; tunnels lead to valves ")
			.or_else(|| input.split_once("; tunnel leads to valve "))
			.unwrap();
		let flow_rate = flow_rate.parse().unwrap();
		let adjacencies = adjacencies
			.split(", ")
			.map(|s| <[u8; 2]>::try_from(s.as_ref()).unwrap())
			.collect();
		(
			<[u8; 2]>::try_from(name.as_ref()).unwrap(),
			Self {
				flow_rate,
				adjacencies,
			},
		)
	}
}

#[derive(PartialEq, Eq)]
struct Map {
	all_valves: HashMap<[u8; 2], Valve>,
	openables: HashMap<[u8; 2], (i32, Valve)>,
}

impl Map {
	fn new(input: &str) -> Self {
		let valves: Vec<_> = input.lines().map(Valve::new).collect();
		let all_valves = valves.iter().cloned().collect();
		let openables = valves
			.iter()
			.filter(|(_name, v)| v.flow_rate > 0)
			.enumerate()
			.map(|(i, (name, v))| (*name, (i as i32, v.clone())))
			.collect();
		Self {
			all_valves,
			openables,
		}
	}

	fn adjacencies_at(&self, valve_name: [u8; 2]) -> impl Iterator<Item = [u8; 2]> + '_ {
		self.all_valves
			.get(&valve_name)
			.into_iter()
			.flat_map(|v| &v.adjacencies)
			.copied()
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
	time: i32,
	current_valve: [u8; 2],
	open_valves: ValveHash,
}

impl State {
	fn new() -> Self {
		Self {
			time: 0,
			open_valves: ValveHash::default(),
			current_valve: *b"AA",
		}
	}

	fn with_try_open_valve(&self, map: &Map, valve_name: [u8; 2]) -> Option<(Self, i32)> {
		if let Some((offset, valve)) = map.openables.get(&valve_name) {
			if !self.open_valves.contains_offset(*offset) {
				let mut new = self.clone();
				new.open_valves.insert_offset(*offset);
				return Some((new, valve.flow_rate));
			}
		}
		None
	}

	fn with_new_location(&self, new_location: [u8; 2]) -> Self {
		Self {
			current_valve: new_location,
			..self.clone()
		}
	}

	fn adjacencies<'a>(&self, map: &'a Map) -> impl Iterator<Item = (Self, i32)> + 'a {
		if self.time >= 30 {
			Choice::Left(std::iter::empty())
		} else {
			let proto = Self {
				time: self.time + 1,
				..self.clone()
			};

			Choice::Right(
				proto
					.with_try_open_valve(map, self.current_valve)
					.map(|(state, flow_rate)| (state, (30 - proto.time) * flow_rate))
					.into_iter()
					.chain(
						map.adjacencies_at(proto.current_valve)
							.map(move |v| (proto.with_new_location(v), 0)),
					),
			)
		}
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State2 {
	time: i32,
	current_valve: [u8; 2],
	open_valves: ValveHash,
}

impl State2 {
	fn new() -> Self {
		Self {
			time: 0,
			open_valves: ValveHash::default(),
			current_valve: *b"AA",
		}
	}

	fn with_try_open_valve(&self, map: &Map, valve_name: [u8; 2]) -> Option<(Self, i32)> {
		if let Some((offset, valve)) = map.openables.get(&valve_name) {
			if !self.open_valves.contains_offset(*offset) {
				let mut new = self.clone();
				new.open_valves.insert_offset(*offset);
				return Some((new, valve.flow_rate));
			}
		}
		None
	}

	fn with_new_location(&self, new_location: [u8; 2]) -> Self {
		Self {
			current_valve: new_location,
			..self.clone()
		}
	}

	fn adjacencies<'a>(&self, map: &'a Map) -> impl Iterator<Item = (Self, i32)> + 'a {
		if self.time < 26 * 2 {
			let proto = if self.time == 26 {
				Self {
					time: self.time + 1,
					current_valve: *b"AA",
					..self.clone()
				}
			} else {
				Self {
					time: self.time + 1,
					..self.clone()
				}
			};

			Choice::Left(
				proto
					.with_try_open_valve(map, proto.current_valve)
					.map(|(state, flow_rate)| {
						(state, (26 - ((proto.time - 1) % 26 + 1)) * flow_rate)
					})
					.into_iter()
					.chain(
						map.adjacencies_at(proto.current_valve)
							.map(move |v| (proto.with_new_location(v), 0)),
					),
			)
		} else {
			Choice::Right(std::iter::empty())
		}
	}
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveHash(u16);

impl ValveHash {
	fn contains_offset(&self, offset: i32) -> bool {
		self.0 & 1 << offset > 0
	}

	fn insert_offset(&mut self, offset: i32) {
		self.0 |= 1 << offset;
	}
}

enum Choice<L, R> {
	Left(L),
	Right(R),
}

impl<L, R> Iterator for Choice<L, R>
where
	L: Iterator,
	R: Iterator<Item = L::Item>,
{
	type Item = L::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::Left(l) => l.next(),
			Self::Right(r) => r.next(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/16/test");

	#[test]
	fn part1() {
		assert_eq!("1651", &get_answer1(TEST).to_string());
		// assert_eq!("1873", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("1707", &get_answer2(TEST).to_string());
		// assert_eq!("2425", &get_answer2(INPUT).to_string());
	}
}
