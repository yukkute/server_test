// use std::{
//     cell::{Ref, RefCell},
//     sync::Arc,
// };

// use super::{bank::Bank, ingame_types::Money, observer::Si};

// struct Clicker {
//     clicks_made: Si<u64>,
//     earned_by_clicking: Si<Money>,
// }

// impl Clicker {
//     fn new(bank: Option<Arc<RefCell<Bank>>>) -> Arc<RefCell<Self>> {
//         let c = Clicker {
//             clicks_made: Si::new(0),
//             earned_by_clicking: Si::new(0.0),
//         };

//         let rc = Arc::new(RefCell::new(c));

//         rc
//     }

//     pub fn click(&self) {
//         self.clicks_made.set(*self.clicks_made + 1);
//     }
// }

// #[cfg(test)]
// mod tests {
//     fn integration_with_clicker() {}
// }
