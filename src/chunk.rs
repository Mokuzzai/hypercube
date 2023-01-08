pub type Chunk16d2<T> = Chunk<T, u8, 16, 2, 256>;
pub type Chunk16d3<T> = Chunk<T, u8, 16, 3, 4096>;
pub type Chunk32d2<T> = Chunk<T, u8, 32, 2, 1024>;
pub type Chunk32d3<T> = Chunk<T, u8, 32, 3, 32768>;

use crate::Positions;

use crate::Coordinate;
use crate::SubChunkPosition;

use std::marker::PhantomData;
use std::mem::MaybeUninit;

pub struct Chunk<T, U: Coordinate, const S: usize, const D: usize, const C: usize> {
	inner: [T; C],
	_coord: PhantomData<U>,
}

/// # Constructors
///
/// ## Panics
/// All constructors panic if the following conditions are not met:
/// * `S.pow(D as u32) == C`
/// * `D < u32::MAX`
impl<T, U: Coordinate, const S: usize, const D: usize, const C: usize> Chunk<T, U, S, D, C> {
	pub fn from_array(inner: [T; C]) -> Self {
		assert!(S.pow(D as u32) == C);
		assert!(D < u32::MAX as usize);

		Self {
			inner,
			_coord: PhantomData,
		}
	}

	pub fn from_fn(mut f: impl FnMut(SubChunkPosition<U, D>) -> T) -> Self {
		let mut positions = Positions::<U, S, D, C>::new();

		Self::from_array(std::array::from_fn(|_| f(positions.next().unwrap())))
	}

	pub fn new_uninit() -> Chunk<MaybeUninit<T>, U, S, D, C> {
		// SAFETY: An uninitialized `[MaybeUninit<_>; C]` is valid.
		let array = unsafe { MaybeUninit::<[MaybeUninit<T>; C]>::uninit().assume_init() };

		Chunk::from_array(array)
	}
}

impl<T, U: Coordinate, const S: usize, const D: usize, const C: usize> Chunk<T, U, S, D, C> {
	pub const STRIDE: usize = S;
	pub const DIMENSIONS: usize = D;
	pub const CAPACITY: usize = C;

	pub fn stride(&self) -> usize {
		S
	}

	pub fn dimensions(&self) -> usize {
		D
	}

	pub fn capacity(&self) -> usize {
		C
	}

	pub fn get(&self, position: SubChunkPosition<U, D>) -> Option<&T> {
		self.inner.get(position.to_index(S)?)
	}

	pub fn get_mut(&mut self, position: SubChunkPosition<U, D>) -> Option<&mut T> {
		self.inner.get_mut(position.to_index(S)?)
	}

	pub fn into_iter(self) -> IntoIter<T, U, S, D, C> {
		IntoIter {
			values: self.inner.into_iter(),
			positions: Positions::new(),
		}
	}

	pub fn iter(&self) -> Iter<T, U, S, D, C> {
		Iter {
			values: self.inner.iter(),
			positions: Positions::new(),
		}
	}

	pub fn iter_mut(&mut self) -> IterMut<T, U, S, D, C> {
		IterMut {
			values: self.inner.iter_mut(),
			positions: Positions::new(),
		}
	}
}

impl<T, U: Coordinate, const S: usize, const D: usize, const C: usize>
	Chunk<MaybeUninit<T>, U, S, D, C>
{
	/// # Safety
	/// It is up to the caller to ensure that `Self` is initialized fully
	pub unsafe fn assume_init(self) -> Chunk<T, U, S, D, C> {
		let forget = std::mem::ManuallyDrop::new(self);

		let array = std::ptr::addr_of!(forget.inner).cast::<[T; C]>().read();

		Chunk::from_array(array)
	}
}

pub struct IntoIter<T, U: Coordinate, const S: usize, const D: usize, const C: usize> {
	values: std::array::IntoIter<T, C>,
	positions: Positions<U, S, D, C>,
}

impl<T, U: Coordinate, const S: usize, const D: usize, const C: usize> Iterator
	for IntoIter<T, U, S, D, C>
{
	type Item = (SubChunkPosition<U, D>, T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}

pub struct Iter<'a, T, U: Coordinate, const S: usize, const D: usize, const C: usize> {
	values: std::slice::Iter<'a, T>,
	positions: Positions<U, S, D, C>,
}

impl<'a, T, U: Coordinate, const S: usize, const D: usize, const C: usize> Iterator
	for Iter<'a, T, U, S, D, C>
{
	type Item = (SubChunkPosition<U, D>, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}

pub struct IterMut<'a, T, U: Coordinate, const S: usize, const D: usize, const C: usize> {
	values: std::slice::IterMut<'a, T>,
	positions: Positions<U, S, D, C>,
}

impl<'a, T, U: Coordinate, const S: usize, const D: usize, const C: usize> Iterator
	for IterMut<'a, T, U, S, D, C>
{
	type Item = (SubChunkPosition<U, D>, &'a mut T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}
