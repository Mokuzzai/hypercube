use crate::na;

pub trait Shape {
	type Dimension: na::Dim;
}

pub trait IndexableShape: Shape
where
	na::DefaultAllocator: na::Allocator<i32, Self::Dimension>,
{
	fn capacity(&self) -> usize;

	fn position_to_index(&self, position: SVector<Self>) -> Option<usize>;
	fn index_to_position(&self, index: usize) -> Option<SVector<Self>>;
}

/// [`na::OVector`] used to index [`IndexableShape`]
pub type SVector<S> = na::OVector<i32, <S as Shape>::Dimension>;
