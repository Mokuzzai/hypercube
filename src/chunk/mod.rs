
use crate::nd;
use crate::na;

pub mod ctx;

pub trait Chunk<const B: usize> {
	type Block;

	fn array(&self) -> &nd::Array<Self::Block, B>;
	fn array_mut(&mut self) -> &mut nd::Array<Self::Block, B>;


	fn get(&self, position: na::Vector<i32, B>) -> Option<&Self::Block>
	where
		[usize; B]: nd::Dimension,
	{
		let position: na::Vector<usize, B> = position.try_cast()?;
		let position: [usize; B] = position.into();

		self.array().get(position)
	}
	fn get_mut(&mut self, position: na::Vector<i32, B>) -> Option<&mut Self::Block>
	where
		[usize; B]: nd::Dimension,
	{
		let position: na::Vector<usize, B> = position.try_cast()?;
		let position: [usize; B] = position.into();

		self.array_mut().get_mut(position)
	}
}

pub struct DefaultChunk<T, const B: usize> {
	array: nd::Array<T, B>,
}

impl<T, const B: usize> Chunk<B> for DefaultChunk<T, B> {
	type Block = T;

	fn array(&self) -> &nd::Array<Self::Block, B> { &self.array }
	fn array_mut(&mut self) -> &mut nd::Array<Self::Block, B> { &mut self.array }
}
