use std::collections::HashSet;
use std::fmt::Display;

static INPUT: &str = include_str!("../../../input/18/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let map: HashSet<_> = input.lines().map(Cube::new).collect();
	let mut count = map.len() * 6;
	for cube in &map {
		for offset in cube.adjacencies() {
			if map.contains(&offset) {
				count -= 1;
			}
		}
	}
	count
}

fn get_answer2(input: &str) -> impl Display {
	let map: HashSet<_> = input.lines().map(Cube::new).collect();
	let min_x = map.iter().map(|Cube { x, .. }| x).min().unwrap() - 1;
	let max_x = map.iter().map(|Cube { x, .. }| x).max().unwrap() + 1;
	let min_y = map.iter().map(|Cube { y, .. }| y).min().unwrap() - 1;
	let max_y = map.iter().map(|Cube { y, .. }| y).max().unwrap() + 1;
	let min_z = map.iter().map(|Cube { z, .. }| z).min().unwrap() - 1;
	let max_z = map.iter().map(|Cube { z, .. }| z).max().unwrap() + 1;

	let mut outer_map = HashSet::new();
	let mut frontier = Vec::new();
	frontier.push(Cube {
		x: max_x,
		y: 0,
		z: 0,
	});
	while let Some(cube) = frontier.pop() {
		for offset in cube.adjacencies() {
			if min_x <= offset.x
				&& offset.x <= max_x
				&& min_y <= offset.y
				&& offset.y <= max_y
				&& min_z <= offset.z
				&& offset.z <= max_z
				&& !map.contains(&offset)
				&& outer_map.insert(offset.clone())
			{
				frontier.push(offset)
			}
		}
	}

	let mut count = 0;
	for cube in &map {
		for offset in cube.adjacencies() {
			if outer_map.contains(&offset) {
				count += 1;
			}
		}
	}
	count
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Cube {
	x: i32,
	y: i32,
	z: i32,
}

impl Cube {
	fn new(input: &str) -> Self {
		let mut iter = input.split(',');
		let x = iter.next().unwrap().parse().unwrap();
		let y = iter.next().unwrap().parse().unwrap();
		let z = iter.next().unwrap().parse().unwrap();
		Self { x, y, z }
	}

	fn adjacencies(&self) -> impl Iterator<Item = Self> + '_ {
		[
			(-1, 0, 0),
			(1, 0, 0),
			(0, -1, 0),
			(0, 1, 0),
			(0, 0, -1),
			(0, 0, 1),
		]
		.into_iter()
		.map(|(x, y, z)| Self {
			x: self.x + x,
			y: self.y + y,
			z: self.z + z,
		})
	}
}

// #[cfg(test)]
// mod test {
// 	use super::*;
// 	static TEST: &str = include_str!("../../../input/01/test");

// 	#[test]
// 	fn part1() {
// 		assert_eq!("0", &get_answer1(TEST).to_string());
// 		// assert_eq!("0", &get_answer1(INPUT).to_string());
// 	}

// 	#[test]
// 	fn part2() {
// 		assert_eq!("0", &get_answer2(TEST).to_string());
// 		// assert_eq!("0", &get_answer2(INPUT).to_string());
// 	}
// }
