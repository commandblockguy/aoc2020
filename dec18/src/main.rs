use std::fs;
use std::iter;

#[derive(Debug, Clone)]
enum Node {
	Number(isize),
	Operation(Operation)
}

#[derive(Debug, Clone)]
struct Operation {
	left: Box<Node>,
	op: char,
	right: Box<Node>,
}

fn flat(string: &str, stage: usize) -> (Vec<Node>, Vec<char>) {
	if string.len() == 0 {
		return (Vec::new(), Vec::new())
	}

	let (first, pos) = match &string[..1] {
		"(" => {
			let mut paren_count = 1;
			let mut pos = None;
			for (i, c) in string.chars().enumerate().skip(1) {
				match c {
					'(' => paren_count = paren_count + 1,
					')' => paren_count = paren_count - 1,
					_ => ()
				};
				if paren_count == 0 {
					pos = Some(i);
					break;
				}
			};
			let pos = pos.unwrap();
			let inner = &string[1..pos];
			(parse_operation(inner, stage), pos + 1)
		},
		_ => {
			let len = match string.find(' ') {
				Some(x) => x,
				None => string.len()
			};

			let num = string[..len].parse().unwrap();

			(Node::Number(num), len)
		}
	};

	if pos == string.len() {
		(vec![first], Vec::new())
	} else {
		let op = string.chars().nth(pos + 1);

		let mut nodes = vec![first];
		let mut ops = match op {
			Some(x) => vec![x],
			None => Vec::new()
		};

		let rest = &string[pos+3..];

		let (mut rest_nodes, mut rest_ops) = flat(rest, stage);

		nodes.append(&mut rest_nodes);
		ops.append(&mut rest_ops);

		(nodes, ops)
	}
}

fn unflatten1(flat: Vec<Node>, ops: Vec<char>) -> Node {
	let mut current: Node = flat.iter().nth(0).unwrap().clone();

	for (node, op) in flat.iter().skip(1).zip(ops.iter()) {
		current = Node::Operation(Operation {
			left: Box::new(current.clone()),
			op: *op,
			right: Box::new(node.clone())
		});
	}

	current
}

fn priority(op: char) -> usize {
	match op {
		'+' => 2,
		'*' => 1,
		_ => 0
	}
}

fn unflatten2(flat: Vec<Node>, ops: Vec<char>) -> Node {
	if flat.len() == 1 {
		return flat[0].clone();
	}

	let (new_flat, new_ops) = if ops.len() == 1 || priority(ops[0]) >= priority(ops[1]) {
		let first = Node::Operation(Operation {
			left: Box::new(flat[0].clone()),
			op: ops[0],
			right: Box::new(flat[1].clone())
		});
		let new_flat:Vec<Node> = iter::once(first).chain(flat.iter().cloned().skip(2)).collect();
		(new_flat, ops.iter().skip(1).copied().collect())
	} else {
		let next = Node::Operation(Operation {
			left: Box::new(flat[1].clone()),
			op: ops[1],
			right: Box::new(flat[2].clone())
		});
		let new_flat = iter::once(flat[0].clone()).chain(iter::once(next)).chain(flat.iter().cloned().skip(3)).collect();
		let new_ops = iter::once(ops[0]).chain(ops.iter().copied().skip(2)).collect();
		(new_flat, new_ops)
	};

	unflatten2(new_flat, new_ops)
}

fn parse_operation(string: &str, stage: usize) -> Node {
	let (flat, ops) = flat(string, stage);

	match stage {
		1 => unflatten1(flat, ops),
		2 => unflatten2(flat, ops),
		_ => panic!("Invalid stage number")
	}
}

fn node_to_string(node: &Node) -> String {
	match node {
		Node::Number(x) => x.to_string(),
		Node::Operation(op) => {
			format!("({} {} {})", node_to_string(&op.left), op.op, node_to_string(&op.right))
		}
	}
}

fn interpret(node: &Node) -> isize {
	//println!("{}", node_to_string(node));
	match node {
		Node::Number(x) => *x,
		Node::Operation(op) => {
			let left = interpret(&op.left);
			let right = interpret(&op.right);
			match op.op {
				'+' => left + right,
				'*' => left * right,
				_ => {
					panic!("Invalid operation");
				}
			}
		}
	}
}

fn part1(contents: &str) -> isize {
	let mut sum = 0;
	for line in contents.split("\n") {
		sum = sum + interpret(&parse_operation(&line, 1));
	}
	sum
}

fn part2(contents: &str) -> isize {
	let mut sum = 0;
	for line in contents.split("\n") {
		sum = sum + interpret(&parse_operation(&line, 2));
	}
	sum
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}
