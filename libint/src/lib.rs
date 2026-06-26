mod atom;
mod basis;
mod element;
mod shell;

pub use atom::Atom;
pub use basis::BasisSet;
pub use libint_sys::initialize::{finalize, initialize};
pub use shell::Shell;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn initialize() {
//         crate::initialize(true);
//         assert!(false);
//     }
// }
