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
	let mut beam_positions: HashMap<usize, u64> = HashMap::new();
	beam_positions.insert(initial_beam_position, 1);

	for row in 0..=max_rows {
		if let Some(splitters) = splitters.get(&row) {
			let mut new_beam_positions = HashMap::new();
			for (beam, beam_count) in beam_positions {
				if splitters.contains(&beam) {
					if beam > 0 {
						*new_beam_positions.entry(beam - 1).or_default() += beam_count;
					}
					*new_beam_positions.entry(beam + 1).or_default() += beam_count;
				} else {
					*new_beam_positions.entry(beam).or_default() += beam_count;
				}
			}
			beam_positions = new_beam_positions;
		}
	}

	let total_timelines: u64 = beam_positions.values().sum();
	println!("{total_timelines}");

	Ok(())
}
