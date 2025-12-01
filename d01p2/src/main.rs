enum RotationDirection {
	Left,
	Right,
}

struct Rotation {
	direction: RotationDirection,
	distance: u32,
}

impl Rotation {
	fn apply(&self, mut position: u32) -> (u32, u32) {
		match self.direction {
			RotationDirection::Left => {
				let mut zeroes = 0;
				while position < self.distance {
					if position != 0 {
						zeroes += 1;
					}
					position += 100;
				}
				let new_position = position - self.distance;
				if new_position == 0 {
					zeroes += 1;
				}
				(new_position, zeroes)
			}
			RotationDirection::Right => {
				let end = position + self.distance;
				(end % 100, end / 100)
			}
		}
	}
}

fn main() {
	let inputs = {
		let input = include_str!("../input.txt");
		let mut inputs: Vec<Rotation> = Vec::new();
		for line in input.lines() {
			let (direction, distance) = if let Some(distance) = line.strip_prefix('L') {
				(
					RotationDirection::Left,
					distance.parse().expect("numeric rotation distance"),
				)
			} else if let Some(distance) = line.strip_prefix('R') {
				(
					RotationDirection::Right,
					distance.parse().expect("numeric rotation distance"),
				)
			} else {
				panic!("Invalid input line");
			};
			inputs.push(Rotation { direction, distance });
		}
		inputs
	};

	let mut dial_position = 50;
	let mut zero_points = 0;
	for input in inputs {
		let (new_dial_position, new_zeroes) = input.apply(dial_position);
		dial_position = new_dial_position;
		zero_points += new_zeroes;
	}

	println!("{zero_points}");
}
