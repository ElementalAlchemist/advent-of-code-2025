use std::collections::HashMap;
use std::error::Error;

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

	let mut states = vec![String::from("you")];
	let mut outs = 0;
	while !states.is_empty() {
		let mut new_states = Vec::new();
		for state in states {
			for output in map.get(&state).unwrap() {
				if output == "out" {
					outs += 1;
				} else {
					new_states.push(output.clone());
				}
			}
		}
		states = new_states;
	}

	println!("{outs}");

	Ok(())
}
