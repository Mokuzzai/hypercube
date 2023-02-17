pub use super::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DynamicUniformShape<const B: usize> {
	stride: usize,
}

impl<const B: usize> DynamicUniformShape<B> {
	pub fn new(stride: usize) -> Self {
		Self { stride }
	}
}

impl<const B: usize> Shape<B> for DynamicUniformShape<B> {
	fn extents(&self) -> math::Vector<usize, B> {
		math::Vector::from_element(self.stride)
	}
	fn capacity(&self) -> usize {
		self.stride.pow(B as u32)
	}
}

impl<const B: usize> UniformShape<B> for DynamicUniformShape<B> {
	fn stride(&self) -> usize {
		self.stride
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DynamicMultiformShape<const B: usize> {
	extents: math::Vector<usize, B>,
}

impl<const B: usize> DynamicMultiformShape<B> {
	pub fn new(extents: math::Vector<usize, B>) -> Self {
		Self { extents }
	}
}

impl<const B: usize> Default for DynamicMultiformShape<B> {
	fn default() -> Self {
		Self::new(math::Vector::from_element(0))
	}
}

impl<const B: usize> Shape<B> for DynamicMultiformShape<B> {
	fn extents(&self) -> math::Vector<usize, B> {
		self.extents
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DynamicShape<const B: usize> {
	Uniform(DynamicUniformShape<B>),
	Multiform(DynamicMultiformShape<B>),
}

impl<const B: usize> Default for DynamicShape<B> {
	fn default() -> Self {
		Self::Uniform(DynamicUniformShape::default())
	}
}

impl<const B: usize> Shape<B> for DynamicShape<B> {
	fn extents(&self) -> math::Vector<usize, B> {
		match self {
			Self::Uniform(uniform) => uniform.extents(),
			Self::Multiform(multiform) => multiform.extents(),
		}
	}
	fn capacity(&self) -> usize {
		match self {
			Self::Uniform(uniform) => uniform.capacity(),
			Self::Multiform(multiform) => multiform.capacity(),
		}
	}
}

mod macros {
	#![no_implicit_prelude]

	#[doc(hidden)]
	#[macro_export]
	macro_rules! multiform_chunk {
		($Shape:ident, [$($N:ident),*; $D:expr]) => {
			// #[doc = ::std::concat!("[`Shape`]($crate::shape): A hyperrectangle with `", stringify!($D), "` dimensions and sides of lengths ", $("`", stringify!($N), "` "),*)]
			#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
			pub struct $Shape<$(const $N: ::std::primitive::usize),*>;

			impl<$(const $N: ::std::primitive::usize),*> $crate::Shape<$D> for $Shape<$($N),*> {
				fn extents(&self) -> $crate::math::Vector<::std::primitive::usize, $D> {
					$crate::math::Vector::from([$($N),*])
				}
			}
		}
	}
}

crate::multiform_chunk! { Shape1, [X; 1] }
crate::multiform_chunk! { Shape2, [X, Y; 2]  }
crate::multiform_chunk! { Shape3, [X, Y, Z; 3]}
crate::multiform_chunk! { Shape4, [X, Y, Z, W; 4] }



