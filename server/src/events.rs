use std::{cell::RefCell, rc::Weak};
pub trait HasEvents {
	type Events: ?Sized;

	fn listeners(&self) -> &RefCell<Vec<Weak<Self::Events>>>;

	fn connect_events(&self, listener: Weak<Self::Events>) {
		self.listeners().borrow_mut().push(listener);
	}
}
