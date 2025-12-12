use std::collections::HashSet;
use std::error::Error;

struct Machine {
	indicator_target: Vec<bool>,
	buttons: Vec<Vec<usize>>,
}

impl Machine {
	fn required_presses(&self) -> u32 {
		let mut states: HashSet<Vec<bool>> = HashSet::new();
		states.insert(self.indicator_target.clone());
		let mut presses = 0;
		loop {
			presses += 1;
			let mut new_states: HashSet<Vec<bool>> = HashSet::new();
			for state in states {
				for button in self.buttons.iter() {
					let mut new_state = state.clone();
					for toggle in button.iter().copied() {
						new_state[toggle] = !new_state[toggle];
					}
					if new_state.iter().all(|light| !light) {
						return presses;
					}
					new_states.insert(new_state);
				}
			}
			states = new_states;
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let machines: Vec<Machine> = {
		let input = include_str!("../input.txt");
		let mut machines = Vec::new();

		for line in input.lines() {
			let line = line.strip_prefix('[').expect("lines start with indicator target");
			let line = line.strip_suffix('}').expect("lines end with braced set");
			let (indicator_target, other_data) =
				line.split_once("] ").expect("lines have an end-of-indicator delimiter");
			let (buttons_data, _) = other_data
				.split_once(" {")
				.expect("lines have an end-of-buttons delimiter");

			let indicator_target: Vec<bool> = indicator_target.chars().map(|c| c == '#').collect();

			let buttons_data = buttons_data.strip_prefix('(').expect("buttons start with paren");
			let buttons_data = buttons_data.strip_suffix(')').expect("buttons end with paren");
			let mut buttons = Vec::new();
			for button in buttons_data.split(") (") {
				let mut button_lights: Vec<usize> = Vec::new();
				for light in button.split(',') {
					let light: usize = light.parse()?;
					button_lights.push(light);
				}
				buttons.push(button_lights);
			}

			machines.push(Machine {
				indicator_target,
				buttons,
			});
		}

		machines
	};

	let mut required_presses = 0;
	for machine in machines {
		required_presses += machine.required_presses();
	}
	println!("{required_presses}");

	Ok(())
}
