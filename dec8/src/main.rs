use std::fs;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)] enum Instruction {
	Acc(i32),
	Nop(i32),
	Jmp(i32)
}

fn run_program(program: Vec<Instruction>) -> (i32, bool) {
	let mut pc = 0;
	let mut acc = 0;

	let mut visited = Vec::new();
	for _ in &program {
		visited.push(false);
	}

	loop {
		//println!("{} {} {:?}", acc, pc, program[pc]);
		if pc >= program.len() {
			return (acc, true);
		}
		if visited[pc] {
			return (acc, false);
		}
		visited[pc] = true;
		match program[pc] {
			Instruction::Acc(x) => {
				acc = acc + x;
			},
			Instruction::Nop(_) => {},
			Instruction::Jmp(x) => {
				pc = ((pc as i32) + x - 1) as usize;
			}
		}
		pc = pc + 1;
	}
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut prgm = Vec::new();

    for line in contents.split("\n") {
    	let (inst_str, arg_str) = line.split(" ").next_tuple().unwrap();
    	let arg: i32 = arg_str.parse().unwrap();
    	let inst = match inst_str {
    		"acc" => {
    			Instruction::Acc(arg)
    		},
    		"jmp" => {
				Instruction::Jmp(arg)
    		},
    		"nop" | _ => {
    			Instruction::Nop(arg)
    		}
    	};
    	prgm.push(inst);
	}

	println!("{:?}", run_program(prgm.clone()).0);

	for pos in 0..prgm.len() {
		let mut mod_prgm = prgm.clone();
		mod_prgm[pos] = Instruction::Nop(0);
		let (acc, returned) = run_program(mod_prgm);
		if returned {
			println!("{:?}", acc);
			break;
		}
	}
}
