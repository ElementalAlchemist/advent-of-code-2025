enum RotationDirection {
	Left,
	Right,
}

struct Rotation {
	direction: RotationDirection,
	distance: u32,
}

impl Rotation {
	fn apply(&self, mut position: u32) -> u32 {
		match self.direction {
			RotationDirection::Left => {
				while position < self.distance {
					position += 100;
				}
				position - self.distance
			}
			RotationDirection::Right => (position + self.distance) % 100,
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
		dial_position = input.apply(dial_position);
		if dial_position == 0 {
			zero_points += 1;
		}
	}

	println!("{zero_points}");
}
