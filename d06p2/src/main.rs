use std::error::Error;

enum Operation {
	Add,
	Multiply,
}

struct Calculation {
	operation: Operation,
	numbers: Vec<u64>,
}

impl Calculation {
	fn solve(&self) -> u64 {
		match self.operation {
			Operation::Add => self.numbers.iter().copied().sum(),
			Operation::Multiply => self.numbers.iter().copied().product(),
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let calculations: Vec<Calculation> = {
		let input = include_str!("../input.txt");
		let mut sideways_input: Vec<String> = Vec::new();
		for line in input.lines() {
			for (index, c) in line.char_indices() {
				while sideways_input.len() <= index {
					sideways_input.push(String::new());
				}
				sideways_input[index].push(c);
			}
		}

		let mut numbers: Vec<u64> = Vec::new();
		let mut calculations: Vec<Calculation> = Vec::new();

		for number in sideways_input.into_iter().rev() {
			let number = number.trim();
			if number.is_empty() {
				continue;
			}
			if let Some(number) = number.strip_suffix('+') {
				numbers.push(number.trim().parse()?);
				calculations.push(Calculation {
					operation: Operation::Add,
					numbers: std::mem::take(&mut numbers),
				});
			} else if let Some(number) = number.strip_suffix('*') {
				numbers.push(number.trim().parse()?);
				calculations.push(Calculation {
					operation: Operation::Multiply,
					numbers: std::mem::take(&mut numbers),
				});
			} else {
				numbers.push(number.parse()?);
			}
		}

		calculations
	};

	let mut grand_total = 0;
	for calculation in calculations {
		grand_total += calculation.solve();
	}
	println!("{grand_total}");

	Ok(())
}
