pub use super::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
