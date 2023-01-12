use crate::na;
use crate::Shape;

/// [`Shape`]: A hypercube with `D` dimensions and side length of `S`

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct UniformShape<const S: usize, const D: usize>;

impl<const S: usize, const D: usize> Shape<D> for UniformShape<S, D> {
	fn new() -> Self {
		UniformShape
	}

	fn shape(&self) -> na::Vector<i32, D> {
		na::Vector::from_element(S as i32)
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

mod macros {
	#![no_implicit_prelude]

	#[macro_export]
	macro_rules! uniform_chunk {
		($vis:vis $Chunk:ident[$S:expr; $D:expr] $(, $World:ident)? $(,)?) => {
			#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
			$vis struct $Chunk<T> {
				buffer: [T; ::std::convert::identity::<usize>($S).pow(::std::convert::identity::<usize>($D) as ::std::primitive::u32)],
			}

			impl<T> $crate::Chunk<$D> for $Chunk<T> {
				type Item = T;
				type Shape = $crate::uniform::UniformShape<{ $S }, { $D }>;

				fn shape(&self) -> Self::Shape {
					$crate::uniform::UniformShape
				}
				fn index(&self, index: ::std::primitive::usize) -> ::std::option::Option<&Self::Item> {
					self.buffer.get(index)
				}
				fn index_mut(&mut self, index: ::std::primitive::usize) -> ::std::option::Option<&mut Self::Item> {
					self.buffer.get_mut(index)
				}
			}

			$($vis type $World<T> = $crate::World<$Chunk<T>, $D, $D>;)*

			// TODO impl `Default`
		}
	}

	uniform_chunk! { TestChunk[16; 2] }
}

crate::uniform_chunk! { pub UniformChunk2x16[16; 2], UniformWorld2x16 }
crate::uniform_chunk! { pub UniformChunk3x16[16; 3], UniformWorld3x16 }
crate::uniform_chunk! { pub UniformChunk2x32[32; 2], UniformWorld2x32 }
crate::uniform_chunk! { pub UniformChunk3x32[32; 3], UniformWorld3x32 }
