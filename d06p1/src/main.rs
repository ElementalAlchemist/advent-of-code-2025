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
		let mut numberses: Vec<Vec<u64>> = Vec::new();
		let mut calculations: Vec<Calculation> = Vec::new();

		for line in input.lines() {
			let mut number_index = 0;
			for value in line.split_whitespace() {
				if value.is_empty() {
					continue;
				}
				if value == "+" {
					calculations.push(Calculation {
						operation: Operation::Add,
						numbers: numberses[number_index].clone(),
					});
				} else if value == "*" {
					calculations.push(Calculation {
						operation: Operation::Multiply,
						numbers: numberses[number_index].clone(),
					});
				} else {
					while numberses.len() <= number_index {
						numberses.push(Vec::new());
					}
					numberses[number_index].push(value.parse()?);
				}
				number_index += 1;
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
