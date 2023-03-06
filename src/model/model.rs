use std::collections::BTreeMap;

use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl<T, U> Model3<T, U> {
	pub fn num_quads(&self) -> usize {
		self.transformed_faceless_quads.values().map(PairedQuads::num_quads).sum()
	}
	pub fn clear(&mut self) {
		self.transformed_faceless_quads.values_mut().for_each(PairedQuads::clear)
	}
}

use crate::storage::ReadStorage;
use crate::Shape;

impl<T: Plane, U: Copy + Eq> Model3<T, U> {
	pub fn cull_overlapping_quads(&mut self) {
		for pair in self.transformed_faceless_quads.values_mut() {
			pair.cull_overlapping();
		}
	}
	pub fn cull_overlapping_quads_from_bitmask<V: ReadStorage<usize, Item = bool>, S: Shape<3>>(&mut self, transform: &T, mask: ViewRef<V, S>) {
		let Some(pair) = self.transformed_faceless_quads.get_mut(transform) else { return };

		for index in 0..mask.shape().capacity() {
			let position = mask.shape().index_to_position(index).expect("Ill-formed mask; `Storage` capacity does not match `Shape`s");
			let cull = mask.read(index).expect("Ill-formed mask; `Storage` capacity does not match `Shape`s");

			if cull {
				let quad = Quad::new(transform.unform_point(position));

				pair.iter_mut().for_each(|(_, quads)| quads.cull_occluded_quads(&quad));
			}
		}
	}

}

impl<T: Copy, U> Model3<T, U> {
	pub fn iter(&self) -> impl Iterator<Item = (FacedTransform<T>, &Quads<U>)> {
		self.transformed_faceless_quads.iter().flat_map(|(&t, v)| v.iter().map(move |(f, v)| (FacedTransform::new(t, f), v)))
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
	fn cull_occluded_faces_noop() {
		let mut model = Model3::<AaPlane3>::default();

		model.push_cube(Point3::new(0, 0, 0), ());

		let clone = model.clone();

		model.cull_overlapping_quads();

		assert_eq!(model, clone);
	}

	#[test]
	fn cull_occluded_faces() {
		let mut model = Model3::<AaPlane3>::default();

		model.push_cube(Point3::new(0, 0, 0), ());
		model.push_cube(Point3::new(1, 0, 0), ());

		let clone = model.clone();

		model.cull_overlapping_quads();

		assert_ne!(model, clone);
	}
}
