use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u64,
	y: u64,
}

enum Axis {
	X,
	Y,
}

#[derive(Eq, PartialEq)]
struct CoordinatePair {
	first: Coordinate,
	second: Coordinate,
	area: u64
}

impl CoordinatePair {
	fn new(first: Coordinate, second: Coordinate, x_uncompress_map: &HashMap<u64, u64>, y_uncompress_map: &HashMap<u64, u64>) -> Self {
		let first_x = *x_uncompress_map.get(&first.x).unwrap();
		let first_y = *y_uncompress_map.get(&first.y).unwrap();
		let second_x = *x_uncompress_map.get(&second.x).unwrap();
		let second_y = *y_uncompress_map.get(&second.y).unwrap();

		let area = (first_x.abs_diff(second_x) + 1) * (first_y.abs_diff(second_y) + 1);

		Self { first, second, area }
	}
}

impl Ord for CoordinatePair {
	fn cmp(&self, other: &Self) -> Ordering {
		self.area.cmp(&other.area)
	}
}

impl PartialOrd for CoordinatePair {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut red_tile_coordinates: Vec<Coordinate> = {
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
		coordinates.push(coordinates[0].clone());
		coordinates
	};

	let mut x_uncompression_map: HashMap<u64, u64> = HashMap::new();
	let mut y_uncompression_map: HashMap<u64, u64> = HashMap::new();
	let mut x_compression_map: HashMap<u64, u64> = HashMap::new();
	let mut y_compression_map: HashMap<u64, u64> = HashMap::new();

	let x_coordinates: HashSet<u64> = red_tile_coordinates.iter().map(|coord| coord.x).collect();
	let y_coordinates: HashSet<u64> = red_tile_coordinates.iter().map(|coord| coord.y).collect();

	let mut x_coordinates: Vec<u64> = x_coordinates.into_iter().collect();
	let mut y_coordinates: Vec<u64> = y_coordinates.into_iter().collect();
	x_coordinates.sort_unstable();
	y_coordinates.sort_unstable();

	let mut next_count = 1;
	for x in x_coordinates {
		x_compression_map.insert(x, next_count);
		x_uncompression_map.insert(next_count, x);
		next_count += 2;
	}
	let all_max_x = next_count + 1;
	next_count = 1;
	for y in y_coordinates {
		y_compression_map.insert(y, next_count);
		y_uncompression_map.insert(next_count, y);
		next_count += 2;
	}
	let all_max_y = next_count + 1;

	for coord in red_tile_coordinates.iter_mut() {
		coord.x = *x_compression_map.get(&coord.x).unwrap();
		coord.y = *y_compression_map.get(&coord.y).unwrap();
	}

	let mut perimeter: HashSet<Coordinate> = HashSet::new();
	for coordinate_pair in red_tile_coordinates.windows(2) {
		if coordinate_pair[0].x == coordinate_pair[1].x {
			let x = coordinate_pair[0].x;
			let min_y = coordinate_pair[0].y.min(coordinate_pair[1].y);
			let max_y = coordinate_pair[0].y.max(coordinate_pair[1].y);

			// Why not ..=? It'll get taken care of by the next loop iteration (or, for the last iteration, was by the first
			// iteration), so we don't need to duplicate the effort.
			for y in min_y..max_y {
				let edge_coord = Coordinate { x, y };
				perimeter.insert(edge_coord);
			}
		} else {
			let y = coordinate_pair[0].y;
			let min_x = coordinate_pair[0].x.min(coordinate_pair[1].x);
			let max_x = coordinate_pair[0].x.max(coordinate_pair[1].x);
			for x in min_x..max_x {
				let edge_coord = Coordinate { x, y };
				perimeter.insert(edge_coord);
			}
		}
	}
	let perimeter = perimeter; // drop mut

	let mut coordinate_pairs: BinaryHeap<CoordinatePair> = BinaryHeap::new();
	for (first_index, first_coord) in red_tile_coordinates.iter().enumerate() {
		for second_coord in red_tile_coordinates.iter().skip(first_index + 1) {
			coordinate_pairs.push(CoordinatePair::new(first_coord.clone(), second_coord.clone(), &x_uncompression_map, &y_uncompression_map));
		}
	}

	let mut max_area = 0;
	'check_coord: while let Some(pair) = coordinate_pairs.pop() {
		let min_x = pair.first.x.min(pair.second.x);
		let max_x = pair.first.x.max(pair.second.x);
		let min_y = pair.first.y.min(pair.second.y);
		let max_y = pair.first.y.max(pair.second.y);

		let x_len = max_x - min_x + 1;
		let y_len = max_y - min_y + 1;
		if x_len <= 2 || y_len <= 2 {
			max_area = pair.area;
			break;
		}

		// Verify that the perimeter doesn't sneak inside
		for x in min_x..=max_x {
			let upper = Coordinate { x, y: min_y };
			if !perimeter.contains(&upper)
				&& !is_in(all_max_x, all_max_y, &perimeter, &upper, Axis::Y)
			{
				continue 'check_coord;
			}
			let lower = Coordinate { x, y: max_y };
			if !perimeter.contains(&lower)
				&& !is_in(all_max_x, all_max_y, &perimeter, &lower, Axis::Y)
			{
				continue 'check_coord;
			}
		}
		for y in min_y..=max_y {
			let left = Coordinate { x: min_x, y };
			if !perimeter.contains(&left)
				&& !is_in(all_max_x, all_max_y, &perimeter, &left, Axis::X)
			{
				continue 'check_coord;
			}
			let right = Coordinate { x: max_x, y };
			if perimeter.contains(&right)
				&& !is_in(all_max_x, all_max_y, &perimeter, &right, Axis::X)
			{
				continue 'check_coord;
			}
		}

		max_area = pair.area;
		break;
	}

	println!("{max_area}");

	Ok(())
}

fn is_in(
	all_max_x: u64,
	all_max_y: u64,
	perimeter: &HashSet<Coordinate>,
	coord: &Coordinate,
	axis: Axis,
) -> bool {
	let (cross_coord, all_max) = match axis {
		Axis::X => (coord.x, all_max_x),
		Axis::Y => (coord.y, all_max_y),
	};

	let mut before = 0;
	let mut after = 0;
	let mut crossed = false;
	let mut previous_was_edge = false;
	for check_value in 0..=all_max {
		if check_value == cross_coord {
			crossed = true;
			previous_was_edge = false;
			continue;
		}

		let check_coord = match axis {
			Axis::X => Coordinate {
				x: check_value,
				y: coord.y,
			},
			Axis::Y => Coordinate {
				x: coord.x,
				y: check_value,
			},
		};
		if perimeter.contains(&check_coord) {
			if previous_was_edge {
				continue;
			}
			if crossed {
				after += 1;
			} else {
				before += 1;
			}
			previous_was_edge = true;
		} else {
			previous_was_edge = false;
		}
	}

	before % 2 == 1 || after % 2 == 1
}
