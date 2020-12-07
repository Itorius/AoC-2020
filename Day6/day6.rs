use std::fs;
use std::ops::{Deref, AddAssign};
use std::borrow::Borrow;
use std::cmp::max;

pub(crate) fn run() {
	let contents = fs::read_to_string("/home/itorius/Development/AoC/Day6/input.txt").expect("Something went wrong reading the file");
	let split: Vec<&str> = contents.lines().collect();

	let mut records: Vec<Vec<&str>> = Vec::new();
	let mut temp: Vec<&str> = vec![];
	for line in split {
		if line != "" {
			temp.push(line);
		} else {
			records.push(temp.to_owned());
			temp.clear()
		}
	}
	records.push(temp.to_owned());

	let mut sum =0;
	for record in records {
		let s = *record.first().unwrap();
		let mut count = 0;

		for c in s.chars() {
			let ca = record[1..].iter().all(|x| x.contains(c));
			if ca {
				count += 1;
			}
		}

		sum += count;
	}

	println!("{0}", sum);
}