use crate::na;
use crate::Chunk;
use crate::Shape;

// /// [`Shape`]: A hypercube with `D` dimensions and side length of `S`
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct UniformShape<const S: usize, const D: usize>;

impl<const S: usize, const D: usize> Shape<D> for UniformShape<S, D> {
	// fn new() -> Self {
	// 	UniformShape
	// }

	fn extents(&self) -> na::Vector<usize, D> {
		na::Vector::from_element(S)
	}

	fn capacity(&self) -> usize {
		S.pow(D as u32)
	}

	fn position_to_index(&self, position: na::Vector<i32, D>) -> Option<usize> {
		crate::position_index_conversion::uniform::position_to_index(
			S,
			na::itou(na::vtoa(position))?,
		)
	}
	fn index_to_position(&self, index: usize) -> Option<na::Vector<i32, D>> {
		let src = crate::position_index_conversion::uniform::index_to_position::<D>(S, index)?;

		Some(na::atov(na::utoi(src)?))
	}
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct UniformChunk<T, const S: usize, const D: usize, const C: usize> {
	pub buffer: [T; C],
}

/// # Panics
///
/// if `debug_assertions` are enabled constructors panic if:
/// * `S.pow(D as u32) != C`
/// * `D == 0`
/// * `D > u32::MAX`
impl<T, const S: usize, const D: usize, const C: usize> UniformChunk<T, S, D, C> {
	pub fn new(buffer: [T; C]) -> Self {
		debug_assert_eq!(C, S.pow(D as u32));
		debug_assert_ne!(D, 0);
		debug_assert!(D <= i32::MAX as usize);

		Self { buffer }
	}
	pub fn from_indices(f: impl FnMut(usize) -> T) -> Self {
		Self::new(std::array::from_fn(f))
	}
	pub fn from_positions(mut f: impl FnMut(na::Vector<i32, D>) -> T) -> Self {
		Self::from_indices(|index| f(UniformShape::<S, D>.index_to_position(index).unwrap()))
	}
}

impl<T, const S: usize, const D: usize, const C: usize> Chunk<D> for UniformChunk<T, S, D, C> {
	type Item = T;
	type Shape = UniformShape<S, D>;

	const SHAPE: Self::Shape = UniformShape;

	fn index(&self, index: usize) -> Option<&Self::Item> {
		self.buffer.get(index)
	}
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.buffer.get_mut(index)
	}
}

impl<T, const S: usize, const D: usize, const C: usize> Default for UniformChunk<T, S, D, C>
where
	T: Default,
{
	fn default() -> Self {
		Self::new(std::array::from_fn(|_| Default::default()))
	}
}
