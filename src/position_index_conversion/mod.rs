use crate::na;

pub fn position_to_index<const B: usize>(extents: na::Vector<usize, B>, position: na::Vector<i32, B>) -> Option<usize> {
	(0..B).try_fold(0, |acc, i| {
		let stride = extents[i];

		let coordinate: usize = position[i].try_into().ok()?;

		if coordinate >= stride {
			return None;
		}

		let subd: usize = extents.into_iter().take(i).product();

		Some(acc + coordinate * subd)
	})
}

pub fn index_to_position<const B: usize>(extents: na::Vector<usize, B>, index: usize) -> Option<na::Vector<i32, B>> {
	let capacity = extents.into_iter().product();

	let extents_i32: na::Vector<i32, B> = extents.cast();

	if index >= capacity {
		return None;
	}

	let mut out = na::Vector::from_element(0i32);

	let prev = 0;

	for i in 0..B {
		let subd: i32 = extents_i32.into_iter().take(i).product();

		out[i] = (index as i32 - prev) / subd % extents_i32[i]
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
							let result =
								position_to_index(na::Vector::from([X, Y, Z, W, V]), na::Vector::from([x, y, z, w, v]).cast()).unwrap();

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
							let expected = na::Vector::from([x, y, z, w, v]);
							let result =
								index_to_position(na::Vector::from([X, Y, Z, W, V]), it.next().unwrap()).unwrap();

							assert_eq!(expected.cast(), result);
						}
					}
				}
			}
		}
	}
}
