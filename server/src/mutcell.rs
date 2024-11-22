use std::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
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
	#![allow(
		unsafe_code,
		clippy::mut_from_ref,
		reason = "Uses unsafe code to allow shared mutable ownership"
	)]

	pub fn new(value: T) -> Self {
		MutCell {
			value: UnsafeCell::new(value),
		}
	}

	pub fn get_mut(&self) -> &mut T {
		// SAFETY:
		// No safety measures are taken explicitly within Rust code.
		// This code relies on UnsafeCell being excluded from deref caching in clang compiler
		// If it is unsound, we cannot really do much
		unsafe { &mut *self.value.get() }
	}

	pub fn get(&self) -> &T {
		// SAFETY:
		// No safety measures are taken explicitly within Rust code.
		// This code relies on UnsafeCell being excluded from deref caching in clang compiler
		// If it is unsound, we cannot really do much
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

		// The same reference has different value noew (expecting no deref caching optimization)
		assert_eq!(shared_reference, "Hello, world!");
	}
}
