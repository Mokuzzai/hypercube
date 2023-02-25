#![allow(unused, missing_debug_implementations)]

use std::mem::MaybeUninit;
use std::alloc::Layout;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Cow;

use crate::Shape;
use crate::math;

struct BlockAttr {
	name: Cow<'static, str>,
	type_id: TypeId,
	layout: Layout,
	offset: usize,
}


pub struct BlockAttrs {
	attrs: Vec<BlockAttr>,

	layout: Layout,
}

impl BlockAttrs {
	fn new() -> Self {
		Self {
			attrs: Vec::new(),
			layout: Layout::new::<()>(),
		}
	}
	fn bind<T: Any>(&mut self, name: impl Into<Cow<'static, str>>) {
		let name = name.into();

		if self.attrs.iter().find(|attr| attr.name == name).is_some() {
			panic!("duplicate attr: {}", name);
		}

		let layout = Layout::new::<T>();
		let type_id = TypeId::of::<T>();

		let (new_layout, offset) = self.layout.extend(layout).unwrap();

		self.layout = new_layout;

		self.attrs.push(BlockAttr { name, type_id, layout, offset })
	}
	fn with<T: Any>(mut self, name: impl Into<Cow<'static, str>>) -> Self {
		self.bind::<T>(name);
		self
	}
}

pub struct DynamicChunk<S, const B: usize> {
	shape: S,
	bytes: Box<[MaybeUninit<u8>]>,
}

impl<S: Shape<B>, const B: usize> DynamicChunk<S, B> {
	pub fn new_uninit(shape: S, attrs: &BlockAttrs) -> Self {
		let capacity = shape.capacity() * attrs.layout.size();

		let mut vec = Vec::<MaybeUninit<u8>>::with_capacity(capacity);

		unsafe { vec.set_len(capacity) };

		Self {
			bytes: vec.into_boxed_slice(),
			shape
		}
	}
	pub fn get(&self, attrs: &BlockAttrs, point: math::Point<i32, B>) -> Option<&Block> {
		let index = self.shape.position_to_index(point)?;
		let stride = attrs.layout.size();

		let index = index * stride;

		let bytes = self.bytes.get(index..stride)?;

		Some(Block::new_ref(bytes))
	}
	pub fn get_mut(&mut self, attrs: &BlockAttrs, point: math::Point<i32, B>) -> Option<&mut Block> {
		let index = self.shape.position_to_index(point)?;
		let stride = attrs.layout.size();

		let index = index * stride;

		let bytes = self.bytes.get_mut(index..stride)?;

		Some(Block::new_mut(bytes))
	}
}

pub struct Block {
	bytes: [MaybeUninit<u8>],
}

impl Block {
	pub fn new_ref(bytes: &[MaybeUninit<u8>]) -> &Self {
		unsafe { &*(bytes as *const _ as *const _) }
	}
	pub fn new_mut(bytes: &mut [MaybeUninit<u8>]) -> &mut Self {
		unsafe { &mut *(bytes as *mut _ as *mut _) }
	}
	pub fn get<T: Any>(&self, attrs: &BlockAttrs, name: &str) -> Option<&T> {
		let attr = attrs.attrs.iter().find(|attr| attr.name == name)?;

		let type_id = TypeId::of::<T>();

		if attr.type_id != type_id {
			return None;
		}

		let bytes = self.bytes.get(attr.offset..attr.offset + attr.layout.size()).unwrap();

		unsafe { Some(&*(bytes as *const _ as *const T)) }
	}
	pub fn get_mut<T: Any>(&mut self, attrs: &BlockAttrs, name: &str) -> Option<&mut  T> {
		let attr = attrs.attrs.iter().find(|attr| attr.name == name)?;

		let type_id = TypeId::of::<T>();

		if attr.type_id != type_id {
			return None;
		}

		let bytes = self.bytes.get_mut(attr.offset..attr.offset + attr.layout.size()).unwrap();

		unsafe { Some(&mut *(bytes as *mut _ as *mut T)) }
	}
}

#[test]
fn main() {
	let mut attrs = BlockAttrs::new()
		.with::<[u8; 3]>("color_index");

	let mut chunk = DynamicChunk::new_uninit(crate::prelude_3::ct::Uniform::<16>::new(), &attrs);

	let block: &mut Block = chunk.get_mut(&attrs, math::Point::from([4, 2, 4])).unwrap();

	*block.get_mut::<[u8; 3]>(&attrs, "color_index").unwrap() = [255, 0, 0];
}
