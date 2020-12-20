use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

// So this is a huge mess compared to everything before this, as I tried two solutions while sharing a lot of the code
// As a result like half of this is completely unused.

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Side {
	TOP,
	RIGHT,
	BOTTOM,
	LEFT
}

impl Side {
	fn rotate_c(&self) -> Side {
		match self {
			Side::TOP => Side::RIGHT,
			Side::RIGHT => Side::BOTTOM,
			Side::BOTTOM => Side::LEFT,
			Side::LEFT => Side::TOP
		}
	}

	fn rotate_cc(&self) -> Side {
		match self {
			Side::TOP => Side::LEFT,
			Side::RIGHT => Side::TOP,
			Side::BOTTOM => Side::RIGHT,
			Side::LEFT => Side::BOTTOM
		}
	}

	fn reverse(&self) -> Side {
		match self {
			Side::TOP => Side::BOTTOM,
			Side::RIGHT => Side::LEFT,
			Side::BOTTOM => Side::TOP,
			Side::LEFT => Side::RIGHT
		}
	}

	fn rotate_by(&self, from: &Side, to: &Side) -> Side {
		let mut current = *from;
		let mut result = *self;
		while current != *to {
			current = current.rotate_c();
			result = result.rotate_c();
		}
		result
	}

	fn rotate_by_inv(&self, from: &Side, to: &Side) -> Side {
		let mut current = *from;
		let mut result = *self;
		while current != *to {
			current = current.rotate_c();
			result = result.rotate_cc();
		}
		result
	}

	fn offset(&self) -> (isize, isize) {
		match self {
			Side::TOP => (0, 1),
			Side::RIGHT => (1, 0),
			Side::BOTTOM => (0, -1),
			Side::LEFT => (-1, 0)
		}
	}
}

fn get_sides(tile: &Vec<Vec<bool>>) -> HashMap<(Side, bool), Vec<bool>> {
	let mut result = HashMap::new();
	result.insert((Side::TOP, false), tile[0].clone());
	let mut top_rev = tile[0].clone();
	top_rev.reverse();
	result.insert((Side::TOP, true), top_rev);
	result.insert((Side::BOTTOM, true), tile[tile.len()-1].clone());
	let mut bot_rev = tile[tile.len()-1].clone();
	bot_rev.reverse();
	result.insert((Side::BOTTOM, false), bot_rev);
	let mut left = Vec::new();
	let mut right = Vec::new();
	for row in tile {
		left.push(row[0]);
		right.push(row[row.len()-1]);
	}
	let mut left_rev = left.clone();
	left_rev.reverse();
	let mut right_rev = right.clone();
	right_rev.reverse();
	result.insert((Side::LEFT, true), left);
	result.insert((Side::RIGHT, false), right);
	result.insert((Side::LEFT, false), left_rev);
	result.insert((Side::RIGHT, true), right_rev);
	result
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Neighbor {
	other: usize,
	my_side: Side,
	other_side: Side,
	reversed: bool
}

fn get_neighbors(tiles: &HashMap<usize, Vec<Vec<bool>>>) -> HashMap<usize, HashSet<Neighbor>> {
	let mut sides = HashMap::new();
	for (num, tile) in tiles {
		sides.insert(num, get_sides(tile));
	}
	let mut neighbor = HashMap::new();
	for (&num, _) in &sides {
		neighbor.insert(*num, HashSet::new());
	}
	for (num, my_sides) in &sides {
		for (&other_num, other_sides) in &sides {
			if *num == other_num {
				continue;
			}
			for (&(my_side, my_reversed), my_side_vec) in my_sides {
				for (&(other_side, other_reversed), other_side_vec) in other_sides {
					if my_side_vec == other_side_vec {
						neighbor.get_mut(num).unwrap().insert(Neighbor {
							other: *other_num,
							my_side, other_side,
							reversed: my_reversed ^ other_reversed
						});
					}
				}
			}
		}
	}
	neighbor
}

fn get_corners(tiles: &HashMap<usize, Vec<Vec<bool>>>) -> Vec<usize> {
	let mut neighbors = HashMap::new();
	for (num, nbrs) in get_neighbors(tiles){
		let mut set = HashSet::new();
		for Neighbor{other, ..} in nbrs {
			set.insert(other);
		}
		neighbors.insert(num, set);
	}
	let mut result = Vec::new();
	for (x, set) in neighbors {
		if set.len() == 2 {
			result.push(x);
		}
	}
	result
}

fn part1(tiles: &HashMap<usize, Vec<Vec<bool>>>) -> usize {
	let mut prod = 1;
	for x in get_corners(tiles) {
		println!("{:?}", x);
		prod *= x;
	}
	prod
}

// #[derive(Debug, Clone)]
// struct Tile {
// 	num: usize,
// 	side: Side,
// 	mirror_x: bool
// }

// fn add_tile(pos: (isize, isize), tile: Tile, neighbors: &HashMap<usize, HashSet<Neighbor>>, tiles: &mut HashMap<(isize, isize), Tile>) {
// 	tiles.insert(pos, tile.clone());
// 	let (x, y) = pos;
// 	println!("Tile: {:?} @ {:?}", tile, pos);
// 	let Tile{num, side, mirror_x} = tile;
// 	for nbr in neighbors.get(&num).unwrap() {
// 		let Neighbor{other, my_side, other_side, reversed} = nbr;
// 		let mut abs_side = my_side.rotate_by(&side, &Side::TOP);
// 		let (offset_x, offset_y) = abs_side.offset();
// 		let new_pos = (x + offset_x, y + offset_y);
// 		let new_rot = Side::TOP.rotate_by_inv(&abs_side.reverse(), &other_side);
// 		let new_mirror = match (side, ) {

// 		};

// 		let new_tile = Tile{num: *other, side: new_rot, mirror_x: new_mirror};

// 		add_tile(new_pos, new_tile, neighbors, tiles);
		
// 		println!("{:?}", nbr);
// 		println!("{:?}", abs_side);
// 		println!("{:?}", new_rot);
// 		println!("{:?}", new_pos);
// 	}
// }

// fn part2(tiles: &HashMap<usize, Vec<Vec<bool>>>) -> usize {
// 	let neighbors = get_neighbors(tiles);
// 	let corners = get_corners(tiles);
// 	let mut tiles = HashMap::new();
// 	add_tile((0, 0), Tile{
// 		num: corners[0],
// 		side: Side::TOP,
// 		mirror_x: false
// 	}, &neighbors, &mut tiles);

// 	0
// }

fn rotate(tile: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut result = Vec::new();
	for y in 0..tile.len() {
		let mut row = Vec::new();
		for x in 0..tile.len() {
			row.push(tile[x][tile.len()-1-y]);
		}
		result.push(row);
	}
	result
}

fn flip(tile: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut result = Vec::new();
	for row in tile {
		let mut row = row.clone();
		row.reverse();
		result.push(row);
	}
	result
}

fn get_sides_vec(tile: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut result = Vec::new();
	let mut left = Vec::new();
	let mut right = Vec::new();
	for row in tile {
		left.push(row[0]);
		right.push(row[row.len()-1]);
	}
	result.push(tile[0].clone());
	result.push(right);
	result.push(tile[tile.len()-1].clone());
	result.push(left);
	result
}

fn set_tile(pos: (isize, isize), tile: &Vec<Vec<bool>>, map: &mut HashMap<(isize, isize), (Vec<Vec<bool>>, Vec<Vec<bool>>)>) {
	map.insert(pos, (tile.clone(), get_sides_vec(tile)));
}

fn part2(tiles: &HashMap<usize, Vec<Vec<bool>>>) -> usize {
	let mut tiles = tiles.clone();
	let mut map = HashMap::new();
	let (num, first) = tiles.iter().next().unwrap();
	set_tile((0, 0), first, &mut map);
	tiles.remove(&num.clone());
	while tiles.len() > 0 {
		for (num, tile) in &tiles {
			let mut tile = tile.clone();
			let mut pos = Option::None;
			'found: for _ in 0..2 {
				for _ in 0..4 {
					tile = rotate(tile);
					for (&(ex_x, ex_y), (_, ex_sides)) in &map {
						let sides = get_sides_vec(&tile);
						if sides[0] == ex_sides[2] {
							pos = Option::Some((ex_x, ex_y + 1));
							break 'found;
						}
						if sides[1] == ex_sides[3] {
							pos = Option::Some((ex_x - 1, ex_y));
							break 'found;
						}
						if sides[2] == ex_sides[0] {
							pos = Option::Some((ex_x, ex_y - 1));
							break 'found;
						}
						if sides[3] == ex_sides[1] {
							pos = Option::Some((ex_x + 1, ex_y));
							break 'found;
						}
					}
				}
				tile = flip(tile);
			}
			if let Some(x) = pos {
				set_tile(x, &tile, &mut map);
				tiles.remove(&num.clone());
				break;
			}
		}
		println!("{:?}", tiles.len());
	}

	let mut min_x = isize::MAX;
	let mut min_y = isize::MAX;
	let mut max_x = isize::MIN;
	let mut max_y = isize::MIN;
	for (&(x, y), _) in &map {
		if x < min_x {
			min_x = x;
		}
		if y < min_y {
			min_y = y;
		}
		if x > max_x {
			max_x = x;
		}
		if y > max_y {
			max_y = y;
		}
	}

	let size = 10;

	let mut result = Vec::new();
	for y_major in min_y..=max_y {
		for y_minor in 1..size-1 {
			let mut row = Vec::new();
			for x_major in min_x..=max_x {
				for x_minor in 1..size-1 {
					row.push(map.get(&(x_major, y_major)).unwrap().0[y_minor][x_minor]);
				}
			}
			result.push(row);
		}
	}
	let mut on_count = 0;
	for row in &result {
		for c in row {
			if *c {
				on_count += 1;
				//print!("#");
			} else {
				//print!(".");
			}
		}
		//println!("");
	}
	let mut result = result;
	let mut snake_count = 0;
	let snake = parse_text(&vec![
		"                  # ",
		"#    ##    ##    ###",
		" #  #  #  #  #  #   "
	]);
	let snake_width = snake[0].len();
	let snake_height = snake.len();
	for _ in 0..2 {
		for _ in 0..4 {
			result = rotate(result);
			for base_x in 0..result[0].len() - snake_width {
				for base_y in 0..result.len() - snake_height {
					let mut is_snake = true;
					'pos: for x in 0..snake_width {
						for y in 0..snake_height {
							if snake[y][x] && !result[base_y + y][base_x + x] {
								is_snake = false;
								break 'pos;
							}
						}
					}
					if is_snake {
						snake_count += 1;
					}
				}
			}
		}
		result = flip(result);
	}
	on_count - snake_count * 15
}

fn parse_text(lines: &Vec<&str>) -> Vec<Vec<bool>> {
	let mut result = Vec::new();
	for line in lines {
		let mut line_vec = Vec::new();
		for c in line.chars() {
			line_vec.push(c == '#');
		}
		result.push(line_vec);
	}
	result
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut tiles = HashMap::new();

    for tile in contents.split("\n\n") {
    	let mut lines = tile.split("\n");
    	let line1 = lines.next().unwrap();
    	let num = line1[5..line1.len()-1].parse().unwrap();
    	let tile_vec = parse_text(&lines.collect());
    	tiles.insert(num, tile_vec);
    }

    println!("{}", part1(&tiles));
    println!("{}", part2(&tiles));
}
