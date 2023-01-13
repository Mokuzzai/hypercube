use crate::Chunk;
use crate::World;
use crate::na;
use crate::Shape;

mod macros {
	#![no_implicit_prelude]

	#[macro_export]
	macro_rules! multiform_shape {
		($vis:vis $Shape:ident[$($N:ident),*; $D:expr]) => {


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

				fn position_to_index(&self, position: $crate::na::Vector<i32, $D>) -> Option<usize> {
					crate::position_index_conversion::multiform::position_to_index(
						[$($N),*],
						$crate::na::itou($crate::na::vtoa(position))?,
					)
				}
				fn index_to_position(&self, index: usize) -> Option<$crate::na::Vector<i32, $D>> {
					let src = crate::position_index_conversion::multiform::index_to_position::<{ $D }>([$($N),*], index)?;

					Some($crate::na::atov($crate::na::utoi(src)?))
				}
			}
		}
	}
}

// TODO generate generalized chunk for each

crate::multiform_shape! { pub MultiformShape1[X; 1] }
crate::multiform_shape! { pub MultiformShape2[X, Y; 2] }
crate::multiform_shape! { pub MultiformShape3[X, Y, Z; 3] }
crate::multiform_shape! { pub MultiformShape4[X, Y, Z, W; 4] }

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct CollumnChunk16x16x256<T> {
	buffer: [T; 16 * 16 * 256],
}

impl<T> CollumnChunk16x16x256<T> {
	pub fn new(buffer: [T; 16 * 16 * 256]) -> Self {
		Self { buffer }
	}
	pub fn from_indices(f: impl FnMut(usize) -> T) -> Self {
		Self::new(std::array::from_fn(f))
	}
	pub fn from_positions(mut f: impl FnMut(na::Vector<i32, 3>) -> T) -> Self {
		Self::from_indices(|index| f(MultiformShape3::<16, 16, 256>.index_to_position(index).unwrap()))
	}
}

impl<T> Chunk<3> for CollumnChunk16x16x256<T> {
	type Item = T;
	type Shape = MultiformShape3<16, 16, 256>;

	fn shape(&self) -> Self::Shape {
		MultiformShape3
	}

	fn index(&self, index: usize) -> Option<&T> {
		self.buffer.get(index)
	}
	fn index_mut(&mut self, index: usize) -> Option<&mut T> {
		self.buffer.get_mut(index)
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
