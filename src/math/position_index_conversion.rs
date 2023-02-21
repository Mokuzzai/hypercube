use super::*;

pub fn subdimension<const B: usize>(extents: Extents<B>, limit: usize) -> usize {
	extents.into_iter().take(limit).product()
}

pub fn position_to_index<const B: usize>(
	extents: Extents<B>,
	position: Position<B>,
) -> Option<usize> {
	(0..B).try_fold(0, |acc, i| {
		let stride = extents[i];

		let coordinate: usize = position[i].try_into().ok()?;

		if coordinate >= stride {
			return None;
		}

		Some(acc + coordinate * subdimension(extents, i))
	})
}

pub fn index_to_position<const B: usize>(
	extents: Extents<B>,
	index: usize,
) -> Option<Position<B>> {
	if index >= subdimension(extents, B) {
		return None;
	}

	Some(Position::from_iterator((0..B).map(|i| {
		let subd = subdimension(extents, i);

		let stride = index / subd % extents[i];

		stride.try_into().expect("coordinate greater than i32::MAX")
	})))
}

pub fn position_to_index_offset<const B: usize>(
	extents: Extents<B>,
	offset: Position<B>,
	position: Position<B>,
) -> Option<usize> {
	position_to_index(extents, position - offset)
}

pub fn index_to_position_offset<const B: usize>(
	extents: Extents<B>,
	offset: Position<B>,
	index: usize,
) -> Option<Position<B>> {
	index_to_position(extents, index).map(|position| position + offset)
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

	fn helper(mut f: impl FnMut(usize, Extents<5>)) {
		let mut it = 0..C;

		for v in 0..V {
			for w in 0..W {
				for z in 0..Z {
					for y in 0..Y {
						for x in 0..X {
							f(it.next().unwrap(), Extents::from([x, y, z, w, v]))
						}
					}
				}
			}
		}
	}
	#[test]
	fn test_position_to_index() {
		helper(|expected, position| {
			let result = position_to_index(Extents::from([X, Y, Z, W, V]), position.cast()).unwrap();

			assert_eq!(expected as usize, result);
		})
	}
	#[test]
	fn test_index_to_position() {
		helper(|index, expected| {
			let result = index_to_position(Extents::from([X, Y, Z, W, V]), index).unwrap();

			assert_eq!(expected.cast(), result);
		})
	}
}
