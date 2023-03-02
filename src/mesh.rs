#![allow(unused)]

//! TODO: generalize `Transform3` to a trait to allow non axis-aligned quads

use nalgebra::Point2;
use nalgebra::Point3;
use nalgebra::Vector2;


pub trait MergeStrategy {
	type Strategy: CanMergeWith;

	fn get_merge_strategy(&self) -> Option<Self::Strategy>;
}

pub trait CanMergeWith {
	fn can_merge_with(&self, other: &Self) -> bool;
}

impl CanMergeWith for () {
	fn can_merge_with(&self, other: &Self) -> bool {
		true
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Quad<T = ()> {
	uv: Point2<i32>,
	wh: Vector2<u32>,

	data: T,
}

impl<T> Quad<T> {
	pub fn from_axis_position(axis: Axis3, position: Point3<i32>, data: T) -> Self {
		Self {
			uv: position.coords.remove_row(axis as usize).into(),
			wh: Vector2::new(1, 1),
			data,
		}
	}
}

impl<T: CanMergeWith> Quad<T> {
	pub fn can_merge_with(&self, other: &Self, axis: Axis2) -> bool {
		let r0 = self.uv1()[axis as usize];
		let r1 = other.uv[axis as usize];

		let ah = self.wh[(!axis) as usize];
		let bh = other.wh[(!axis) as usize];

		let edges_touch = r0 == r1;
		let edges_are_same_length = ah == bh;

		edges_touch && edges_are_same_length && self.data.can_merge_with(&other.data)
	}
	pub fn try_merge_with(&mut self, other: &Self, axis: Axis2) -> Result<(), ()> {
		if self.can_merge_with(other, axis) {
			self.wh[axis as usize] += other.wh[axis as usize];

			Ok(())
		} else {
			Err(())
		}
	}
	pub fn uv1(&self) -> Point2<i32> {
		self.uv + self.wh.cast()
	}
	pub fn contains_point(&self, point: Point2<i32>) -> bool {
		let a0 = self.uv;
		let a1 = self.uv1();

		let x = a0.x..a1.x;
		let y = a0.y..a1.y;

		x.contains(&point.x) && y.contains(&point.y)
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
	pub fn try_merge_sorted_slice<'a>(&mut self, quads: &'a [Self], axis: Axis2) -> usize {
		let mut iter = quads.iter();
		let mut merge_count = 0;

		while let Some(quad) = iter.next() {
			if self.try_merge_with(quad, axis).is_err() {
				return merge_count;
			} else {
				merge_count += 1;
			}
		}

		return 0
	}
	pub fn try_merge_sorted_slice_all(mut quads: &[Self], axis: Axis2) -> Vec<Self>
	where
		T: Copy,
	{
		let mut vec = Vec::new();

		while let [mut first, ref rest @ ..] = *quads {
			let merge_count = first.try_merge_sorted_slice(rest, axis);

			vec.push(first);

			quads = &quads[merge_count..];
		}

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Facing {
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
pub trait Plane {
	fn transform_point(point: Point2<i32>) -> Point3<i32>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// Axis-aligned transform
pub struct Transform3 {
	facing: Facing,

	axis: Axis3,

	coordinate: i32,
}

impl std::ops::Add<i32> for Transform3 {
	type Output = Self;

	fn add(mut self, offset: i32) -> Self {
		self.coordinate += offset;
		self
	}
}

impl std::ops::Not for Transform3 {
	type Output = Self;

	fn not(mut self) -> Self {
		self.facing = !self.facing;
		self
	}
}

impl Transform3 {
	pub fn from_axis_position(axis: Axis3, position: Point3<i32>) -> Self {
		Transform3 { facing: Facing::PosZ, axis, coordinate: position[axis as usize] }
	}
}

#[derive(Debug)]
pub struct Model3<T> {
	transformed_quads: std::collections::BTreeMap<Transform3, Vec<Quad<T>>>, // NOTE: alternatively use `BTreeSet` but `Vec` is probably faster
}

impl<T> Default for Model3<T> {
	fn default() -> Self {
		Self {
			transformed_quads: std::collections::BTreeMap::new(),
		}
	}
}

impl<T> Model3<T> {
	pub fn push(&mut self, transform: Transform3, quad: Quad<T>) {
		self.transformed_quads.entry(transform).or_default().push(quad)
	}
}
impl<T> Model3<T>
where
	T: Copy + CanMergeWith,
{
	pub fn optimize_merge_quads(&mut self, axis: Axis2) {
		for quads in self.transformed_quads.values_mut() {
			quads.sort_unstable_by_key(|quad| (quad.uv.x, quad.uv.y));

			let new_quads = Quad::try_merge_sorted_slice_all(quads, axis);

			#[cfg(test)]
			{
				let src = quads.len();
				let dst = new_quads.len();
				let delta = src - dst;

				let percent = dst as f32 / src as f32;

				if delta != 0 {
					eprintln!("along axis `{:?}`: optimized from `{}` quads to `{}`, total `{}` quads removed, approx. `{}`%", axis, src, dst, delta, percent);
				}
			}
		}
	}
}

impl<T: Copy> Model3<T> {
	pub fn push_cube(&mut self, position: Point3<i32>, data: T) {
		let x = Transform3::from_axis_position(Axis3::X, position);
		let y = Transform3::from_axis_position(Axis3::Y, position);
		let z = Transform3::from_axis_position(Axis3::Z, position);

		// positive quads are 1 unit further along their axis than negative quads
		self.push(x + 1, Quad::from_axis_position(x.axis, position, data));
		self.push(y + 1, Quad::from_axis_position(y.axis, position, data));
		self.push(z + 1, Quad::from_axis_position(z.axis, position, data));

		self.push(!x, Quad::from_axis_position(x.axis, position, data));
		self.push(!y, Quad::from_axis_position(y.axis, position, data));
		self.push(!z, Quad::from_axis_position(z.axis, position, data));
	}
}

use crate::Shape;
use crate::prelude3::ViewRef;
use crate::storage::ContiguousMemory;

pub fn generate_quads<'a, T: ContiguousMemory, S: Shape<3>, M: Copy>(chunk: ViewRef<'a, T, S>) -> Model3<M>
where
	T::Item: MergeStrategy<Strategy = M>,
{
	let mut model = Model3::default();

	for (position, block) in chunk.block_positions() {
		let Some(merge_strategy) = block.get_merge_strategy() else { continue };

		model.push_cube(position, merge_strategy)
	}

	model
}

#[cfg(test)]
mod tests {
	use crate::prelude3::View;
	use crate::prelude3::ct::Uniform;

	use super::*;

	use std::hint::black_box;

	#[derive(Copy, Clone)]
	struct Block(bool);

	impl MergeStrategy for Block {
		type Strategy = ();

		fn get_merge_strategy(&self) -> Option<Self::Strategy> {
			self.0.then_some(())
		}
	}

	impl CanMergeWith for Block {
		fn can_merge_with(&self, other: &Self) -> bool {
			self.0 == other.0
		}
	}

	#[test]
	fn bench() {
		let chunk = black_box(View::<Box<[_]>, Uniform<32>>::from_index(|_| Block(true)));

		let mut model = black_box(generate_quads(chunk.borrow()));

		model.optimize_merge_quads(Axis2::X);
		model.optimize_merge_quads(Axis2::Y);
	}
}







