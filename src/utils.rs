pub struct Range<const END: usize> {
	pub start: usize,
}

impl<const END: usize> Range<END> {
	pub const END: usize = END;

	pub fn new(start: usize) -> Self {
		Self { start }
	}
	pub fn end(&self) -> usize {
		Self::END
	}
}

impl<const END: usize> Iterator for Range<END> {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		let mut it = self.start..END;

		let next = it.next()?;

		self.start = it.start;

		Some(next)
	}
}
