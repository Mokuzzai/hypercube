
use crate::na;
use crate::Chunk;

pub trait Shape {
	type Dimension: na::Dim;
	type Coordinate;
}

pub trait IndexableShape: Shape {
	fn position_to_index(&self, position: SVector<Self>) -> Option<usize>
	where
		na::DefaultAllocator: na::Allocator<Self::Coordinate, Self::Dimension>;
	fn index_to_position(&self, index: usize) -> Option<SVector<Self>>
	where
		na::DefaultAllocator: na::Allocator<Self::Coordinate, Self::Dimension>;
}

pub type SVector<S> = na::OVector<<S as Shape>::Coordinate, <S as Shape>::Dimension>;
