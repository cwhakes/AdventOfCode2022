use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

fn main() {
	let mut buf = String::new();
	let mut file = File::open("input/03/input").unwrap();
	file.read_to_string(&mut buf).unwrap();

	let answer = get_answer1(&buf);
	println!("{}", answer);

	let answer = get_answer2(&buf);
	println!("{}", answer);
}

fn get_answer1(input: &str) -> impl Display {
	let mut acc = 0;
	for rucksack in input.lines() {
		let (a, b) = rucksack.split_at(rucksack.len() / 2);
		let hash_a: HashSet<_> = a.chars().collect();
		let hash_b: HashSet<_> = b.chars().collect();
		for a in hash_a {
			if hash_b.contains(&a) {
				if a.is_ascii_uppercase() {
					acc += a as u32 - 64 + 26
				} else {
					acc += a as u32 - 96
				}
			}
		}
	}
	acc
}

fn get_answer2(input: &str) -> impl Display {
	let mut acc = 0;
	let mut iter = input.lines();
	while let [Some(a), Some(b), Some(c)] = [iter.next(), iter.next(), iter.next()] {
		let hash_a: HashSet<_> = a.chars().collect();
		let hash_b: HashSet<_> = b.chars().collect();
		let hash_c: HashSet<_> = c.chars().collect();
		for a in hash_a {
			if hash_b.contains(&a) && hash_c.contains(&a) {
				if a.is_ascii_uppercase() {
					acc += a as u32 - 64 + 26
				} else {
					acc += a as u32 - 96
				}
			}
		}
	}
	acc
}
