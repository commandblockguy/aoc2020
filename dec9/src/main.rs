use std::fs;

fn get_invalid(nums: Vec<usize>, preamble_len: usize) -> usize {
	let mut window = Vec::new();
	for num in nums.iter() {
    	window.push(num);
    	if window.len() > preamble_len + 1 {
    		window.remove(0);
    		let mut found = false;
    		for i in 0..preamble_len {
    			for j in i+1..preamble_len {
    				//println!("{} {} {}", window[i], window[j], num);
    				if window[i] + window[j] == *num {
    					found = true;
    				}
    			}
    		}
    		if !found {
    			return *num;
    		}
    	}
    }
    return 0;
}

fn main() {
    let contents = fs::read_to_string("input").expect("Couldn't read file");

    let mut nums = Vec::new();

    for line in contents.split("\n") {
    	let num:usize = line.parse().unwrap();
    	nums.push(num);
    }

    let preamble_len = 25;

    let invalid = get_invalid(nums.clone(), preamble_len);

    println!("{}", invalid);

    let mut cum_sum = Vec::new();

    'outer: for (index, num) in nums.iter().enumerate() {
    	cum_sum.push(0);
    	for x in 0..index {
    		cum_sum[x] = cum_sum[x] + num;
    		if cum_sum[x] == invalid {
    			let mut min = usize::MAX;
    			let mut max = usize::MIN;
    			for y in x+1..index+1 {
    				if nums[y] < min {
    					min = nums[y];
    				}
    				if nums[y] > max {
    					max = nums[y];
    				}
    			}
    			println!("{}", min + max);
    			break 'outer;
    		}
    	}
    }
}
