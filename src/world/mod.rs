mod ordered_vector;

use ordered_vector::OrderedVector;

use crate::na;
use crate::Chunk;
use crate::Shape;

use std::collections::BTreeMap;

/// `N` dimensional space containing some [`Chunk`]s
///
/// * `C`: `Chunk`
/// * `E`: dimensions in the world
/// * `V`: dimensions in a chunk
pub struct World<C: Chunk<V>, const E: usize, const V: usize> {
	chunks: BTreeMap<OrderedVector<E>, C>,
}

impl<C: Chunk<V>, const E: usize, const V: usize> World<C, E, V> {
	pub fn new() -> Self {
		Self {
			chunks: BTreeMap::new(),
		}
	}
	pub fn chunk(&self, position: na::Vector<i32, E>) -> Option<&C> {
		self.chunks.get(&OrderedVector::new(position))
	}
	pub fn chunk_mut(&mut self, position: na::Vector<i32, E>) -> Option<&mut C> {
		self.chunks.get_mut(&OrderedVector::new(position))
	}
	pub fn chunk_insert(&mut self, position: na::Vector<i32, E>, chunk: C) -> Option<C> {
		self.chunks.insert(OrderedVector::new(position), chunk)
	}
	pub fn chunk_or_insert_with(&mut self, position: na::Vector<i32, E>, chunk: impl FnMut() -> C) -> &mut C {
		self.chunks
			.entry(OrderedVector::new(position))
			.or_insert_with(chunk)
	}
	pub fn iter(&self) -> impl Iterator<Item = (&na::Vector<i32, E>, &C)> {
		self.chunks.iter().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&na::Vector<i32, E>, &mut C)> {
		self.chunks.iter_mut().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn positions(&self) -> impl Iterator<Item = &na::Vector<i32, E>> {
		self.chunks.keys().map(|a| &a.coordinates)
	}
	pub fn chunks(&self) -> impl Iterator<Item = &C> {
		self.chunks.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut C> {
		self.chunks.values_mut()
	}
}

impl<C: Chunk<V>, const E: usize, const V: usize, const W: usize> World<C, E, V>
where
	na::Const<E>: na::DimMax<na::Const<V>, Output = na::Const<W>>,
{
	pub fn global_to_chunk_subchunk(&self, position: na::Vector<i32, W>) -> (na::Vector<i32, E>, na::Vector<i32, V>) {
		let chunk_shape = <C::Shape as Shape<V>>::new().shape();

		let chunk_shape_as_global = chunk_shape.resize_generic(na::Const::<W>, na::Const::<1>, 0);


		// this subchunk might be negative and if it is it should be inversed
		let mut subchunk_as_global = position.zip_map(&chunk_shape_as_global, std::ops::Rem::rem);

		for (value, &extent) in subchunk_as_global.iter_mut().zip(chunk_shape_as_global.iter()) {
			*value = (*value + extent) % extent
		}

		let chunk_as_global = position.zip_map(&chunk_shape_as_global, std::ops::Div::div);

		let subchunk = subchunk_as_global.resize_generic(na::Const::<V>, na::Const::<1>, 0);
		let chunk = chunk_as_global.resize_generic(na::Const::<E>, na::Const::<1>, 0);

		(chunk, subchunk)
	}
	pub fn global_to_chunk(&self, position: na::Vector<i32, W>) -> na::Vector<i32, E> {
		self.global_to_chunk_subchunk(position).0
	}
	pub fn global_to_subchunk(&self, position: na::Vector<i32, W>) -> na::Vector<i32, V> {
		self.global_to_chunk_subchunk(position).1
	}
	pub fn subchunk(&mut self, position: na::Vector<i32, W>) -> Option<&C::Item> {
		let (chunk, subchunk) = self.global_to_chunk_subchunk(position);

		self.chunk(chunk)?.get(subchunk)
	}
	pub fn subchunk_mut(&mut self, position: na::Vector<i32, W>) -> Option<&mut C::Item> {
		let (chunk, subchunk) = self.global_to_chunk_subchunk(position);

		self.chunk_mut(chunk)?.get_mut(subchunk)
	}
}

impl<C: Chunk<V>, const E: usize, const V: usize> Default for World<C, E, V> {
	fn default() -> Self {
		Self::new()
	}
}

const _: () = {
	use std::fmt::*;

	impl<C: Chunk<V>, const E: usize, const V: usize> Debug for World<C, E, V>
	where
		C: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("Wolrd")
				.field("chunks", &self.chunks)
				.finish()
		}
	}
};

#[cfg(test)]
mod tests {
	// how do i test this??

	// use super::*;
	// use crate::multiform::World2Collumns3;
	// use crate::multiform::CollumnChunk16x16x256;
 //
	// #[test]
	// fn test_global_to_chunk_subchunk_1() {
	// 	std::thread::Builder::new()
	// 		.name(module_path!().into())
	// 		.stack_size(2usize.pow(24) * 2)
	// 		.spawn(|| {
	// 			let mut world = World2Collumns3::<na::Vector<i32, 3>>::new();
 //
	// 			world.chunk_insert(na::Vector::from([0, 0]), CollumnChunk16x16x256::from_fn(std::convert::identity));
 //
	// 			for z in 0..256 {
	// 				for y in 0..16{
	// 					for x in 0..16 {
	// 						let (chunk, subchunk) = world.global_to_chunk_subchunk(na::Vector::from([x, y, z]));
 //
	// 						eprintln!("{:?} {:?}", chunk, subchunk);
	// 					}
	// 				}
	// 			}
	// 		})
	// 		.expect("failed to spawn thread")
	// 		.join()
	// 		.unwrap();
	// }
}


