use std::{collections::HashMap, fmt::Display};

static INPUT: &str = include_str!("../../../input/15/input");

fn main() {
	let answer = get_answer1(INPUT, 2000000);
	println!("{}", answer);

	let answer = get_answer2(INPUT, 4000000);
	println!("{}", answer);
}

fn get_answer1(input: &str, row: i32) -> impl Display {
	let sensors = Sensors::new(input);
	let mut exclusion_zones: Vec<Zone> = Vec::new();
	let mut inclusion_zones: Vec<Zone> = Vec::new();
	for zone in sensors.exclusion_zones(row) {
		let new_inclusion_zones: Vec<Zone> = exclusion_zones
			.iter()
			.filter_map(|ez| ez.overlap(&zone))
			.collect();
		let new_exclusion_zones: Vec<Zone> = inclusion_zones
			.iter()
			.filter_map(|iz| iz.overlap(&zone))
			.collect();
		exclusion_zones.push(zone);
		exclusion_zones.extend(new_exclusion_zones);
		inclusion_zones.extend(new_inclusion_zones);
	}

	exclusion_zones.into_iter().map(|z| z.len()).sum::<u32>()
		- inclusion_zones.into_iter().map(|z| z.len()).sum::<u32>()
}

fn get_answer2(input: &str, max_pos: i32) -> impl Display {
	let sensors = Sensors::new(input);
	let pos = sensors
		.0
		.iter()
		.flat_map(|(sensor, &dist)| sensor.border(dist))
		.filter(|pos| (0..=max_pos).contains(&pos.x) && (0..=max_pos).contains(&pos.y))
		.find(|pos| sensors.validate(pos))
		.unwrap();
	pos.tuning_frequency()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Position {
	x: i32,
	y: i32,
}

impl Position {
	fn new(input: &str) -> Self {
		let (x, y) = input.trim_start_matches("x=").split_once(", y=").unwrap();
		Self {
			x: x.parse().unwrap(),
			y: y.parse().unwrap(),
		}
	}

	fn dist(&self, other: &Self) -> u32 {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
	}

	fn border(& self, dist: u32) -> impl Iterator<Item = Self> + '_ {
		let (x, y) = (self.x, self.y);
		let radius = dist as i32 + 1;
		(0..radius)
			.map(move |i| Self {
				x: x - radius + i,
				y: y + i,
			})
			.chain((0..radius).map(move |i| Self {
				x: x + i,
				y: y + radius - i,
			}))
			.chain((0..radius).map(move |i| Self {
				x: x + radius - i,
				y: y - i,
			}))
			.chain((0..radius).map(move |i| Self {
				x: x - i,
				y: y - radius + i,
			}))
	}

	fn tuning_frequency(&self) -> impl Display {
		self.x as i64 * 4000000 + self.y as i64
	}
}

struct Sensors(HashMap<Position, u32>);

impl Sensors {
	fn new(input: &str) -> Self {
		let mut sensors = HashMap::new();
		for line in input.lines() {
			let (sensor, beacon) = line
				.trim_start_matches("Sensor at ")
				.split_once(": closest beacon is at ")
				.unwrap();
			let (sensor, beacon) = (Position::new(sensor), Position::new(beacon));
			let dist = sensor.dist(&beacon);
			sensors.insert(sensor, dist);
		}
		Self(sensors)
	}

	fn validate(&self, pos: &Position) -> bool {
		self.0.iter().all(|(sensor, &dist)| sensor.dist(pos) > dist)
	}

	fn exclusion_zones(&self, row: i32) -> Vec<Zone> {
		let mut zones = Vec::new();
		for (sensor, dist) in self.0.iter() {
			let height = sensor.y.abs_diff(row);
			let Some(width) = dist.checked_sub(height) else { continue; };
			zones.push(Zone {
				start: sensor.x - width as i32,
				end: sensor.x + width as i32,
			})
		}
		zones
	}
}

struct Zone {
	start: i32,
	end: i32,
}

impl Zone {
	fn len(&self) -> u32 {
		(self.end - self.start) as u32
	}

	fn overlap(&self, other: &Self) -> Option<Self> {
		let start = i32::max(self.start, other.start);
		let end = i32::min(self.end, other.end);
		(start <= end).then_some(Self { start, end })
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/15/test");

	#[test]
	fn part1() {
		assert_eq!("26", &get_answer1(TEST, 10).to_string());
		assert_eq!("5403290", &get_answer1(INPUT, 2000000).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("56000011", &get_answer2(TEST, 20).to_string());
		// assert_eq!("10291582906626", &get_answer2(INPUT, 4000000).to_string());
	}
}
