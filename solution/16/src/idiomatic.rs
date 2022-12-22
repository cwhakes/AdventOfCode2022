use std::{
	collections::{BinaryHeap, HashMap, HashSet},
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

	let adjacencies = Adjacencies::new(&map.all_valves)
		.floyd_warshall()
		.filter(|(a, b)| {
			(a == *b"AA" || map.all_valves.get(&a).map_or(false, |v| v.flow_rate > 0))
				&& map.all_valves.get(&b).map_or(false, |v| v.flow_rate > 0)
		});

	let mut frontier = BinaryHeap::new();
	frontier.push((0, State::new(30)));
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
			best = dbg!(current_pressure);
		}
		if state.time > 0 {
			frontier.extend(
				state
					.adjacencies(&map, &adjacencies)
					.map(|(state, new_pressure)| (current_pressure + new_pressure, state)),
			);
		}
		visited.insert(state, current_pressure);
	}
	best
}

fn get_answer2(input: &str) -> impl Display {
	let map = Map::new(input);

	let adjacencies = Adjacencies::new(&map.all_valves)
		.floyd_warshall()
		.filter(|(a, b)| {
			(a == *b"AA" || map.all_valves.get(&a).map_or(false, |v| v.flow_rate > 0))
				&& map.all_valves.get(&b).map_or(false, |v| v.flow_rate > 0)
		});

	let mut frontier = BinaryHeap::new();
	frontier.push((0, State2::new(26)));
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
		if state.time_ele > 0 {
			frontier.extend(
				state
					.adjacencies(&map, &adjacencies)
					.map(|(state, new_pressure)| (current_pressure + new_pressure, state)),
			);
		}
		visited.insert(state, current_pressure);
	}
	best
}

type Name = [u8; 2];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
	flow_rate: i32,
	adjacencies: Vec<Name>,
}

impl Valve {
	fn new(input: &str) -> (Name, Self) {
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
			.map(|s| <Name>::try_from(s.as_ref()).unwrap())
			.collect();
		(
			<Name>::try_from(name.as_ref()).unwrap(),
			Self {
				flow_rate,
				adjacencies,
			},
		)
	}
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
	all_valves: HashMap<Name, Valve>,
	openables: HashMap<Name, (i32, Valve)>,
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
}

#[derive(Debug)]
struct Adjacencies(HashMap<Name, HashMap<Name, u16>>);

impl Adjacencies {
	fn new<'a>(iter: impl IntoIterator<Item = (&'a Name, &'a Valve)>) -> Self {
		let mut adjacencies: HashMap<Name, HashMap<Name, u16>> = HashMap::new();
		for (from_name, valve) in iter {
			for to_name in &valve.adjacencies {
				adjacencies
					.entry(*from_name)
					.or_default()
					.insert(*to_name, u16::from(from_name != to_name));
			}
		}
		Self(adjacencies)
	}

	fn get(&self, (from_name, to_name): (Name, Name)) -> Option<u16> {
		self.0
			.get(&from_name)
			.and_then(|sa| sa.get(&to_name))
			.copied()
	}

	fn floyd_warshall(mut self) -> Self {
		let names: HashSet<Name> = self.0.keys().copied().collect();
		for &k_name in &names {
			let mut new: HashMap<Name, HashMap<Name, u16>> = HashMap::new();
			for &from_name in &names {
				for &to_name in &names {
					let old_cost = self.get((from_name, to_name));
					let new_cost = self
						.get((from_name, k_name))
						.and_then(|c| Some(c + self.get((k_name, to_name))?));
					if let Some(final_cost) = match (old_cost, new_cost) {
						(Some(o), Some(n)) => Some(o.min(n)),
						(Some(o), None) => Some(o),
						(None, Some(n)) => Some(n),
						(None, None) => None,
					} {
						new.entry(from_name)
							.or_default()
							.insert(to_name, final_cost);
					}
				}
			}
			self.0 = new;
		}
		self
	}

	fn filter(self, mut f: impl FnMut((Name, Name)) -> bool) -> Self {
		let mut new: HashMap<Name, HashMap<Name, u16>> = HashMap::new();
		for (from_name, sub_adjacencies) in self.0 {
			for (to_name, cost) in sub_adjacencies {
				if f((from_name, to_name)) {
					new.entry(from_name).or_default().insert(to_name, cost);
				}
			}
		}
		Self(new)
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
	time: u16,
	current_valve: Name,
	open_valves: ValveHash,
}

impl State {
	fn new(time: u16) -> Self {
		Self {
			time,
			open_valves: ValveHash::default(),
			current_valve: *b"AA",
		}
	}

	fn adjacencies<'a>(
		&'a self,
		map: &'a Map,
		adjacencies: &'a Adjacencies,
	) -> impl Iterator<Item = (Self, i32)> + 'a {
		let adjacencies = adjacencies.0.get(&self.current_valve);

		std::iter::once((
			Self {
				time: 0,
				..self.clone()
			},
			0,
		))
		.chain(
			adjacencies
				.into_iter()
				.flatten()
				.filter_map(|(to_name, cost)| {
					let (offset, valve) = map.openables.get(to_name).unwrap();
					if !self.open_valves.contains_offset(*offset) {
						let time = self.time.checked_sub(*cost)?.checked_sub(1)?;
						let released = time as i32 * valve.flow_rate;
						let mut open_valves = self.open_valves.clone();
						open_valves.insert_offset(*offset);

						Some((
							Self {
								time,
								current_valve: *to_name,
								open_valves,
							},
							released,
						))
					} else {
						None
					}
				}),
		)
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State2 {
	time: u16,
	time_ele: u16,
	current_valve: Name,
	open_valves: ValveHash,
}

impl State2 {
	fn new(time: u16) -> Self {
		Self {
			time,
			time_ele: time,
			open_valves: ValveHash::default(),
			current_valve: *b"AA",
		}
	}

	fn adjacencies<'a>(
		&'a self,
		map: &'a Map,
		adjacencies: &'a Adjacencies,
	) -> impl Iterator<Item = (Self, i32)> + 'a {
		let adjacencies = adjacencies.0.get(&self.current_valve);

		if self.time > 0 {
			Choice::Left(
				std::iter::once((
					Self {
						time: 0,
						current_valve: *b"AA",
						..self.clone()
					},
					0,
				))
				.chain(
					adjacencies
						.into_iter()
						.flatten()
						.filter_map(|(to_name, cost)| {
							let (offset, valve) = map.openables.get(to_name).unwrap();
							if !self.open_valves.contains_offset(*offset) {
								let time = self.time.checked_sub(*cost)?.checked_sub(1)?;
								let released = time as i32 * valve.flow_rate;
								let mut open_valves = self.open_valves.clone();
								open_valves.insert_offset(*offset);

								Some((
									Self {
										time,
										current_valve: if time != 0 { *to_name } else { *b"AA" },
										open_valves,
										..self.clone()
									},
									released,
								))
							} else {
								None
							}
						}),
				),
			)
		} else {
			Choice::Right(
				std::iter::once((
					Self {
						time_ele: 0,
						..self.clone()
					},
					0,
				))
				.chain(
					adjacencies
						.into_iter()
						.flatten()
						.filter_map(|(to_name, cost)| {
							let (offset, valve) = map.openables.get(to_name).unwrap();
							if !self.open_valves.contains_offset(*offset) {
								let time_ele = self.time_ele.checked_sub(*cost)?.checked_sub(1)?;
								let released = time_ele as i32 * valve.flow_rate;
								let mut open_valves = self.open_valves.clone();
								open_valves.insert_offset(*offset);

								Some((
									Self {
										time_ele,
										current_valve: *to_name,
										open_valves,
										..self.clone()
									},
									released,
								))
							} else {
								None
							}
						}),
				),
			)
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
