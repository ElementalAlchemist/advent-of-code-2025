use std::error::Error;
use std::ops::RangeInclusive;

fn main() -> Result<(), Box<dyn Error>> {
	let (fresh_ingredient_ranges, available_ingredients) = {
		let input = include_str!("../input.txt");
		let mut input_iter = input.lines();
		let mut fresh_ingredient_ranges: Vec<RangeInclusive<u64>> = Vec::new();
		let mut available_ingredients: Vec<u64> = Vec::new();

		for fresh_range in input_iter.by_ref() {
			if fresh_range.is_empty() {
				break;
			}

			let (start, end) = fresh_range.split_once('-').expect("line contains a range");
			let start: u64 = start.parse()?;
			let end: u64 = end.parse()?;
			fresh_ingredient_ranges.push(start..=end);
		}

		for available_ingredient in input_iter {
			available_ingredients.push(available_ingredient.parse()?);
		}

		(fresh_ingredient_ranges, available_ingredients)
	};

	let mut number_available = 0;
	'ingredient: for ingredient in available_ingredients {
		for fresh_range in fresh_ingredient_ranges.iter() {
			if fresh_range.contains(&ingredient) {
				number_available += 1;
				continue 'ingredient;
			}
		}
	}

	println!("{number_available}");

	Ok(())
}
