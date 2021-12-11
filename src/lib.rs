use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(path: &str) -> impl Iterator {
	BufReader::new(File::open(path).unwrap())
		.lines()
		.filter_map(Result::ok)
}
