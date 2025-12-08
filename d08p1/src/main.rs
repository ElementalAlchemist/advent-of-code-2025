use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;

const TO_CONNECT: usize = 1000;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
	z: u32,
}

impl Coordinate {
	fn distance_to(&self, other: &Self) -> f64 {
		let x_1: f64 = self.x.into();
		let x_2: f64 = other.x.into();
		let y_1: f64 = self.y.into();
		let y_2: f64 = other.y.into();
		let z_1: f64 = self.z.into();
		let z_2: f64 = other.z.into();
		((x_1 - x_2).powi(2) + (y_1 - y_2).powi(2) + (z_1 - z_2).powi(2)).sqrt()
	}
}

#[derive(Eq, PartialEq)]
struct CoordinateGroup(HashSet<Coordinate>);

impl CoordinateGroup {
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl Ord for CoordinateGroup {
	fn cmp(&self, other: &Self) -> Ordering {
		self.len().cmp(&other.len())
	}
}

impl PartialOrd for CoordinateGroup {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let boxes: Vec<Coordinate> = {
		let input = include_str!("../input.txt");
		let mut boxes = Vec::new();
		for line in input.lines() {
			let (x, rest) = line.split_once(',').expect("Lines should have three coordinate values");
			let (y, z) = rest.split_once(',').expect("Lines should have three coordinate values");

			boxes.push(Coordinate {
				x: x.parse()?,
				y: y.parse()?,
				z: z.parse()?,
			});
		}

		boxes
	};

	let mut box_pairs: HashMap<Coordinate, HashSet<Coordinate>> = HashMap::new();

	for _ in 0..TO_CONNECT {
		let mut shortest_distance_and_pair: Option<(f64, Coordinate, Coordinate)> = None;
		for (index, coord) in boxes.iter().enumerate() {
			for other_coord in boxes.iter().skip(index + 1) {
				update_shortest(&mut shortest_distance_and_pair, &box_pairs, coord, other_coord);
			}
		}

		if let Some((_, first_coord, second_coord)) = shortest_distance_and_pair {
			box_pairs
				.entry(first_coord.clone())
				.or_default()
				.insert(second_coord.clone());
			box_pairs.entry(second_coord).or_default().insert(first_coord);
		}
	}

	let mut circuits: Vec<HashSet<Coordinate>> = Vec::new();
	for (coordinate, connected_coordinates) in box_pairs {
		let mut this_group = connected_coordinates;
		this_group.insert(coordinate);
		let mut all_matching_circuit_indices: Vec<usize> = Vec::new();
		for (circuit_index, circuit) in circuits.iter().enumerate() {
			if !circuit.is_disjoint(&this_group) {
				all_matching_circuit_indices.push(circuit_index);
			}
		}
		match all_matching_circuit_indices.len() {
			0 => circuits.push(this_group),
			1 => {
				for coordinate in this_group {
					circuits[all_matching_circuit_indices[0]].insert(coordinate);
				}
			}
			_ => {
				while let Some(index) = all_matching_circuit_indices.pop() {
					let circuit = circuits.swap_remove(index);
					for coordinate in circuit {
						this_group.insert(coordinate);
					}
				}
				circuits.push(this_group);
			}
		}
	}

	let mut circuits: BinaryHeap<CoordinateGroup> = circuits.into_iter().map(CoordinateGroup).collect();

	let mut largest_circuit_sizes = 1;
	for _ in 0..3 {
		largest_circuit_sizes *= circuits.pop().expect("at least 3 circuits").len();
	}

	println!("{largest_circuit_sizes}");

	Ok(())
}

fn update_shortest(
	shortest_distance_and_pair: &mut Option<(f64, Coordinate, Coordinate)>,
	pairs: &HashMap<Coordinate, HashSet<Coordinate>>,
	first_coord: &Coordinate,
	second_coord: &Coordinate,
) {
	if let Some(connected_coords) = pairs.get(first_coord)
		&& connected_coords.contains(second_coord)
	{
		return;
	}

	let distance = first_coord.distance_to(second_coord);
	match shortest_distance_and_pair.as_mut() {
		Some((shortest_distance, shortest_first_coord, shortest_second_coord)) => {
			if distance < *shortest_distance {
				*shortest_distance = distance;
				*shortest_first_coord = first_coord.clone();
				*shortest_second_coord = second_coord.clone();
			}
		}
		None => *shortest_distance_and_pair = Some((distance, first_coord.clone(), second_coord.clone())),
	}
}
