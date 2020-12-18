use std::fs;

fn left(c: char) -> char {
	match c {
		'N' => 'W',
		'E' => 'N',
		'S' => 'E',
		'W' => 'S',
		_ => '_'
	}
}

fn right(c: char) -> char {
	match c {
		'N' => 'E',
		'E' => 'S',
		'S' => 'W',
		'W' => 'N',
		_ => '_'
	}
}

fn mv(mut x:i32, mut y:i32, dir: char, mut facing: char, amount: i32) -> (i32, i32, char) {
	match dir {
		'N' => {
			y -= amount;
		},
		'S' => {
			y += amount;
		},
		'E' => {
			x += amount;
		},
		'W' => {
			x -= amount;
		},
		'F' => {
			let (nx, ny, _) = mv(x, y, facing, facing, amount);
			x = nx;
			y = ny;
		},
		'L' => {
			for _ in 0..amount/90 {
				facing = left(facing);
			}
		},
		'R' => {
			for _ in 0..amount/90 {
				facing = right(facing);
			}
		},
		_ => {
			println!("invalid");
		}
	}
	(x, y, facing)
}

fn mv2(mut x:i32, mut y:i32, dir: char, mut facing: char, amount: i32) -> (i32, i32, char) {
	match dir {
		'N' => {
			y -= amount;
		},
		'S' => {
			y += amount;
		},
		'E' => {
			x += amount;
		},
		'W' => {
			x -= amount;
		},
		'F' => {
			let (nx, ny, _) = mv(x, y, facing, facing, amount);
			x = nx;
			y = ny;
		},
		'L' => {
			for _ in 0..amount/90 {
				facing = left(facing);
			}
		},
		'R' => {
			for _ in 0..amount/90 {
				facing = right(facing);
			}
		},
		_ => {
			println!("invalid");
		}
	}
	(x, y, facing)
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut lines = Vec::new();

    for line in contents.split("\n") {
    	let dir = line.to_string().chars().next().unwrap();
    	let num: i32 = line[1..].to_string().parse().unwrap();
    	lines.push((dir, num));
    }

    let mut x = 0;
    let mut y = 0;
    let mut facing = 'E';

    for (dir, num) in lines.clone() {
    	let (nx, ny, nface) = mv(x, y, dir, facing, num);
    	x = nx;
    	y = ny;
    	facing = nface;
    	//println!("{} {} {}", x, y, facing);
    }

    println!("{}", x.abs() + y.abs());

    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = -1;

    for (dir, num) in lines {
    	match dir {
    		'N' => {
    			waypoint_y -= num;
    		},
    		'S' => {
    			waypoint_y += num;
    		},
    		'E' => {
    			waypoint_x += num;
    		},
    		'W' => {
    			waypoint_x -= num;
    		},
    		'F' => {
    			for _ in 0..num {
	    			ship_x = ship_x + waypoint_x;
	    			ship_y = ship_y + waypoint_y;
	    		}
    		},
    		'L' => {
    			for _ in 0..num/90 {
    				let x_bkp = waypoint_x;
    				waypoint_x = waypoint_y;
    				waypoint_y = -x_bkp;
    			}
    		},
    		'R' => {
    			for _ in 0..num/90 {
    				let x_bkp = waypoint_x;
    				waypoint_x = -waypoint_y;
    				waypoint_y = x_bkp;
    			}
    		},
    		_ => {
    			println!("invalid");
    		}
    	}
    	//println!("{} {} {} {}", ship_x, ship_y, waypoint_x, waypoint_y);
    }
    println!("{}", ship_x.abs() + ship_y.abs());
}
