use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn part1(mut decks: Vec<VecDeque<usize>>) -> usize {
	while !(&decks).iter().any(|x| x.len() == 0) {
		let mut cards = Vec::new();
		for deck in &mut decks {
			cards.push(deck.pop_front().unwrap());
		}
		let (max_index, _) = cards.iter().enumerate().max_by_key(|&(_, x)| x).unwrap();
		cards.sort();
		cards.reverse();
		for card in cards {
			decks[max_index].push_back(card);
		}
	}
	let winning_deck = decks.iter().max_by_key(|x| x.len()).unwrap();
	let mut result = 0;
	for (i, card) in winning_deck.iter().enumerate() {
		result += card * (winning_deck.len() - i);
	}
	result
}

fn play_recursive(mut decks: Vec<VecDeque<usize>>) -> (usize, VecDeque<usize>) {
	let mut previous_combos = HashSet::new();
	while !(&decks).iter().any(|x| x.len() == 0) {
		if previous_combos.contains(&decks) {
			return (0, decks[0].clone());
		}
		previous_combos.insert(decks.clone());
		let mut cards = Vec::new();
		for deck in &mut decks {
			cards.push(deck.pop_front().unwrap());
		}
		let winner = if cards.iter().zip(decks.iter()).all(|(&card, deck)| deck.len() >= card) {
			let mut new_decks = Vec::new();
			for (deck, &card) in decks.iter().zip(cards.iter()) {
				new_decks.push(deck.iter().take(card).copied().collect());
			}
			let (winner, _) = play_recursive(new_decks);
			winner
		} else {
			let (max_index, _) = cards.iter().enumerate().max_by_key(|&(_, x)| x).unwrap();
			max_index
		};
		decks[winner].push_back(cards[winner]);
		decks[winner].push_back(cards[1 - winner]);
	}
	let (winner, winning_deck) = decks.iter().enumerate().max_by_key(|(_, x)| x.len()).unwrap();
	(winner, winning_deck.clone())
}

fn part2(decks: Vec<VecDeque<usize>>) -> usize {
	let (winner, winning_deck) = play_recursive(decks);
	let mut result = 0;
	for (i, card) in winning_deck.iter().enumerate() {
		result += card * (winning_deck.len() - i);
	}
	result
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut decks = Vec::new();

    for player in contents.split("\n\n") {
    	let mut deck = VecDeque::new();
    	for line in player.split("\n").skip(1) {
    		deck.push_back(line.parse().unwrap());
    	}
    	decks.push(deck);
    }

    println!("{}", part1(decks.clone()));
    println!("{}", part2(decks));
}
