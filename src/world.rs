use crate::na;

use crate::SVector;
use crate::Shape;

use crate::Chunk;

/// How many dimensions does you [`World`] have?
pub struct WorldShape<const D: usize>;

impl<const D: usize> Shape for WorldShape<D> {
	type Dimension = na::Const<D>;
	type Coordinate = i32;
}

/// `N` dimensional space containing some [`Chunk`]s
pub struct World<S: Shape, C: Chunk>
where
	na::DefaultAllocator: na::Allocator<S::Coordinate, S::Dimension>,
{
	chunks: std::collections::BTreeMap<SVector<S>, C>,
}
