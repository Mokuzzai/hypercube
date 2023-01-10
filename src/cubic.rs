mod macros {
	#![no_implicit_prelude]

	#[macro_export]
	macro_rules! cubic_chunk {
		($vis:vis $Chunk:ident[$S:expr; $D:expr] $(, $World:ident)? $(,)?) => {
			$vis struct $Chunk<T> {
				buffer: [T; ::std::convert::identity::<usize>($S).pow(::std::convert::identity::<usize>($D) as ::std::primitive::u32)],
			}

			impl<T> $crate::Chunk for $Chunk<T> {
				type Item = T;
				type Shape = $crate::Cube<{ $S }, { $D }>;

				fn shape(&self) -> &Self::Shape {
					&$crate::Cube
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

	cubic_chunk! { TestChunk[16; 2] }
}

crate::cubic_chunk! { pub CubicChunk2x16[16; 2], CubicWorld2x16 }
crate::cubic_chunk! { pub CubicChunk3x16[16; 3], CubicWorld3x16 }
crate::cubic_chunk! { pub CubicChunk2x32[32; 2], CubicWorld2x32 }
crate::cubic_chunk! { pub CubicChunk3x32[32; 3], CubicWorld3x32 }



