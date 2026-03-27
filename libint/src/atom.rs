use std::pin::Pin;

use libint_sys::atom as ffi;

use crate::utils::AsPin;

pub struct Atom(libint_sys::UniquePtr<ffi::Atom>);

impl AsPin for Atom {
    type T = ffi::Atom;

    fn as_pin(&self) -> std::pin::Pin<&Self::T> {
        unsafe { Pin::new_unchecked(self.0.as_ref().unwrap()) }
    }
}

impl Atom {
    /// Construct atom with atomic number and coordinates.
    ///
    /// # Panics
    ///
    /// Panics if an invalid atomic number was entered.
    #[must_use]
    pub fn new(atomic_number: u8, x: f64, y: f64, z: f64) -> Self {
        if atomic_number <= 118 {
            Self(ffi::atom(i32::from(atomic_number), x, y, z))
        } else {
            panic!("invalid atomic_number: {atomic_number}")
        }
    }

    /// Returns atomic number.
    ///
    /// # Panics
    ///
    /// Panics if [`Atom`] contains an invalid atomic number.
    #[must_use]
    pub fn atomic_number(&self) -> u8 {
        u8::try_from(ffi::atomic_number(self.as_pin())).expect("invalid atomic number")
    }

    /// Returns x coordinate.
    #[must_use]
    pub fn x(&self) -> f64 {
        ffi::x(self.as_pin())
    }

    /// Returns y coordinate.
    #[must_use]
    pub fn y(&self) -> f64 {
        ffi::y(self.as_pin())
    }

    /// Returns z coordinate.
    #[must_use]
    pub fn z(&self) -> f64 {
        ffi::z(self.as_pin())
    }
}

#[cfg(test)]
mod tests {
    use super::Atom;

    #[test]
    #[allow(clippy::float_cmp)]
    fn atom() {
        let atom = Atom::new(1, 0., 1., 2.);
        assert_eq!(atom.atomic_number(), 1);
        assert_eq!(atom.x(), 0.);
        assert_eq!(atom.y(), 1.);
        assert_eq!(atom.z(), 2.);
    }
}
