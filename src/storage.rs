pub trait Storage {
	type Item;
}

pub trait ComputeStorage<I>: Storage {
	fn compute(&self, index: I) -> Self::Item;
}

pub trait ContiguousMemory: Storage {
	fn as_slice(&self) -> &[Self::Item];
}

pub trait ContiguousMemoryMut: ContiguousMemory {
	fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

pub trait FromFn: Sized + ContiguousMemory {
	fn from_fn(capacity: usize, f: impl FnMut(usize) -> Self::Item) -> Self;
}

// impl<T> ItemStorage<usize> for T where T: ContiguousMemory {
// 	fn capacity(&self) -> usize {
// 		self.as_slice().len()
// 	}
// 	fn get(&self, index: usize) -> Option<&Self::Item> {
// 		self.as_slice().get(index)
// 	}
// }
//
// impl<T> ItemStorageMut<usize> for T where T: ContiguousMemoryMut {
// 	fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
// 		self.as_mut_slice().get_mut(index)
// 	}
// }

impl<T> Storage for [T] {
	type Item = T;
}
impl<T, const N: usize> Storage for [T; N] {
	type Item = T;
}
impl<T> Storage for Vec<T> {
	type Item = T;
}

impl<T> ContiguousMemory for [T] {
	fn as_slice(&self) -> &[Self::Item] {
		self
	}
}

impl<T> ContiguousMemoryMut for [T] {
	fn as_mut_slice(&mut self) -> &mut [Self::Item] {
		self
	}
}

impl<T, const N: usize> ContiguousMemory for [T; N] {
	fn as_slice(&self) -> &[Self::Item] {
		self
	}
}

impl<T, const N: usize> ContiguousMemoryMut for [T; N] {
	fn as_mut_slice(&mut self) -> &mut [Self::Item] {
		self
	}
}

impl<T, const N: usize> FromFn for [T; N] {
	fn from_fn(capacity: usize, f: impl FnMut(usize) -> Self::Item) -> Self {
		assert_eq!(capacity, N);

		std::array::from_fn(f)
	}
}

impl<T> ContiguousMemory for Vec<T> {
	fn as_slice(&self) -> &[Self::Item] {
		self
	}
}

impl<T> ContiguousMemoryMut for Vec<T> {
	fn as_mut_slice(&mut self) -> &mut [Self::Item] {
		self
	}
}

impl<T> FromFn for Vec<T> {
	fn from_fn(capacity: usize, mut f: impl FnMut(usize) -> Self::Item) -> Self {
		let mut buffer = Vec::with_capacity(capacity);

		for index in 0..capacity {
			buffer.push(f(index));
		}

		buffer
	}
}

impl<T> FromFn for Box<[T]> {
	fn from_fn(capacity: usize, f: impl FnMut(usize) -> Self::Item) -> Self {
		Vec::from_fn(capacity, f).into_boxed_slice()
	}
}

macro_rules! defer_s {
	($S:ident, $Self:ty $(, $lft:tt)?) => {
		impl<$($lft,)* $S: ?Sized + Storage> Storage for $Self {
			type Item = $S::Item;
		}

		impl<$($lft,)* $S: ?Sized + ComputeStorage<I>, I> ComputeStorage<I> for $Self {
			fn compute(&self, index: I) -> Self::Item {
				$S::compute(&**self, index)
			}
		}

		impl<$($lft,)* $S: ?Sized + ContiguousMemory> ContiguousMemory for $Self {
			fn as_slice(&self) -> &[Self::Item] {
				$S::as_slice(&**self)
			}
		}
	}
}

macro_rules! defer_s_rm {
	($S:ident, $Self:ty $(, $lft:tt)?) => {
		impl<$($lft,)* $S: ?Sized + ContiguousMemoryMut> ContiguousMemoryMut for $Self {
			fn as_mut_slice(&mut self) -> &mut [Self::Item] {
				$S::as_mut_slice(&mut **self)
			}
		}
	}
}

defer_s! { S, Box<S> }
defer_s_rm! { S, Box<S> }

defer_s! { S, &'a S, 'a }

defer_s! { S, &'a mut S, 'a }
defer_s_rm! { S, &'a mut S, 'a }
