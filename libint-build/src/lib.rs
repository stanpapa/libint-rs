//! Helper crate for libint-src/build.rs
//!
//! Libint is either downloaded from Github, or detected on your system through `cmake`.
//!
//! Requirements
//! ------------
//!
//! This crate executes `cmake` as external command.

mod build;
mod download;
// pub mod error;
pub use build::Configure;
pub use download::*;
