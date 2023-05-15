use super::*;
use crate::math::Coordinate;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Quad<S: Coordinate, T> {
	pub position: Point2<S>,
	pub extents: Vector2<S>,

	pub data: T,
}

impl<S: Coordinate> Quad<S, ()> {
	pub fn new(position: Point2<S>) -> Self {
		Quad { position, extents: Vector2::from_element(S::one()), data: () }
	}
	pub fn from_axis_position(axis: Axis3, position: Point3<S>) -> Self {
		Quad::new(position.coords.remove_row(axis.axis()).into())
	}
}

impl<S: Coordinate, T> Quad<S, T> {
	pub fn with_data<U>(self, data: U) -> Quad<S, U> {
		Quad {
			data,
			position: self.position,
			extents: self.extents,
		}
	}
	pub fn drop_data(self) -> Quad<S, ()> {
		Quad::new(self.position).with_extents(self.extents)
	}
	pub fn with_extents(mut self, extents: Vector2<S>) -> Self {
		self.extents = extents;
		self
	}
	pub fn with_position(mut self, position: Point2<S>) -> Self {
		self.position = position;
		self
	}
	pub fn as_ref(&self) -> Quad<S, &T> {
		Quad::new(self.position).with_extents(self.extents).with_data(&self.data)
	}
	pub fn uv1(&self) -> Point2<S> {
		self.position + self.extents
	}
	pub fn points(&self) -> [Point2<S>; 4] {
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
	}
	pub fn contains_point(&self, point: Point2<S>) -> bool {
		let a0 = self.position;
		let a1 = self.uv1();

		let x = a0.x..=a1.x;
		let y = a0.y..=a1.y;

		x.contains(&point.x) && y.contains(&point.y)
	}
	pub fn contains_quad<U>(&self, other: &Quad<S, U>) -> bool {
		self.contains_point(other.position) && self.contains_point(other.uv1())
	}
}

impl<S: Coordinate, T: PartialEq> Quad<S, T> {
	pub fn can_merge_with(&self, other: &Self, axis: Axis2) -> bool {
		if self.data != other.data {
			return false
		}

		let far = self.position[axis.axis()] + self.extents[axis.axis()];
		let near = other.position[axis.axis()];

		if far != near {
			return false
		}

		let this_y = self.position[(!axis).axis()];
		let other_y = other.position[(!axis).axis()];

		if this_y != other_y {
			return false
		}

		let this_height = self.extents[(!axis).axis()];
		let other_height = other.extents[(!axis).axis()];

		if this_height != other_height {
			return false
		}

		true
	}
	pub fn try_merge_with(&mut self, other: &Self, axis: Axis2) -> bool {
		if self.can_merge_with(&other, axis) {
			self.extents[axis.axis()] += other.extents[axis.axis()];

			true
		} else {
			false
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

#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Quads<S: Coordinate, T>(pub Vec<Quad<S, T>>);

impl<S: Coordinate, T> Quads<S, T> {
	pub fn iter(&self) -> impl Iterator<Item = &Quad<S, T>> {
		self.0.iter()
	}
	pub fn clear(&mut self) {
		self.0.clear()
	}
	pub fn num_quads(&self) -> usize {
		self.0.len()
	}
	pub fn contains_quad<U>(&self, pat: &Quad<S, U>) -> bool {
		self.iter().any(|quad| quad.contains_quad(&pat))
	}
	pub fn push(&mut self, quad: Quad<S, T>) {
		self.0.push(quad)
	}
	pub fn cull_occluded_quads<U>(&mut self, pat: &Quad<S, U>) {
		drain_filter(&mut self.0, |quad| pat.contains_quad(quad), drop)
	}
	pub fn cull_overlapping(&mut self) {
		let mut new = Self::default();

		for quad in self.0.drain(..) {
			if !new.contains_quad(&quad) {
				new.push(quad)
			}
		}

		*self = new;
	}
}

impl<S: Coordinate, T: PartialEq> Quads<S, T> {
	pub fn push_merge(&mut self, quad: Quad<S, T>, axis: Axis2) {
		if self._merge_into(&quad, axis) {

		} else {
			self.push(quad)
		}
	}
	pub fn merge_adjacent(&mut self) {
		self.merge_adjacent_axis(Axis2::X);
		self.merge_adjacent_axis(Axis2::Y);
	}
	fn _merge_into(&mut self, quad: &Quad<S, T>, axis: Axis2) -> bool {
		for acc in self.0.iter_mut() {
			if acc.try_merge_with(quad, axis) {
				return true
			}
		}

		false
	}
	pub fn merge_adjacent_axis(&mut self, axis: Axis2) {
		let mut new = Self::default();

		for mut quad in self.0.drain(..) {
			new.push_merge(quad, axis)
		}

		*self = new;
	}
}

impl<S: Coordinate, T> Default for Quads<S, T> {
	fn default() -> Self {
		Self(Default::default())
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PairedQuads<S: Coordinate, T> {
	pos: Quads<S, T>,
	neg: Quads<S, T>,
}

impl<S: Coordinate, T> Default for PairedQuads<S, T> {
	fn default() -> Self {
		Self { pos: Default::default(), neg: Default::default() }
	}
}

impl<S: Coordinate, T> PairedQuads<S, T> {
	pub fn iter(&self) -> impl Iterator<Item = (Facing, &Quads<S, T>)> {
		let Self { pos, neg } = self;

		[(Facing::PosZ, pos), (Facing::NegZ, neg)].into_iter()
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (Facing, &mut Quads<S, T>)> {
		let Self { pos, neg } = self;

		[(Facing::PosZ, pos), (Facing::NegZ, neg)].into_iter()
	}
	pub fn num_quads(&self) -> usize {
		self.iter().map(|(_, quads)| quads.num_quads()).sum()
	}
	pub fn clear(&mut self) {
		self.iter_mut().for_each(|(_, quads)| quads.clear())
	}
	pub fn get_mut(&mut self, facing: Facing) -> &mut Quads<S, T> {
		match facing {
			Facing::PosZ => &mut self.pos,
			Facing::NegZ => &mut self.neg
		}
	}
	pub fn cull_overlapping(&mut self) {
		let Self { pos, neg } = self;

		let pos_clone = Quads(pos.iter().map(|quad| quad.as_ref().drop_data()).collect());
		let neg_clone = Quads(neg.iter().map(|quad| quad.as_ref().drop_data()).collect());

		pos.cull_overlapping();
		neg.cull_overlapping();

		for pos in pos_clone.iter() {
			neg.cull_occluded_quads(pos)
		}

		for neg in neg_clone.iter() {
			pos.cull_occluded_quads(neg)
		}
	}
}

impl<S: Coordinate, T: PartialEq> PairedQuads<S, T> {
	pub fn push_cull_merge(&mut self, facing: Facing, quad: Quad<S, T>, axis: Axis2) {
		for (_, quads) in self.iter_mut() {
			quads.cull_occluded_quads(&quad);
		}

		self.get_mut(facing).push_merge(quad, axis)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod quad {
		use super::*;

		#[test]
		fn contains_quad() {
			let a = Quad::new(Point2::new(0, 0));
			let a_copy = a;

			let big_a = a.with_extents(Vector2::new(3, 3));
			let offset_a = a.with_position(Point2::new(1, 1));

			assert!(a.contains_quad(&a_copy));
			assert!(big_a.contains_quad(&a));
			assert!(big_a.contains_quad(&offset_a));
			assert!(!a.contains_quad(&big_a));
		}
	}

	mod quads {
		use super::*;

		#[test]
		fn contains_quad() {
			let mut quads = Quads::<i32, ()>::default();

			quads.push(Quad::new(Point2::new(1, 0)));
			quads.push(Quad::new(Point2::new(0, 0)));
			quads.push(Quad::new(Point2::new(0, 1)));
			quads.push(Quad::new(Point2::new(0, 0)));
			quads.push(Quad::new(Point2::new(1, 0)));
			quads.push(Quad::new(Point2::new(0, 1)));
			quads.push(Quad::new(Point2::new(1, 0)));
			quads.push(Quad::new(Point2::new(3, 3)));

			assert!(quads.contains_quad(&Quad::new(Point2::new(3, 3))));
		}

		#[test]
		fn cull_overlapping() {
			let mut quads = Quads::<i32, ()>::default();

			quads.push(Quad::new(Point2::new(0, 0)).with_extents(Vector2::new(3, 3)));
			quads.push(Quad::new(Point2::new(3, 3)));

			let expected = quads.clone();

			quads.push(Quad::new(Point2::new(1, 0)));
			quads.push(Quad::new(Point2::new(0, 0)));
			quads.push(Quad::new(Point2::new(0, 1)));
			quads.push(Quad::new(Point2::new(0, 0)));
			quads.push(Quad::new(Point2::new(1, 0)));
			quads.push(Quad::new(Point2::new(0, 1)));
			quads.push(Quad::new(Point2::new(1, 0)));

			quads.cull_overlapping();

			assert_eq!(quads, expected);
		}

		#[test]
		fn cull_occluded_quads() {
			let mut quads = Quads::<i32, ()>::default();

			let q00 = Quad::new(Point2::new(0, 0));

			quads.push(q00);
			quads.push(Quad::new(Point2::new(1, 0)));

			quads.cull_occluded_quads(&q00);

			assert_eq!(quads.0, &[Quad::new(Point2::new(1, 0))][..])
		}
	}

	#[test]
	fn paired_quads_cull_overlapping() {
		let mut result = PairedQuads::<i32, ()>::default();
		let mut expected = PairedQuads::<i32, ()>::default();

		let q00 = Quad::new(Point2::new(0, 0));
		let q01 = Quad::new(Point2::new(0, 1));
		let q10 = Quad::new(Point2::new(1, 0));

		expected.pos.push(q01);
		expected.neg.push(q10);
		result.pos.push(q01);
		result.neg.push(q10);

		assert_eq!(result, expected);

		result.pos.push(q00);
		result.neg.push(q00);

		result.cull_overlapping();

		assert_eq!(result, expected);
	}
}
