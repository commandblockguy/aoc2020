use std::fs;
use std::collections::HashSet;
use itertools::Itertools;
use std::i64;
use std::panic;

fn main() {
	let contents = fs::read_to_string("../input").expect("Couldn't read file");

	let mut reference = HashSet::new();
	reference.insert("byr");
	reference.insert("iyr");
	reference.insert("eyr");
	reference.insert("hgt");
	reference.insert("hcl");
	reference.insert("ecl");
	reference.insert("pid");

	let mut count = 0;

	'outer: for passport in contents.split("\n\n") {
		let sections = passport.split_whitespace();
		let mut fields = HashSet::new();
		for section in sections {
			let (field, value) = section.split(":").next_tuple().unwrap();
			if !match field {
				"byr" => {
					let num:i32 = value.parse().unwrap();
					num >= 1920 && num <= 2002
				},
				"iyr" => {
					let num:i32 = value.parse().unwrap();
					num >= 2010 && num <= 2020
				},
				"eyr" => {
					let num:i32 = value.parse().unwrap();
					num >= 2020 && num <= 2030
				},
				"hgt" => {
					let units = &value[value.len() - 2..];
					let maybe_val: Result<i32, _> = value[..value.len() - 2].parse();
					match maybe_val {
						Err(_) => false,
						Ok(val) => {
							let num = val;
							match units {
								"cm" => num >= 150 && num <= 193,
								"in" => num >= 59 && num <= 76,
								_ => false
							}
						}
					}
				},
				"hcl" => {
					if value.chars().nth(0).unwrap() == '#' {
						!panic::catch_unwind(|| {
							i64::from_str_radix(&value[1..], 16)
						}).is_err()
					} else { false }
				},
				"ecl" => {
					match value {
						"amb" | "brn" | "blu" | "gry" | "grn" | "hzl" | "oth" => true,
						_ => false
					}
				},
				"pid" => {
					if value.len() != 9 {
						false
					} else {
						let maybe_val: Result<i32, _> = value.parse();
						!maybe_val.is_err()
					}
				}
				_ => true
			} {
				continue 'outer;
			}

			fields.insert(field);
		}
		if fields.is_superset(&reference) {
			count = count + 1;
		}
	}

	println!("Count: {}", count);
}
