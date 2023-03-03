#![allow(unused)]

//! TODO: generalize `Transform3` to a trait to allow non axis-aligned quads

use nalgebra::Point2;
use nalgebra::Point3;
use nalgebra::Vector2;
use nalgebra::Vector3;

use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone)]
pub struct Quad<T = ()> {
	pub position: Point2<i32>,
	pub extents: Vector2<u32>,

	pub data: T,
}

impl Quad<()> {
	pub fn new(uv: Point2<i32>) -> Self {
		Self::with_data(uv, ())
	}

}

impl<T> Quad<T> {
	pub fn drop_data(&self) -> Quad {
		Quad {
			position: self.position,
			extents: self.extents,
			data: (),
		}
	}
	pub fn with_data(uv: Point2<i32>, data: T) -> Self {
		Self {
			data,
			position: uv,
			extents: Vector2::new(1, 1),
		}
	}
	pub fn from_axis_position(axis: Axis3, position: Point3<i32>, data: T) -> Self {
		Self::with_data(position.coords.remove_row(axis as usize).into(), data)
	}
	pub fn uv1(&self) -> Point2<i32> {
		self.position + self.extents.cast()
	}
	pub fn points(&self) -> [Point2<i32>; 4] {
		let u = self.position.x;
		let v = self.position.y;

		let position2 = self.uv1();
		let w = position2.x;
		let h = position2.y;

		[
			Point2::new(u, v),
			Point2::new(u, h),
			Point2::new(w, v),
			Point2::new(w, h),
		]
	}
}

impl<T: Eq> Quad<T> {
	pub fn can_merge_with(&self, other: &Self, axis: Axis2) -> bool {
		let r0 = self.uv1()[axis as usize];
		let r1 = other.position[axis as usize];

		let ah = self.extents[(!axis) as usize];
		let bh = other.extents[(!axis) as usize];

		let edges_touch = r0 == r1;
		let edges_are_same_length = ah == bh;

		edges_touch && edges_are_same_length && self.data == other.data
	}
	pub fn try_merge_with(mut self, other: Self, axis: Axis2) -> Result<Self, (Self, Self)> {
		if self.can_merge_with(&other, axis) {
			self.extents[axis as usize] += other.extents[axis as usize];

			Ok(self)
		} else {
			Err((self, other))
		}
	}
	pub fn contains_point(&self, point: Point2<i32>) -> bool {
		let a0 = self.position;
		let a1 = self.uv1();

		let x = a0.x..a1.x;
		let y = a0.y..a1.y;

		x.contains(&point.x) && y.contains(&point.y)
	}
	pub fn contains_quad<U>(&self, other: &Quad<U>) -> bool {
		self.contains_point(other.position) && self.contains_point(other.uv1())
	}
	// pub fn try_occluded_by(&self, other: &Self) -> Result<bool, ()> {
	// 	if self.facing != other.facing {
	// 		Err(())
	// 	} else {
	// 		Ok(self.contains_point(other.uv) && self.contains_point(other.uv1()))
	// 	}
	// }
	/// # Return
	/// Returns how many values were merged
	pub fn try_merge_sorted_slice<'a>(&mut self, quads: &'a [Self], axis: Axis2) -> usize
	where
		T: Copy,
	{
		let mut iter = quads.iter();
		let mut merge_count = 0;

		while let Some(quad) = iter.next().copied() {
			if self.try_merge_with(quad, axis).is_err() {
				return merge_count;
			} else {
				merge_count += 1;
			}
		}

		return 0;
	}
	pub fn try_merge_sorted_slice_all(mut quads: &[Self], axis: Axis2) -> Vec<Self>
	where
		T: Copy,
	{
		let mut vec = Vec::new();

		let mut iter = quads.iter().copied();

		loop {
			println!("first iteration: {} quads left", quads.len());

			let Some(mut acc) = iter.next() else { break };

			'a: loop {
				if let Some(next) = iter.next() {
					match acc.try_merge_with(next, axis) {
						Ok(a) => acc = a,
						Err((a, b)) => {
							vec.push(a);

							acc = b;

							break 'a;
						}
					}
				} else {
					vec.push(acc);

					break 'a;
				}
			}

			std::thread::sleep(std::time::Duration::from_secs(1));
		}

		dbg!(vec.len());

		vec
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Axis3 {
	X = 0,
	Y = 1,
	Z = 2,
}

#[derive(Debug, Copy, Clone)]
pub enum Axis2 {
	X = 0,
	Y = 1,
}

impl std::ops::Not for Axis2 {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Self::X => Self::Y,
			Self::Y => Self::X,
		}
	}
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Facing {
	#[default]
	PosZ,
	NegZ,
}

impl std::ops::Not for Facing {
	type Output = Self;

	fn not(self) -> Self {
		match self {
			Self::PosZ => Self::NegZ,
			Self::NegZ => Self::PosZ,
		}
	}
}

/// Represents a plane in 3d space
pub trait FacelessPlane: Ord {
	fn from_axis_offset(axis: Axis3, offset: i32) -> Self;
	fn from_axis_position(axis: Axis3, position: Point3<i32>) -> Self where Self: Sized {
		Self::from_axis_offset(axis, position[axis as usize])
	}

	fn transform_point(&self, uv: Point2<i32>) -> Point3<i32>;
	fn normal(&self) -> Vector3<i32>;

	/// Offset this plane along its normal
	fn offset(&mut self, offset: i32);

	fn with_offset(mut self, offset: i32) -> Self
	where
		Self: Sized,
	{
		self.offset(offset);
		self
	}

	fn with_facing(self, facing: Facing) -> FacedTransform<Self> where Self: Sized {
		FacedTransform::new(self, facing)
	}

	fn with_default_facing(self) -> FacedTransform<Self> where Self: Sized {
		FacedTransform::new(self, Facing::default())
	}

}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FacedTransform<T> {
	transform: T,
	facing: Facing,
}

impl<T> FacedTransform<T> {
	pub fn new(transform: T, facing: Facing) -> Self {
		Self { transform, facing }
	}
	pub fn transform(&self) -> &T {
		&self.transform
	}
	pub fn transform_mut(&mut self) -> &mut T {
		&mut self.transform
	}
	pub fn facing(&self) -> &Facing {
		&self.facing
	}
	pub fn facing_mut(&mut self) -> &mut Facing {
		&mut self.facing
	}
	pub fn flip(&mut self) {
		self.facing = !self.facing
	}
	pub fn flipped(mut self) -> Self {
		self.flip();
		self
	}
}

impl<T: FacelessPlane> FacedTransform<T> {
	pub fn offset(&mut self, offset: i32) {
		self.transform.offset(offset)
	}

	pub fn with_offset(mut self, offset: i32) -> Self {
		self.offset(offset);
		self
	}

	pub fn normal(&self) -> Vector3<i32> {
		self.transform.normal() * (self.facing as i32 * 2 - 1)
	}

	pub fn transform_point(&self, uv: Point2<i32>) -> Point3<i32> {
		self.transform.transform_point(uv)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// Axis-aligned transform
pub struct AaTransform3 {
	axis: Axis3,

	offset: i32,
}

impl FacelessPlane for AaTransform3 {
	fn transform_point(&self, point: Point2<i32>) -> Point3<i32> {
		point.coords.insert_row(self.axis as usize, self.offset).into()
	}
	fn normal(&self) -> Vector3<i32> {
		Vector2::new(0, 0).insert_row(self.axis as usize, 1)
	}
	fn offset(&mut self, offset: i32) {
		self.offset += offset
	}
	fn from_axis_offset(axis: Axis3, offset: i32) -> Self {
		AaTransform3 {
			axis,
			offset,
		}
	}
	fn from_axis_position(axis: Axis3, position: Point3<i32>) -> Self {
		Self::from_axis_offset(axis, position[axis as usize])
	}
}

#[derive(Debug)]
pub struct Model3<T, U = ()> {
	pos_z: BTreeMap<T, Vec<Quad<U>>>,
	neg_z: BTreeMap<T, Vec<Quad<U>>>,
}

impl<T, U> Model3<T, U> {
	pub fn faceless_mut(&mut self, facing: Facing) -> &mut BTreeMap<T, Vec<Quad<U>>> {
		if facing == Facing::PosZ {
			&mut self.pos_z
		} else {
			&mut self.neg_z
		}
	}
}

impl<T, U> Default for Model3<T, U> {
	fn default() -> Self {
		Self {
			pos_z: BTreeMap::new(),
			neg_z: BTreeMap::new(),
		}
	}
}

fn filter_map<T>(vec: &mut Vec<T>, mut predicate: impl FnMut(usize, &mut T) -> bool, mut callback: impl FnMut(T)) {
	let mut i = 0;
	while i < vec.len() {
		if predicate(i, &mut vec[i]) {
			let val = vec.remove(i);

			callback(val)
		} else {
			i += 1;
		}
	}
}

impl<T: FacelessPlane, U: Copy + Eq> Model3<T, U> {
	pub fn optimize_cull_touching_faces(&mut self) {
		for (transform, pos_quads) in self.pos_z.iter_mut() {
			let Some(neg_quads) = self.neg_z.get_mut(transform) else { continue; };

			let mut i = 0;

			while i < pos_quads.len(){
				let target = pos_quads[i];

				filter_map(pos_quads, |j, quad| j != i && target.contains_quad(quad), drop);
				filter_map(neg_quads, |_ ,quad| target.contains_quad(quad), drop);

				i += 1;
			}
		}
	}
}

impl<T: Copy, U> Model3<T, U> {
	pub fn iter(&self) -> impl Iterator<Item = (FacedTransform<T>, &[Quad<U>])> {
		self.pos_z.iter().map(|(&t, v)| (FacedTransform::new(t, Facing::PosZ), &**v))
			.chain(self.neg_z.iter().map(|(&t, v)| (FacedTransform::new(t, Facing::NegZ), &**v)))
	}
}


impl<T: Ord, U> Model3<T, U> {
	pub fn push(&mut self, transform: FacedTransform<T>, quad: Quad<U>) {
		self.faceless_mut(transform.facing)
			.entry(transform.transform)
			.or_default()
			.push(quad)
	}
}


impl<T: FacelessPlane + Copy, U: Copy> Model3<T, U> {
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

						let transform = AaTransform3::from_axis_position(axis, position);

						let quad = Quad::from_axis_position(axis, position, ());

						let new_position = transform.transform_point(quad.position);

						assert_eq!(position, new_position);
					}
				}
			}
		}
	}

	#[test]
	fn push_cube() {
		let mut model = Model3::<AaTransform3>::default();

		model.push_cube(Point3::new(0, 0, 0), ());

		let mut quad_vertices = Vec::new();

		for (t, q) in model.iter() {
			let v = q.first().unwrap().points().map(|point| t.transform_point(point)).map(|p| *p.coords.as_ref());

			quad_vertices.push(v);
		}

		#[rustfmt::skip]
		let result = [
			[[1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1]],
			[[0, 1, 0], [0, 1, 1], [1, 1, 0], [1, 1, 1]],
			[[0, 0, 1], [0, 1, 1], [1, 0, 1], [1, 1, 1]],
			[[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1]],
			[[0, 0, 0], [0, 0, 1], [1, 0, 0], [1, 0, 1]],
			[[0, 0, 0], [0, 1, 0], [1, 0, 0], [1, 1, 0]],
		];

		assert_eq!(*quad_vertices, result[..]);
	}

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
