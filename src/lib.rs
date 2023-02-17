#![allow(unused)]
#![warn(missing_debug_implementations)]

mod chunk;
pub mod dynamic;

mod positions;
mod shape;
mod world;

pub mod na;

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

