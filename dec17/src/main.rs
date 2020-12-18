use std::fs;
use std::collections::HashSet;

fn draw(state: &HashSet<(isize, isize, isize)>) {
	for z in -2..2 {
		println!("z={}", z);
		for y in -2..5 {
			for x in -2..5 {
				if state.contains(&(x, y, z)) {
					print!("{}", '#');
				} else {
					print!("{}", '.');
				}
			}
			println!();
		}
	}
}

fn step(state: &HashSet<(isize, isize, isize)>) -> HashSet<(isize, isize, isize)> {
	let mut has_neighbors = HashSet::new();
	for (x, y, z) in state {
		for nx in x-1..x+2 {
			for ny in y-1..y+2 {
				for nz in z-1..z+2 {
					has_neighbors.insert((nx, ny, nz));
				}
			}
		}
	}
	let mut new_state = state.clone();
	for (x, y, z) in has_neighbors {
		let mut count = 0;
		for nx in x-1..x+2 {
			for ny in y-1..y+2 {
				for nz in z-1..z+2 {
					if (x, y, z) != (nx, ny, nz) && state.contains(&(nx, ny, nz)) {
						count = count + 1;
					}
				}
			}
		}
		if state.contains(&(x, y, z)) {
			if count < 2 || count > 3 {
				new_state.remove(&(x, y, z));
			}
		} else {
			if count == 3 {
				new_state.insert((x, y, z));
			}
		}
	}
	//println!("{:?}", new_state.len());
	new_state
}

fn step4(state: &HashSet<(isize, isize, isize, isize)>) -> HashSet<(isize, isize, isize, isize)> {
	let mut has_neighbors = HashSet::new();
	for (x, y, z, w) in state {
		for nx in x-1..x+2 {
			for ny in y-1..y+2 {
				for nz in z-1..z+2 {
					for nw in w-1..w+2 {
						has_neighbors.insert((nx, ny, nz, nw));
					}
				}
			}
		}
	}
	let mut new_state = state.clone();
	for (x, y, z, w) in has_neighbors {
		let mut count = 0;
		for nx in x-1..x+2 {
			for ny in y-1..y+2 {
				for nz in z-1..z+2 {
					for nw in w-1..w+2 {
						if (x, y, z, w) != (nx, ny, nz, nw) && state.contains(&(nx, ny, nz, nw)) {
							count = count + 1;
						}
					}
				}
			}
		}
		if state.contains(&(x, y, z, w)) {
			if count < 2 || count > 3 {
				new_state.remove(&(x, y, z, w));
			}
		} else {
			if count == 3 {
				new_state.insert((x, y, z, w));
			}
		}
	}
	//println!("{:?}", new_state.len());
	new_state
}

fn part1(state: &HashSet<(isize, isize, isize)>) -> usize {
	//draw(&state);
	let mut current_state = state.clone();
	for cycle in 0..6 {
		current_state = step(&current_state);
		//println!("Cycle {}", cycle);
		//draw(&current_state);
	}
	current_state.len()
}

fn part2(state: &HashSet<(isize, isize, isize, isize)>) -> usize {
	//draw(&state);
	let mut current_state = state.clone();
	for cycle in 0..6 {
		current_state = step4(&current_state);
		//println!("Cycle {}", cycle);
		//draw(&current_state);
	}
	current_state.len()
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let size = 8;

    let mut state = HashSet::new();
    let mut state4 = HashSet::new();

    for (y, line) in contents.split("\n").enumerate() {
    	for (x, c) in line.chars().enumerate() {
    		if c == '#' {
    			state.insert((x as isize, y as isize, 0));
    			state4.insert((x as isize, y as isize, 0, 0));
    		}
    	}
    }

    println!("{}", part1(&state));
    println!("{}", part2(&state4));
}
