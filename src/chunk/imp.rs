pub mod chunk16d2 {
	const S: usize = 16;
	const D: usize = 2;
	const C: usize = S.pow(D as u32);

	pub struct Chunk16d2<T>([T; C]);

	impl<T> crate::chunk::Chunk<S, D, C> for Chunk16d2<T> {
		type Item = T;
		type Coordinate = u8;

		fn array(&self) -> &[Self::Item; C] {
			&self.0
		}

		fn array_mut(&mut self) -> &mut [Self::Item; C] {
			&mut self.0
		}
	}
}

pub mod chunk32d2 {
	const S: usize = 32;
	const D: usize = 2;
	const C: usize = S.pow(D as u32);

	pub struct Chunk32d2<T>([T; C]);

	impl<T> crate::chunk::Chunk<S, D, C> for Chunk32d2<T> {
		type Item = T;
		type Coordinate = u8;

		fn array(&self) -> &[Self::Item; C] {
			&self.0
		}

		fn array_mut(&mut self) -> &mut [Self::Item; C] {
			&mut self.0
		}
	}
}

pub mod chunk16d3 {
	const S: usize = 16;
	const D: usize = 3;
	const C: usize = S.pow(D as u32);

	pub struct Chunk16d3<T>([T; C]);

	impl<T> crate::chunk::Chunk<S, D, C> for Chunk16d3<T> {
		type Item = T;
		type Coordinate = u8;

		fn array(&self) -> &[Self::Item; C] {
			&self.0
		}

		fn array_mut(&mut self) -> &mut [Self::Item; C] {
			&mut self.0
		}
	}
}

pub mod chunk32d3 {
	const S: usize = 32;
	const D: usize = 3;
	const C: usize = S.pow(D as u32);

	pub struct Chunk32d3<T>([T; C]);

	impl<T> crate::chunk::Chunk<S, D, C> for Chunk32d3<T> {
		type Item = T;
		type Coordinate = u8;

		fn array(&self) -> &[Self::Item; C] {
			&self.0
		}

		fn array_mut(&mut self) -> &mut [Self::Item; C] {
			&mut self.0
		}
	}
}
