use crate::na;
use crate::Chunk;
use crate::DynamicUniformShape;
use crate::Shape;

// /// [`Shape`]: A hypercube with `D` dimensions and side length of `S`
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct UniformShape<const S: usize, const D: usize>;

impl<const S: usize, const B: usize> Shape<B> for UniformShape<S, B> {
	fn extents(&self) -> na::Vector<usize, B> {
		na::Vector::from_element(S)
	}
	fn capacity(&self) -> usize {
		S.pow(B as u32)
	}
	fn position_to_index(&self, block: na::Vector<i32, B>) -> Option<usize> {
		DynamicUniformShape::<B>::new(S).position_to_index(block)
	}
	fn index_to_position(&self, index: usize) -> Option<na::Vector<i32, B>> {
		DynamicUniformShape::<B>::new(S).index_to_position(index)
	}
}

impl<const S: usize, const B: usize> crate::UniformShape<B> for UniformShape<S, B> {
	fn stride(&self) -> usize {
		S
	}
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct UniformChunk<T, const S: usize, const B: usize, const C: usize> {
	pub buffer: [T; C],
}

/// # Panics
///
/// if `debug_assertions` are enabled constructors panic if:
/// * `S.pow(D as u32) != C`
/// * `B == 0`
/// * `D > u32::MAX`
impl<T, const S: usize, const B: usize, const C: usize> UniformChunk<T, S, B, C> {
	pub fn new(buffer: [T; C]) -> Self {
		debug_assert_eq!(C, S.pow(B as u32));
		debug_assert_ne!(B, 0);
		debug_assert!(B <= i32::MAX as usize);

		Self { buffer }
	}
	pub fn from_indices(f: impl FnMut(usize) -> T) -> Self {
		Self::new(std::array::from_fn(f))
	}
	pub fn from_positions(mut f: impl FnMut(na::Vector<i32, B>) -> T) -> Self {
		Self::from_indices(|index| f(UniformShape::<S, B>.index_to_position(index).unwrap()))
	}
}

impl<T, const S: usize, const B: usize, const C: usize> Chunk<B> for UniformChunk<T, S, B, C> {
	type Item = T;
	type Shape = UniformShape<S, B>;

	fn shape(&self) -> &Self::Shape {
		&UniformShape
	}

	fn index(&self, index: usize) -> Option<&Self::Item> {
		self.buffer.get(index)
	}
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.buffer.get_mut(index)
	}
}

impl<T, const S: usize, const B: usize, const C: usize> Default for UniformChunk<T, S, B, C>
where
	T: Default,
{
	fn default() -> Self {
		Self::new(std::array::from_fn(|_| Default::default()))
	}
}
