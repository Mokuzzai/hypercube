use crate::na;

pub trait Shape<const D: usize> {
	fn new() -> Self;

	fn extents(&self) -> na::Vector<i32, D>;

	fn capacity(&self) -> usize;

	fn position_to_index(&self, position: na::Vector<i32, D>) -> Option<usize>;
	fn index_to_position(&self, index: usize) -> Option<na::Vector<i32, D>>;
}
