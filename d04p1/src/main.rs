use std::collections::HashSet;
use std::error::Error;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn adjacent_coordinates(&self) -> Vec<Self> {
		let mut adjacents = Vec::new();

		if self.x > 0 && self.y > 0 {
			adjacents.push(Self {
				x: self.x - 1,
				y: self.y - 1,
			});
		}
		if self.x > 0 {
			adjacents.push(Self {
				x: self.x - 1,
				y: self.y,
			});
			adjacents.push(Self {
				x: self.x - 1,
				y: self.y + 1,
			});
		}
		if self.y > 0 {
			adjacents.push(Self {
				x: self.x,
				y: self.y - 1,
			});
		}
		adjacents.push(Self {
			x: self.x,
			y: self.y + 1,
		});
		if self.y > 0 {
			adjacents.push(Self {
				x: self.x + 1,
				y: self.y - 1,
			});
		}
		adjacents.push(Self {
			x: self.x + 1,
			y: self.y,
		});
		adjacents.push(Self {
			x: self.x + 1,
			y: self.y + 1,
		});

		adjacents
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let roll_locations = {
		let mut locations: HashSet<Coordinate> = HashSet::new();
		let input = include_str!("../input.txt");
		for (y, row) in input.lines().enumerate() {
			for (x, location) in row.char_indices() {
				if location == '@' {
					locations.insert(Coordinate { x, y });
				}
			}
		}
		locations
	};

	let mut accessible_rolls = 0;

	for location in roll_locations.iter() {
		let mut occupied_sides = 0;
		let adjacent_coords = location.adjacent_coordinates();
		for coord in adjacent_coords.iter() {
			if roll_locations.contains(coord) {
				occupied_sides += 1;
			}
		}
		if occupied_sides < 4 {
			accessible_rolls += 1;
		}
	}

	println!("{accessible_rolls}");

	Ok(())
}
