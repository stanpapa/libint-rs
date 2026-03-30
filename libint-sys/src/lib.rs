mod bridge;
pub use bridge::atom::ffi as atom;
pub use bridge::initialize::ffi as initialize;
pub use bridge::shell::ffi as shell;

// re-export, so `libint` doesn't have to declare `cxx` as a dependency
pub use cxx::UniquePtr;
