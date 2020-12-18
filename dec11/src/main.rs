use std::fs;

fn count_adjacent(lines: &Vec<String>, x: i32, y: i32) -> u32 {
	let mut count = 0;
	for col in x-1..x+2 {
		for row in y-1..y+2 {
			if !(col == x && row == y) && col >= 0 && row >= 0 && col < lines[0].len() as i32 && row < lines.len() as i32 {
				if lines[row as usize].chars().nth(col as usize).unwrap() == '#' {
					count = count + 1;
				}
			}
		}
	}
	count
}

fn count_visible(lines: &Vec<String>, x: i32, y: i32) -> u32 {
	let mut count = 0;
	//println!("[{} {}]", x, y);
	for dir_x in -1..2 {
		for dir_y in -1..2 {
			//print!("{} {}: ", dir_x, dir_y);
			if dir_x != 0 || dir_y != 0 {
				let mut col = x;
				let mut row = y;
				loop {
					col = col + dir_x;
					row = row + dir_y;
					//print!("({} {})", col, row);
					if col < 0 || col >= lines[0].len() as i32 || row < 0 || row >= lines.len() as i32 {
						break;
					}
					if lines[row as usize].chars().nth(col as usize).unwrap() == 'L' {
						break;
					}
					if lines[row as usize].chars().nth(col as usize).unwrap() == '#' {
						count = count + 1;
						//print!("#");
						break;
					}
				}
			}
			//println!("");
		}
	}
	count
}

fn get_next(lines: &Vec<String>) -> Vec<String> {
	let mut new = Vec::new();
	for y in 0..lines.len() {
		let mut string = "".to_string();
		for x in 0..lines[0].len() {
			let count = count_adjacent(lines, x as i32, y as i32);
			if lines[y].chars().nth(x).unwrap() == '.' {
				string.push('.');
			} else if count == 0 {
				string.push('#');
			} else if count >= 4 {
				string.push('L');
			} else {
				string.push(lines[y].chars().nth(x).unwrap());
			}
		}
		new.push(string);
	}
	new
}

fn get_next2(lines: &Vec<String>) -> Vec<String> {
	let mut new = Vec::new();
	for y in 0..lines.len() {
		let mut string = "".to_string();
		for x in 0..lines[0].len() {
			let count = count_visible(lines, x as i32, y as i32);
			if lines[y].chars().nth(x).unwrap() == '.' {
				string.push('.');
			} else if count == 0 {
				string.push('#');
			} else if count >= 5 {
				string.push('L');
			} else {
				string.push(lines[y].chars().nth(x).unwrap());
			}
		}
		new.push(string);
	}
	new
}

fn part1(lines: &Vec<String>) -> u32 {
	let mut last = lines.clone();
	loop {
		// for y in 0..lines.len() {
		// 	for x in 0..lines[0].len() {
		// 		print!("{}", count_adjacent(&last, x as i32, y as i32));
		// 	}
		// 	println!("");
		// }
		let next = get_next2(&last);
		// for line in &next {
		// 	println!("{}", line);
		// }
		// println!("");
		if next == last {
			let mut count = 0;
			for line in &next {
				count = count + line.matches("#").count();
			}
			return count as u32;
		}
		last = next.clone();
	}
}

fn part2(lines: &Vec<String>) -> u32 {
	let mut last = lines.clone();
	loop {
		// for y in 0..lines.len() {
		// 	for x in 0..lines[0].len() {
		// 		print!("{}", count_visible(&last, x as i32, y as i32));
		// 	}
		// 	println!("");
		// }
		let next = get_next2(&last);
		// for line in &next {
		// 	println!("{}", line);
		// }
		// println!("");
		if next == last {
			let mut count = 0;
			for line in &next {
				count = count + line.matches("#").count();
			}
			return count as u32;
		}
		last = next.clone();
	}
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut lines = Vec::new();

    for line in contents.split("\n") {
    	lines.push(line.to_string());
    }

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
