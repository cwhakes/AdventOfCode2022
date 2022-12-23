use std::collections::HashMap;
use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/22/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let (map, path) = input.split_once("\n\n").unwrap();
	let map = Map::new(map);
	let path = Path::new(path);

	let mut cursor = Cursor::new(&map);
	cursor.walk_all(path.0, &map);
	cursor.score()
}

fn get_answer2(input: &str) -> impl Display {
	let (map, path) = input.split_once("\n\n").unwrap();
	let map = Map::new(map);
	let path = Path::new(path);

	let mut cursor = Cursor::new(&map);
	cursor.fake_caches();
	cursor.walk_all(path.0, &map);
	cursor.score()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
	Space,
	Wall,
}

struct Map(HashMap<(i32, i32), Tile>);

impl Map {
	fn new(input: &str) -> Self {
		let mut map = HashMap::new();
		for (y, line) in input.lines().enumerate() {
			for (x, tile) in line.bytes().enumerate() {
				match tile {
					b'.' => {
						map.insert((x as i32, y as i32), Tile::Space);
					},
					b'#' => {
						map.insert((x as i32, y as i32), Tile::Wall);
					},
					_ => {},
				}
			}
		}
		Self(map)
	}
}

#[derive(Debug, Clone)]
enum Inst {
	Forward(i32),
	Left,
	Right,
}

#[derive(Debug, Clone)]
struct Path(Vec<Inst>);

impl Path {
	fn new(input: &str) -> Self {
		use nom::branch::alt;
		use nom::bytes::complete::tag;
		use nom::character::complete::i32 as parse_i32;
		use nom::combinator::{map, value};
		use nom::multi::many0;
		use nom::IResult;

		let mut parser = many0(alt((
			map(parse_i32, Inst::Forward),
			value(Inst::Left, tag("L")),
			value(Inst::Right, tag("R")),
		)));
		let parsed: IResult<_, _> = parser(input);
		let (_, new) = parsed.unwrap();
		Self(new)
	}
}

#[derive(Debug, Default)]
struct Cursor {
	loc: (i32, i32),
	dir: u8,
	min_x: HashMap<i32, ((i32, i32), u8)>,
	max_x: HashMap<i32, ((i32, i32), u8)>,
	min_y: HashMap<i32, ((i32, i32), u8)>,
	max_y: HashMap<i32, ((i32, i32), u8)>,
}

impl Cursor {
	fn new(map: &Map) -> Self {
		let (&loc, _) = map
			.0
			.iter()
			.filter(|((_, y), t)| *y == 0 && **t == Tile::Space)
			.min_by_key(|((x, _), _)| x)
			.unwrap();
		Self {
			loc,
			dir: 0,
			..Self::default()
		}
	}

	fn score(&self) -> i32 {
		1000 * (self.loc.1 + 1) + 4 * (self.loc.0 + 1) + self.dir as i32
	}

	fn walk(&mut self, inst: Inst, map: &Map) {
		match inst {
			Inst::Left => self.dir = (self.dir + 3) % 4,
			Inst::Right => self.dir = (self.dir + 1) % 4,
			Inst::Forward(num) => {
				for _ in 0..num {
					let mut new_loc;
					let mut new_dir = self.dir;
					let new_tile = match self.dir {
						0 => {
							new_loc = (self.loc.0 + 1, self.loc.1);
							map.0
								.get(&new_loc)
								.or_else(|| {
									(new_loc, new_dir) =
										*self.min_x.entry(self.loc.1).or_insert_with(|| {
											(
												*map.0
													.keys()
													.filter(|(_, y)| *y == self.loc.1)
													.min_by_key(|(x, _)| x)
													.unwrap(),
												self.dir,
											)
										});
									map.0.get(&new_loc)
								})
								.unwrap()
						},
						1 => {
							new_loc = (self.loc.0, self.loc.1 + 1);
							map.0
								.get(&new_loc)
								.or_else(|| {
									(new_loc, new_dir) =
										*self.min_y.entry(self.loc.0).or_insert_with(|| {
											(
												*map.0
													.keys()
													.filter(|(x, _)| *x == self.loc.0)
													.min_by_key(|(_, y)| y)
													.unwrap(),
												self.dir,
											)
										});
									map.0.get(&new_loc)
								})
								.unwrap()
						},
						2 => {
							new_loc = (self.loc.0 - 1, self.loc.1);
							map.0
								.get(&new_loc)
								.or_else(|| {
									(new_loc, new_dir) =
										*self.max_x.entry(self.loc.1).or_insert_with(|| {
											(
												*map.0
													.keys()
													.filter(|(_, y)| *y == self.loc.1)
													.max_by_key(|(x, _)| x)
													.unwrap(),
												self.dir,
											)
										});
									map.0.get(&new_loc)
								})
								.unwrap()
						},
						3 => {
							new_loc = (self.loc.0, self.loc.1 - 1);
							map.0
								.get(&new_loc)
								.or_else(|| {
									(new_loc, new_dir) =
										*self.max_y.entry(self.loc.0).or_insert_with(|| {
											(
												*map.0
													.keys()
													.filter(|(x, _)| *x == self.loc.0)
													.max_by_key(|(_, y)| y)
													.unwrap(),
												self.dir,
											)
										});
									map.0.get(&new_loc)
								})
								.unwrap()
						},
						_ => panic!(),
					};
					if Tile::Space == *new_tile {
						self.loc = new_loc;
						self.dir = new_dir;
					} else {
						break;
					}
				}
			},
		}
	}

	fn walk_all(&mut self, insts: impl IntoIterator<Item = Inst>, map: &Map) {
		for inst in insts {
			self.walk(inst, map);
		}
	}

	fn fake_caches(&mut self) {
		self.max_y = ((0..50).map(|x| (x, ((50, x + 50), 0))))
			.chain((50..100).map(|x| (x, ((0, x - 50 + 150), 0))))
			.chain((100..150).map(|x| (x, ((x - 100, 199), 3))))
			.collect();
		self.max_x = ((0..50).map(|y| (y, ((0, 149 - y), 0))))
			.chain((50..100).map(|y| (y, ((y - 50, 100), 1))))
			.chain((100..150).map(|y| (y, ((50, 149 - y), 0))))
			.chain((150..200).map(|y| (y, ((y - 150 + 50, 0), 1))))
			.collect();
		self.min_y = ((0..50).map(|x| (x, ((x + 100, 0), 1))))
			.chain((50..100).map(|x| (x, ((49, x + 100), 2))))
			.chain((100..150).map(|x| (x, ((99, x - 50), 2))))
			.collect();
		self.min_x = ((0..50).map(|y| (y, ((99, 149 - y), 2))))
			.chain((50..100).map(|y| (y, ((y + 50, 49), 3))))
			.chain((100..150).map(|y| (y, ((149, 149 - y), 2))))
			.chain((150..200).map(|y| (y, ((y - 100, 149), 3))))
			.collect();
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/22/test");

	#[test]
	fn part1() {
		assert_eq!("6032", &get_answer1(TEST).to_string());
		assert_eq!("30552", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		// assert_eq!("0", &get_answer2(TEST).to_string());
		assert_eq!("184106", &get_answer2(INPUT).to_string());
	}
}
