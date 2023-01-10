pub fn position_to_index<const N: usize>(shape: [usize; N], position: [usize; N]) -> Option<usize> {
	(0..N).try_fold(0, |acc, i| {
		let coordinate = position[i];
		let stride = shape[i];

		if coordinate >= stride {
			return None
		}

		let subd: usize = shape.into_iter().take(i).product();

		Some(acc + coordinate * subd)
	})
}

pub fn index_to_position<const N: usize>(shape: [usize; N], index: usize) -> Option<[usize; N]> {
	let capacity = shape.into_iter().product();

	if index >= capacity {
		return None
	}

	let mut out = [0; N];

	let prev = 0;

	for i in 0..N {
		let subd: usize = shape.into_iter().take(i).product();

		out[i] = (index - prev) / subd % shape[i]
	}

	Some(out)
}

#[cfg(test)]
mod tests {
	use super::*;

	const X: usize = 5;
	const Y: usize = 7;
	const Z: usize = 1;
	const W: usize = 4;
	const V: usize = 10;

	const C: usize = X * Y * Z * W * V;

	#[test]
	fn test_from_indices() {
		let mut it = 0..C;

		for v in 0..V {
			for w in 0..W {
				for z in 0..Z {
					for y in 0..Y {
						for x in 0..X {
							let expected = it.next().unwrap();
							let result = position_to_index([X, Y, Z, W, V], [x, y, z, w, v]).unwrap();

							assert_eq!(expected, result);
						}
					}
				}
			}
		}

	}

	#[test]
	fn test_to_indices() {
		let mut it = 0..C;

		for v in 0..V {
			for w in 0..W {
				for z in 0..Z {
					for y in 0..Y {
						for x in 0..X {
							let expected = [x, y, z, w, v];
							let result = index_to_position([X, Y, Z, W, V], it.next().unwrap()).unwrap();

							assert_eq!(expected, result);
						}
					}
				}
			}
		}

	}
}
