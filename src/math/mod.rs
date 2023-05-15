/// `Nalgebra` re-export
pub mod position_index_conversion;
pub mod world_chunk_block_conversion;
pub mod uniform_position_index_conversion;

pub use position_index_conversion::index_to_position;
pub use position_index_conversion::position_to_index;
pub use world_chunk_block_conversion::chunk_block_to_world;
pub use world_chunk_block_conversion::world_to_chunk_block;

pub use nalgebra;
pub use nalgebra::dimension::Const;
pub use nalgebra::dimension::DimMax;
pub use nalgebra::dimension::DimMaximum;
pub use nalgebra::Dim;

/// A vector with `D` elements
pub type Vector<T, const D: usize> = nalgebra::OVector<T, Const<D>>;
pub use nalgebra::Point;

pub trait Coordinate:
	'static
	+ Copy
	+ PartialOrd
	+ std::fmt::Debug
	+ num::Num
	+ nalgebra::ClosedAdd
	+ nalgebra::ClosedDiv
	+ nalgebra::ClosedMul
	+ nalgebra::ClosedSub
	+ simba::scalar::SupersetOf<usize>
	+ simba::scalar::SupersetOf<i32>
{
}

impl<T> Coordinate for T where
	T: 'static
	+ Copy
	+ PartialOrd
	+ std::fmt::Debug
	+ num::Num
	+ nalgebra::ClosedAdd
	+ nalgebra::ClosedDiv
	+ nalgebra::ClosedMul
	+ nalgebra::ClosedSub
	+ simba::scalar::SupersetOf<usize>
	+ simba::scalar::SupersetOf<i32>
{
}


fn _assert_coordinates() {
	fn _assert_coordinate<T: Coordinate>() {}

	_assert_coordinate::<i32>();
	_assert_coordinate::<f32>();
}

pub fn cast<T, U>(t: T) -> U
where
	U: simba::scalar::SupersetOf<T>,
{
	simba::scalar::SupersetOf::from_subset(&t)
}

pub use simba::scalar::SubsetOf;
pub use simba::scalar::SupersetOf;

// use crate::WorldCoordinate;
use crate::UniformWorldCoordinate;














