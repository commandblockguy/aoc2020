use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

enum Inst {
	Mask(u64, u64, u64),
	Set(u64, u64)
}

fn part1(lines: &Vec<Inst>) -> u64 {
	let mut memory: HashMap<u64, u64> = HashMap::new();
	let mut mask_and = 0;
	let mut mask_or = 0;
	for line in lines {
		match line {
			Inst::Mask(and, or, _) => {
				mask_and = *and;
				mask_or = *or;
			}
			Inst::Set(addr, val) => {
				if !memory.contains_key(addr) {
					memory.insert(*addr, 0);
				}
				memory.insert(*addr, (val | mask_or) & mask_and);
			}
		}
	}
	let mut sum = 0;
	for (_, val) in memory {
		sum += val;
	}
	sum
}

fn get_addresses(floating: u64) -> HashSet<u64> {
	let mut result = HashSet::new();
	match floating {
		0 => {
			result.insert(0);
		},
		_ => {
			let addresses = get_addresses(floating >> 1);
			for address in addresses {
				result.insert(address << 1);
				if floating & 1 == 1 {
					result.insert(address << 1 | 1);
				}
			}
		}
	}
	result
}

fn part2(lines: &Vec<Inst>) -> u64 {
	let mut memory: HashMap<u64, u64> = HashMap::new();
	let mut mask_or = 0;
	let mut mask_floating = 0;
	for line in lines {
		match line {
			Inst::Mask(_, or, floating) => {
				mask_or = *or;
				mask_floating = *floating;
			}
			Inst::Set(addr, val) => {
				for mask in get_addresses(mask_floating) {
					let new_addr = (addr & !mask_floating) | mask | mask_or;
					if !memory.contains_key(&new_addr) {
						memory.insert(new_addr, 0);
					}
					memory.insert(new_addr, *val);
				}
			}
		}
	}
	let mut sum = 0;
	for (_, val) in memory {
		sum += val;
	}
	sum
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut lines = Vec::new();

    for line in contents.split("\n") {
    	lines.push(match &line[..4] {
    		"mask" => {
    			let val = line.split(" ").nth(2).unwrap();
    			let mask_and = u64::from_str_radix(&val.replace("X", "1"), 2).unwrap();
    			let mask_or = u64::from_str_radix(&val.replace("X", "0"), 2).unwrap();
    			let mask_floating = u64::from_str_radix(&val.replace("1", "0").replace("X", "1"), 2).unwrap();
    			Inst::Mask(mask_and, mask_or, mask_floating)
    		},
    		"mem[" => {
    			let addr = line.split("]").next().unwrap().split("[").nth(1).unwrap().parse().unwrap();
    			let val = line.split(" ").nth(2).unwrap().parse().unwrap();
    			Inst::Set(addr, val)
    		},
    		_ => {
    			println!("invalid line");
    			Inst::Mask(0, 0, 0)
    		}
    	});
    }

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
