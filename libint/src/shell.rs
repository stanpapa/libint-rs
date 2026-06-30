use std::{fmt::Debug, ops::Deref};

use cxx::UniquePtr;
use libint_sys::shell as ffi;

pub struct Shell(UniquePtr<ffi::Shell>);

impl Deref for Shell {
    type Target = UniquePtr<ffi::Shell>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shell")
            .field("alpha", &self.alpha())
            .field("contr", &self.contractions().collect::<Vec<_>>())
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

impl From<UniquePtr<ffi::Shell>> for Shell {
    fn from(value: UniquePtr<ffi::Shell>) -> Self {
        Self(value)
    }
}

impl Shell {
    fn alpha(&self) -> Vec<f64> {
        ffi::alpha(self)
    }

    fn contractions(&self) -> impl Iterator<Item = Contraction<'_>> {
        (0..self.ncontr()).map(|i| Contraction(ffi::at_contraction(self, i)))
    }

    fn origin(&self) -> [f64; 3] {
        ffi::O(self)
    }

    fn max_ln_coeff(&self) -> Vec<f64> {
        ffi::max_ln_coeff(self)
    }

    fn len_cartesian(&self) -> usize {
        ffi::cartesian_size_shell(self)
    }

    pub fn len(&self) -> usize {
        ffi::size_shell(self)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn ncontr(&self) -> usize {
        ffi::ncontr(self)
    }

    fn nprim(&self) -> usize {
        ffi::nprim(self)
    }
}

struct Contraction<'a>(&'a ffi::Contraction);

impl Deref for Contraction<'_> {
    type Target = ffi::Contraction;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Debug for Contraction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Contraction")
            .field("l", &self.l())
            .field("pure", &self.pure())
            .field("coeff", &self.coeff())
            .finish()
    }
}

impl PartialEq for Contraction<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.l() == other.l() && self.pure() == other.pure() && self.coeff() == other.coeff()
    }
}

impl Contraction<'_> {
    fn l(&self) -> i32 {
        ffi::l(self)
    }

    fn pure(&self) -> bool {
        ffi::pure(self)
    }

    fn coeff(&self) -> Vec<f64> {
        ffi::coeff(self)
    }

    fn len_cartesian(&self) -> usize {
        ffi::cartesian_size_contraction(self)
    }

    fn len(&self) -> usize {
        ffi::size_contraction(self)
    }
}

#[cfg(test)]
mod tests {}
