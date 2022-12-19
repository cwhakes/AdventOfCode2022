use std::collections::HashSet;
use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/19/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	input
		.lines()
		.map(Blueprint::new)
		.enumerate()
		.map(|(i, bp)| bp.simulate(24) * (i as u32 + 1))
		.sum::<u32>()
}

fn get_answer2(input: &str) -> impl Display {
	input
		.lines()
		.take(3)
		.map(Blueprint::new)
		.map(|bp| bp.simulate(32))
		.product::<u32>()
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Resources {
	geode: u32,
	obsidian: u32,
	clay: u32,
	ore: u32,
}

impl Resources {
	fn new(input: &str) -> Self {
		let mut iter = input.split(" and ").peekable();
		let ore = iter
			.peek()
			.and_then(|s| s.strip_suffix(" ore"))
			.map_or(0, |ore| {
				let _ = iter.next(); // Consume
				ore.parse().unwrap()
			});
		let clay = iter
			.peek()
			.and_then(|s| s.strip_suffix(" clay"))
			.map_or(0, |clay| {
				let _ = iter.next(); // Consume
				clay.parse().unwrap()
			});
		let obsidian = iter
			.peek()
			.and_then(|s| s.strip_suffix(" obsidian"))
			.map_or(0, |obsidian| {
				let _ = iter.next(); // Consume
				obsidian.parse().unwrap()
			});
		let geode = 0;
		Self {
			geode,
			obsidian,
			clay,
			ore,
		}
	}

	fn checked_sub(&self, rhs: &Self) -> Option<Self> {
		let geode = self.geode.checked_sub(rhs.geode)?;
		let obsidian = self.obsidian.checked_sub(rhs.obsidian)?;
		let clay = self.clay.checked_sub(rhs.clay)?;
		let ore = self.ore.checked_sub(rhs.ore)?;
		Some(Self {
			geode,
			obsidian,
			clay,
			ore,
		})
	}

	fn with_collection(mut self, robots: &Robots) -> Self {
		self.geode += robots.geode;
		self.obsidian += robots.obsidian;
		self.clay += robots.clay;
		self.ore += robots.ore;
		self
	}
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Robots {
	geode: u32,
	obsidian: u32,
	clay: u32,
	ore: u32,
}

#[derive(Debug)]
struct Blueprint {
	robot_ore: Resources,
	robot_clay: Resources,
	robot_obsidian: Resources,
	robot_geode: Resources,
}

impl Blueprint {
	fn new(input: &str) -> Self {
		let (_name, input) = input.split_once(": ").unwrap();
		let mut iter = input.trim_end_matches('.').split(". ");
		let robot_ore = Resources::new(
			iter.next()
				.unwrap()
				.trim_start_matches("Each ore robot costs "),
		);
		let robot_clay = Resources::new(
			iter.next()
				.unwrap()
				.trim_start_matches("Each clay robot costs "),
		);
		let robot_obsidian = Resources::new(
			iter.next()
				.unwrap()
				.trim_start_matches("Each obsidian robot costs "),
		);
		let robot_geode = Resources::new(
			iter.next()
				.unwrap()
				.trim_start_matches("Each geode robot costs "),
		);

		Self {
			robot_ore,
			robot_clay,
			robot_obsidian,
			robot_geode,
		}
	}

	fn max_ore_cost(&self) -> u32 {
		[
			self.robot_ore.ore,
			self.robot_clay.ore,
			self.robot_obsidian.ore,
			self.robot_geode.ore,
		]
		.into_iter()
		.max()
		.unwrap()
	}

	fn max_clay_cost(&self) -> u32 {
		[
			self.robot_ore.clay,
			self.robot_clay.clay,
			self.robot_obsidian.clay,
			self.robot_geode.clay,
		]
		.into_iter()
		.max()
		.unwrap()
	}

	fn max_obsidian_cost(&self) -> u32 {
		[
			self.robot_ore.obsidian,
			self.robot_clay.obsidian,
			self.robot_obsidian.obsidian,
			self.robot_geode.obsidian,
		]
		.into_iter()
		.max()
		.unwrap()
	}

	fn simulate(&self, time: u32) -> u32 {
		let mut state = State {
			time,
			..State::default()
		};
		state.robot.ore = 1;
		let mut frontier = vec![state];
		let mut visited = HashSet::new();
		let mut max = 0;
		while let Some(state) = frontier.pop() {
			if visited.insert(state.clone()) {
				if state.time > 0 {
					frontier.extend(state.adjacencies(self));
				} else {
					max = max.max(state.resource.geode);
				}
			}
		}
		max
	}
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
	time: u32,
	robot: Robots,
	resource: Resources,
}

impl State {
	fn adjacencies(&self, bp: &Blueprint) -> impl Iterator<Item = Self> + '_ {
		[
			self.clone().try_build(&bp.robot_geode).map(|mut s| {
				s.robot.geode += 1;
				s
			}),
			if self.should_build_obsidian_robot(bp) {
				self.clone().try_build(&bp.robot_obsidian).map(|mut s| {
					s.robot.obsidian += 1;
					s
				})
			} else {
				None
			},
			if self.should_build_clay_robot(bp) {
				self.clone().try_build(&bp.robot_clay).map(|mut s| {
					s.robot.clay += 1;
					s
				})
			} else {
				None
			},
			if self.should_build_ore_robot(bp) {
				self.clone().try_build(&bp.robot_ore).map(|mut s| {
					s.robot.ore += 1;
					s
				})
			} else {
				None
			},
			Some(Self {
				time: 0,
				resource: Resources {
					geode: self.resource.geode + self.robot.geode * self.time,
					obsidian: self.resource.obsidian + self.robot.obsidian * self.time,
					clay: self.resource.clay + self.robot.clay * self.time,
					ore: self.resource.ore + self.robot.ore * self.time,
				},
				robot: self.robot.clone(),
			}),
		]
		.into_iter()
		.flatten()
	}

	fn should_build_ore_robot(&self, bp: &Blueprint) -> bool {
		self.robot.ore * self.time + self.resource.ore < self.time * bp.max_ore_cost()
	}

	fn should_build_clay_robot(&self, bp: &Blueprint) -> bool {
		self.robot.clay * self.time + self.resource.clay < self.time * bp.max_clay_cost()
	}

	fn should_build_obsidian_robot(&self, bp: &Blueprint) -> bool {
		self.robot.obsidian * self.time + self.resource.obsidian
			< self.time * bp.max_obsidian_cost()
	}

	fn try_build(mut self, cost: &Resources) -> Option<Self> {
		while self.time > 0 {
			self.time -= 1;
			if let Some(res) = self.resource.checked_sub(cost) {
				return Some(Self {
					resource: res.with_collection(&self.robot),
					..self
				});
			}
			self.resource = self.resource.with_collection(&self.robot);
		}
		None
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/19/test");

	#[test]
	fn part1() {
		assert_eq!("33", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	// 	#[test]
	// 	fn part2() {
	// 		assert_eq!("0", &get_answer2(TEST).to_string());
	// 		// assert_eq!("0", &get_answer2(INPUT).to_string());
	// 	}
}
