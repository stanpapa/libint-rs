mod atom;
mod element;
mod utils;

pub use atom::Atom;
pub use libint_sys::initialize::{finalize, initialize};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn initialize() {
//         crate::initialize(true);
//         assert!(false);
//     }
// }
