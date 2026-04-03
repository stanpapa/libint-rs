use std::{fmt::Debug, pin::Pin};

use libint_sys::{UniquePtr, shell as ffi};

use crate::utils::AsPin;

pub struct Shell(pub(crate) UniquePtr<ffi::Shell>);

impl AsPin for Shell {
    type T = ffi::Shell;

    fn as_pin(&self) -> std::pin::Pin<&Self::T> {
        unsafe { Pin::new_unchecked(self.0.as_ref().unwrap()) }
    }
}

impl Debug for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shell")
            .field("alpha", &self.alpha())
            // .field("contr", &self.contr())
            .field("origin", &self.origin())
            .field("max_ln_coeff", &self.max_ln_coeff())
            .finish()
    }
}

impl PartialEq for Shell {
    fn eq(&self, other: &Self) -> bool {
        self.alpha() == other.alpha()
            // && self.contr() == other.contr()
            && self.origin() == other.origin()
            && self.max_ln_coeff() == other.max_ln_coeff()
    }
}

impl Shell {
    fn alpha(&self) -> Vec<f64> {
        ffi::alpha(self.as_pin())
    }

    // fn contr(&self) -> Vec<Contraction> {
    //     let ptrs = ffi::contr(self.as_pin());
    //     assert!(!ptrs.is_null(), "Contractions are not defined!");

    //     unsafe {
    //         std::slice::from_raw_parts(ptrs, self.alpha().len())
    //             .iter()
    //             .map(|&inner| {
    //                 assert!(!inner.is_null(), "inner pointer is null");
    //                 // Dereference each inner pointer and clone the value
    //                 Contraction(UniquePtr::from_raw(inner))
    //             })
    //             .collect()
    //     }
    // }

    fn origin(&self) -> [f64; 3] {
        ffi::O(self.as_pin())
    }

    fn max_ln_coeff(&self) -> Vec<f64> {
        ffi::max_ln_coeff(self.as_pin())
    }
}

struct Contraction(UniquePtr<ffi::Contraction>);

impl AsPin for Contraction {
    type T = ffi::Contraction;

    fn as_pin(&self) -> std::pin::Pin<&Self::T> {
        unsafe { Pin::new_unchecked(self.0.as_ref().unwrap()) }
    }
}

impl Debug for Contraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Contraction")
            .field("l", &self.l())
            .field("pure", &self.pure())
            .field("coeff", &self.coeff())
            .finish()
    }
}

impl PartialEq for Contraction {
    fn eq(&self, other: &Self) -> bool {
        self.l() == other.l() && self.pure() == other.pure() && self.coeff() == other.coeff()
    }
}

impl Contraction {
    fn l(&self) -> i32 {
        ffi::l(self.as_pin())
    }

    fn pure(&self) -> bool {
        ffi::pure(self.as_pin())
    }

    fn coeff(&self) -> Vec<f64> {
        ffi::coeff(self.as_pin())
    }
}

#[cfg(test)]
mod tests {}
