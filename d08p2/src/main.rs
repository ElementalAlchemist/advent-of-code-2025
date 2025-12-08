use std::collections::{HashMap, HashSet};
use std::error::Error;

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
	let mut circuits: Vec<HashSet<Coordinate>> = Vec::new();
	let mut last_connect_x_product: u64 = 0;

	while circuits.len() != 1 || circuits[0].len() != boxes.len() {
		let mut shortest_distance_and_pair: Option<(f64, Coordinate, Coordinate)> = None;
		for (index, coord) in boxes.iter().enumerate() {
			for other_coord in boxes.iter().skip(index + 1) {
				update_shortest(&mut shortest_distance_and_pair, &box_pairs, coord, other_coord);
			}
		}

		if let Some((_, first_coord, second_coord)) = shortest_distance_and_pair {
			let first_x: u64 = first_coord.x.into();
			let second_x: u64 = second_coord.x.into();
			last_connect_x_product = first_x * second_x;

			box_pairs
				.entry(first_coord.clone())
				.or_default()
				.insert(second_coord.clone());
			box_pairs
				.entry(second_coord.clone())
				.or_default()
				.insert(first_coord.clone());

			let first_coord_circuit_index = circuits
				.iter()
				.enumerate()
				.find(|(_, circuit)| circuit.contains(&first_coord))
				.map(|(index, _)| index);
			let second_coord_circuit_index = circuits
				.iter()
				.enumerate()
				.find(|(_, circuit)| circuit.contains(&second_coord))
				.map(|(index, _)| index);

			match (first_coord_circuit_index, second_coord_circuit_index) {
				(Some(first_index), Some(second_index)) if first_index == second_index => (),
				(Some(first_index), Some(second_index)) => {
					let higher_index = first_index.max(second_index);
					let lower_index = first_index.min(second_index);
					let higher_group = circuits.swap_remove(higher_index);
					for coordinate in higher_group {
						circuits[lower_index].insert(coordinate);
					}
				}
				(Some(first_index), None) => {
					circuits[first_index].insert(second_coord);
				}
				(None, Some(second_index)) => {
					circuits[second_index].insert(first_coord);
				}
				(None, None) => {
					let mut new_group: HashSet<Coordinate> = HashSet::new();
					new_group.insert(first_coord);
					new_group.insert(second_coord);
					circuits.push(new_group);
				}
			}
		}
	}

	println!("{last_connect_x_product}");

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
