use crate::na;
use std::ops::Range;

pub fn position_to_index<const B: usize>(
	extents: na::Vector<usize, B>,
	position: na::Vector<i32, B>,
) -> Option<usize> {
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

pub fn index_to_position<const B: usize>(
	extents: na::Vector<usize, B>,
	index: usize,
) -> Option<na::Vector<i32, B>> {
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

pub(crate) fn unroll_extents_range<const B: usize>(
	range: Range<na::Vector<i32, B>>,
) -> na::Vector<usize, B> {
	let extents = range.end - range.start;
	let extents: na::Vector<usize, B> = extents.try_cast().unwrap();
	extents
}

pub fn position_to_index_with_range<const B: usize>(
	range: Range<na::Vector<i32, B>>,
	position: na::Vector<i32, B>,
) -> Option<usize> {
	let offset = range.start;

	position_to_index(unroll_extents_range(range), position + offset)
}

pub fn index_to_position_with_range<const B: usize>(
	range: Range<na::Vector<i32, B>>,
	index: usize,
) -> Option<na::Vector<i32, B>> {
	let offset = range.start;

	let ret = index_to_position(unroll_extents_range(range), index)?;

	Some(ret + offset)
}

#[cfg(test)]
mod origin {
	use super::*;

	const X: usize = 5;
	const Y: usize = 7;
	const Z: usize = 1;
	const W: usize = 4;
	const V: usize = 10;

	const C: usize = X * Y * Z * W * V;

	fn helper(mut f: impl FnMut(usize, na::Vector<usize, 5>)) {
		let mut it = 0..C;

		for v in 0..V {
			for w in 0..W {
				for z in 0..Z {
					for y in 0..Y {
						for x in 0..X {
							f(it.next().unwrap(), na::Vector::from([x, y, z, w, v]))
						}
					}
				}
			}
		}
	}
	#[test]
	fn test_position_to_index() {
		helper(|expected, position| {
			let result =
				position_to_index(na::Vector::from([X, Y, Z, W, V]), position.cast()).unwrap();

			assert_eq!(expected as usize, result);
		})
	}
	#[test]
	fn test_index_to_position() {
		helper(|index, expected| {
			let result = index_to_position(na::Vector::from([X, Y, Z, W, V]), index).unwrap();

			assert_eq!(expected.cast(), result);
		})
	}
	mod with_range {
		use super::*;
		#[test]
		fn test_position_to_index_with_range_0() {
			helper(|expected, position| {
				let result = position_to_index_with_range(
					na::Vector::from([0; 5])..na::Vector::from([X, Y, Z, W, V]).cast(),
					position.cast(),
				)
				.unwrap();

				assert_eq!(expected as usize, result);
			})
		}
		#[test]
		fn test_position_to_index_with_range_1() {
			helper(|expected, position| {
				let result = position_to_index_with_range(
					na::Vector::from([0; 5])..na::Vector::from([X, Y, Z, W, V]).cast(),
					position.cast(),
				)
				.unwrap();

				assert_eq!(expected as usize, result);
			})
		}
		#[test]
		fn test_index_to_position_with_range() {
			let mut it = 0..C;

			for v in 0..V {
				for w in 0..W {
					for z in 0..Z {
						for y in 0..Y {
							for x in 0..Y {
								let expected = na::Vector::from([x, y, z, w, v]);
								let result = index_to_position_with_range(
									na::Vector::from([0; 5])..na::Vector::from([X, Y, Z, W, V]).cast(),
									it.next().unwrap() as usize,
								)
								.unwrap();

								assert_eq!(expected.cast(), result);
							}
						}
					}
				}
			}
		}
	}
	mod unroll_extents {
		use super::*;

		#[test]
		fn start_0() {
			assert_eq!(
				na::Vector::from([5, 1, 3]),
				unroll_extents_range(na::Vector::from([0; 3])..na::Vector::from([5, 1, 3]))
			);
		}

		#[test]
		fn start_negative_1() {
			assert_eq!(
				na::Vector::from([6, 2, 4]),
				unroll_extents_range(na::Vector::from([-1; 3])..na::Vector::from([5, 1, 3]))
			);
		}

		#[test]
		fn start_positive_1() {
			assert_eq!(
				na::Vector::from([4, 0, 2]),
				unroll_extents_range(na::Vector::from([1; 3])..na::Vector::from([5, 1, 3]))
			);
		}
	}
}
