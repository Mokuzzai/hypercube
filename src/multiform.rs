use crate::Chunk;
use crate::World;
use crate::WorldShape;

mod macros {
	#![no_implicit_prelude]

	#[macro_export]
	macro_rules! multiform_shape {
		($vis:vis $Shape:ident[$($N:ident),*; $D:expr]) => {


			#[doc = ::std::concat!("[`Shape`]($crate::shape): A hyperrectangle with `", stringify!($D), "` dimensions and sides of lengths ", $("`", stringify!($N), "` "),*)]
			$vis struct $Shape<$(const $N: usize),*>;

			impl<$(const $N: usize),*> $crate::Shape for $Shape<$($N),*> {
				type Dimension = $crate::na::Const<{ $D }>;
				type Coordinate = u8;
			}

			impl<$(const $N: usize),*> $crate::IndexableShape for $Shape<$($N),*> {
				fn capacity(&self) -> usize {
					1 $(* ::std::convert::identity::<usize>($N))*
				}

				fn position_to_index(&self, position:$crate:: SVector<Self>) -> Option<usize>
				where
					$crate::na::DefaultAllocator: $crate::na::Allocator<Self::Coordinate, Self::Dimension>,
				{
					crate::position_index_conversion::multiform::position_to_index(
						[$($N),*],
						$crate::na::vtoa(position).map(Into::into),
					)
				}
				fn index_to_position(&self, index: usize) -> Option<$crate::SVector<Self>>
				where
					$crate::na::DefaultAllocator: $crate::na::Allocator<Self::Coordinate, Self::Dimension>,
				{
					let src = crate::position_index_conversion::multiform::index_to_position::<{ $D }>([$($N),*], index)?;
					let mut dst = [0; ::std::convert::identity::<usize>($D)];

					for (slot, value) in dst.iter_mut().zip(src.into_iter()) {
						*slot = match u8::try_from(value) {
							Ok(value) => Some(value),
							Err(_) => None,
						}?;
					}

					Some($crate::na::atov(dst))
				}
			}
		}
	}
}

crate::multiform_shape! { pub MultiformShape1[X; 1] }
crate::multiform_shape! { pub MultiformShape2[X, Y; 2] }
crate::multiform_shape! { pub MultiformShape3[X, Y, Z; 3] }
crate::multiform_shape! { pub MultiformShape4[X, Y, Z, W; 4] }

pub struct CollumnChunk16x16x256<T> {
	buffer: [T; 16 * 16 * 256],
}

impl<T> Chunk for CollumnChunk16x16x256<T> {
	type Item = T;
	type Shape = MultiformShape3<16, 16, 256>;

	fn shape(&self) -> &Self::Shape {
		&MultiformShape3
	}

	fn index(&self, index: usize) -> Option<&T> {
		self.buffer.get(index)
	}
	fn index_mut(&mut self, index: usize) -> Option<&mut T> {
		self.buffer.get_mut(index)
	}
}

pub type World2Collumns3<T> = World<WorldShape<2>, CollumnChunk16x16x256<T>>;








