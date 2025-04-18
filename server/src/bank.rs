use std::cell::Cell;
use std::fmt::Debug;
use std::rc::Rc;

trait Bank: Debug {
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
	funds_earned: Cell<f64>,
	funds_spent: Cell<f64>,
}

impl Bank for BankImpl {
	fn funds_earned(&self) -> f64 {
		self.funds_earned.get()
	}

	fn funds_spent(&self) -> f64 {
		self.funds_spent.get()
	}

	fn earn(&self, amount: f64) {
		let current = self.funds_earned.get();
		self.funds_earned.set(current + amount);
	}

	fn spend(&self, amount: f64) -> bool {
		if self.affordable(amount) {
			let current = self.funds_spent.get();
			self.funds_spent.set(current + amount);
			true
		} else {
			false
		}
	}
}

impl BankImpl {
	fn new() -> Rc<dyn Bank> {
		Rc::new(BankImpl {
			funds_earned: Cell::new(0.0),
			funds_spent: Cell::new(0.0),
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

		bank.spend(100.0);
		assert_eq!(bank.funds_spent(), 100.0);
		assert_eq!(bank.balance(), 200.0);
	}
}
