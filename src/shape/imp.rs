pub use super::*;

/// Runtime [`Shape`]s
pub mod rt {
	use super::*;

	#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
	#[cfg_attr(
		feature = "serde-serialize",
		derive(serde::Serialize, serde::Deserialize)
	)]
	pub struct Uniform<const B: usize> {
		stride: usize,
	}
	impl<const B: usize> Uniform<B> {
		pub const fn new(stride: usize) -> Self {
			Self { stride }
		}
	}

	impl<const B: usize> Shape<B> for Uniform<B> {
		fn extents(&self) -> math::Vector<usize, B> {
			math::Vector::from_element(self.stride())
		}
		fn capacity(&self) -> usize {
			self.stride()
				.pow(B.try_into().expect("more than `u32::MAX` dimensions"))
		}
	}

	impl<const B: usize> UniformShape<B> for Uniform<B> {
		fn stride(&self) -> usize {
			self.stride
		}
	}

	#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
	#[cfg_attr(
		feature = "serde-serialize",
		derive(serde::Serialize, serde::Deserialize)
	)]
	pub struct Multiform<const B: usize> {
		extents: math::Vector<usize, B>,
	}
	impl<const B: usize> Multiform<B> {
		pub const fn new(extents: math::Vector<usize, B>) -> Self {
			Self { extents }
		}
	}

	impl<const B: usize> Default for Multiform<B> {
		fn default() -> Self {
			Self::new(math::Vector::from_element(0))
		}
	}

	impl<const B: usize> Shape<B> for Multiform<B> {
		fn extents(&self) -> math::Vector<usize, B> {
			self.extents
		}
	}

	#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
	#[cfg_attr(
		feature = "serde-serialize",
		derive(serde::Serialize, serde::Deserialize)
	)]
	pub enum Dynamic<const B: usize> {
		Uniform(Uniform<B>),
		Multiform(Multiform<B>),
	}

	impl<const B: usize> Default for Dynamic<B> {
		fn default() -> Self {
			Self::Uniform(Uniform::default())
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
}

/// Compile time [`Shape`]s
pub mod ct {
	use super::*;

	mod macros {
		#![no_implicit_prelude]

		#[doc(hidden)]
		#[macro_export]
		macro_rules! multiform_chunk {
			($Shape:ident, [$($N:ident),*; $D:expr]) => {
				// #[doc = ::std::concat!("[``]($crate::shape): A hyperrectangle with `", stringify!($D), "` dimensions and sides of lengths ", $("`", stringify!($N), "` "),*)]
				#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
				#[cfg_attr(feature = "serde-serialize", derive(serde::Serialize, serde::Deserialize))]
				pub struct $Shape<$(const $N: ::std::primitive::usize),*>;

				impl<$(const $N: ::std::primitive::usize),*> $Shape<$($N),*> {
					pub const fn new() -> Self {
						$Shape
					}
				}

				impl<$(const $N: ::std::primitive::usize),*> $crate::Shape<$D> for $Shape<$($N),*> {
					fn extents(&self) -> $crate::math::Vector<usize, $D> {
						$crate::math::Vector::from([$($N),*])
					}
				}
			}
		}
	}

	crate::multiform_chunk! { Multiform1, [X; 1] }
	crate::multiform_chunk! { Multiform2, [X, Y; 2]  }
	crate::multiform_chunk! { Multiform3, [X, Y, Z; 3]}
	crate::multiform_chunk! { Multiform4, [X, Y, Z, W; 4] }

	#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
	#[cfg_attr(
		feature = "serde-serialize",
		derive(serde::Serialize, serde::Deserialize)
	)]
	pub struct Uniform<const S: usize, const D: usize>;

	impl<const S: usize, const B: usize> Uniform<S, B> {
		pub const fn new() -> Self {
			Uniform
		}
	}

	impl<const S: usize, const B: usize> Shape<B> for Uniform<S, B> {
		fn extents(&self) -> math::Vector<usize, B> {
			math::Vector::from_element(self.stride())
		}
		fn capacity(&self) -> usize {
			self.stride()
				.pow(B.try_into().expect("more than `u32::MAX` dimensions"))
		}
	}

	impl<const S: usize, const B: usize> UniformShape<B> for Uniform<S, B> {
		fn stride(&self) -> usize {
			S
		}
	}
}
