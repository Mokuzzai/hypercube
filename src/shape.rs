use crate::na;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WorldCoordinate<const C: usize, const B: usize> {
	pub chunk: na::Vector<i32, C>,
	pub block: na::Vector<i32, B>
}

pub trait Shape<const B: usize> {
	fn new() -> Self;

	fn extents(&self) -> na::Vector<i32, B>;

	fn capacity(&self) -> usize;

	fn position_to_index(&self, sublocal: na::Vector<i32, B>) -> Option<usize>;
	fn index_to_position(&self, index: usize) -> Option<na::Vector<i32, B>>;

	fn world_to_chunk_block<const W: usize, const C: usize>(
		&self,
		world: na::Vector<i32, W>,
	) -> WorldCoordinate<C, B>
	where
		Self: Sized,
		na::Const<C>: na::DimMax<na::Const<B>, Output = na::Const<W>>,
	{
		let chunk_shape = Self::new().extents();

		let chunk_shape_as_global = chunk_shape.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		// this subchunk might be negative and if it is it should be inversed
		let mut block_as_global = world.zip_map(&chunk_shape_as_global, std::ops::Rem::rem);

		for (value, &extent) in block_as_global
			.iter_mut()
			.zip(chunk_shape_as_global.iter())
		{
			*value = (*value + extent) % extent
		}

		let chunk_as_global = world.zip_map(&chunk_shape_as_global, std::ops::Div::div);

		let chunk = chunk_as_global.resize_generic(na::Const::<C>, na::Const::<1>, 0);
		let block = block_as_global.resize_generic(na::Const::<B>, na::Const::<1>, 0);

		WorldCoordinate { chunk, block }
	}

	fn chunk_block_to_world<const W: usize, const C: usize, >(&self, chunk: na::Vector<i32, C>, block: na::Vector<i32, B>) -> na::Vector<i32, W>
	where
		Self: Sized,
		na::Const<C>: na::DimMax<na::Const<B>, Output = na::Const<W>>,
	{
		let chunk_shape = Self::new().extents();

		let chunk_shape_as_global = chunk_shape.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		let chunk_as_global = chunk.resize_generic(na::Const::<W>, na::Const::<1>, 0);
		let block_as_global = block.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		chunk_as_global + block_as_global.component_mul(&chunk_shape_as_global)
	}
}

