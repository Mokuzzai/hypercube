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

// /// Implementation of uniform [`Chunk`]s and [`World`]s
// pub mod uniform;



pub use shape::DynamicMultiformShape;
pub use shape::DynamicShape;
pub use shape::DynamicUniformShape;

pub use shape::Shape;
pub use shape::Shape1;
pub use shape::Shape2;
pub use shape::Shape3;
pub use shape::Shape4;

pub use shape::UniformShape;
pub use shape::WorldCoordinate;
pub use shape::Cow;

pub use chunk::Chunk;
pub use chunk::WithPayload;

pub use world::UniformWorld;
pub use world::World;

pub use positions::Positions;

pub use boxed::Boxed;
pub use array::Array;

pub type CollumnChunk16x16x256<T> = Boxed<T, Shape3<16, 16, 256>, 3>;
pub type World16x16x256<T> = World<CollumnChunk16x16x256<T>, 3, 2, 3>;
