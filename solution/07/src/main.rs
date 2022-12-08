use std::{collections::BTreeMap, fmt::Display};

static INPUT: &str = include_str!("../../../input/07/input");

fn main() {
	let answer = get_answer1(INPUT);
	println!("{}", answer);

	let answer = get_answer2(INPUT);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let object = Object::new(input);
	object
		.iter()
		.map(Object::size)
		.filter(|&s| s <= 100000)
		.sum::<i64>()
}

fn get_answer2(input: &str) -> impl Display {
	let object = Object::new(input);
	let free_space = 70000000 - object.size();
	let needed_space = 30000000 - free_space;
	object
		.iter()
		.map(Object::size)
		.filter(|&s| s >= needed_space)
		.min()
		.unwrap()
}

#[derive(Debug)]
enum Object {
	Directory(BTreeMap<String, Object>),
	File(i64),
}

impl Object {
	fn new_dir() -> Self {
		Self::Directory(BTreeMap::new())
	}

	fn new_file(size: i64) -> Self {
		Self::File(size)
	}

	fn new(input: &str) -> Self {
		let mut root = Object::new_dir();
		let mut path = Vec::new();
		let mut iter = input.lines().peekable();
		while let Some(command) = iter.next() {
			if command.starts_with("$ cd ") {
				let name = command.trim_start_matches("$ cd ");
				if name == "/" {
					path.clear();
				} else if name == ".." {
					path.pop();
				} else {
					path.push(name);
				}
			} else if command.starts_with("$ ls") {
				let Some(Object::Directory(dir)) = root.get_mut(&path) else { panic!() };
				while let Some(command) = iter.peek() {
					if command.starts_with('$') {
						break;
					}
					let object = iter.next().unwrap();
					if object.starts_with("dir ") {
						let name = object.trim_start_matches("dir ");
						if !dir.contains_key(name) {
							dir.insert(name.to_owned(), Object::new_dir());
						}
					} else {
						let (size, name) = object.split_once(' ').unwrap();
						let size = size.parse().unwrap();
						if !dir.contains_key(name) {
							dir.insert(name.to_owned(), Object::new_file(size));
						}
					}
				}
			} else {
				panic!()
			}
		}
		root
	}

	fn size(&self) -> i64 {
		match self {
			Self::Directory(map) => map.values().map(Object::size).sum(),
			Self::File(size) => *size,
		}
	}

	fn get_mut(&mut self, path: &[impl AsRef<str>]) -> Option<&mut Self> {
		let mut cursor = &mut *self;
		for dir in path {
			if let Object::Directory(map) = cursor {
				cursor = map.get_mut(dir.as_ref())?;
			} else {
				return None;
			}
		}
		Some(cursor)
	}

	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self> + 'a> {
		match self {
			Self::Directory(map) => Box::new(
				[self]
					.into_iter()
					.chain(map.values().flat_map(Object::iter)),
			) as Box<_>,
			Self::File(_) => Box::new([].into_iter()) as Box<_>,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	static TEST: &str = include_str!("../../../input/07/test");

	#[test]
	fn part1() {
		assert_eq!("95437", &get_answer1(TEST).to_string());
		// assert_eq!("0", &get_answer1(INPUT).to_string());
	}

	#[test]
	fn part2() {
		assert_eq!("24933642", &get_answer2(TEST).to_string());
		// assert_eq!("0", &get_answer2(INPUT).to_string());
	}
}
