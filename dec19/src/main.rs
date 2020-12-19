use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn substitute(rules: &HashMap<usize, &str>, n: usize, round2: bool) -> String {
	match n {
		8 if round2 => {
			let mut string = substitute(rules, 42, round2).to_owned();
			string.push_str("+");
			string
		},
		11 if round2 => {
			let left = substitute(rules, 42, round2);
			let right = substitute(rules, 31, round2);
			let mut string = "(".to_owned();
			let max_count = 8; // I am have lazy
			for x in 1..=max_count {
				string.push_str(&left);
				string.push_str("{");
				string.push_str(&x.to_string());
				string.push_str("}");
				string.push_str(&right);
				string.push_str("{");
				string.push_str(&x.to_string());
				string.push_str("}");
				if x != max_count {
					string.push_str("|");
				}
			}
			string.push_str(")");
			string
		},
		x => {
			let mut string = "(".to_owned();
			for x in rules.get(&n).unwrap().split(" ") {
				string.push_str(&match x {
					"|" => "|".to_string(),
					x if x.chars().nth(0).unwrap() == '"' => x[1..2].to_string(),
					x => {
						let num = x.parse().unwrap();
						substitute(rules, num, round2)
					}
				});
			}
			string.push_str(")");
			string
		}
	}
}

fn part1(rules: &HashMap<usize, &str>, messages: &Vec<&str>) -> usize {
	// println!("{:?} {:?}", rules, messages);
	let mut re_str = "^".to_owned();
	re_str.push_str(&substitute(rules, 0, false));
	re_str.push_str("$");
	// println!("{:?}", re_str);
	let re = Regex::new(&re_str).unwrap();
	let mut count = 0;
	for x in messages {
		if re.is_match(x) {
			//println!("{:?}", x);
			count += 1;
		}
	}
	count
}

fn part2(rules: &HashMap<usize, &str>, messages: &Vec<&str>) -> usize {
	// println!("{:?} {:?}", rules, messages);
	let mut re_str = "^".to_owned();
	re_str.push_str(&substitute(rules, 0, true));
	re_str.push_str("$");
	// println!("{:?}", re_str);
	let re = Regex::new(&re_str).unwrap();
	let mut count = 0;
	for x in messages {
		if re.is_match(x) {
			//println!("{:?}", x);
			count += 1;
		}
	}
	count
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut split = contents.split("\n\n");
    let (rules_str, messages_str) = (split.next().unwrap(), split.next().unwrap());

    let mut rules = HashMap::new();
    for line in rules_str.split("\n") {
    	let num = line.split(":").nth(0).unwrap().parse().unwrap();
    	rules.insert(num, &line[line.find(" ").unwrap()+1..]);
    }

    let mut messages = Vec::new();
    for line in messages_str.split("\n") {
    	messages.push(line);
    }

    println!("{}", part1(&rules, &messages));
    println!("{}", part2(&rules, &messages));
}
