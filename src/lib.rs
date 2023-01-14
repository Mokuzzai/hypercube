//! # Coordinates
//!
//! * `world` refers to an unique block in a [`World`] and is equal to `chunk_coordinate * chunk_stride + block_coordinate`
//! * `chunk` refers to a single [`Chunk`] in a [`World`] and
//! * `block` refers to a single block in a [`Chunk`]

#![allow(unused)]
#![warn(missing_debug_implementations)]

mod chunk;
mod positions;
mod shape;
mod world;
mod position_index_conversion;

mod na;

pub use shape::DynamicMultiformShape;
pub use shape::DynamicShape;
pub use shape::DynamicUniformShape;
pub use shape::Shape;
pub use shape::WorldCoordinate;

pub use chunk::Chunk;
pub use chunk::WithPayload;

pub use world::World;

pub use positions::Positions;

/// Implementation of uniform [`Chunk`]s and [`World`]s
pub mod uniform;

/// Implementation of multiform [`Chunk`]s and [`World`]s
pub mod multiform;
