#![allow(unused)]
#![warn(missing_debug_implementations)]

mod chunk;
mod positions;
mod shape;
mod world;

mod na;

pub use shape::IndexableShape;
pub use shape::SDim;
pub use shape::SVector;
pub use shape::Shape;

pub use chunk::CDim;
pub use chunk::CVector;
pub use chunk::Chunk;
pub use chunk::WithPayload;
pub use world::WDim;
pub use world::WVector;
pub use world::World;
pub use world::WorldShape;

pub use positions::Positions;

/// Implementation of uniform [`Chunk`]s and [`World`]s
pub mod uniform;

/// Implementation of multiform [`Chunk`]s and [`World`]s
pub mod multiform;

/// Helper methods for implementing [`IndexableShape`]
pub mod position_index_conversion;
