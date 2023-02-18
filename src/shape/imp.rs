pub use super::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DynamicUniform<const B: usize> {
	stride: usize,
}

impl<const B: usize> DynamicUniform<B> {
	pub fn new(stride: usize) -> Self {
		Self { stride }
	}
}

impl<const B: usize> Shape<B> for DynamicUniform<B> {
	fn extents(&self) -> math::Vector<usize, B> {
		math::Vector::from_element(self.stride)
	}
	fn capacity(&self) -> usize {
		self.stride
			.pow(B.try_into().expect("more than `u32::MAX` dimensions"))
	}
}

impl<const B: usize> UniformShape<B> for DynamicUniform<B> {
	fn stride(&self) -> usize {
		self.stride
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DynamicMultiform<const B: usize> {
	extents: math::Vector<usize, B>,
}

impl<const B: usize> DynamicMultiform<B> {
	pub fn new(extents: math::Vector<usize, B>) -> Self {
		Self { extents }
	}
}

impl<const B: usize> Default for DynamicMultiform<B> {
	fn default() -> Self {
		Self::new(math::Vector::from_element(0))
	}
}

impl<const B: usize> Shape<B> for DynamicMultiform<B> {
	fn extents(&self) -> math::Vector<usize, B> {
		self.extents
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Dynamic<const B: usize> {
	Uniform(DynamicUniform<B>),
	Multiform(DynamicMultiform<B>),
}

impl<const B: usize> Default for Dynamic<B> {
	fn default() -> Self {
		Self::Uniform(DynamicUniform::default())
	}
}

impl<const B: usize> Shape<B> for Dynamic<B> {
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
			// #[doc = ::std::concat!("[``]($crate::shape): A hyperrectangle with `", stringify!($D), "` dimensions and sides of lengths ", $("`", stringify!($N), "` "),*)]
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

crate::multiform_chunk! { Static1, [X; 1] }
crate::multiform_chunk! { Static2, [X, Y; 2]  }
crate::multiform_chunk! { Static3, [X, Y, Z; 3]}
crate::multiform_chunk! { Static4, [X, Y, Z, W; 4] }
