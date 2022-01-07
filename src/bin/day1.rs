//! https://adventofcode.com/2021/day/1

use std::str::FromStr;

use aoc2021::*;

fn main() {
	let samples: Vec<i32> = read_lines("data/1/input.txt")
		.map(|s| i32::from_str(&s))
		.filter_map(Result::ok)
		.collect();

	if samples.len() == 0 {
		eprintln!("warning: no valid samples read");
	}

	// How many measurements are larger than the previous measurement?
	let depth_increases = samples
		.iter()
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

	println!("increases in depth samples: {}", depth_increases);

	// How many sliding 3-sample window sums are larger than previous ones?
	let mut last_window = (None, None, None);

	let window_increases = samples
		.iter()
		// more witchcraft
		.fold(0, |mut count, sample| {
			match last_window {
				(Some(a), Some(b), Some(c)) => {
					if b + c + sample > a + b + c {
						count += 1;
					}
				}
				_ => (),
			}
			last_window = (last_window.1, last_window.2, Some(sample));
			count
		});

	println!(
		"increases in sliding 3-sample windows: {}",
		window_increases
	);
}
