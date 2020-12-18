use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;


fn get_combos(color: String, map: &HashMap<String, HashSet<String>>, depth: i32) -> HashSet<String> {
	let mut set = HashSet::new();
	set.insert(color.clone());
	match map.get(&color) {
		None => (),
		Some(contents) => {
			for content_color in contents.iter() {
				for result_color in get_combos(content_color.to_string(), map, depth + 1) {
					set.insert(result_color);
				}
			}
		}
	}
	set
}

fn count_combos(color: String, map: &HashMap<String, HashSet<(String, usize)>>, depth: i32) -> usize {
	match map.get(&color) {
		None => 1,
		Some(contents) => {
			let mut sum = 0;
			for (content_color, count) in contents.iter() {
				sum = sum + count * count_combos(content_color.to_string(), map, depth + 1);
			}
			sum + 1
		}
	}
}

fn part_1(contents: &String) -> usize {
	let re_container = Regex::new(r"^(.*) bags? contain").unwrap();
    let re_content = Regex::new(r"(\d+) ([^,.]*) bags?").unwrap();

	let mut map: HashMap<String, HashSet<String>> = HashMap::new();

	for line in contents.split("\n") {
		let container = &re_container.captures(line).unwrap()[1];
		for content in re_content.captures_iter(line) {
			let string = content[2].to_string();
			if !map.contains_key(&string) {
				map.insert(string.clone(), HashSet::new());
			}
			map.get_mut(&string).unwrap().insert(container.to_string());
		}
	}

	println!("{:?}", map);

	let result = get_combos("shiny gold".to_string(), &map, 0);
	result.len() - 1
}

fn part_2(contents: &String) -> usize {
	let re_container = Regex::new(r"^(.*) bags? contain").unwrap();
    let re_content = Regex::new(r"(\d+) ([^,.]*) bags?").unwrap();

	let mut map: HashMap<String, HashSet<(String, usize)>> = HashMap::new();

	for line in contents.split("\n") {
		let container = &re_container.captures(line).unwrap()[1];
		for content in re_content.captures_iter(line) {
			let count:usize = content[1].parse().unwrap();
			let string = content[2].to_string();
			if !map.contains_key(&container.to_string()) {
				map.insert(container.to_string(), HashSet::new());
			}
			map.get_mut(&container.to_string()).unwrap().insert((string, count));
		}
	}

	count_combos("shiny gold".to_string(), &map, 0) - 1
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    println!("{} {}", part_1(&contents), part_2(&contents));
}
