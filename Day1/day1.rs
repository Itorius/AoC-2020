use std::fs;

pub(crate) fn run(){
	let contents = fs::read_to_string("/home/itorius/Development/AoC/Day1/input1.txt").expect("Something went wrong reading the file");
	let split: Vec<i32> = contents.lines().map(|letter| { letter.parse::<i32>().unwrap() }).collect();
	let count = split.len();

	for i in 0..count {
		for j in i..count {
			for k in j..count {
				if split[i] + split[j]+ split[k] == 2020 {
					println!("{0}, {1}, {2}", split[i], split[j], split[k]);
					println!("{0}", split[i] * split[j] * split[k]);
				}
			}
		}
	}
}