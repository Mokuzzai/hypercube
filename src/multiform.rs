use crate::na;
use crate::Chunk;
use crate::Shape;
use crate::World;

mod macros {
	#![no_implicit_prelude]

	#[doc(hidden)]
	#[macro_export]
	macro_rules! multiform_chunk {
		($Shape:ident, $Chunk:ident, [$($N:ident),*; $D:expr]) => {


			// #[doc = ::std::concat!("[`Shape`]($crate::shape): A hyperrectangle with `", stringify!($D), "` dimensions and sides of lengths ", $("`", stringify!($N), "` "),*)]
			#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
			pub struct $Shape<$(const $N: usize),*>;

			impl<$(const $N: ::std::primitive::usize),*> $crate::Shape<$D> for $Shape<$($N),*> {
				fn extents(&self) -> $crate::na::Vector<::std::primitive::usize, $D> {
					$crate::na::Vector::from([$($N),*])
				}
			}


			#[allow(non_camel_case_types, non_upper_case_globals)]
			#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
			pub struct $Chunk<T, $(const $N: ::std::primitive::usize,)* const C: ::std::primitive::usize> {
				pub buffer: [T; C]
			}

			#[allow(non_camel_case_types, non_upper_case_globals)]
			impl<T, $(const $N: ::std::primitive::usize,)* const C: ::std::primitive::usize> $crate::Chunk<$D> for $Chunk<T, $($N,)* C> {
				type Item = T;
				type Shape = $Shape<$($N),*>;

				fn shape(&self) -> &Self::Shape {
					&$Shape
				}

				fn index(&self, index: ::std::primitive::usize) -> ::std::option::Option<&Self::Item> {
					self.buffer.get(index)
				}
				fn index_mut(&mut self, index: ::std::primitive::usize) -> ::std::option::Option<&mut Self::Item> {
					self.buffer.get_mut(index)
				}
			}

			#[allow(non_camel_case_types, non_upper_case_globals)]
			impl<T, $(const $N: ::std::primitive::usize,)* const C: ::std::primitive::usize> $Chunk<T, $($N,)* C> {
				pub fn new(buffer: [T; C]) -> Self {
					::std::debug_assert_eq!(1 $(* ::std::convert::identity::<::std::primitive::usize>($N))*, C);
					::std::debug_assert_ne!($D, 0);
					::std::debug_assert!($D <= ::std::primitive::i32::MAX as ::std::primitive::usize);

					Self { buffer }
				}
				pub fn from_indices(f: impl ::std::ops::FnMut(::std::primitive::usize) -> T) -> Self {
					Self::new(::std::array::from_fn(f))
				}
				pub fn from_positions(mut f: impl ::std::ops::FnMut($crate::na::Vector<::std::primitive::i32, $D>) -> T) -> Self {
					Self::from_indices(|index| {
						f($Shape::<$($N),*>
							.index_to_position(index)
							.unwrap())
					})
				}
			}

			#[allow(non_camel_case_types, non_upper_case_globals)]
			impl<T, $(const $N: usize,)* const C: usize> ::std::default::Default for $Chunk<T, $($N,)* C>
			where
				T: Default,
			{
				fn default() -> Self {
					Self::new(std::array::from_fn(|_| std::default::Default::default()))
				}
			}
		}
	}
}

crate::multiform_chunk! { MultiformShape1, MultiformChunk1, [X; 1] }
crate::multiform_chunk! { MultiformShape2, MultiformChunk2, [X, Y; 2]  }
crate::multiform_chunk! { MultiformShape3, MultiformChunk3, [X, Y, Z; 3]}
crate::multiform_chunk! { MultiformShape4, MultiformChunk4, [X, Y, Z, W; 4] }

pub type CollumnChunk16x16x256<T> = MultiformChunk3<T, 16, 16, 256, { 16 * 16 * 256 }>;

pub type World2Collumns3<T> = World<CollumnChunk16x16x256<T>, 3, 2, 3>;
