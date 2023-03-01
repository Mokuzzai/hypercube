#![allow(unused)]

#[derive(Debug)]
pub struct Quad<T> {
	uv: nalgebra::Point2<i32>,
	wh: nalgebra::Vector2<u32>,

	facing: Facing,

	data: T,
}

pub trait MergeStrategy {
	fn can_merge_with(&self, other: &Self) -> bool;
}

impl<T: MergeStrategy> Quad<T> {
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
	pub fn uv1(&self) -> nalgebra::Point2<i32> {
		self.uv + self.wh.cast()
	}
	pub fn contains_point(&self, point: nalgebra::Point2<i32>) -> bool {
		let a0 = self.uv;
		let a1 = self.uv1();

		let x = a0.x..a1.x;
		let y = a0.y..a1.y;

		x.contains(&point.x)
		&& y.contains(&point.y)
	}
	pub fn try_occluded_by(&self, other: &Self) -> Result<bool, ()> {
		if self.facing != other.facing {
			Err(())
		} else {
			Ok(self.contains_point(other.uv)
			&& self.contains_point(other.uv1()))
		}
	}
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Axis3 {
	X = 0,
	Y = 1,
	Z = 3,
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

#[derive(Debug, Eq, PartialEq)]
pub enum Facing {
	PosZ,
	NeqZ,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Transform3 {
	axis: Axis3,

	coordinate: i32,
}

#[derive(Debug)]
pub struct Model3<T> {
	transformed_quads: std::collections::BTreeMap<Transform3, Vec<Quad<T>>>,
}
