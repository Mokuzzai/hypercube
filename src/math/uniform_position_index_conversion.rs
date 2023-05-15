use super::*;

#[inline(always)]
pub fn subdimension(extents: usize, limit: usize) -> usize {
	extents.pow(limit as u32)
}

#[inline(always)]
pub fn position_to_index<S: Coordinate, const B: usize>(
	extent: usize,
	position: Point<S, B>,
) -> Option<usize> {
	(0..B).try_fold(0, |acc, i| {
		let coordinate = position[i].to_usize()?;

		if coordinate >= extent {
			return None;
		}

		Some(acc + coordinate * subdimension(extent, i))
	})
}

#[inline(always)]
pub fn index_to_position<S: Coordinate, const B: usize>(
	extent: usize,
	index: usize,
) -> Option<Point<S, B>> {
	if index >= subdimension(extent, B) {
		return None;
	}

	Some(Point::from(Vector::from_iterator((0..B).map(|i| {
		let subd = subdimension(extent, i);

		let stride = index / subd % extent;

		S::from(stride).unwrap()
	}))))
}
