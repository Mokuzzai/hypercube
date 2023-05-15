use super::*;

use crate::math::Coordinate;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Facing {
	#[default]
	PosZ = 1,
	NegZ = -1,
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
	type Scalar: Coordinate;

	fn from_axis_offset(axis: Axis3, offset: Self::Scalar) -> Self;
	fn from_axis_position(axis: Axis3, position: Point3<Self::Scalar>) -> Self where Self: Sized {
		Self::from_axis_offset(axis, position[axis.axis()])
	}

	/// Converts a point in uv space to world space
	fn transform_point(&self, uv: Point2<Self::Scalar>) -> Point3<Self::Scalar>;
	fn unform_point(&self, point: Point3<Self::Scalar>) -> Point2<Self::Scalar>;

	fn normal(&self) -> Vector3<Self::Scalar>;


	/// Offset this plane along its normal
	fn offset(&mut self, offset: Self::Scalar);

	fn with_offset(mut self, offset: Self::Scalar) -> Self
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
#[derive(serde::Serialize, serde::Deserialize)]
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

use num::FromPrimitive;

impl<T: Plane> FacedTransform<T> {
	pub fn offset(&mut self, offset: T::Scalar) {
		self.transform.offset(offset)
	}

	pub fn with_offset(mut self, offset: T::Scalar) -> Self {
		self.offset(offset);
		self
	}

	pub fn normal(&self) -> Vector3<T::Scalar> {
		use num::NumCast;

		self.transform.normal() * <T::Scalar as NumCast>::from(self.facing as i32).unwrap()
	}

	pub fn transform_point(&self, uv: Point2<T::Scalar>) -> Point3<T::Scalar> {
		self.transform.transform_point(uv)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
/// Axis-aligned plane
pub struct AaPlane3<S> {
	pub axis: Axis3,

	pub offset: S,
}

impl<S: Coordinate + Ord> Plane for AaPlane3<S> {
	type Scalar = S;

	fn transform_point(&self, point: Point2<Self::Scalar>) -> Point3<Self::Scalar> {
		point.coords.insert_row(self.axis.axis(), self.offset).into()
	}
	fn unform_point(&self, point: Point3<Self::Scalar>) -> Point2<Self::Scalar> {
		point.coords.remove_row(self.axis.axis()).into()
	}
	fn normal(&self) -> Vector3<Self::Scalar> {
		Vector2::zeros().insert_row(self.axis.axis(), S::one())
	}
	fn offset(&mut self, offset: Self::Scalar) {
		self.offset += offset
	}
	fn from_axis_offset(axis: Axis3, offset: Self::Scalar) -> Self {
		AaPlane3 {
			axis,
			offset,
		}
	}
	fn from_axis_position(axis: Axis3, position: Point3<Self::Scalar>) -> Self {
		Self::from_axis_offset(axis, position[axis.axis()])
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AaRot3<S> {
	plane: AaPlane3<S>,

	rotation: Rotation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
enum Rotation {
	U(i32),
	V(i32),
}

impl<S: Coordinate + Ord> Plane for AaRot3<S> {
	type Scalar = S;

	fn transform_point(&self, point: Point2<Self::Scalar>) -> Point3<Self::Scalar> {
		todo!()
	}
	fn unform_point(&self, point: Point3<Self::Scalar>) -> Point2<Self::Scalar> {
		todo!()
	}
	fn normal(&self) -> Vector3<Self::Scalar> {
		todo!()
	}
	fn offset(&mut self, offset: Self::Scalar) {
		todo!()
	}
	fn from_axis_offset(axis: Axis3, offset: Self::Scalar) -> Self {
		todo!()
	}
	fn from_axis_position(axis: Axis3, position: Point3<Self::Scalar>) -> Self {
		todo!()
	}
}

#[test]
fn test_facing() {
	let t = FacedTransform::new(AaPlane3::from_axis_offset(Axis3::X, 0), Facing::PosZ);
	let t = t.flipped();

	assert_eq!(t.facing, Facing::NegZ);
}
