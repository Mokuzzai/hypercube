pub fn position_to_index<const N: usize>(shape: [usize; N], position: [usize; N]) -> Option<usize> {
	todo!()
}

pub fn index_to_position<const N: usize>(shape: [usize; N], index: usize) -> Option<[usize; N]> {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	const X: usize = 5;
	const Y: usize = 7;
	const Z: usize = 3;
	const W: usize = 4;

	const C: usize = X * Y * Z * W;

	#[test]
	fn test_from_indices() {
		let mut it = 0..C;

		for w in 0..W {
			for z in 0..Z {
				for y in 0..Y {
					for x in 0..X {
						let expected = it.next().unwrap();
						let result = position_to_index([X, Y, Z, W], [x, y, z, W]).unwrap();

						assert_eq!(expected, result);
					}
				}
			}
		}
	}

	#[test]
	fn test_to_indices() {
		let mut it = 0..C;

		for w in 0..W {
			for z in 0..Z {
				for y in 0..Y {
					for x in 0..X {
						let expected = [x, y, z, w];
						let result = index_to_position([X, Y, Z, W], it.next().unwrap()).unwrap();

						assert_eq!(expected, result);
					}
				}
			}
		}
	}
}
