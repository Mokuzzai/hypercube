pub struct Shape<const S: usize, const D: usize>;

impl<const S: usize, const D: usize> Shape<S, D> {
	pub fn position_to_index(position: [usize; D]) -> usize {
		position
			.into_iter()
			.enumerate()
			.fold(0, |acc, (exp, index)| acc + index * S.pow(exp as u32))
	}

	pub fn index_to_position(index: usize) -> [usize; D] {
		let mut prev = 0;

		std::array::from_fn(|exp| {
			let this = ((index - prev) / S.pow(exp as u32)) % S;

			prev = this;

			this
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const S: usize = 16;
	const D: usize = 3;
	const A: usize = 16 * 16 * 16;

	type TestShape = Shape<S, D>;

	#[test]
	fn test_from_indices() {
		let mut it = 0..A;

		for z in 0..S {
			for y in 0..S {
				for x in 0..S {
					let expected = it.next().unwrap();
					let result = TestShape::position_to_index([x, y, z]);

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
					let result = TestShape::index_to_position(it.next().unwrap());

					assert_eq!(expected, result);
				}
			}
		}
	}
}
