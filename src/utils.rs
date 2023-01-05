pub struct Shape<const S: usize, const D: usize, const C: usize> {
	_lock: (),
}

impl<const S: usize, const D: usize, const C: usize> Shape<S, D, C> {
	pub const fn new() -> Option<Self> {
		// 1. here
		if D > u32::MAX as usize {
			return None
			}

		// 2. here
		if let Some(c) = S.checked_pow(D as u32) {
			if c != C {
				return None
			}
		} else {
			return None
		}

		Some(unsafe {
			// SAFETY: see 1. & 2.
			Self::new_unchecked()
		})
	}

	/// SAFETY:
	/// 1. D <= u32::MAX
	/// 2. S.checked_pow(D as u32) == Some(C)
	pub const unsafe fn new_unchecked() -> Self {
		Self { _lock: () }
	}

	pub fn get<T>(self, array: &[T; C], position: [usize; D]) -> Option<&T> {
		let index = self.position_to_index(position)?;

		unsafe {
			// SAFETY: `position_to_index` always returns index in 0..C
			Some(array.get_unchecked(index))
		}
	}

	pub fn get_mut<T>(self, array: &mut [T; C], position: [usize; D]) -> Option<&mut T> {
		let index = self.position_to_index(position)?;

		unsafe {
			// SAFETY: `position_to_index` always returns index in 0..C
			Some(array.get_unchecked_mut(index))
		}
	}

	pub fn position_to_index(self, position: [usize; D]) -> Option<usize> {
		position
			.into_iter()
			.enumerate()
			.try_fold(0, |acc, (exp, coordinate)| {
				if coordinate < S {
					Some(acc + coordinate * S.pow(exp as u32))
				} else {
					None
				}
			})
	}

	pub fn index_to_position(self, index: usize) -> Option<[usize; D]> {
		if index >= C {
			return None
		}

		let mut prev = 0;

		Some(std::array::from_fn(|exp| {
			let this = ((index - prev) / S.pow(exp as u32)) % S;

			prev = this;

			this
		}))
	}
}

pub struct Positions<const S: usize, const D: usize, const C: usize> {
	shape: Shape<S, D, C>,
	index: usize,
}

impl<const S: usize, const D: usize, const C: usize> Positions<S, D, C> {
	pub const fn new() -> Option<Self> {
		if let Some(shape) = Shape::new() {
			Some(Self { shape, index: 0 })
		} else {
			None
		}
	}

	/// SAFETY: see [`Shape::new_unchecked`]
	pub const unsafe fn new_unchecked() -> Self {
		Self {
			shape: Shape::new_unchecked(),
			index: 0,
		}
	}
}

impl<const S: usize, const D: usize, const C: usize> Iterator for Positions<S, D, C> {
	type Item = [usize; D];

	fn next(&mut self) -> Option<Self::Item> {
		let mut it = self.index..C;

		let index = it.next()?; // '1 here

		it.start = index;

		unsafe {
			// SAFETY: we already checked that `index` is less than `C` '1
			Some(Shape::<S, D, C>::new_unchecked().index_to_position(index).unwrap_unchecked())
		}
	}

	fn count(self) -> usize {
		C - self.index
	}

	fn last(self) -> Option<Self::Item> {
		unsafe { Shape::<S, D, C>::new_unchecked().index_to_position(C - 1) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const S: usize = 16;
	const D: usize = 3;
	const A: usize = 16 * 16 * 16;

	type TestShape = Shape<S, D, A>;

	#[test]
	fn test_from_indices() {
		let mut it = 0..A;

		for z in 0..S {
			for y in 0..S {
				for x in 0..S {
					let expected = it.next().unwrap();
					let result = TestShape::new().unwrap().position_to_index([x, y, z]).unwrap();

					assert_eq!(expected, result);
				}
			}
		}
	}

	#[test]
	fn test_to_indices() {
		let mut it = 0..A;

		for z in 0..S {
			for y in 0..S {
				for x in 0..S {
					let expected = [x, y, z];
					let result = TestShape::new().unwrap().index_to_position(it.next().unwrap()).unwrap();

					assert_eq!(expected, result);
				}
			}
		}
	}
}
