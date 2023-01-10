use crate::na;

use crate::Shape;
use crate::SVector;

use crate::Chunk;

pub struct World<S: Shape, C: Chunk>
where
	na::DefaultAllocator: na::Allocator<S::Coordinate, S::Dimension>,
{
	chunks: std::collections::BTreeMap<SVector<S>, C>,
}




