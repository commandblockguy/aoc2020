use std::fs;
use std::collections::HashSet;

fn get_set(line: &str) -> HashSet<char>  {
	let mut set = HashSet::new();
	for c in line.chars() {
		set.insert(c);
	}
	set
}

fn main() {
	let contents = fs::read_to_string("input").expect("Couldn't read file");

	let mut count1 = 0;

	for group in contents.split("\n\n") {
		let mut split = group.split("\n");
		let mut a: HashSet<char> = get_set(split.next().unwrap());
		for person in split {
			let set = get_set(person);
			let intersection: HashSet<_> = set.intersection(&a).collect();
			let mut new = HashSet::new();
			for x in intersection {
				new.insert(*x);
			}
			a = new;
		}
		println!("{}", a.len());
		count1 = count1 + a.len();
	}

	println!("{}", count1);
}
