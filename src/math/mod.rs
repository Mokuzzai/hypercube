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
pub type Matrix<T, const N: usize, const M: usize> = nalgebra::OMatrix<T, Const<N>, Const<M>>;
pub use nalgebra::Point;

pub trait Coordinate:
	'static
	+ Copy
	+ PartialOrd
	+ std::fmt::Debug
	+ num::Num
	+ num::NumCast
	+ nalgebra::ClosedAdd
	+ nalgebra::ClosedDiv
	+ nalgebra::ClosedMul
	+ nalgebra::ClosedSub
{
}

impl<T> Coordinate for T where
	T: 'static
	+ Copy
	+ PartialOrd
	+ std::fmt::Debug
	+ num::Num
	+ num::NumCast
	+ nalgebra::ClosedAdd
	+ nalgebra::ClosedDiv
	+ nalgebra::ClosedMul
	+ nalgebra::ClosedSub
{
}

pub fn matrix_cast<T: Coordinate, U: Coordinate, const N: usize, const M: usize>(v: Matrix<T, N, M>) -> Option<Matrix<U, N, M>> {
	let mut out = Matrix::zeros();

	if false { return Some(out) }

	for (slot, &scalar) in out.iter_mut().zip(v.iter()) {
		*slot = U::from(scalar)?;
	}

	Some(out)
}

pub fn point_cast<T: Coordinate, U: Coordinate, const N: usize>(p: Point<T, N>) -> Option<Point<U, N>> {
	matrix_cast(p.coords).map(Into::into)
}

pub use simba::scalar::SubsetOf;
pub use simba::scalar::SupersetOf;

// use crate::WorldCoordinate;
use crate::UniformWorldCoordinate;














