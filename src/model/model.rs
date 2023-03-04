use std::collections::BTreeMap;

use super::*;

#[derive(Debug)]
pub struct Model3<T, U = ()> {
	transformed_faceless_quads: BTreeMap<T, PairedQuads<U>>,
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
		for pair in self.transformed_faceless_quads.values_mut() {
			pair.cull_overlapping();
		}
	}
}

impl<T: Copy, U> Model3<T, U> {
	pub fn iter(&self) -> impl Iterator<Item = (FacedTransform<T>, &Quads<U>)> {
		self.transformed_faceless_quads.iter().flat_map(|(&t, v)| v.iter().map(move |v| (FacedTransform::new(t, Facing::PosZ), v)))
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
