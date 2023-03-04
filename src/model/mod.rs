#![allow(unused)]

//! TODO: generalize `Transform3` to a trait to allow non axis-aligned quads

pub mod model;
pub mod plane;
pub mod quad;

pub use quad::Quad;
pub use plane::Facing;
pub use plane::FacedTransform;
pub use plane::Plane;
pub use plane::AaPlane3;
pub use model::Model3;

use nalgebra::Point2;
use nalgebra::Point3;
use nalgebra::Vector2;
use nalgebra::Vector3;

type Axis2 = Axis<2>;
type Axis3 = Axis<3>;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Axis<const D: usize> {
	axis: usize,
}

impl<const D: usize> Axis<D> {
	pub const fn new(axis: usize) -> Self {
		assert!(axis < D);

		Self { axis }
	}
	pub fn axis(self) -> usize {
		self.axis
	}
}

impl Axis<0> {}

impl Axis<1> {
	const X: Self = Self::new(0);
}

impl Axis<2> {
	const X: Self = Self::new(0);
	const Y: Self = Self::new(1);
}

impl Axis<3> {
	const X: Self = Self::new(0);
	const Y: Self = Self::new(1);
	const Z: Self = Self::new(2);
}

impl std::ops::Not for Axis2 {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Self::X => Self::Y,
			Self::Y => Self::X,
			_ => unreachable!(),
		}
	}
}

use crate::prelude3::ViewRef;
use crate::storage::ContiguousMemory;
use crate::Shape;

#[cfg(test)]
mod tests {
	use crate::prelude3::ct::Uniform;
	use crate::prelude3::View;

	use super::*;

	use std::hint::black_box;

	#[derive(Copy, Clone)]
	struct Block(bool);

	#[test]
	fn quad_points() {
		let mut quad = Quad::new(Point2::new(3, 5));
		quad.extents.x = 2;
		quad.extents.y = 3;

		assert_eq!(quad.points(), [
			Point2::new(3, 5),
			Point2::new(3, 8),
			Point2::new(5, 5),
			Point2::new(5, 8),
		])
	}

	#[test]
	fn transform_point() {
		for z in -10..10 {
			for y in -10..10 {
				for x in -10..10 {
					for axis in [Axis3::X, Axis3::Y, Axis3::Z] {
						let position = Point3::new(x, y, z);

						let transform = AaPlane3::from_axis_position(axis, position);

						let quad = Quad::from_axis_position(axis, position, ());

						let new_position = transform.transform_point(quad.position);

						assert_eq!(position, new_position);
					}
				}
			}
		}
	}

// 	#[test]
// 	fn push_cube() {
// 		let mut model = Model3::<AaPlane3>::default();
//
// 		model.push_cube(Point3::new(0, 0, 0), ());
//
// 		let mut quad_vertices = Vec::new();
//
// 		for (t, q) in model.iter() {
// 			let Some(v) = q.first() else { continue };
//
// 			let v = v.points().map(|point| t.transform_point(point)).map(|p| *p.coords.as_ref());
//
// 			quad_vertices.push(v);
// 		}
//
// 		#[rustfmt::skip]
// 		let result = [
// 			[[1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1]],
// 			[[0, 1, 0], [0, 1, 1], [1, 1, 0], [1, 1, 1]],
// 			[[0, 0, 1], [0, 1, 1], [1, 0, 1], [1, 1, 1]],
// 			[[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1]],
// 			[[0, 0, 0], [0, 0, 1], [1, 0, 0], [1, 0, 1]],
// 			[[0, 0, 0], [0, 1, 0], [1, 0, 0], [1, 1, 0]],
// 		];
//
// 		assert_eq!(*quad_vertices, result[..]);
// 	}

	// #[test]
	// fn try_merge_sorted_with_x() {
	// 	let mut a = Quad::new(Point2::new(0, 0));
	// 	let b = Quad::new(Point2::new(1, 0));
 //
	// 	a.try_merge_with(b, Axis2::X).unwrap();
 //
	// 	assert_eq!(a.extents, Vector2::new(2, 1));
	// }
 //
	// #[test]
	// fn try_merge_sorted_with_y() {
	// 	let mut a = Quad::new(Point2::new(0, 0));
	// 	let b = Quad::new(Point2::new(0, 1));
 //
	// 	a.try_merge_with(b, Axis2::Y).unwrap();
 //
	// 	assert_eq!(a.extents, Vector2::new(1, 2));
	// }
 //
	// #[test]
	// fn try_merge_sorted_with_fail() {
	// 	let mut a = Quad::new(Point2::new(0, 0));
	// 	let b = Quad::new(Point2::new(2, 0));
 //
	// 	assert!(a.try_merge_with(b, Axis2::X).is_err());
	// }

	// #[test]
	// fn try_merge_sorted_slice_all() {
	// 	let mut a = Quad::new(Point2::new(0, 0));
	// 	let mut b = [
	// 		Quad::new(Point2::new(1, 0)),
	// 		Quad::new(Point2::new(1, 1)),
	// 		Quad::new(Point2::new(0, 1)),
	// 	];
 //
	// 	b.sort_unstable_by_key(|quad| *quad.position.coords.as_ref());
 //
	// 	let merged = Quad::try_merge_sorted_slice_all(&b, Axis2::Y);
	// 	let merged = Quad::try_merge_sorted_slice_all(&merged, Axis2::X);
 //
	// 	eprintln!("{:?}", merged[0]);
 //
	// 	assert_eq!(merged[0].position, Point2::new(0, 0));
	// 	assert_eq!(merged[0].extents, Vector2::new(2, 2));
	// }

	// WARNING:
	// #[test]
	// fn bench() {
	// 	let chunk = black_box(View::<Box<[_]>, Uniform<3>>::from_index(|_| Block(true)));
	//
	// 	let mut model = black_box(generate_quads(chunk.borrow()));
	//
	// 	model.optimize_merge_quads(Axis2::X);
	// 	// model.optimize_merge_quads(Axis2::Y);
	// }
}
