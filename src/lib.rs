#![allow(unused)]
#![warn(missing_debug_implementations)]

mod chunk;
mod boxed;
mod positions;
mod shape;
mod world;
mod array;

/// `Nalgebra` re-export
pub mod math;

pub use shape::DynamicMultiform;
pub use shape::Dynamic;
pub use shape::DynamicUniform;
pub use shape::Static1;
pub use shape::Static2;
pub use shape::Static3;
pub use shape::Static4;

pub use shape::Shape;
pub use shape::UniformShape;

pub use shape::WorldCoordinate;
pub use shape::Cow;

pub use chunk::Chunk;
pub use chunk::WithPayload;

pub use world::SubformWorld;
pub use world::UniformWorld;
pub use world::World;

pub use positions::Positions;

pub use boxed::Boxed;
pub use array::Array;

type Boxed16x16x256<T> = Boxed<T, Static3<16, 16, 256>, 3>;
type World16x16x256<T> = World<Boxed16x16x256<T>, 3, 2, 3>;

macro_rules! lazy_unreachable {
	($($t:tt)*) => {{ || unreachable!($($t)*)} }
}

pub(crate) use lazy_unreachable;
