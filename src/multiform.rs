use crate::na;
use crate::Chunk;
use crate::Shape;
use crate::World;

mod macros {
	#![no_implicit_prelude]

	#[macro_export]
	macro_rules! multiform_chunk {
		($vis:vis $Shape:ident[$($N:ident),*; $D:expr], $Chunk:ident) => {


			#[doc = ::std::concat!("[`Shape`]($crate::shape): A hyperrectangle with `", stringify!($D), "` dimensions and sides of lengths ", $("`", stringify!($N), "` "),*)]
			#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
			$vis struct $Shape<$(const $N: usize),*>;

			impl<$(const $N: usize),*> $crate::Shape<$D> for $Shape<$($N),*> {
				fn new() -> Self {
					$Shape
				}
				fn shape(&self) -> $crate::na::Vector<i32, $D> {
					$crate::na::Vector::from([$($N as i32),*])
				}

				fn capacity(&self) -> usize {
					1 $(* ::std::convert::identity::<usize>($N))*
				}

				fn position_to_index(&self, position: $crate::na::Vector<i32, $D>) -> ::std::option::Option<usize> {
					crate::position_index_conversion::multiform::position_to_index(
						[$($N),*],
						$crate::na::itou($crate::na::vtoa(position))?,
					)
				}
				fn index_to_position(&self, index: usize) -> ::std::option::Option<$crate::na::Vector<i32, $D>> {
					let src = crate::position_index_conversion::multiform::index_to_position::<{ $D }>([$($N),*], index)?;

					::std::option::Option::Some($crate::na::atov($crate::na::utoi(src)?))
				}
			}

			#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
			$vis struct $Chunk<T, $(const $N: usize,)* const C: usize> {
				buffer: [T; C]
			}

			impl<T, $(const $N: usize,)* const C: usize> $crate::Chunk<$D> for $Chunk<T, $($N,)* C> {
				type Item = T;
				type Shape = $Shape<$($N),*>;

				fn shape(&self) -> Self::Shape {
					$Shape
				}

				fn index(&self, index: usize) -> ::std::option::Option<&Self::Item> {
					self.buffer.get(index)
				}
				fn index_mut(&mut self, index: usize) -> ::std::option::Option<&mut Self::Item> {
					self.buffer.get_mut(index)
				}
			}
		}
	}
}

// TODO generate generalized chunk for each

crate::multiform_chunk! { pub MultiformShape2[X, Y; 2], MultiformChunk2 }
crate::multiform_chunk! { pub MultiformShape3[X, Y, Z; 3], MultiformChunk3 }
crate::multiform_chunk! { pub MultiformShape4[X, Y, Z, W; 4], MultiformChunk4 }

const CAPACITY: usize = 16 * 16 * 256;

pub type CollumnChunk16x16x256<T> = MultiformChunk3<T, 16, 16, 256, CAPACITY>;

impl<T> CollumnChunk16x16x256<T> {
	pub fn new(buffer: [T; 16 * 16 * 256]) -> Self {
		Self { buffer }
	}
	pub fn from_indices(f: impl FnMut(usize) -> T) -> Self {
		Self::new(std::array::from_fn(f))
	}
	pub fn from_positions(mut f: impl FnMut(na::Vector<i32, 3>) -> T) -> Self {
		Self::from_indices(|index| {
			f(MultiformShape3::<16, 16, 256>
				.index_to_position(index)
				.unwrap())
		})
	}
}

impl<T> Default for CollumnChunk16x16x256<T>
where
	T: Default,
{
	fn default() -> Self {
		Self::new(std::array::from_fn(|_| Default::default()))
	}
}

pub type World2Collumns3<T> = World<CollumnChunk16x16x256<T>, 2, 3>;
