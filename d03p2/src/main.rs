use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let batteries_by_bank: Vec<Vec<u64>> = {
		let battery_list = include_str!("../input.txt");
		let mut batteries_by_bank = Vec::new();
		for bank in battery_list.lines() {
			let mut batteries: Vec<u64> = Vec::new();
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
		find_max_joltage(&mut highest_bank_joltage, &bank)?;
		max_joltage += highest_bank_joltage;
	}

	println!("{max_joltage}");

	Ok(())
}

fn find_max_joltage(max_joltage: &mut u64, bank: &[u64]) -> Result<(), Box<dyn Error>> {
	find_max_joltage_for_sub(max_joltage, 0, 12, bank)
}

fn find_max_joltage_for_sub(
	max_joltage: &mut u64,
	current_joltage: u64,
	digits_remaining: usize,
	bank: &[u64],
) -> Result<(), Box<dyn Error>> {
	let new_digits_remaining = digits_remaining - 1;
	let first_battery = bank.iter().take(bank.len() - new_digits_remaining).max().unwrap();
	let index = bank
		.iter()
		.enumerate()
		.find(|(_, x)| *x == first_battery)
		.map(|(index, _)| index)
		.unwrap();
	let new_joltage = current_joltage + (first_battery * 10u64.pow(new_digits_remaining.try_into()?));
	if new_digits_remaining == 0 {
		if new_joltage > *max_joltage {
			*max_joltage = new_joltage;
		}
	} else {
		find_max_joltage_for_sub(max_joltage, new_joltage, new_digits_remaining, &bank[(index + 1)..])?;
	}

	Ok(())
}
