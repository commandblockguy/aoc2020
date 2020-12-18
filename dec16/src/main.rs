use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn part1(ranges: &Vec<(usize, usize, usize, usize)>, other_tickets: &Vec<Vec<usize>>) -> (usize, Vec<Vec<usize>>) {
	let mut error_rate = 0;
	let mut valid_tickets = Vec::new();
	for ticket in other_tickets {
		let mut ticket_valid = true;
		for value in ticket {
			let mut valid = false;
			for range in ranges {
				let (min1, max1, min2, max2) = range;
				if (value >= min1 && value <= max1) || (value >= min2 && value <= max2) {
					valid = true;
					break;
					//println!(" {} {:?}", value, range);
				} else {
					//println!("!{} {:?}", value, range);
				}
			}
			if !valid {
				//println!("{}", value);
				error_rate = error_rate + value;
				ticket_valid = false;
			}
		}
		if ticket_valid {
			valid_tickets.push(ticket.clone());
		}
	}
	(error_rate, valid_tickets)
}

fn part2(ranges: &Vec<(usize, usize, usize, usize)>, my_ticket: &Vec<usize>, valid_tickets: &Vec<Vec<usize>>) -> usize {
	// maps index of field in input to a bitmap of possible locations on ticket
	let mut possible_fields: HashMap<usize, usize> = HashMap::new();
	for x in 0..valid_tickets[0].len() {
		possible_fields.insert(x, 0b11111111111111111111);
	}

	for ticket in valid_tickets {
		for (value_i, value) in ticket.iter().enumerate() {
			for (range_i, (min1, max1, min2, max2)) in ranges.iter().enumerate() {
				if !(value >= min1 && value <= max1) && !(value >= min2 && value <= max2) {
					possible_fields.insert(range_i, possible_fields.get(&range_i).unwrap() & !(1 << value_i));
				}
			}
		}
	}

	let mut known_fields = HashMap::new();
	while possible_fields.len() > 0 {
		let mut found_field = Option::None;
		for (field, loc) in &possible_fields {
			if loc.count_ones() == 1 {
				found_field = Option::Some(*field);
				break;
			}
		}
		match found_field {
			Some(field) => {
				let loc = *possible_fields.get(&field).unwrap();
				known_fields.insert(field, loc.trailing_zeros());
				possible_fields.remove(&field);
				for (_, other_loc) in possible_fields.iter_mut() {
					if (*other_loc & loc) != 0 {
						*other_loc = *other_loc & !loc;
					}
				}
			},
			None => {}
		}
	}

	//println!("{:?}", known_fields);

	let mut prod = 1;
	for x in 0..6 {
		prod = prod * my_ticket[*known_fields.get(&x).unwrap() as usize];
	}

	prod
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let num_fields = 20;

    let mut lines = contents.split("\n");

    let mut ranges = Vec::new();
    let mut my_ticket = Vec::new();
    let mut other_tickets = Vec::new();

    let ranges_re = Regex::new(r".*: (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    for line in (&mut lines).take(num_fields) {
    	let capture = ranges_re.captures(line).unwrap();
    	ranges.push((capture[1].parse().unwrap(), capture[2].parse().unwrap(), capture[3].parse().unwrap(), capture[4].parse().unwrap()));
    }

    for num in (&mut lines).skip(2).next().unwrap().split(",") {
    	my_ticket.push(num.parse().unwrap());
    }

    for line in (&mut lines).skip(2) {
    	let mut ticket = Vec::new();
    	for num in line.split(",") {
    		ticket.push(num.parse().unwrap());
    	}
    	other_tickets.push(ticket);
    }

    let (p1_score, valid) = part1(&ranges, &other_tickets);

    println!("{}", p1_score);
    println!("{}", part2(&ranges, &my_ticket, &valid));
}
