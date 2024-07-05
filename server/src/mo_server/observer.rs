use std::{cell::RefCell, ops::Deref, rc::Weak};

pub struct Lv<Data>
where
    Data: PartialEq,
{
    data: Data,
    ears: Vec<Box<dyn Callback<Data>>>,
}

impl<Data> Lv<Data>
where
    Data: PartialEq,
{
    pub fn new(data: Data) -> Self {
        Self {
            data,
            ears: Vec::new(),
        }
    }
    
    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn set(&mut self, data: Data) {
        if self.data == data {
            return;
        }
        self.data = data;

        self.ears.retain(|ear| ear.is_actual());
        for ear in &self.ears {
            ear.notify(&self.data);
        }
    }

    pub fn add_ear(&mut self, ear: Box<dyn Callback<Data>>) {
        if !ear.is_actual() {
            return;
        };

        // if self.ears.contains(&ear) {return;} // No idea

        ear.notify(&self.data);
        self.ears.push(ear);
    }
}

impl<Data> Deref for Lv<Data>
where
    Data: PartialEq,
{
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// impl<Data> Fn() for Lv<Data> {
//     extern "rust-call" fn call(&self, args: Args) -> Self::Output {
//         todo!()
//     }
// }

pub struct CallbackClosure<T, Data>
where
    Data: PartialEq,
{
    object: Weak<RefCell<T>>,
    pointer: Box<dyn Fn(&mut T, &Data)>,
}

/// Use CallbackClosure to add callback
pub trait Callback<Data> {
    fn is_actual(&self) -> bool;
    fn notify(&self, new_data: &Data);
}

impl<T, Data> Callback<Data> for CallbackClosure<T, Data>
where
    Data: PartialEq,
{
    fn is_actual(&self) -> bool {
        self.object.upgrade().is_some()
    }

    fn notify(&self, data: &Data) {
        if !self.is_actual() {
            return;
        };
        let object = unsafe { self.object.upgrade().unwrap().as_ptr().as_mut().unwrap() };
        (self.pointer)(object, &data);
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    struct Fancy {
        bingo: Lv<i32>,
        lucky: bool,
    }

    impl Fancy {
        fn new() -> Rc<RefCell<Fancy>> {
            let f = Fancy {
                bingo: Lv::new(0),
                lucky: false,
            };

            let rc = Rc::new(RefCell::new(f));

            rc.borrow_mut()
                .bingo
                .add_ear(Box::new(CallbackClosure::<Fancy, i32> {
                    object: Rc::downgrade(&rc),
                    pointer: Box::new(&Fancy::check_bingo),
                }));

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
    fn self_update() {
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
        let mut longlive: Lv<i32> = Lv::new(3);

        {
            let dull = Dull { val: -1 };
            let dull_rc = Rc::new(RefCell::new(dull));

            longlive.add_ear(Box::new(CallbackClosure::<Dull, i32> {
                object: Rc::downgrade(&dull_rc),
                pointer: Box::new(&Dull::go_dull),
            }));

            assert_eq!((*dull_rc).borrow().val, 3);

            assert_eq!(longlive.ears.len(), 1);
        }

        longlive.set(1);
        assert_eq!(longlive.ears.len(), 0);
    }

    struct TwoEars {
        price: Lv<i32>,
        quantity: Lv<i32>,
        total_cached: i32,
    }

    impl TwoEars {
        fn new() -> Rc<RefCell<Self>> {
            let s = Self {
                price: Lv::new(10),
                quantity: Lv::new(5),
                total_cached: -1,
            };

            let rc = Rc::new(RefCell::new(s));

            rc.borrow_mut()
                .price
                .add_ear(Box::new(CallbackClosure::<TwoEars, i32> {
                    object: Rc::downgrade(&rc),
                    pointer: Box::new(&TwoEars::recache_total),
                }));

            rc.borrow_mut()
                .quantity
                .add_ear(Box::new(CallbackClosure::<TwoEars, i32> {
                    object: Rc::downgrade(&rc),
                    pointer: Box::new(&TwoEars::recache_total),
                }));

            rc
        }

        fn recache_total(&mut self, _: &i32) {
            self.total_cached = *self.price * *self.quantity;
        }
    }

    #[test]
    fn two_ears() {
        let x = TwoEars::new();
        assert!((*x).borrow().total_cached == 50);

        x.borrow_mut().quantity.set(7);
        assert!((*x).borrow().total_cached == 70);

        x.borrow_mut().price.set(6);
        assert!((*x).borrow().total_cached == 42);
    }
}
