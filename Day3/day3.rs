use std::fs;
use std::ops::Deref;
use std::borrow::Borrow;

pub(crate) fn run() {
	let contents = fs::read_to_string("/home/itorius/Development/AoC/Day3/input.txt").expect("Something went wrong reading the file");
	let split: Vec<&str> = contents.lines().collect();

	let width = split[0].chars().count();
	let height = split.len();

	let mut array: Vec<Vec<char>> = Vec::new();

	for (i, str) in split.iter().enumerate() {
		let line: Vec<char> = (*str).chars().collect();
		array.push(line);
	}

	let temp1 = GetTreesForSlope(array.borrow(), 1, 1);
	let temp2 = GetTreesForSlope(array.borrow(), 3, 1);
	let temp3 = GetTreesForSlope(array.borrow(), 5, 1);
	let temp4 = GetTreesForSlope(array.borrow(), 7, 1);
	let temp5 = GetTreesForSlope(array.borrow(), 1, 2);
	let sum: u64 = temp1 * temp2 * temp3 * temp4 * temp5;
	println!("{0}", sum)
}

fn GetTreesForSlope(array: &Vec<Vec<char>>, traverseX: usize, traverseY: usize) -> u64 {
	let mut posX = 0;
	let mut posY = 0;

	let height = array.len();
	let width = array[0].len();

	let mut sum = 0;

	while posY < height {
		if array[posY][posX % width] == '#' {
			sum += 1;
		}

		posX += traverseX;
		posY += traverseY;
	}

	sum
}