use std::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
	rc::Rc,
};

#[derive(Debug, Default)]
pub struct MutCell<T>
where
	T: Sized,
{
	value: UnsafeCell<T>,
}

//impl<T> !Sync for MutCell<T> {}

impl<T> MutCell<T> {
	pub fn new(value: T) -> Self {
		MutCell {
			value: UnsafeCell::new(value),
		}
	}

	#[allow(clippy::mut_from_ref)]
	pub fn get_mut(&self) -> &mut T {
		unsafe { &mut *self.value.get() }
	}

	pub fn get(&self) -> &T {
		unsafe { &*self.value.get() }
	}
}

impl<T> From<T> for MutCell<T> {
	fn from(value: T) -> Self {
		MutCell {
			value: UnsafeCell::new(value),
		}
	}
}

impl<T> Deref for MutCell<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.get()
	}
}

impl<T> DerefMut for MutCell<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.get_mut()
	}
}

pub trait MakeShared {
	fn make_shared() -> Rc<MutCell<Self>>
	where
		Self: Sized;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_shr_mutability() {
		let mut_cell: MutCell<String> = "Hello".to_owned().into();

		let shared_reference = mut_cell.get();
		assert_eq!(shared_reference, "Hello");

		{
			let mutable_referennce = mut_cell.get_mut();
			mutable_referennce.push_str(", world!");
		}

		assert_eq!(shared_reference, "Hello, world!");
	}
}
