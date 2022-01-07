//! https://adventofcode.com/2021/day/3

use aoc2021::*;

fn main() {
	let counter = read_lines("data/3/input.txt")
		.fold(Vec::new(), |mut counter, line| {
			for (index, char) in line.chars().enumerate() {
				if counter.len() < index + 1 {
					counter.push(0);
				}
				match char {
					'0' => counter[index] -= 1,
					'1' => counter[index] += 1,
					_ => (),
				}
			}
			counter
		});

	let mut gamma: u32 = 0;
	let mut epsilon: u32 = 0;

	for (exp, count) in counter.iter().rev().enumerate() {
		if count > &0 {
			gamma += 2u32.pow(exp as u32);
		}
		else if count < &0 {
			epsilon += 2u32.pow(exp as u32);
		}
	}

	println!("gamma rate {}", gamma);
	println!("epsilon rate {}", epsilon);
	println!("power consumption {}", gamma * epsilon);
}
