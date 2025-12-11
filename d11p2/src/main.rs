use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Clone, Eq, Hash, PartialEq)]
struct SearchState {
	at: String,
	dac: bool,
	fft: bool,
}

impl SearchState {
	fn new(at: String) -> Self {
		Self {
			at,
			dac: false,
			fft: false,
		}
	}

	fn move_to(&mut self, new_at: &str) {
		self.at = new_at.to_string();
		if new_at == "dac" {
			self.dac = true;
		} else if new_at == "fft" {
			self.fft = true;
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let map: HashMap<String, Vec<String>> = {
		let input = include_str!("../input.txt");
		let mut map = HashMap::new();
		for line in input.lines() {
			let (machine, output) = line.split_once(": ").expect("line has a source and outputs");
			map.insert(machine.to_string(), output.split(' ').map(String::from).collect());
		}
		map
	};

	let mut reverse_map: HashMap<String, HashSet<String>> = HashMap::new();
	for (source, outputs) in map.iter() {
		for output in outputs.iter() {
			reverse_map.entry(output.clone()).or_default().insert(source.clone());
		}
	}

	let mut states: HashMap<SearchState, u64> = HashMap::new();
	states.insert(SearchState::new(String::from("svr")), 1);
	let mut outs = 0;
	while !states.is_empty() {
		let mut new_states = HashMap::new();
		for (state, count) in states {
			for output in map.get(&state.at).unwrap() {
				if output == "out" {
					if state.dac && state.fft {
						outs += count;
					}
				} else {
					let mut new_state = state.clone();
					new_state.move_to(output);
					*new_states.entry(new_state).or_default() += count;
				}
			}
		}
		states = new_states;
	}

	println!("{outs}");

	Ok(())
}
