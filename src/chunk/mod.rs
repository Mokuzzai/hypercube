
mod with_payload;

pub use with_payload::WithPayload;

use crate::na;
use crate::Positions;
use crate::Shape;


pub trait Chunk<const B: usize> {
	type Item;
	type Shape: Shape<B>;

	fn shape(&self) -> &Self::Shape;

	fn index(&self, index: usize) -> Option<&Self::Item>;
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

	fn get(&self, position: na::Vector<i32, B>) -> Option<&Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index(index)
	}
	fn get_mut(&mut self, position: na::Vector<i32, B>) -> Option<&mut Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index_mut(index)
	}
	fn replace(&mut self, position: na::Vector<i32, B>, with: Self::Item) -> Option<Self::Item> {
		Some(std::mem::replace(self.get_mut(position)?, with))
	}
	fn positions(&self) -> Positions<B> {
		self.shape().positions()
	}
}

#[repr(i8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FaceLabel {
	Near = 0,
	Far = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FaceIndex<const B: usize> {
	axis: usize,
	face: FaceLabel,
}

impl<const B: usize> FaceIndex<B> {
	fn to_vector(&self) -> na::Vector<i32, B> {
		let mut out = na::Vector::from([0; B]);

		out[self.axis] = self.face as i8 as i32;

		out
	}
}

pub trait Faces<const SUBDIM: usize, const B: usize>: Chunk<B> {
	type Face: Chunk<SUBDIM, Item = Self::Item>;

	fn face_shape(&self, index: FaceIndex<B>) -> <Self::Face as Chunk<SUBDIM>>::Shape;

	fn face(&self, index: FaceIndex<B>) -> Self::Face {
		let face_shape = self.face_shape(index);



		todo!()
	}
}
