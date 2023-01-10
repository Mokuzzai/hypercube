use crate::*;

pub struct Cube<const S: usize, const D: usize>;

impl<const S: usize, const D: usize> Shape for Cube<S, D> {
	type Dimension = na::Const<D>;
	type Coordinate = u8;
}

impl<const S: usize, const D: usize> IndexableShape for Cube<S, D> {
	fn position_to_index(&self, position: SVector<Self>) -> Option<usize>
	where
		na::DefaultAllocator: na::Allocator<Self::Coordinate, Self::Dimension>
	{

		crate::position_index_conversion::position_to_index(S, na::vtoa(position).map(Into::into))
	}
	fn index_to_position(&self, index: usize) -> Option<SVector<Self>>
	where
		na::DefaultAllocator: na::Allocator<Self::Coordinate, Self::Dimension>
	{
		let src = crate::position_index_conversion::index_to_position::<D>(S, index)?;
		let mut dst = [0; D];

		for (slot, value) in dst.iter_mut().zip(src.into_iter()) {
			*slot = match u8::try_from(value){
				Ok(value) => Some(value),
				Err(err) => {
					eprintln!("Cube::<{}, {}>::index_to_position: `{}` returning `None` instead", S, D, err);

					None
				},
			}?;
		}

		Some(na::atov(dst))
	}
}

pub struct Chunk2x16<T> {
	buffer: [T; 16usize.pow(2)],
}

impl<T> Chunk for Chunk2x16<T> {
	type Item = T;
	type Shape = Cube<32, 2>;

	fn shape(&self) -> &Self::Shape {
		&Cube
	}
	fn index(&self, index: usize) -> Option<&Self::Item> {
		self.buffer.get(index)
	}
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.buffer.get_mut(index)
	}
}

pub struct Chunk3x16<T> {
	buffer: [T; 16usize.pow(3)],
}

impl<T> Chunk for Chunk3x16<T> {
	type Item = T;
	type Shape = Cube<32, 3>;

	fn shape(&self) -> &Self::Shape {
		&Cube
	}
	fn index(&self, index: usize) -> Option<&Self::Item> {
		self.buffer.get(index)
	}
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.buffer.get_mut(index)
	}
}

pub struct WorldShape<const D: usize>;

impl<const D: usize> Shape for WorldShape<D> {
	type Dimension = na::Const<D>;
	type Coordinate = i32;
}


pub type World2<T> = World<WorldShape<2>, Chunk2x16<T>>;
pub type World3<T> = World<WorldShape<3>, Chunk3x16<T>>;
