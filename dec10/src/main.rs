use std::fs;

fn part1(nums: Vec<u32>) -> u32 {
	let mut last = nums[0];

    let mut counts = vec![0, 0, 0];

    for num in nums {
    	let diff = (num - last) as usize;
    	if diff == 0 {continue;}
    	last = num;
    	let index:usize = diff - 1;
    	counts[index] = counts[index] + 1;
    }
    counts[0] * counts[2]
}

fn part2(nums: Vec<u32>) -> u64 {
	let mut combos: Vec<u64> = Vec::new();

	for _ in 0..nums.len() {
		combos.push(0);
	}

	combos[0] = 1;

	for index in 0..nums.len() {
		let current = nums[index];
		for next_index in index+1..nums.len() {
			let next = nums[next_index];
			if next > current + 3 {
				break;
			}
			combos[next_index] = combos[next_index] + combos[index];
		}
	}
	combos[combos.len() - 1]
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut nums = Vec::new();

    for line in contents.split("\n") {
    	let num:u32 = line.parse().unwrap();
    	nums.push(num);
    }

    nums.push(0);

    nums.sort();

    nums.push(nums[nums.len() - 1] + 3);

    println!("{}", part1(nums.clone()));
    println!("{}", part2(nums));
}
