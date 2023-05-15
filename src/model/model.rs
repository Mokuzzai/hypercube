use std::collections::BTreeMap;
use num::One;
use super::*;
use crate::math::Coordinate;
use rayon::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[serde(bound = "T: serde::Serialize + for<'f> serde::Deserialize<'f>,
				 U: serde::Serialize + for<'f> serde::Deserialize<'f>,
				 <T as Plane>::Scalar: serde::Serialize + for<'f> serde::Deserialize<'f>")]
pub struct Model3<T: Plane, U> {
	transformed_faceless_quads: BTreeMap<T, PairedQuads<T::Scalar, U>>,
}

impl<T: Plane, U> Default for Model3<T, U> {
	fn default() -> Self {
		Self {
			transformed_faceless_quads: BTreeMap::new(),
		}
	}
}

impl<T: Plane, U> Model3<T, U> {
	pub fn num_quads(&self) -> usize {
		self.transformed_faceless_quads.values().map(PairedQuads::num_quads).sum()
	}
	pub fn clear(&mut self) {
		self.transformed_faceless_quads.values_mut().for_each(PairedQuads::clear)
	}
}

use crate::storage::ReadStorage;
use crate::Shape;

impl<T: Plane + Send + Sync, U: Clone + PartialEq + Send + Sync> Model3<T, U> {
	pub fn cull_overlapping(&mut self) {
		for pair in self.transformed_faceless_quads.values_mut() {
			pair.cull_overlapping();
		}
	}
	pub fn cull_overlapping_quads_from_bitmask<V: ReadStorage<usize, Item = bool>, S: Shape<3>>(&mut self, transform: &T, mask: ChunkRef<V, S>) {
		let Some(pair) = self.transformed_faceless_quads.get_mut(transform) else { return };

		for index in 0..mask.shape().capacity() {
			let position = mask.shape().index_to_position(index).expect("Ill-formed mask; `Storage` capacity does not match `Shape`s");
			let cull = mask.read(index).expect("Ill-formed mask; `Storage` capacity does not match `Shape`s");

			if cull {
				let quad = Quad::new(transform.unform_point(position.map(crate::math::cast)));

				pair.iter_mut().for_each(|(_, quads)| quads.cull_occluded_quads(&quad));
			}
		}
	}
	pub fn merge_adjacent(&mut self) {
		for paired_quads in self.transformed_faceless_quads.values_mut() {
			for (_, quads) in paired_quads.iter_mut() {
				quads.merge_adjacent()
			}
		}
	}
	pub fn merge_adjacent_axis(&mut self, axis: Axis2) {
		for paired_quads in self.transformed_faceless_quads.values_mut() {
			for (_, quads) in paired_quads.iter_mut() {
				quads.merge_adjacent_axis(axis)
			}
		}
	}
	pub fn push_cull_merge(&mut self, transform: FacedTransform<T>, quad: Quad<T::Scalar, U>, axis: Axis2) {
		self.transformed_faceless_quads
			.entry(transform.transform)
			.or_default()
			.push_cull_merge(transform.facing, quad, axis)
	}
}

impl<T: Plane + Clone, U> Model3<T, U> {
	pub fn iter(&self) -> impl Iterator<Item = (FacedTransform<T>, &Quads<T::Scalar, U>)> {
		self.transformed_faceless_quads.iter().flat_map(|(t, v)| v.iter().map(move |(f, v)| (FacedTransform::new(t.clone(), f), v)))
	}
}


impl<T: Plane, U> Model3<T, U> {
	pub fn push(&mut self, transform: FacedTransform<T>, quad: Quad<T::Scalar, U>) {
		self.transformed_faceless_quads
			.entry(transform.transform)
			.or_default()
			.get_mut(transform.facing)
			.push(quad)
	}
}


impl<T: Plane + Clone, U: Clone> Model3<T, U> {
	pub fn push_cube(&mut self, position: Point3<T::Scalar>, data: U) {
		let mut push_axis = |axis| {
			let transform = T::from_axis_position(axis, position).with_facing(Facing::PosZ);

			let quad = Quad::from_axis_position(axis, position).with_data(data.clone());

			// NOTE: why does this fix our mesh?
			if axis != Axis3::Y {
				// positive quads are 1 unit further along their axis than negative quads
				self.push(transform.clone().with_offset(T::Scalar::one()), quad.clone());
				self.push(transform.clone().flipped(), quad.clone());
			} else {
				// special `Axis3::Y` case which should not be needed
				self.push(transform.clone().with_offset(T::Scalar::one()).flipped(), quad.clone());
				self.push(transform.clone(), quad.clone());
			}
		};

		push_axis(Axis3::X);
		push_axis(Axis3::Y);
		push_axis(Axis3::Z);
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use crate::model::*;
//
// 	#[test]
// 	fn cull_occluded_faces_noop() {
// 		let mut model = Model3::<AaPlane<i32>, ()>::default();
//
// 		model.push_cube(Point3::new(0, 0, 0), ());
//
// 		let clone = model.clone();
//
// 		model.cull_overlapping();
//
// 		assert_eq!(model, clone);
// 	}
//
// 	#[test]
// 	fn cull_occluded_faces() {
// 		let mut model = Model3::<AaPlane3>::default();
//
// 		model.push_cube(Point3::new(0, 0, 0), ());
// 		model.push_cube(Point3::new(1, 0, 0), ());
//
// 		let clone = model.clone();
//
// 		model.cull_overlapping();
//
// 		assert_ne!(model, clone);
// 	}
// }
