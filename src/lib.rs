#![warn(missing_debug_implementations)]
#![allow(soft_unstable)]

pub mod chunk;
pub mod math;
pub mod position_map;
pub mod shape;
pub mod storage;
pub mod world;

pub use position_map::PositionMap;

pub use shape::ct;
pub use shape::rt;

pub use shape::Shape;
pub use shape::UniformShape;

// pub use shape::Cow;
pub(crate) use shape::UniformWorldCoordinate;
pub(crate) use shape::WorldCoordinate;

pub use chunk::Chunk;
pub use chunk::ChunkMut;
pub use chunk::ChunkRef;

macro_rules! lazy_unreachable {
	($($t:tt)*) => {{ || unreachable!($($t)*)} }
}

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

			pub type PositionMap<T> = $crate::PositionMap<T, $D>;

			pub type Chunk<T, S> = $crate::Chunk<T, S, $D>;
			pub type ChunkRef<'a, T, S> = $crate::ChunkRef<'a, T, S, $D>;
			pub type ChunkMut<'a, T, S> = $crate::ChunkMut<'a, T, S, $D>;

			// pub trait Shape = $crate::Shape<$D>;
			// pub trait UniformShape = $crate::UniformShape<$D>;

			pub use $crate::Shape;
			pub use $crate::UniformShape;

			pub type World<T, S> = $crate::world::Uniform<T, S, $D>;
		}
	}
}

make_prelude! { prelude1, [A; 1], Multiform1 }
make_prelude! { prelude2, [A, B; 2], Multiform2 }
make_prelude! { prelude3, [A, B, C; 3], Multiform3 }
make_prelude! { prelude4, [A, B, C, D; 4], Multiform4 }
