use crate::nd;
use crate::na;

use nd::IntoDimension;

use super::World;

pub struct FullCtx<'a, T, const C: usize> {
	ctx: nd::Array<Option<&'a T>, nd::Shape<C>>,
}

impl<'a, T, const C: usize> Default for FullCtx<'a, T, C>
where
	nd::Shape<C>: nd::Dimension,
{
	fn default() -> Self {
		Self { ctx: nd::Array::from_elem([3; C], None)}
	}
}

impl<'a, T, const C: usize> FullCtx<'a, T, C>
where
	nd::Shape<C>: nd::Dimension,
{
	pub fn new<const B: usize>(world: &'a World<T, C, B>, position: na::Vector<i32, C>) -> Self {
		let mut this = Self::default();

		this.ctx.indexed_iter_mut().for_each(|(index, slot)| {
			let mut dim = index.into_dimension();

			let position: na::Vector<i32, C> = na::Vector::from(dim).cast();
			let position = position - na::Vector::from([-1; C]);

			*slot = world.chunk(position);
		});

		this
	}
}
