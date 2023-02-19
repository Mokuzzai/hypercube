// #![allow(unused)]
#![warn(missing_debug_implementations)]

pub mod chunk;
pub mod positions;
pub mod shape;
pub mod buffer;
pub mod world;

pub mod math;

pub use world::World;

pub use shape::comp;
pub use shape::dynamic;

pub use shape::Shape;
pub use shape::UniformShape;

pub use shape::Cow;
pub use shape::WorldCoordinate;

pub use chunk::Chunk;
pub use chunk::WithPayload;

pub use positions::Positions;

pub use buffer::Buffer;

/// Stack allocated [`Chunk`] with static capacity
pub type Array<T, S, const B: usize, const N: usize> = Buffer<[T; N], S, B>;

/// Heap allocated [`Chunk`] with runtime determined capacity
pub type Boxed<T, S, const B: usize> = Buffer<Box<T>, S, B>;

pub type Slice<T, S, const B: usize> = Buffer<[T], S, B>;

// make sure that `Array` coerces to `slice`
#[allow(unused)]
fn test_trait_bounds<T, S, const B: usize, const N: usize>(array: &Array<T, S, B, N>) -> &Slice<T, S, B> {
	array
}

macro_rules! lazy_unreachable {
	($($t:tt)*) => {{ || unreachable!($($t)*)} }
}

pub(crate) use lazy_unreachable;
