use std::fs;
use std::ops::Deref;

pub(crate) fn run() {
	let contents = fs::read_to_string("/home/itorius/Development/AoC/Day2/input.txt").expect("Something went wrong reading the file");
	let split = contents.lines();

	let mut sum = 0;
	for line in split {
		let splitLine: Vec<&str> = line.split(|c| c == '-' || c == ' ' || c == ':').filter(|s| !s.is_empty()).collect();

		let char = splitLine[2].chars().next().unwrap();
		let min = splitLine[0].parse::<usize>().unwrap() - 1;
		let max = splitLine[1].parse::<usize>().unwrap() - 1;

		if splitLine[3].chars().nth(min).unwrap() == char && splitLine[3].chars().nth(max).unwrap() != char || splitLine[3].chars().nth(min).unwrap() != char && splitLine[3].chars().nth(max).unwrap() == char {
			sum += 1;
		}
	}

	println!("{0}", sum);
}
