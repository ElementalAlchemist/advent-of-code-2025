use std::error::Error;

struct Coordinate {
	x: u64,
	y: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
	let red_tile_coordinates: Vec<Coordinate> = {
		let input = include_str!("../input.txt");
		let mut coordinates = Vec::new();
		for line in input.lines() {
			let (x, y) = line
				.split_once(',')
				.expect("line has two values representing a coordinate");
			let x: u64 = x.parse()?;
			let y: u64 = y.parse()?;
			coordinates.push(Coordinate { x, y });
		}
		coordinates
	};

	let mut max_area = 0;
	for (first_index, first_coord) in red_tile_coordinates.iter().enumerate() {
		for second_coord in red_tile_coordinates.iter().skip(first_index + 1) {
			let x_len = first_coord.x.abs_diff(second_coord.x) + 1;
			let y_len = first_coord.y.abs_diff(second_coord.y) + 1;
			max_area = max_area.max(x_len * y_len);
		}
	}

	println!("{max_area}");

	Ok(())
}
