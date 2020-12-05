use std::fs;
use std::ops::Deref;
use std::borrow::Borrow;
use std::cmp::max;

pub(crate) fn run() {
	let contents = fs::read_to_string("/home/itorius/Development/AoC/Day5/input.txt").expect("Something went wrong reading the file");
	let split: Vec<&str> = contents.lines().collect();
	let mut seatIDs: Vec<i32> = vec![];

	let mut maxRow = 0;

	for entry in split {
		let row = &entry[0..7];
		let column = &entry[7..];

		let mut intervalStartC = 0;
		let mut intervalEndC = 7;

		for c in column.chars() {
			if c == 'R' {
				intervalStartC = (intervalStartC + intervalEndC) / 2;
			} else {
				intervalEndC = (intervalStartC + intervalEndC) / 2
			}
		}

		let mut intervalStartR = 0;
		let mut intervalEndR = 127;

		for c in row.chars() {
			if c == 'B' {
				intervalStartR = (intervalStartR + intervalEndR) / 2;
			} else {
				intervalEndR = (intervalStartR + intervalEndR) / 2
			}
		}

		if intervalEndR > maxRow {
			maxRow = intervalEndR;
		}
		seatIDs.push(intervalEndR * 8 + intervalEndC);

		println!("{0}-{3}, {1}-{2}", row, column, intervalEndC, intervalEndR);
	}

	for row in 0..maxRow {
		for column in 0..7 {
			if !seatIDs.contains(&(row * 8 + column)) {
				println!("{0} {1}", row, column)
			}
		}
	}

	// 102 rows

	// println!("{0}", maxRow)
	// println!("{0}", seatIDs.iter().max().expect("00"))
}