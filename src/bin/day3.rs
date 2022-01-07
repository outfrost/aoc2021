//! https://adventofcode.com/2021/day/3

use aoc2021::*;

fn main() {
	let mut lines = read_lines("data/3/input.txt").peekable();

	let width = lines.peek().unwrap().chars().count();

	let numbers: Vec<u32> = lines
		.filter_map(|s| u32::from_str_radix(&s, 2).ok())
		.collect();

	let counter = numbers.iter().fold(vec![0; width], |mut counter, num| {
		for i in 0..width {
			if num & 1 << i != 0 {
				counter[i] += 1;
			} else {
				counter[i] -= 1;
			}
		}
		counter
	});

	let mut gamma: u32 = 0;
	let mut epsilon: u32 = 0;

	for (exp, count) in counter.iter().enumerate() {
		if *count > 0 {
			gamma += 2u32.pow(exp as u32);
		} else if *count < 0 {
			epsilon += 2u32.pow(exp as u32);
		}
	}

	println!("gamma rate {}", gamma);
	println!("epsilon rate {}", epsilon);
	println!("power consumption {}", gamma * epsilon);

	let mut o2_candidates = numbers.clone();

	for i in (0..width).rev() {
		let one = digit_balance(&o2_candidates, i) >= 0;

		o2_candidates.retain(|num| (num & 1 << i != 0) == one);

		if o2_candidates.len() <= 1 {
			break;
		}
	}

	if o2_candidates.len() != 1 {
		eprintln!("cannot find oxygen generator rating");
		eprintln!("candidates: {:?}", o2_candidates);
		return;
	}

	let mut co2_candidates = numbers.clone();

	for i in (0..width).rev() {
		let one = digit_balance(&co2_candidates, i) < 0;

		co2_candidates.retain(|num| (num & 1 << i != 0) == one);

		if co2_candidates.len() <= 1 {
			break;
		}
	}

	if co2_candidates.len() != 1 {
		eprintln!("cannot find CO2 scrubber rating");
		eprintln!("candidates: {:?}", co2_candidates);
		return;
	}

	let o2_rating = o2_candidates[0];
	let co2_rating = co2_candidates[0];

	println!("oxygen generator rating {}", o2_rating);
	println!("CO2 scrubber rating {}", co2_rating);
	println!("life support rating {}", o2_rating * co2_rating);
}

fn digit_balance(numbers: &Vec<u32>, col: usize) -> i32 {
	numbers.iter().fold(0, |count, num| {
		if num & 1 << col != 0 {
			count + 1
		} else {
			count - 1
		}
	})
}
