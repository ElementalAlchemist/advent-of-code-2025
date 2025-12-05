use std::error::Error;
use std::ops::RangeInclusive;

fn ranges_overlap(lhs: &RangeInclusive<u64>, rhs: &RangeInclusive<u64>) -> bool {
	lhs.end() >= rhs.start() && rhs.end() >= lhs.start()
}

fn main() -> Result<(), Box<dyn Error>> {
	let fresh_ingredient_ranges = {
		let input = include_str!("../input.txt");
		let mut fresh_ingredient_ranges: Vec<RangeInclusive<u64>> = Vec::new();

		for fresh_range in input.lines() {
			if fresh_range.is_empty() {
				break;
			}

			let (start, end) = fresh_range.split_once('-').expect("line contains a range");
			let start: u64 = start.parse()?;
			let end: u64 = end.parse()?;
			fresh_ingredient_ranges.push(start..=end);
		}

		fresh_ingredient_ranges
	};

	let mut dedup_ingredient_ranges: Vec<RangeInclusive<u64>> = Vec::new();
	for mut range in fresh_ingredient_ranges {
		let mut new_dedup_ingredient_ranges = dedup_ingredient_ranges.clone();

		loop {
			let mut overlap_index: Option<usize> = None;
			for (index, dedup_range) in new_dedup_ingredient_ranges.iter().enumerate() {
				if ranges_overlap(&range, dedup_range) {
					overlap_index = Some(index);
					break;
				}
			}
			match overlap_index {
				Some(overlap_index) => {
					let overlap_range = new_dedup_ingredient_ranges.remove(overlap_index);
					range = (*range.start().min(overlap_range.start()))..=(*range.end().max(overlap_range.end()));
				}
				None => break,
			}
		}

		new_dedup_ingredient_ranges.push(range);
		dedup_ingredient_ranges = new_dedup_ingredient_ranges;
	}

	let mut count = 0;
	for range in dedup_ingredient_ranges {
		count += range.end() - range.start() + 1;
	}
	println!("{count}");

	Ok(())
}
