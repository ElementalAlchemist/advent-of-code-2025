use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::io::{Write, stdout};

#[derive(Clone, Eq, PartialEq)]
struct JoltageState {
	remaining_joltage: Vec<u32>,
	presses_so_far: u32,
	buttons_pressed: HashSet<usize>,
}

impl JoltageState {
	fn new(starting_joltage: Vec<u32>) -> Self {
		Self {
			remaining_joltage: starting_joltage,
			presses_so_far: 0,
			buttons_pressed: HashSet::new(),
		}
	}
}

impl Ord for JoltageState {
	fn cmp(&self, other: &Self) -> Ordering {
		self.presses_so_far
			.cmp(&other.presses_so_far)
			.then_with(|| {
				let self_count = self.remaining_joltage.iter().filter(|x| **x == 0).count();
				let other_count = other.remaining_joltage.iter().filter(|x| **x == 0).count();
				other_count.cmp(&self_count)
			})
			.then_with(|| {
				let self_min: Option<u32> = self.remaining_joltage.iter().filter(|x| **x > 0).min().copied();
				let other_min: Option<u32> = other.remaining_joltage.iter().filter(|x| **x > 0).min().copied();
				match (self_min, other_min) {
					(Some(self_min), Some(other_min)) => self_min.cmp(&other_min),
					// None means we have no non-zero values, so sort those first
					(Some(_), None) => Ordering::Less,
					(None, Some(_)) => Ordering::Greater,
					(None, None) => Ordering::Equal,
				}
			})
			.then_with(|| {
				let self_sum: u32 = self.remaining_joltage.iter().sum();
				let other_sum: u32 = other.remaining_joltage.iter().sum();
				self_sum.cmp(&other_sum)
			})
			.then_with(|| self.remaining_joltage.cmp(&other.remaining_joltage))
	}
}

impl PartialOrd for JoltageState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

struct Machine {
	buttons: Vec<Vec<usize>>,
	required_joltage: Vec<u32>,
}

impl Machine {
	fn required_presses(&self) -> u32 {
		let mut states: BTreeSet<JoltageState> = BTreeSet::new();
		states.insert(JoltageState::new(self.required_joltage.clone()));

		'state: while let Some(state) = states.pop_first() {
			if state.remaining_joltage.iter().all(|joltage| *joltage == 0) {
				return state.presses_so_far;
			}

			let mut buttons_for_joltage: Vec<Vec<usize>> = vec![Vec::new(); state.remaining_joltage.len()];
			for (button_index, button) in self.buttons.iter().enumerate() {
				if state.buttons_pressed.contains(&button_index) {
					continue;
				}
				if button.iter().any(|counter| state.remaining_joltage[*counter] == 0) {
					continue;
				}
				for counter in button.iter().copied() {
					buttons_for_joltage[counter].push(button_index);
				}
			}

			let mut single_button_state = state.clone();
			for (first_counter, first_buttons) in buttons_for_joltage.iter().enumerate() {
				if first_buttons.is_empty() {
					if state.remaining_joltage[first_counter] > 0 {
						continue 'state;
					}
					continue;
				}

				if first_buttons.len() == 1 {
					let button_index = first_buttons[0];
					let button = &self.buttons[button_index];
					let presses = single_button_state.remaining_joltage[first_counter];
					for counter in button.iter().copied() {
						if single_button_state.remaining_joltage[counter] < presses {
							continue 'state;
						}
						single_button_state.remaining_joltage[counter] -= presses;
					}
					single_button_state.presses_so_far += presses;
					single_button_state.buttons_pressed.insert(button_index);
				}

				for (second_counter, second_buttons) in buttons_for_joltage.iter().enumerate().skip(first_counter + 1) {
					if second_buttons.is_empty() {
						// we'll come back to this
						continue;
					}

					if first_buttons == second_buttons {
						if state.remaining_joltage[first_counter] == state.remaining_joltage[second_counter] {
							continue;
						}
						continue 'state;
					}

					let first_buttons_set: HashSet<usize> = first_buttons.iter().copied().collect();
					let second_buttons_set: HashSet<usize> = second_buttons.iter().copied().collect();
					if first_buttons_set.is_subset(&second_buttons_set)
						&& state.remaining_joltage[first_counter] > state.remaining_joltage[second_counter]
					{
						continue 'state;
					}
					if second_buttons_set.is_subset(&first_buttons_set)
						&& state.remaining_joltage[second_counter] > state.remaining_joltage[first_counter]
					{
						continue 'state;
					}
				}
			}

			if single_button_state.presses_so_far != state.presses_so_far {
				states.insert(single_button_state);
				continue;
			}
			drop(single_button_state);

			// The next button we try to press should have the fewest buttons that can affect it.
			// This helps to prevent the number of possible states from completely exploding early.
			// Later, once we stop considering buttons for counters that have reached 0, it should also help reduce state
			// explosion a bit.
			let (next_decrease_index, next_decrease_joltage) = {
				let next_decrease_index = buttons_for_joltage.iter().enumerate().filter(|(_, buttons)| !buttons.is_empty()).min_by_key(|(_, buttons)| buttons.len()).map(|(index, _)| index).unwrap();
				let next_decrease_joltage = state.remaining_joltage[next_decrease_index];
				(next_decrease_index, next_decrease_joltage)
			};

			if buttons_for_joltage[next_decrease_index].is_empty() {
				continue;
			}

			'button: for button_index in buttons_for_joltage[next_decrease_index].iter().copied() {
				let button_counters = &self.buttons[button_index];
				for new_presses in 1..=next_decrease_joltage {
					let mut new_state = state.clone();
					for counter in button_counters.iter() {
						if new_state.remaining_joltage[*counter] < new_presses {
							continue 'button;
						}
						new_state.remaining_joltage[*counter] -= new_presses;
					}
					new_state.presses_so_far += new_presses;
					new_state.buttons_pressed.insert(button_index);
					states.insert(new_state);
				}
			}
		}

		panic!("Ran out of states before finding solution");
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let machines: Vec<Machine> = {
		let input = include_str!("../input.txt");
		let mut machines = Vec::new();

		for line in input.lines() {
			let line = line.strip_prefix('[').expect("lines start with indicator target");
			let line = line.strip_suffix('}').expect("lines end with braced set");
			let (_, other_data) = line.split_once("] ").expect("lines have an end-of-indicator delimiter");
			let (buttons_data, joltage_requirements) = other_data
				.split_once(" {")
				.expect("lines have an end-of-buttons delimiter");

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

			let mut required_joltage: Vec<u32> = Vec::new();
			for requirement in joltage_requirements.split(',') {
				required_joltage.push(requirement.parse()?);
			}

			machines.push(Machine {
				buttons,
				required_joltage,
			});
		}

		machines
	};

	let mut required_presses = 0;
	for (index, machine) in machines.iter().enumerate() {
		print!("{index}: {:?} {:?}", machine.buttons, machine.required_joltage);
		let _ = stdout().flush();

		required_presses += machine.required_presses();
		println!(" {required_presses}");
	}
	println!("{required_presses}");

	Ok(())
}
