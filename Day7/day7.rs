use std::fs;
use std::ops::{Deref, AddAssign};
use std::borrow::Borrow;
use std::cmp::max;
use std::collections::HashMap;

pub(crate) fn run() {
	let contents = fs::read_to_string("/home/itorius/Development/AoC/Day7/input.txt").expect("Something went wrong reading the file");
	let split: Vec<&str> = contents.lines().collect();
	// let mut bags: HashMap<&str, Vec<(&str, i32)>>= HashMap::new();
	let mut bags: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();

	// posh tan bags contain 3 bright yellow bags

	for line in split {
		let mut s: Vec<&str> = line.split(" bags contain ").collect();
		let key = s[0];

		let mut p = s[1];
		p = p.trim_end_matches(".");

		let mut vv = vec![];

		for mut s in p.split(", ") {
			if s == "no other bags" {
				bags.insert(key, vec![]);
				continue;
			}

			s = s.trim_end_matches(" bags");
			s = s.trim_end_matches(" bag");

			vv.push((&s[2..], (&s[0..1]).parse::<i32>().unwrap()));

			// println!("{0}", s);
		}
		// println!("{0}", "---");

		bags.insert(key, vv);
	}

	let mut sum = 0;
	for (key, value) in bags.iter() {
		// println!("{0}:{1}", key, value.iter().map(|x| {
		// 	let mut s=String::from((*x).0);
		// 	s.push_str(format!(" x{0}", (*x).1).as_str());
		// 	s
		// }).fold(String::from("").to_owned(), |mut x, y| {
		// 	x.push_str(y.as_str());
		// 	x
		// }));

		if *key == "shiny gold" {
			// println!("{0}", key);
			sum += scan(bags.clone(), key, 0);
		}

		// let p = bags.clone();
		// println!("{0}", key);
		// if scan(p, key, 0) {
		// 	sum += 1;
		// }
	}
	println!("{0}", sum);

	// let mut records: Vec<Vec<&str>> = Vec::new();
	// let mut temp: Vec<&str> = vec![];
	// for line in split {
	// 	if line != "" {
	// 		temp.push(line);
	// 	} else {
	// 		records.push(temp.to_owned());
	// 		temp.clear()
	// 	}
	// }
	// records.push(temp.to_owned());
	//
	// let mut sum =0;
	// for record in records {
	// 	let s = *record.first().unwrap();
	// 	let mut count = 0;
	//
	// 	for c in s.chars() {
	// 		let ca = record[1..].iter().all(|x| x.contains(c));
	// 		if ca {
	// 			count += 1;
	// 		}
	// 	}
	//
	// 	sum += count;
	// }
	//
	// println!("{0}", sum);
}

fn scan(bags: HashMap<&str, Vec<(&str, i32)>>, bag: &str, level: i32) -> i32 {
	let mut flag = false;

	if bags[bag].is_empty() {
		return 0;
	}
	let mut count: i32 = 0;
	for v in &bags[bag] {
		// if v.0 == "shiny gold" {
		// 	flag = true;
		// 	// println!("{0}", key);
		// } else {
		count+=v.1;
		for i in 0..v.1{
			count += scan(bags.clone(), v.0, level + 1);
		}
		// if flag{
		// 	return true
		// }
		// }
	}
	count
}