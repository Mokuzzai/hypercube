use crate::na;
use crate::IndexableShape;
use crate::SVector;
use crate::Shape;

/// [`Shape`]: A hypercube with `D` dimensions and side length of `S`
pub struct UniformShape<const S: usize, const D: usize>;

impl<const S: usize, const D: usize> Shape for UniformShape<S, D> {
	type Dim = na::Const<D>;
}

impl<const S: usize, const D: usize> IndexableShape for UniformShape<S, D>
where
	na::DefaultAllocator: na::Allocator<i32, Self::Dim>,
{
	fn capacity(&self) -> usize {
		S.pow(D as u32)
	}

	fn position_to_index(&self, position: SVector<Self>) -> Option<usize> {
		crate::position_index_conversion::uniform::position_to_index(
			S,
			na::itou(na::vtoa(position))?,
		)
	}
	fn index_to_position(&self, index: usize) -> Option<SVector<Self>> {
		let src = crate::position_index_conversion::uniform::index_to_position::<D>(S, index)?;

		Some(na::atov(na::utoi(src)?))
	}
}

mod macros {
	#![no_implicit_prelude]

	#[macro_export]
	macro_rules! uniform_chunk {
		($vis:vis $Chunk:ident[$S:expr; $D:expr] $(, $World:ident)? $(,)?) => {
			$vis struct $Chunk<T> {
				buffer: [T; ::std::convert::identity::<usize>($S).pow(::std::convert::identity::<usize>($D) as ::std::primitive::u32)],
			}

			impl<T> $crate::Chunk for $Chunk<T> {
				type Item = T;
				type Shape = $crate::uniform::UniformShape<{ $S }, { $D }>;

				fn shape(&self) -> &Self::Shape {
					&$crate::uniform::UniformShape
				}
				fn index(&self, index: ::std::primitive::usize) -> ::std::option::Option<&Self::Item> {
					self.buffer.get(index)
				}
				fn index_mut(&mut self, index: ::std::primitive::usize) -> ::std::option::Option<&mut Self::Item> {
					self.buffer.get_mut(index)
				}
			}

			$($vis type $World<T> = $crate::World<$crate::WorldShape<{ $D }>, $Chunk<T>>;)*
		}
	}

	uniform_chunk! { TestChunk[16; 2] }
}

crate::uniform_chunk! { pub UniformChunk2x16[16; 2], UniformWorld2x16 }
crate::uniform_chunk! { pub UniformChunk3x16[16; 3], UniformWorld3x16 }
crate::uniform_chunk! { pub UniformChunk2x32[32; 2], UniformWorld2x32 }
crate::uniform_chunk! { pub UniformChunk3x32[32; 3], UniformWorld3x32 }
