use std::collections::BTreeMap;

use super::*;

fn drain_filter<T>(vec: &mut Vec<T>, mut predicate: impl FnMut(&mut T) -> bool, mut callback: impl FnMut(T)) {
	let mut i = 0;

	while i < vec.len() {
		if predicate(&mut vec[i]) {
			let val = vec.remove(i);

			callback(val)
		} else {
			i += 1;
		}
	}
}

#[derive(Debug, Default)]
struct FacingMap<T>([T; 2]);

impl<T> FacingMap<T> {
	fn iter(&self) -> impl Iterator<Item = &T> {
		self.0.iter()
	}
	fn get_mut(&mut self, facing: Facing) -> &mut T {
		&mut self.0[facing as usize]
	}
}

impl<T> FacingMap<Vec<Quad<T>>> {
	fn cull_occluded_faces(&mut self) {
		let [pos, neg] = &mut self.0;

		drain_filter(pos, |pos| neg.iter().any(|neg| pos.contains_quad(neg)), drop);
		drain_filter(neg, |neg| pos.iter().any(|pos| neg.contains_quad(pos)), drop);
	}
}

#[derive(Debug)]
pub struct Model3<T, U = ()> {
	transformed_faceless_quads: BTreeMap<T, FacingMap<Vec<Quad<U>>>>,
}

impl<T, U> Default for Model3<T, U> {
	fn default() -> Self {
		Self {
			transformed_faceless_quads: BTreeMap::new(),
		}
	}
}

impl<T: Plane, U: Copy + Eq> Model3<T, U> {
	pub fn cull_occluded_faces(&mut self) {
		for (_, pair) in self.transformed_faceless_quads.iter_mut() {
			pair.cull_occluded_faces();
		}
	}
}

impl<T: Copy, U> Model3<T, U> {
	pub fn iter(&self) -> impl Iterator<Item = (FacedTransform<T>, &[Quad<U>])> {
		self.transformed_faceless_quads.iter().flat_map(|(&t, v)| v.iter().map(move |v| (FacedTransform::new(t, Facing::PosZ), &**v)))
	}
}


impl<T: Ord, U> Model3<T, U> {
	pub fn push(&mut self, transform: FacedTransform<T>, quad: Quad<U>) {
		self.transformed_faceless_quads
			.entry(transform.transform)
			.or_default()
			.get_mut(transform.facing)
			.push(quad)
	}
}


impl<T: Plane + Copy, U: Copy> Model3<T, U> {
	pub fn push_cube(&mut self, position: Point3<i32>, data: U) {
		let x = T::from_axis_position(Axis3::X, position).with_default_facing();
		let y = T::from_axis_position(Axis3::Y, position).with_default_facing();
		let z = T::from_axis_position(Axis3::Z, position).with_default_facing();

		// positive quads are 1 unit further along their axis than negative quads
		self.push(x.with_offset(1), Quad::from_axis_position(Axis3::X, position, data));
		self.push(y.with_offset(1), Quad::from_axis_position(Axis3::Y, position, data));
		self.push(z.with_offset(1), Quad::from_axis_position(Axis3::Z, position, data));

		self.push(x.flipped(), Quad::from_axis_position(Axis3::X, position, data));
		self.push(y.flipped(), Quad::from_axis_position(Axis3::Y, position, data));
		self.push(z.flipped(), Quad::from_axis_position(Axis3::Z, position, data));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn cull_occluded_faces() {
		let mut model = Model3::<AaPlane3>::default();

		model.push_cube(Point3::new(0, 0, 0), ());



	}
}
