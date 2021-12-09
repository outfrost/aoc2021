//! https://adventofcode.com/2021/day/1

use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
	let file = File::open("data/1/input.txt").unwrap();

	// How many measurements are larger than the previous measurement?
	let depth_increases = io::BufReader::new(file)
		.lines()
		.filter_map(Result::ok)
		.map(|s| i32::from_str(&s))
		.filter_map(Result::ok)
		// functional witchcraft!
		.fold((0, None), |(mut count, last), sample| {
			if let Some(l) = last {
				if sample > l {
					count += 1;
				}
			}
			(count, Some(sample))
		})
		.0;

	println!("{}", depth_increases);
}
