#![allow(unused)]
#![warn(missing_debug_implementations)]

mod chunk;
mod boxed;
mod positions;
mod shape;
mod world;

/// `Nalgebra` re-export
pub mod math;

/// Implementation of uniform [`Chunk`]s and [`World`]s
pub mod uniform;

/// Implementation of multiform [`Chunk`]s and [`World`]s
pub mod multiform;

pub use shape::DynamicMultiformShape;
pub use shape::DynamicShape;
pub use shape::DynamicUniformShape;
pub use shape::Shape;
pub use shape::UniformShape;
pub use shape::WorldCoordinate;
pub use shape::Cow;

pub use chunk::Chunk;
pub use chunk::WithPayload;

pub use world::UniformWorld;
pub use world::World;

pub use positions::Positions;

pub use boxed::Boxed;
