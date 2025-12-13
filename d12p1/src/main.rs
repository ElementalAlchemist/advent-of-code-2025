use std::error::Error;

#[derive(Clone, Eq, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

#[derive(Clone, Default)]
struct Present {
	/// The coordinates the present occupies, relative to the present's origin
	coords: Vec<Coordinate>,
	/// The maximum X coordinate of the present
	width: usize,
	/// The maximum Y coordinate of the present
	height: usize,
}

struct Tree {
	/// The width of the tree
	width: usize,
	/// The height of the tree
	height: usize,
	/// The number of each kind of present that must fit under the tree
	present_counts: Vec<usize>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (presents, trees) = {
		let input = include_str!("../input.txt");
		let mut presents: Vec<Present> = Vec::new();
		let mut trees: Vec<Tree> = Vec::new();
		let mut reading_trees = false;

		for line in input.lines() {
			if reading_trees {
				let (dimensions, counts) = line.split_once(": ").expect("tree line has dimensions and presents");
				let (width, height) = dimensions
					.split_once('x')
					.expect("tree dimension has both width and height");
				let width: usize = width.parse()?;
				let height: usize = height.parse()?;
				let mut present_counts = Vec::new();
				for count in counts.split(' ') {
					let count: usize = count.parse()?;
					present_counts.push(count);
				}
				trees.push(Tree {
					width,
					height,
					present_counts,
				});
				continue;
			}
			if line.is_empty() {
				{
					let current_present = presents.last_mut().unwrap();
					current_present.height -= 1;
				}

				// Both the example and the input have 6 shapes.
				if presents.len() == 6 {
					reading_trees = true;
				}
				continue;
			}
			if line.ends_with(':') {
				presents.push(Present::default());
				continue;
			}
			let current_present = presents.last_mut().unwrap();
			let y = current_present.height;
			for (x, c) in line.char_indices() {
				current_present.width = current_present.width.max(x);
				if c == '#' {
					current_present.coords.push(Coordinate { x, y });
				}
			}
			current_present.height += 1;
		}

		(presents, trees)
	};

	// Get an upper bound on the data
	let mut max_feasible_trees = 0;
	for tree in trees {
		let total_area = tree.width * tree.height;
		let mut present_total_area = 0;
		for (present_index, present) in presents.iter().enumerate() {
			present_total_area += present.coords.len() * tree.present_counts[present_index];
		}
		if total_area >= present_total_area {
			max_feasible_trees += 1;
		}
	}
	println!("{max_feasible_trees}");

	Ok(())
}
