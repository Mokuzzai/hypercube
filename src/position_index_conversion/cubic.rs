pub fn position_to_index<const N: usize>(stride: usize, position: [usize; N]) -> Option<usize> {
	position
		.into_iter()
		.enumerate()
		.try_fold(0, |acc, (exp, coordinate)| {
			if coordinate < stride {
				Some(acc + coordinate * stride.pow(exp as u32))
			} else {
				None
			}
		})
}

pub fn index_to_position<const N: usize>(stride: usize, index: usize) -> Option<[usize; N]> {
	if index >= stride.pow(N as u32) {
		return None;
	}

	let mut prev = 0;

	Some(std::array::from_fn(|exp| {
		let this = ((index - prev) / stride.pow(exp as u32)) % stride;

		prev = this;

		this
	}))
}

#[cfg(test)]
mod tests {
	use super::*;

	const S: usize = 16;
	const C: usize = 16 * 16 * 16;

	#[test]
	fn test_from_indices() {
		let mut it = 0..C;

		for z in 0..S {
			for y in 0..S {
				for x in 0..S {
					let expected = it.next().unwrap();
					let result = position_to_index(S, [x, y, z]).unwrap();

					assert_eq!(expected, result);
				}
			}
		}
	}

	#[test]
	fn test_to_indices() {
		let mut it = 0..C;

		for z in 0..S {
			for y in 0..S {
				for x in 0..S {
					let expected = [x, y, z];
					let result = index_to_position(S, it.next().unwrap()).unwrap();

					assert_eq!(expected, result);
				}
			}
		}
	}
}
