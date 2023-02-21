/// `Nalgebra` re-export
pub mod position_index_conversion;
pub mod world_chunk_block_conversion;

pub use position_index_conversion::index_to_position;
pub use position_index_conversion::position_to_index;
pub use world_chunk_block_conversion::chunk_block_to_world;
pub use world_chunk_block_conversion::world_to_chunk_block;

pub use nalgebra;
pub use nalgebra::dimension::Const;
pub use nalgebra::dimension::DimMax;
pub use nalgebra::dimension::DimMaximum;
pub use nalgebra::Dim;
pub use nalgebra::OVector;
pub use nalgebra::Scalar;

pub type Vector<T, const D: usize> = OVector<T, Const<D>>;
pub type Extents<const D: usize> = Vector<usize, D>;
pub type Position<const D: usize> = Vector<i32, D>;

use crate::WorldCoordinate;
