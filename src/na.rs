
pub use nalgebra::Dim;
pub use nalgebra::allocator::Allocator;

pub use nalgebra::DefaultAllocator;
pub use nalgebra::OVector;

pub use nalgebra::dimension::Const;

pub use nalgebra::Scalar;

pub fn vtoa<T: Scalar, const D: usize>(v: OVector<T, Const<D>>) -> [T; D] {
	v.into()
}

pub fn atov<T: Scalar, const D: usize>(a: [T; D]) -> OVector<T, Const<D>> {
	a.into()
}

