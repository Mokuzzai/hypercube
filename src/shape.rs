use crate::na;

pub trait Shape {
	type Dimension: na::Dim;
	type Coordinate;
}

pub trait IndexableShape: Shape {
	fn capacity(&self) -> usize;

	fn position_to_index(&self, position: SVector<Self>) -> Option<usize>
	where
		na::DefaultAllocator: na::Allocator<Self::Coordinate, Self::Dimension>;
	fn index_to_position(&self, index: usize) -> Option<SVector<Self>>
	where
		na::DefaultAllocator: na::Allocator<Self::Coordinate, Self::Dimension>;
}

/// [`na::OVector`] used to index [`IndexableShape`]
pub type SVector<S> = na::OVector<<S as Shape>::Coordinate, <S as Shape>::Dimension>;

/// [`Shape`]: A hypercube with `D` dimensions and side length of `S`
pub struct Cube<const S: usize, const D: usize>;

impl<const S: usize, const D: usize> Shape for Cube<S, D> {
	type Dimension = na::Const<D>;
	/// Using [`u8`] is probably fine since [`Chunk`](crate::Chunk) with stride greater than 255 *sounds* unreasonable
	type Coordinate = u8;
}

impl<const S: usize, const D: usize> IndexableShape for Cube<S, D> {
	fn capacity(&self) -> usize {
		S.pow(D as u32)
	}

	fn position_to_index(&self, position: SVector<Self>) -> Option<usize>
	where
		na::DefaultAllocator: na::Allocator<Self::Coordinate, Self::Dimension>,
	{
		crate::position_index_conversion::cubic::position_to_index(
			S,
			na::vtoa(position).map(Into::into),
		)
	}
	fn index_to_position(&self, index: usize) -> Option<SVector<Self>>
	where
		na::DefaultAllocator: na::Allocator<Self::Coordinate, Self::Dimension>,
	{
		let src = crate::position_index_conversion::cubic::index_to_position::<D>(S, index)?;
		let mut dst = [0; D];

		for (slot, value) in dst.iter_mut().zip(src.into_iter()) {
			*slot = match u8::try_from(value) {
				Ok(value) => Some(value),
				Err(err) => {
					if cfg!(debug_assertions) {
						eprintln!(
							"Cube::<{}, {}>::index_to_position: `{}` returning `None` instead",
							S, D, err
						);
					}

					None
				}
			}?;
		}

		Some(na::atov(dst))
	}
}
