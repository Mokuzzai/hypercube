
mod shape;
mod chunk;
mod world;

mod na;


pub use shape::Shape;
pub use shape::IndexableShape;
pub use shape::SVector;
pub use shape::Cube;

pub use chunk::Chunk;
pub use chunk::CVector;
pub use chunk::ChunkExt;

pub use world::World;
pub use world::WorldShape;

/// Helper methods for implementing [`IndexableShape`]
pub mod position_index_conversion;
/// Implementation of cubic [`Chunk`]s and [`World`]s
pub mod cubic;
