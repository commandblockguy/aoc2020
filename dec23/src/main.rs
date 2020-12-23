use std::fs;
use std::collections::VecDeque;

fn part1(mut cups: VecDeque<usize>) -> usize {
	let highest = *cups.iter().max().unwrap();
	for _ in 0..100 {
		let current_cup = cups.pop_front().unwrap();
		let mut picked = Vec::new();
		for _ in 0..3 {
			picked.push(cups.pop_front().unwrap());
		}
		cups.push_back(current_cup);
		let mut dest = current_cup;
		loop {
			dest -= 1;
			if dest < 1 {
				dest = highest;
			}
			if cups.contains(&dest) {
				break;
			}
		}
		let dest_index = cups.iter().position(|&x| x == dest).unwrap();
		for cup in picked.iter().rev() {
			cups.insert(dest_index + 1, *cup);
		}
	}
	let index_1 = cups.iter().position(|&x| x == 1).unwrap();
	let mut total = 0;
	for (i, n) in cups.iter().skip(index_1 + 1).chain(cups.iter().take(index_1)).enumerate() {
		total += n * 10_usize.pow(7 - i as u32);
	}
	total
}

fn part2(cups: VecDeque<usize>) -> u64 {
	const LEN: usize = 1000000;
	let mut next = Vec::new();
	// too lazy to zero-index
	for _ in 0..LEN + 1 {
		next.push(0);
	}
	for (i, &x) in cups.iter().enumerate() {
		if i != cups.len() - 1 {
			next[x] = cups[i + 1];
		} else {
			next[x] = cups.len() + 1;
		}
	}
	let highest = *cups.iter().max().unwrap();
	for x in highest+1..LEN {
		next[x] = x + 1;
	}
	next[LEN] = cups[0];
	let mut current_cup = cups[0];
	for turn in 0..10000000 {
		//println!("{}: {:?}", turn, next.iter().enumerate().skip(1).collect::<Vec<(usize, &usize)>>());
		if turn % 1000000 == 0 {
			println!("{:?}", turn);
		}
		let mut picked = Vec::new();
		let mut current_picked = current_cup;
		for _ in 0..3 {
			current_picked = next[current_picked];
			picked.push(current_picked);
		}
		next[current_cup] = next[current_picked];
		let mut dest = current_cup;
		loop {
			dest -= 1;
			if dest < 1 {
				dest = LEN;
			}
			if !picked.contains(&dest) {
				break;
			}
		}
		let dest_next = next[dest];
		next[dest] = picked[0];
		next[picked[2]] = dest_next;
		current_cup = next[current_cup];
	}
	//println!("{:?}", next.iter().enumerate().skip(1).collect::<Vec<(usize, &usize)>>());
	next[1] as u64 * next[next[1]] as u64
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut cups = VecDeque::new();

    for cup in contents.chars() {
    	cups.push_back(cup.to_string().parse().unwrap());
    }

    println!("{}", part1(cups.clone()));
    println!("{}", part2(cups));
}
