use std::fs;
use itertools::Itertools;

fn transform(subject: usize, privkey: usize) -> usize {
	let subject = subject as u64;
	let privkey = privkey as u64;
	let mut current = 1;
	for _ in 0..privkey {
		current *= subject;
		current = current % 20201227;
	}
	current as usize
}

fn get_loop_count(subject: usize, pubkey: usize) -> usize {
	let pubkey = pubkey as u64;
	let subject = subject as u64;
	let mut current = 1;
	let mut loops = 0;
	while current != pubkey {
		current *= subject;
		current = current % 20201227;
		loops += 1;
	}
	loops
}

fn part1(card: usize, door: usize) -> usize {
	let subject = 7;
	let door_priv = get_loop_count(subject, door);
	let key1 = transform(card, door_priv);
	key1
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let (card, door) = contents.split("\n").next_tuple().unwrap();
    let card = card.parse().unwrap();
    let door = door.parse().unwrap();

    println!("{}", part1(card, door));
}
