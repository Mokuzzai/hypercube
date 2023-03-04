use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Quad<T = ()> {
	pub position: Point2<i32>,
	pub extents: Vector2<u32>,

	pub data: T,
}


impl Quad<()> {
	pub fn new(position: Point2<i32>) -> Quad<()> {
		Quad { position, extents: Vector2::from_element(1), data: () }
	}
}

impl<T> Quad<T> {
	pub fn with_data<U>(self, data: U) -> Quad<U> {
		Quad {
			data,
			position: self.position,
			extents: self.extents,
		}
	}
	pub fn drop_data(&self) -> Quad {
		Quad {
			position: self.position,
			extents: self.extents,
			data: (),
		}
	}
	pub fn from_axis_position(axis: Axis3, position: Point3<i32>, data: T) -> Self {
		Quad::new(position.coords.remove_row(axis.axis()).into())
			.with_data(data)
	}
	pub fn uv1(&self) -> Point2<i32> {
		self.position + self.extents.cast()
	}
	pub fn points(&self) -> [Point2<i32>; 4] {
		let u = self.position.x;
		let v = self.position.y;

		let position2 = self.uv1();
		let w = position2.x;
		let h = position2.y;

		[
			Point2::new(u, v),
			Point2::new(u, h),
			Point2::new(w, v),
			Point2::new(w, h),
		]
	}pub fn contains_point(&self, point: Point2<i32>) -> bool {
		let a0 = self.position;
		let a1 = self.uv1();

		let x = a0.x..a1.x;
		let y = a0.y..a1.y;

		x.contains(&point.x) && y.contains(&point.y)
	}
	pub fn contains_quad<U>(&self, other: &Quad<U>) -> bool {
		self.contains_point(other.position) && self.contains_point(other.uv1()) || self.position == other.position && self.extents == other.extents
	}
}

impl<T: Eq> Quad<T> {
	pub fn can_merge_with(&self, other: &Self, axis: Axis2) -> bool {
		let r0 = self.uv1()[axis.axis()];
		let r1 = other.position[axis.axis()];

		let ah = self.extents[(!axis).axis()];
		let bh = other.extents[(!axis).axis()];

		let edges_touch = r0 == r1;
		let edges_are_same_length = ah == bh;

		edges_touch && edges_are_same_length && self.data == other.data
	}
	pub fn try_merge_with(mut self, other: Self, axis: Axis2) -> Result<Self, (Self, Self)> {
		if self.can_merge_with(&other, axis) {
			self.extents[axis.axis()] += other.extents[axis.axis()];

			Ok(self)
		} else {
			Err((self, other))
		}
	}
}

fn drain_filter<T>(vec: &mut Vec<T>, mut predicate: impl FnMut(&mut T) -> bool, mut callback: impl FnMut(T)) {
	let mut i = 0;

	while i < vec.len() {
		if predicate(&mut vec[i]) {
			let val = vec.remove(i);

			callback(val)
		} else {
			i += 1;
		}
	}
}

#[derive(Debug)]
pub struct Quads<T = ()>(pub Vec<Quad<T>>);

impl<T> Quads<T> {
	pub fn iter(&self) -> impl Iterator<Item = &Quad<T>> {
		self.0.iter()
	}
	pub fn push(&mut self, quad: Quad<T>) {
		self.0.push(quad)
	}
	pub fn cull_occluded_faces<U>(&mut self, pat: &Quad<U>) {
		drain_filter(&mut self.0, |quad| pat.contains_quad(quad), drop)
	}
}

impl<T> Default for Quads<T> {
	fn default() -> Self {
		Self(Default::default())
	}
}

#[derive(Debug)]
pub struct PairedQuads<T = ()>([Quads<T>; 2]);

impl<T> Default for PairedQuads<T> {
	fn default() -> Self {
		Self([Default::default(), Default::default()])
	}
}

impl<T> PairedQuads<T> {
	pub fn iter(&self) -> impl Iterator<Item = &Quads<T>> {
		self.0.iter()
	}
	pub fn get_mut(&mut self, facing: Facing) -> &mut Quads<T> {
		&mut self.0[facing as usize]
	}
}

impl<T> PairedQuads<T> {
	pub fn cull_occluded_faces(&mut self) {
		let [pos, neg] = &mut self.0;

		for pos in pos.iter() {
			neg.cull_occluded_faces(pos)
		}

		for neg in neg.iter() {
			pos.cull_occluded_faces(neg)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn quads_cull_occluded_faces() {
		let mut quads = Quads::<()>::default();

		let q00 = Quad::new(Point2::new(0, 0));

		quads.push(q00);
		quads.push(Quad::new(Point2::new(1, 0)));

		quads.cull_occluded_faces(&q00);

		assert_eq!(quads.0, &[Quad::new(Point2::new(1, 0))][..])
	}
}
