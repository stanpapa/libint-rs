mod atom;
mod basis;
mod element;
mod utils;

pub use atom::Atom;
pub use basis::BasisSet;
pub use libint_sys::initialize::{finalize, initialize};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn initialize() {
//         crate::initialize(true);
//         assert!(false);
//     }
// }
