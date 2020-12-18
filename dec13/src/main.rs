use std::fs;

fn part1(time: u128, buses: &Vec<u128>) -> u128 {
	let mut min:u128 = u128::MAX;
	for bus in buses {
		if *bus > 1 {
			if *bus - (time % *bus) < min - (time % min) {
				min = *bus;
			}
		}
	}
	min * (min - (time % min))
}

fn mod_inv(a: i128, module: i128) -> i128 {
  let mut mn = (module, a);
  let mut xy = (0, 1);
 
  while mn.1 != 0 {
    xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
    mn = (mn.1, mn.0 % mn.1);
  }
 
  while xy.0 < 0 {
    xy.0 += module;
  }
  xy.0
}

fn part2(buses: &Vec<u128>) -> u128 {
	let mut sum: u128 = 0;
	let mut prod: u128 = 1;
	for bus in buses {
		prod = prod * bus;
	}
	println!("{}", prod);
	for (index, bus) in buses.iter().enumerate() {
		if *bus != 1 {
			let p = prod / bus;
			println!("{} {}:{} {} {}", sum, index, bus, p, mod_inv(p as i128, *bus as i128));
			sum = sum + (*bus - (index as u128 % bus)) * mod_inv(p as i128, *bus as i128) as u128 * p;
		}
	}
	sum % prod as u128
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut lines = contents.split("\n");

    let time:u128 = lines.next().unwrap().parse().unwrap();

    let mut buses = Vec::new();

    for string in lines.next().unwrap().split(',') {
    	if string != "x" {
    		let num:u128 = string.parse().unwrap();
    		buses.push(num);
    	} else {
    		buses.push(1);
    	}
    }

    println!("{}", part1(time, &buses));
    println!("{}", part2(&buses));
}
