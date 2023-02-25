#![warn(missing_debug_implementations)]

pub mod experimental;
pub mod buffer;
pub mod chunk;
pub mod positions;
pub mod shape;
pub mod world;
pub mod map;
pub mod math;

pub use world::Multiform;
pub use world::Uniform;
pub use world::Subform;

pub use shape::ct;
pub use shape::rt;

pub use shape::Shape;
pub use shape::UniformShape;

pub use shape::Cow;
pub use shape::WorldCoordinate;

pub use chunk::Chunk;
pub use chunk::WithPayload;

pub use positions::OffsetPositions;
pub use positions::Positions;

pub use buffer::Buffer;

/// Stack allocated [`Chunk`] with static capacity
pub type Array<T, S, const B: usize, const N: usize> = Buffer<[T; N], S, B>;

/// Heap allocated [`Chunk`] with runtime determined capacity
pub type Boxed<T, S, const B: usize> = Buffer<Box<[T]>, S, B>;

pub type Slice<T, S, const B: usize> = Buffer<[T], S, B>;

// make sure that `Array` coerces to `slice`
#[allow(unused)]
fn test_trait_bounds<T, S, const B: usize, const N: usize>(
	array: &Array<T, S, B, N>,
) -> &Slice<T, S, B> {
	array
}

macro_rules! lazy_panic {
	($($t:tt)*) => {{ || panic!($($t)*)} }
}

macro_rules! lazy_unreachable {
	($($t:tt)*) => {{ || unreachable!($($t)*)} }
}

pub(crate) use lazy_panic;
pub(crate) use lazy_unreachable;

macro_rules! make_prelude {
	( $name:ident, [$($E:ident),*; $D:expr], $ctm:ident ) => {
		pub mod $name {
			pub mod ct {
				pub type Uniform<const S: usize> = $crate::shape::ct::Uniform<S, $D>;
				pub type Multiform<$(const $E: usize),*> = $crate::shape::ct::$ctm<$($E),*>;
			}
			pub mod rt {
				pub type Uniform = $crate::shape::rt::Uniform<$D>;
				pub type Multiform = $crate::shape::rt::Multiform<$D>;
			}

			// pub trait Shape = $crate::Shape<$D>;
			// pub trait UniformShape = $crate::UniformShape<$D>;
			// pub trait Chunk = $crate::Chunk<$D>;

			pub use $crate::Shape;
			pub use $crate::UniformShape;
			pub use $crate::Chunk;

			pub type World<T> = $crate::world::Uniform<T, $D>;

			/// Stack allocated [`Chunk`] with static capacity
			pub type Array<T, S, const N: usize> = $crate::Array<T, S, $D, N>;

			/// Heap allocated [`Chunk`] with runtime determined capacity
			pub type Boxed<T, S> = $crate::Boxed<T, S, $D>;

			pub type Slice<T, S> = $crate::Slice<T, S, $D>;
		}
	}
}

make_prelude! { prelude_1, [A; 1], Multiform1 }
make_prelude! { prelude_2, [A, B; 2], Multiform2 }
make_prelude! { prelude_3, [A, B, C; 3], Multiform3 }
make_prelude! { prelude_4, [A, B, C, D; 4], Multiform4 }




