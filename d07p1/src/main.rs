use std::collections::{HashMap, HashSet};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let (initial_beam_position, splitters) = {
		let input = include_str!("../input.txt");
		let mut beam_position = 0;
		let mut splitters: HashMap<usize, HashSet<usize>> = HashMap::new();

		for (row, line) in input.lines().enumerate() {
			for (position, c) in line.char_indices() {
				if c == '^' {
					splitters.entry(row).or_default().insert(position);
				} else if c == 'S' {
					beam_position = position;
				}
			}
		}

		(beam_position, splitters)
	};

	let max_rows = splitters.keys().max().copied().unwrap();
	let mut beam_positions = HashSet::new();
	beam_positions.insert(initial_beam_position);
	let mut split_count = 0;

	for row in 0..=max_rows {
		if let Some(splitters) = splitters.get(&row) {
			let mut new_beam_positions = HashSet::new();
			for beam in beam_positions {
				if splitters.contains(&beam) {
					split_count += 1;
					if beam > 0 {
						new_beam_positions.insert(beam - 1);
					}
					new_beam_positions.insert(beam + 1);
				} else {
					new_beam_positions.insert(beam);
				}
			}
			beam_positions = new_beam_positions;
		}
	}

	println!("{split_count}");

	Ok(())
}
