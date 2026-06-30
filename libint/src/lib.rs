mod atom;
mod basis;
mod element;
mod engine;
mod shell;

pub use atom::Atom;
pub use basis::BasisSet;
pub use engine::Engine;
pub use libint_sys::{
    engine::Operator,
    initialize::{finalize, initialize},
};
pub use shell::Shell;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn initialize() {
//         crate::initialize(true);
//         assert!(false);
//     }
// }
