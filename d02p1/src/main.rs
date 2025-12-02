use std::error::Error;
use std::ops::RangeInclusive;

fn main() -> Result<(), Box<dyn Error>> {
	let id_ranges: Vec<RangeInclusive<u64>> = {
		let input = include_str!("../input.txt").trim();
		let mut ranges = Vec::new();
		for range in input.split(',') {
			let (first, last) = range.split_once('-').expect("input piece represents a range");
			let first = first.parse()?;
			let last = last.parse()?;
			ranges.push(first..=last);
		}
		ranges
	};

	let mut invalid_id_sum = 0;
	for range in id_ranges {
		for id in range {
			let id_str = id.to_string();
			if id_str.len() % 2 == 1 {
				continue;
			}
			let (first_half, second_half) = id_str.split_at(id_str.len() / 2);
			if first_half == second_half {
				invalid_id_sum += id;
			}
		}
	}

	println!("{invalid_id_sum}");

	Ok(())
}
