use crate::na;

pub trait Shape {
	type Dim: na::Dim;
}

pub trait IndexableShape: Shape
where
	na::DefaultAllocator: na::Allocator<i32, Self::Dim>,
{
	fn capacity(&self) -> usize;

	fn position_to_index(&self, position: SVector<Self>) -> Option<usize>;
	fn index_to_position(&self, index: usize) -> Option<SVector<Self>>;
}

/// [`na::OVector`] used to index [`IndexableShape`]
pub type SVector<S> = na::OVector<i32, SDim<S>>;
pub type SDim<S> = <S as Shape>::Dim;
