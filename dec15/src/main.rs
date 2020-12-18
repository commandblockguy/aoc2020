use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn part1(nums: &Vec<u32>) -> u32 {
	let mut spoken = nums.clone();
	let mut seen = HashSet::new();
	for x in &spoken {
		seen.insert(*x);
	}
	while spoken.len() < 2020 {
		let last = spoken[spoken.len() - 1];
		if seen.contains(&last) {
			let pos = match (&spoken).into_iter().rev().skip(1).position(|x| *x == last) {
				None => 0,
				Some(x) => x as u32 + 1
			};
			seen.insert(pos);
			spoken.push(pos);
		} else {
			spoken.push(0);
		}
		//println!("{:?}", spoken[spoken.len() - 1]);
	}
	spoken[2020 - 1]
}

fn part2(nums: &Vec<u32>) -> u32 {
	let mut seen: HashMap<u32, usize> = HashMap::new();
	let mut current = 0;
	for (i, x) in nums.into_iter().enumerate() {
		println!("{}: {:?}", i + 1, *x);
		seen.insert(*x, i);
	}
	for num in nums.len()..30000000-1 {
		let next = match seen.get(&current) {
			Some(x) => num - x,
			None => 0
		};
		seen.insert(current as u32, num);
		//println!("{}: {:?}", num + 1, current);
		current = next as u32;
	}
	current
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut nums = Vec::new();

    for line in contents.split(",") {
    	nums.push(line.parse().unwrap());
    }

    println!("{}", part1(&nums));
    println!("{}", part2(&nums));
}
