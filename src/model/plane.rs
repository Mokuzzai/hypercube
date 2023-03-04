use super::*;

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

/// Represents a plane in 3d space that has no negative component
pub trait Plane: Ord {
	fn from_axis_offset(axis: Axis3, offset: i32) -> Self;
	fn from_axis_position(axis: Axis3, position: Point3<i32>) -> Self where Self: Sized {
		Self::from_axis_offset(axis, position[axis.axis()])
	}

	/// Converts a point in uv space to world space
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
	pub transform: T,
	pub facing: Facing,
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

impl<T: Plane> FacedTransform<T> {
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
/// Axis-aligned plane
pub struct AaPlane3 {
	axis: Axis3,

	offset: i32,
}

impl Plane for AaPlane3 {
	fn transform_point(&self, point: Point2<i32>) -> Point3<i32> {
		point.coords.insert_row(self.axis.axis(), self.offset).into()
	}
	fn normal(&self) -> Vector3<i32> {
		Vector2::new(0, 0).insert_row(self.axis.axis(), 1)
	}
	fn offset(&mut self, offset: i32) {
		self.offset += offset
	}
	fn from_axis_offset(axis: Axis3, offset: i32) -> Self {
		AaPlane3 {
			axis,
			offset,
		}
	}
	fn from_axis_position(axis: Axis3, position: Point3<i32>) -> Self {
		Self::from_axis_offset(axis, position[axis.axis()])
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AaRot3 {
	plane: AaPlane3,

	rotation: Rotation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Rotation {
	U(i32),
	V(i32),
}

impl Plane for AaRot3 {
	fn transform_point(&self, point: Point2<i32>) -> Point3<i32> {
		todo!()
	}
	fn normal(&self) -> Vector3<i32> {
		todo!()
	}
	fn offset(&mut self, offset: i32) {
		todo!()
	}
	fn from_axis_offset(axis: Axis3, offset: i32) -> Self {
		todo!()
	}
	fn from_axis_position(axis: Axis3, position: Point3<i32>) -> Self {
		todo!()
	}
}
