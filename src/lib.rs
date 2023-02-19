// #![allow(unused)]
#![warn(missing_debug_implementations)]

mod array;
mod boxed;
mod chunk;
mod positions;
mod shape;

pub mod world;

pub use world::World;

/// `Nalgebra` re-export
pub mod math;

pub use shape::dynamic;
pub use shape::comp;

pub use shape::Shape;
pub use shape::UniformShape;

pub use shape::Cow;
pub use shape::WorldCoordinate;

pub use chunk::Chunk;
pub use chunk::WithPayload;

pub use positions::Positions;

pub use array::Array;
pub use boxed::Boxed;



macro_rules! lazy_unreachable {
	($($t:tt)*) => {{ || unreachable!($($t)*)} }
}

pub(crate) use lazy_unreachable;
