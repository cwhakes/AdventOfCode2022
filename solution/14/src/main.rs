use std::{collections::HashMap, fmt::Display};

static INPUT: &str = include_str!("../../../input/14/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut map = Map::new(input);
	let abyss = *map.0.keys().map(|Coord { y, .. }| y).max().unwrap();

	'outer: for count in 0.. {
		let mut coord = Coord { x: 500, y: 0 };
		map.0.insert(coord, Tile::Sand);
		loop {
			let new_coord = map.process_sand(coord).unwrap();
			if new_coord.y > abyss {
				return count;
			} else if new_coord == coord {
				continue 'outer;
			} else {
				coord = new_coord;
			}
		}
	}
	unreachable!()
}

fn get_answer2(input: &str) -> impl Display {
	let mut map = Map::new(input);
	let floor = *map.0.keys().map(|Coord { y, .. }| y).max().unwrap() + 2;

	'outer: for count in 0.. {
		let mut coord = Coord { x: 500, y: 0 };
		if Some(Tile::Sand) == map.0.insert(coord, Tile::Sand) {
			return count;
		}
		loop {
			let new_coord = map.process_sand(coord).unwrap();
			if new_coord.y == floor - 1 || new_coord == coord {
				continue 'outer;
			} else {
				coord = new_coord;
			}
		}
	}
	unreachable!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
	x: i16,
	y: i16,
}

impl Coord {
	fn new(input: &str) -> Option<Self> {
		let (x, y) = input.split_once(',')?;
		let (x, y) = (x.parse().ok()?, y.parse().ok()?);
		Some(Self { x, y })
	}

	fn range(self, other: Self) -> Box<dyn Iterator<Item = Self>> {
		if self.x == other.x {
			Box::new(
				((self.y.min(other.y))..=(self.y.max(other.y))).map(move |y| Self { x: self.x, y }),
			)
		} else if self.y == other.y {
			Box::new(
				((self.x.min(other.x))..=(self.x.max(other.x))).map(move |x| Self { x, y: self.y }),
			)
		} else {
			Box::new(std::iter::empty())
		}
	}

	fn down(self) -> impl IntoIterator<Item = Self> {
		[(0, 1), (-1, 1), (1, 1)].map(|(x, y)| Self {
			x: x + self.x,
			y: y + self.y,
		})
	}
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Tile {
	#[default]
	Air,
	Rock,
	Sand,
}

#[derive(Debug)]
struct Map(HashMap<Coord, Tile>);

impl Map {
	fn new(input: &str) -> Self {
		let mut map = HashMap::new();
		for line in input.lines() {
			let mut iter = line.split(" -> ").filter_map(Coord::new).peekable();
			while let Some(from) = iter.next() {
				if let Some(&to) = iter.peek() {
					for coord in from.range(to) {
						map.insert(coord, Tile::Rock);
					}
				}
			}
		}
		Self(map)
	}

	fn process_sand(&mut self, coord: Coord) -> Option<Coord> {
		if Some(&Tile::Sand) != self.0.get(&coord) {
			return None;
		}

		for new_coord in coord.down() {
			if &mut Tile::Air == self.0.entry(new_coord).or_default() {
				self.0.insert(coord, Tile::Air);
				self.0.insert(new_coord, Tile::Sand);
				return Some(new_coord);
			}
		}
		Some(coord)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/14/test");

	#[test]
	fn part1() {
		assert_eq!("24", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("93", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
