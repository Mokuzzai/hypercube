pub use nalgebra::dimension::DimMax;
pub use nalgebra::dimension::DimMaximum;
pub use nalgebra::Dim;

pub type Vector<T, const D: usize> = OVector<T, Const<D>>;

pub use nalgebra::OVector;

pub use nalgebra::dimension::Const;

pub use nalgebra::Scalar;

pub fn vtoa<T: Scalar, const D: usize>(v: OVector<T, Const<D>>) -> [T; D] {
	v.into()
}

pub fn atov<T: Scalar, const D: usize>(a: [T; D]) -> OVector<T, Const<D>> {
	a.into()
}

pub fn itou<const D: usize>(i: [i32; D]) -> Option<[usize; D]> {
	let mut dst = [0; D];

	for (slot, value) in dst.iter_mut().zip(i.into_iter()) {
		*slot = match usize::try_from(value) {
			Ok(value) => Some(value),
			Err(_) => None,
		}?;
	}

	Some(dst)
}

pub fn utoi<const D: usize>(i: [usize; D]) -> Option<[i32; D]> {
	let mut dst = [0; D];

	for (slot, value) in dst.iter_mut().zip(i.into_iter()) {
		*slot = match i32::try_from(value) {
			Ok(value) => Some(value),
			Err(_) => None,
		}?;
	}

	Some(dst)
}
