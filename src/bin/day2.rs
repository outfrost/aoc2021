//! https://adventofcode.com/2021/day/2

use std::str::FromStr;

use aoc2021::*;

fn main() {
	let sequence: Vec<(String, i32)> = read_lines("data/2/input.txt")
		.filter_map(|s| {
			s.split_once(' ')
				.and_then(|split| Some((String::from(split.0), String::from(split.1))))
		})
		.filter_map(|(cmd, arg_str)| match i32::from_str(&arg_str) {
			Ok(arg) => Some((cmd, arg)),
			_ => None,
		})
		.collect();

	let (pos, depth) = sequence
		.iter()
		.fold((0, 0), |(pos, depth), (cmd, arg)| match &cmd[..] {
			"forward" => (pos + arg, depth),
			"down" => (pos, depth + arg),
			"up" => (pos, depth - arg),
			_ => (pos, depth),
		});

	println!("part 1 rules");
	println!("horizontal position: {}", pos);
	println!("depth: {}", depth);
	println!("multiplied together: {}", pos * depth);
	println!();

	let (_, pos, depth) =
		sequence
			.iter()
			.fold((0, 0, 0), |(aim, pos, depth), (cmd, arg)| match &cmd[..] {
				"forward" => (aim, pos + arg, depth + (aim * arg)),
				"down" => (aim + arg, pos, depth),
				"up" => (aim - arg, pos, depth),
				_ => (aim, pos, depth),
			});

	println!("part 2 rules");
	println!("horizontal position: {}", pos);
	println!("depth: {}", depth);
	println!("multiplied together: {}", pos * depth);
	println!();
}
