use std::{
	cell::{Cell, RefCell},
	rc::Weak,
};

use crate::{events::HasEvents, pb::PbBank};

pub trait BankEvents {
	fn on_funds_changed(&self, funds: f64);
}

#[derive(Debug, Default)]
pub struct Bank {
	pb_bank: Cell<PbBank>,
	listeners: RefCell<Vec<Weak<dyn BankEvents>>>,
}

impl Bank {
	pub fn imbue(&self, new_pb_bank: PbBank) {
		self.pb_bank.set(new_pb_bank);
		self.on_funds_changed();
	}

	pub fn serialize(&self) -> PbBank {
		self.pb_bank.get()
	}

	pub fn earn(&self, amount: f64) {
		let mut pb = self.pb_bank.get();
		pb.earned += amount;
		self.pb_bank.set(pb);
		self.on_funds_changed();
	}

	pub fn spend(&self, amount: f64) {
		let mut pb = self.pb_bank.get();
		pb.spent += amount;
		self.pb_bank.set(pb);
		self.on_funds_changed();
	}

	pub fn funds(&self) -> f64 {
		let pb = self.pb_bank.get();
		pb.earned - pb.spent
	}

	fn on_funds_changed(&self) {
		let funds = self.funds();
		self.listeners.borrow_mut().retain(|listener| {
			if let Some(listener) = listener.upgrade() {
				listener.on_funds_changed(funds);
				true
			} else {
				false
			}
		});
	}
}

impl HasEvents for Bank {
	type Events = dyn BankEvents;

	fn listeners(&self) -> &RefCell<Vec<Weak<Self::Events>>> {
		&self.listeners
	}
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;

	use super::*;

	#[derive(Debug)]
	struct StockmarketDisplay {
		exchange_rate: f64,
		last_display: Cell<f64>,
	}

	impl StockmarketDisplay {
		fn new(exchange_rate: f64) -> Rc<Self> {
			Rc::new(Self {
				exchange_rate,
				last_display: Cell::new(0.0),
			})
		}
	}

	impl BankEvents for StockmarketDisplay {
		fn on_funds_changed(&self, funds: f64) {
			self.last_display.set(funds * self.exchange_rate);
		}
	}

	#[test]
	fn bank_events() {
		let bank = Bank::default();

		let display1 = StockmarketDisplay::new(10.0);
		let display2 = StockmarketDisplay::new(0.5);

		bank.connect_events(Rc::downgrade(&(display1.clone() as Rc<dyn BankEvents>)));
		bank.connect_events(Rc::downgrade(&(display2.clone() as Rc<dyn BankEvents>)));

		bank.earn(100.0);
		bank.spend(30.0);

		assert_eq!(display1.last_display.get(), 700.0);
		assert_eq!(display2.last_display.get(), 35.0);
	}
}
