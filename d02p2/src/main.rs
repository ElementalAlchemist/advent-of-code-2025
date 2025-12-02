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
		'id: for id in range {
			let id_str = id.to_string();
			'prefix: for prefix_len in 1..=(id_str.len() / 2) {
				if id_str.len() % prefix_len != 0 {
					continue;
				}
				let prefix: Vec<char> = id_str.chars().take(prefix_len).collect();

				let mut id_iter = id_str.chars();
				let mut prefix_iter = prefix.iter().cycle();
				while let (Some(id_char), Some(prefix_char)) = (id_iter.next(), prefix_iter.next()) {
					if id_char != *prefix_char {
						continue 'prefix;
					}
				}
				invalid_id_sum += id;
				continue 'id;
			}
		}
	}

	println!("{invalid_id_sum}");

	Ok(())
}
