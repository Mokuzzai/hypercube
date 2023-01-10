mod chunk;
mod shape;
mod world;
mod positions;

mod na;

pub use shape::Cube;
pub use shape::IndexableShape;
pub use shape::SVector;
pub use shape::Shape;

pub use chunk::CVector;
pub use chunk::Chunk;
pub use chunk::ChunkExt;

pub use world::World;
pub use world::WorldShape;

pub use positions::Positions;

/// Implementation of cubic [`Chunk`]s and [`World`]s
pub mod cubic;
/// Helper methods for implementing [`IndexableShape`]
pub mod position_index_conversion;

