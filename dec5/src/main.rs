use std::fs;
use std::collections::HashSet;

fn main() {
	let contents = fs::read_to_string("input").expect("Couldn't read file");

	let mut max = 0;
	let mut min = 1024;

	let mut set = HashSet::new();

	for line in contents.split("\n") {
		let binary = line.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1");
		if binary.is_empty() { continue }

		let num = i32::from_str_radix(&binary, 2).unwrap();

		set.insert(num);

		if num > max {
			max = num;
		}

		if num < min {
			min = num;
		}
	}

	println!("{}", max);

	for x in min..max {
		if !set.contains(&x) {
			println!("{}", x);
		}
	}
}
