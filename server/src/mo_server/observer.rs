use std::{
    cell::RefCell,
    ops::Deref,
    sync::{Arc, Weak},
};

pub struct Si<Data>
where
    Data: PartialEq,
{
    value: Data,
    slots: Vec<Box<dyn Slot<Data>>>,
}

impl<Data> Si<Data>
where
    Data: PartialEq + 'static,
{
    pub fn new(data: Data) -> Self {
        Self {
            value: data,
            slots: Vec::new(),
        }
    }

    pub fn value(&self) -> &Data {
        &self.value
    }

    pub fn set(&mut self, data: Data) {
        if self.value == data {
            return;
        }
        self.value = data;

        self.slots.retain(|slot| slot.is_active());
        for ear in &mut self.slots {
            ear.process_signal(&self.value);
        }
    }

    pub fn add_slot<T: 'static>(
        &mut self,
        object: Arc<RefCell<T>>,
        method: impl Fn(&mut T, &Data) + 'static,
    ) {
        let mut closure = Box::new(WeakClosure::<T, Data> {
            object: Arc::downgrade(&object),
            method: Box::new(method),
        });

        closure.process_signal(&self.value);
        self.slots.push(closure);
    }
}

impl<Data> Deref for Si<Data>
where
    Data: PartialEq,
{
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

struct WeakClosure<T, Data>
where
    Data: PartialEq,
{
    object: Weak<RefCell<T>>,
    method: Box<dyn FnMut(&mut T, &Data)>,
}

trait Slot<Data> {
    fn is_active(&self) -> bool;
    fn process_signal(&mut self, new_data: &Data);
}

impl<T, Data> Slot<Data> for WeakClosure<T, Data>
where
    Data: PartialEq,
{
    fn is_active(&self) -> bool {
        self.object.upgrade().is_some()
    }

    fn process_signal(&mut self, data: &Data) {
        if !self.is_active() {
            return;
        }

        let object = self.object.upgrade().unwrap();

        let mut borrow_result = object.try_borrow_mut();
        let object_ref: &mut T = match borrow_result {
            Ok(ref mut r) => r,
            Err(_) => {
                let raw_pointer = object.as_ptr();
                let ref_mut = unsafe { raw_pointer.as_mut().unwrap() };
                ref_mut
            }
        };
        //
        // Unsafe usage reason: double borrow can occur when self-listening, case:
        //
        // selflistener
        //   .borrow_mut()
        //   .lv.add_slot(selflistener.clone(), &SelfListener::method);
        //
        // calls selflistener.process_signal()
        // which otherwise would create a second borrow:
        //
        // let ref mut object = object.borrow_mut();
        //
        // In that case the double borrow should be fine, as the pointer should not be accessed at the point

        (self.method)(object_ref, &data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fancy {
        bingo: Si<i32>,
        lucky: bool,
    }

    impl Fancy {
        fn new() -> Arc<RefCell<Fancy>> {
            let f = Fancy {
                bingo: Si::new(0),
                lucky: false,
            };

            let rc = Arc::new(RefCell::new(f));

            rc.borrow_mut()
                .bingo
                .add_slot(rc.clone(), &Fancy::check_bingo);

            rc
        }

        fn check_bingo(&mut self, _: &i32) {
            if *self.bingo == 42 {
                self.lucky = true
            } else {
                self.lucky = false
            }
        }
    }

    #[test]
    fn self_listening() {
        let x = Fancy::new();
        assert!((*x).borrow().lucky == false);

        x.borrow_mut().bingo.set(42);
        assert!((*x).borrow().lucky == true);
    }

    struct Dull {
        val: i32,
    }

    impl Dull {
        fn go_dull(&mut self, data: &i32) {
            self.val = *data;
        }
    }

    #[test]
    fn drop_mechanics() {
        let mut longlive: Si<i32> = Si::new(3);

        {
            let dull = Dull { val: -1 };
            let dull_rc = Arc::new(RefCell::new(dull));

            longlive.add_slot(dull_rc.clone(), &Dull::go_dull);

            assert_eq!((*dull_rc).borrow().val, 3);

            assert_eq!(longlive.slots.len(), 1);
        }

        longlive.set(1);
        assert_eq!(longlive.slots.len(), 0);
    }

    struct TwoSlots {
        price: Si<i32>,
        quantity: Si<i32>,
        total_cached: i32,
    }

    impl TwoSlots {
        fn new() -> Arc<RefCell<Self>> {
            let s = Self {
                price: Si::new(10),
                quantity: Si::new(5),
                total_cached: -1,
            };

            let rc = Arc::new(RefCell::new(s));

            rc.borrow_mut()
                .price
                .add_slot(rc.clone(), &TwoSlots::recache_total);

            rc.borrow_mut()
                .quantity
                .add_slot(rc.clone(), &TwoSlots::recache_total);

            rc
        }

        fn recache_total(&mut self, _: &i32) {
            self.total_cached = *self.price * *self.quantity;
        }
    }

    #[test]
    fn two_slots() {
        let x = TwoSlots::new();
        assert!((*x).borrow().total_cached == 50);

        x.borrow_mut().quantity.set(7);
        assert!((*x).borrow().total_cached == 70);

        x.borrow_mut().price.set(6);
        assert!((*x).borrow().total_cached == 42);
    }
}
