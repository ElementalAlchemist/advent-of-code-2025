use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let batteries_by_bank: Vec<Vec<u32>> = {
		let battery_list = include_str!("../input.txt");
		let mut batteries_by_bank = Vec::new();
		for bank in battery_list.lines() {
			let mut batteries: Vec<u32> = Vec::new();
			for battery in bank.chars() {
				batteries.push(battery.to_string().parse()?);
			}
			batteries_by_bank.push(batteries);
		}

		batteries_by_bank
	};

	let mut max_joltage = 0;

	for bank in batteries_by_bank {
		let mut highest_bank_joltage = 0;
		for start_index in 0..(bank.len() - 1) {
			let mut bank_iter = bank.iter().skip(start_index);
			let first_battery_joltage = bank_iter.next().unwrap() * 10;
			if first_battery_joltage < highest_bank_joltage {
				continue;
			}
			for second_battery in bank_iter {
				let bank_joltage = first_battery_joltage + second_battery;
				if bank_joltage > highest_bank_joltage {
					highest_bank_joltage = bank_joltage;
				}
			}
		}
		max_joltage += highest_bank_joltage;
	}

	println!("{max_joltage}");

	Ok(())
}
