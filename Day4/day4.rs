use std::fs;
use std::ops::{Deref, Add, AddAssign};
use std::borrow::Borrow;
use std::collections::{HashMap};
use std::panic::resume_unwind;
use regex::Regex;

pub(crate) fn run() {
	let contents = fs::read_to_string("/home/itorius/Development/AoC/Day4/input.txt").expect("Something went wrong reading the file");
	let split: Vec<&str> = contents.lines().collect();

	let mut records: Vec<String> = Vec::new();
	let mut temp: String = String::from("");
	for line in split {
		if line != "" {
			temp.add_assign(line);
			temp.add_assign(" ")
		} else {
			records.push(temp.to_owned());
			temp.clear()
		}
	}
	records.push(temp.to_owned());

	let re = Regex::new("#[0-9a-f]{6}").unwrap();
	let re1 = Regex::new("[0-9]{9}").unwrap();
	let re2 = Regex::new("[0-9]+(in|cm)").unwrap();
	let validEyeColors: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

	let mut sum = 0;
	for record in records {
		let r = parse(record.borrow()).expect("cool");

		if !r.contains_key("byr") ||
			!r.contains_key("iyr") ||
			!r.contains_key("eyr") ||
			!r.contains_key("hgt") ||
			!r.contains_key("hcl") ||
			!r.contains_key("ecl") ||
			!r.contains_key("pid")
		{
			continue;
		}

		if r["byr"].len() != 4 {
			continue;
		}

		let p = r["byr"].parse::<i32>().expect("ee");
		if p < 1920 || p > 2002 {
			continue;
		}

		if r["iyr"].len() != 4 {
			continue;
		}

		let p = r["iyr"].parse::<i32>().expect("ee");
		if p < 2010 || p > 2020 {
			continue;
		}

		if r["eyr"].len() != 4 {
			continue;
		}

		let p = r["eyr"].parse::<i32>().expect("ee");
		if p < 2020 || p > 2030 {
			continue;
		}

		if !re2.is_match(r["hgt"]) {
			continue;
		}

		if r["hgt"].ends_with("cm") {
			let num = r["hgt"].strip_suffix("cm").expect("e").parse::<i32>().expect("ee");
			if num < 150 || num > 193 {
				continue;
			}
		} else if r["hgt"].ends_with("in") {
			let num = r["hgt"].strip_suffix("in").expect("e").parse::<i32>().expect("ere");
			if num < 59 || num > 76 {
				continue;
			}
		}

		if !re.is_match(r["hcl"]) {
			continue;
		}



		if !validEyeColors.iter().any(|x| *x == r["ecl"].trim()) {

			continue;
		}

		if r["pid"].len()!= 9{
			println!("{0}", r["pid"]);
			continue;
		}

		if !re1.is_match(r["pid"]) {
			continue;
		}

		sum += 1;
	}

	println!("{0}", sum);
}

fn parse(input: &String) -> Result<HashMap<&str, &str>, String> {
	let mut result = HashMap::new();

	for s in input.split(' ').filter(|s| !s.is_empty()) {
		let keyvalue: Vec<&str> = s.split(':').collect();
		result.insert(keyvalue[0], keyvalue[1]);
	}

	Ok(result)
}