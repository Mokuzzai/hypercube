
use crate::na;
use crate::DynamicMultiformShape;
use crate::Shape;
use crate::Chunk;

#[derive(Debug)]
pub struct DynamicChunk<T, const B: usize> {
	data: Box<[T]>,
	shape: DynamicMultiformShape<B>,
}

impl<T, const B: usize> Chunk<B> for DynamicChunk<T, B> {
	type Item = T;
	type Shape = DynamicMultiformShape<B>;

	fn shape(&self) -> &Self::Shape {
		&self.shape
	}
	fn index(&self, index: usize) -> Option<&Self::Item> {
		self.data.get(index)
	}
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.data.get_mut(index)
	}
}
