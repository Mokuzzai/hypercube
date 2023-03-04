use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Quad<T = ()> {
	pub position: Point2<i32>,
	pub extents: Vector2<u32>,

	pub data: T,
}

impl Quad<()> {
	pub fn new(uv: Point2<i32>) -> Self {
		Self::with_data(uv, ())
	}

}

impl<T> Quad<T> {
	pub fn drop_data(&self) -> Quad {
		Quad {
			position: self.position,
			extents: self.extents,
			data: (),
		}
	}
	pub fn with_data(uv: Point2<i32>, data: T) -> Self {
		Self {
			data,
			position: uv,
			extents: Vector2::new(1, 1),
		}
	}
	pub fn from_axis_position(axis: Axis3, position: Point3<i32>, data: T) -> Self {
		Self::with_data(position.coords.remove_row(axis.axis()).into(), data)
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
		self.contains_point(other.position) && self.contains_point(other.uv1())
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
	// pub fn try_occluded_by(&self, other: &Self) -> Result<bool, ()> {
	// 	if self.facing != other.facing {
	// 		Err(())
	// 	} else {
	// 		Ok(self.contains_point(other.uv) && self.contains_point(other.uv1()))
	// 	}
	// }
	/// # Return
	/// Returns how many values were merged
	pub fn try_merge_sorted_slice<'a>(&mut self, quads: &'a [Self], axis: Axis2) -> usize
	where
		T: Copy,
	{
		let mut iter = quads.iter();
		let mut merge_count = 0;

		while let Some(quad) = iter.next().copied() {
			if self.try_merge_with(quad, axis).is_err() {
				return merge_count;
			} else {
				merge_count += 1;
			}
		}

		return 0;
	}
	pub fn try_merge_sorted_slice_all(mut quads: &[Self], axis: Axis2) -> Vec<Self>
	where
		T: Copy,
	{
		let mut vec = Vec::new();

		let mut iter = quads.iter().copied();

		loop {
			println!("first iteration: {} quads left", quads.len());

			let Some(mut acc) = iter.next() else { break };

			'a: loop {
				if let Some(next) = iter.next() {
					match acc.try_merge_with(next, axis) {
						Ok(a) => acc = a,
						Err((a, b)) => {
							vec.push(a);

							acc = b;

							break 'a;
						}
					}
				} else {
					vec.push(acc);

					break 'a;
				}
			}

			std::thread::sleep(std::time::Duration::from_secs(1));
		}

		dbg!(vec.len());

		vec
	}
}
