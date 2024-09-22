use std::rc::Rc;

use super::{
    ingame_types::Money,
    mutcell::{MakeShared, MutCell},
    observer::Si,
};

pub struct Bank {
    funds_earned: Si<Money>,
    funds_cheated: Si<Money>,
    funds_spent: Si<Money>,
    funds: Si<Money>,
}

impl Bank {
    pub fn funds(&self) -> Money {
        *self.funds_earned + *self.funds_cheated - *self.funds_spent
    }

    fn encache_funds(&mut self) {
        let n = *self.funds_earned + *self.funds_cheated - *self.funds_spent;
        self.funds.set(n);
    }

    fn earn(&mut self, amount: Money) {
        self.funds_earned.set(*self.funds_earned + amount);
    }
}

impl MakeShared for Bank {
    fn make_shared() -> Rc<MutCell<Self>> {

        let bank = Rc::new(MutCell::from(Bank {
            funds_earned: Default::default(),
            funds_cheated: Default::default(),
            funds_spent: Default::default(),
            funds: Default::default(),
        }));

        let b = bank.get_mut();

        let s = &Self::encache_funds;

        b.funds_earned.add_agnostic_slot(bank.clone(), s);
        b.funds_cheated.add_agnostic_slot(bank.clone(), s);
        b.funds_spent.add_agnostic_slot(bank.clone(), s);

        bank
    }
}

#[cfg(test)]
mod tests {
    use crate::mo_server::mutcell::MakeShared;

    use super::Bank;

    #[test]
    fn bank_creation() {
        let b = Bank::make_shared();
        assert!(*b.funds == 0.0);

        b.get_mut().funds_earned.set(1.0);
        assert!(*b.funds == 1.0);

        b.get_mut().funds_spent.set(1.0);
        assert!(*b.funds == 0.0);
    }
}
