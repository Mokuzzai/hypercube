
use super::*;

use nd::Dimension;
use nd::RemoveAxis;

pub type WithParallelContext2<T, C> = WithParallelContext<T, C, 2>;
pub type WithParallelContext3<T, C> = WithParallelContext<T, C, 3>;

// #[derive(Debug)]
pub struct WithParallelContext<T, C, const B: usize> {
	array: nd::Array<T, nd::Shape<B>>,

	ctx: ParallelContextFor<C, B>,
}

impl<T, C, const B: usize> Chunk<B> for WithParallelContext<T, C, B> {
	type Block = T;

	fn array(&self) -> &nd::Array<Self::Block, nd::Shape<B>> { &self.array }
	fn array_mut(&mut self) -> &mut nd::Array<Self::Block, nd::Shape<B>> { &mut self.array }
}

#[derive(Debug)]
pub struct ParallelContextFor<C, const B: usize> {

	/// # Layout
	///
	/// 3d example:
	/// * [x_near, y_near, z_near, x_far, y_far, z_far]
	ctx: Box<[C]>,

	shape: nd::Shape<B>,
}

impl<C, const B: usize> ParallelContextFor<C, B>
where
	nd::Shape<B>: RemoveAxis,
{
	pub fn new(shape: nd::Shape<B>) -> Self
	where
		C: Default,
	{
		let capacity = shape.size() * 2;

		let mut ctx = Vec::with_capacity(capacity);

		ctx.extend((0..capacity).map(|_| C::default()));

		let ctx = ctx.into_boxed_slice();

		Self {
			ctx,

			shape,
		}
	}

	pub fn copy_from_array(&mut self, array: &nd::Array<C, nd::Shape<B>>)
	where
		C: Copy,
	{
		for (axis, stride) in array.shape().iter().copied().enumerate() {
			let subshape = self.subshape(nd::Axis(axis));

			let near = array.index_axis(nd::Axis(axis), 0);
			let near = near.as_slice().unwrap();
			let far = array.index_axis(nd::Axis(axis + B), stride);
			let far = far.as_slice().unwrap();

			self.sub_mut(nd::Axis(axis)).as_slice_mut().unwrap().copy_from_slice(near);
			self.sub_mut(nd::Axis(axis + B)).as_slice_mut().unwrap().copy_from_slice(far);
		}
	}

	pub fn copy_from_supertype_array<T>(&mut self, array: &nd::Array<T, nd::Shape<B>>)
	where
		C: Copy,
		T: AsRef<C>,
	{
		for (axis, stride) in array.shape().iter().copied().enumerate() {
			let subshape = self.subshape(nd::Axis(axis));

			let near = array.index_axis(nd::Axis(axis), 0);
			let near = near.as_slice().unwrap();
			let far = array.index_axis(nd::Axis(axis + B), stride);
			let far = far.as_slice().unwrap();

			self.sub_mut(nd::Axis(axis)).as_slice_mut().unwrap().iter_mut().zip(near.iter()).for_each(|(dst, src)| *dst = *src.as_ref());
			self.sub_mut(nd::Axis(axis + B)).as_slice_mut().unwrap().iter_mut().zip(far.iter()).for_each(|(dst, src)| *dst = *src.as_ref());
		}
	}

	pub fn subshape(&self, axis: nd::Axis) -> <nd::Shape::<B> as Dimension>::Smaller {
		self.shape.remove_axis(axis)
	}

	pub fn sub(&self, axis: nd::Axis) -> nd::ArrayView<C, <nd::Shape::<B> as Dimension>::Smaller> {
		assert!(axis.index() < B, "axis: `{:?}` of range `{:?}`: no such axis", axis, 0..B);

		let subshape = self.subshape(axis);

		let size = subshape.size();

		let start = axis.index() * size;
		let end = start + size;

		nd::ArrayView::from_shape(subshape, &self.ctx[start..end]).unwrap()
	}
	pub fn sub_mut(&mut self, axis: nd::Axis) -> nd::ArrayViewMut<C, <nd::Shape::<B> as Dimension>::Smaller> {
		assert!(axis.index() < B, "axis: `{:?}` of range `{:?}`: no such axis", axis, 0..B);

		let subshape = self.subshape(axis);

		let size = subshape.size();

		let start = axis.index() * size;
		let end = start + size;

		nd::ArrayViewMut::from_shape(subshape, &mut self.ctx[start..end]).unwrap()
	}
}
