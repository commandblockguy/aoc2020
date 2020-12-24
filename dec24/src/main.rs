use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
enum Dir {
	East,
	SouthEast,
	SouthWest,
	West,
	NorthWest,
	NorthEast
}

impl Dir {
	fn offset(&self) -> (isize, isize) {
		match self {
			Dir::East => (1, 0),
			Dir::SouthEast => (0, 1),
			Dir::SouthWest => (-1, 1),
			Dir::West => (-1, 0),
			Dir::NorthWest => (0, -1),
			Dir::NorthEast => (1, -1),
		}
	}
}

const OFFSETS: [(isize, isize); 6] = [
	(1, 0),
	(0, 1),
	(-1, 1),
	(-1, 0),
	(0, -1),
	(1, -1),
];

fn get_coords(dirs: &Vec<Dir>) -> (isize, isize) {
	let mut current_x = 0;
	let mut current_y = 0;
	for dir in dirs {
		let (offset_x, offset_y) = dir.offset();
		current_x += offset_x;
		current_y += offset_y;
	}
	(current_x, current_y)
}

fn get_black_tiles(tiles: &Vec<Vec<Dir>>) -> HashSet<(isize, isize)> {
	let mut black_tiles = HashSet::new();
	for tile in tiles {
		let pos = get_coords(tile);
		if black_tiles.contains(&pos) {
			black_tiles.remove(&pos);
		} else {
			black_tiles.insert(pos);
		}
	}
	black_tiles
}

fn get_neighbors(pos: &(isize, isize)) -> HashSet<(isize, isize)> {
	let mut result = HashSet::new();
	let (base_x, base_y) = pos;
	for offset in &OFFSETS {
		let (offset_x, offset_y) = offset;
		result.insert((base_x + offset_x, base_y + offset_y));
	}
	result
}

fn part1(tiles: &Vec<Vec<Dir>>) -> usize {
	get_black_tiles(tiles).len()
}

fn part2(tiles: &Vec<Vec<Dir>>) -> usize {
	let mut black_tiles = get_black_tiles(tiles);
	for day in 0..100 {
		let mut all_neighbors = HashSet::new();
		for pos in &black_tiles {
			for nbr in get_neighbors(pos) {
				all_neighbors.insert(nbr);
			}
		}
		let mut next_black_tiles = HashSet::new();
		for pos in all_neighbors {
			let (x, y) = pos;
			let mut count = 0;
			for (x_offset, y_offset) in &OFFSETS {
				if black_tiles.contains(&(x + x_offset, y + y_offset)) {
					count += 1;
				}
			}
			if black_tiles.contains(&pos) {
				if count == 1 || count == 2 {
					next_black_tiles.insert(pos);
				}
			} else {
				if count == 2 {
					next_black_tiles.insert(pos);
				}
			}
		}
		black_tiles = next_black_tiles;
	}
	black_tiles.len()
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut tiles = Vec::new();

    for line in contents.split("\n") {
    	let mut tile = Vec::new();
    	let mut prev = Option::None;
    	for c in line.chars() {
    		match prev {
    			Option::None => {
    				match c {
    					'e' => tile.push(Dir::East),
    					'w' => tile.push(Dir::West),
    					'n' => prev = Option::Some('n'),
    					's' => prev = Option::Some('s'),
    					_ => panic!("invalid char")
    				}
    			},
    			Option::Some('n') => {
    				tile.push(match c {
						'e' => Dir::NorthEast,
						'w' => Dir::NorthWest,
						_ => panic!("invalid 2nd char")
					});
					prev = Option::None;
    			},
    			Option::Some('s') => {
    				tile.push(match c {
						'e' => Dir::SouthEast,
						'w' => Dir::SouthWest,
						_ => panic!("invalid 2nd char")
					});
					prev = Option::None;
    			},
    			_ => panic!("invalid prev char")
    		}
    	}
    	tiles.push(tile);
    }

    println!("{}", part1(&tiles));
    println!("{}", part2(&tiles));
}
