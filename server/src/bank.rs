use std::{
	fmt::Debug,
	sync::{atomic::Ordering, Arc},
};

use atomic_float::AtomicF64;

trait Bank: Debug + Send + Sync {
	fn funds_earned(&self) -> f64;
	fn funds_spent(&self) -> f64;

	fn balance(&self) -> f64 {
		self.funds_earned() - self.funds_spent()
	}

	fn affordable(&self, amount: f64) -> bool {
		self.balance() >= amount
	}

	fn earn(&self, amount: f64);
	fn spend(&self, amount: f64) -> bool;
}

#[derive(Debug)]
struct BankImpl {
	funds_earned: AtomicF64,
	funds_spent: AtomicF64,
}

impl Bank for BankImpl {
	fn funds_earned(&self) -> f64 {
		self.funds_earned.load(Ordering::Acquire)
	}

	fn funds_spent(&self) -> f64 {
		self.funds_spent.load(Ordering::Acquire)
	}

	fn earn(&self, amount: f64) {
		self.funds_earned.fetch_add(amount, Ordering::Release);
	}

	fn spend(&self, amount: f64) -> bool {
		if self.affordable(amount) {
			self.funds_spent.fetch_add(amount, Ordering::Release);
			true
		} else {
			false
		}
	}
}

impl BankImpl {
	fn new() -> Arc<dyn Bank> {
		Arc::new(BankImpl {
			funds_earned: AtomicF64::new(0.0),
			funds_spent: AtomicF64::new(0.0),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn banking() {
		let bank = BankImpl::new();

		bank.earn(300.0);
		assert_eq!(bank.funds_earned(), 300.0);
		assert_eq!(bank.balance(), 300.0);

		assert!(bank.spend(100.0));
		assert_eq!(bank.funds_spent(), 100.0);
		assert_eq!(bank.balance(), 200.0);

		assert!(!bank.spend(300.0));
	}
}
