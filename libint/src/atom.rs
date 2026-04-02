use std::{fmt::Debug, num::ParseFloatError, pin::Pin, str::FromStr};

use libint_sys::{UniquePtr, atom as ffi};

use crate::{
    element::{Element, ElementError},
    utils::AsPin,
};

pub struct Atom(pub(crate) UniquePtr<ffi::Atom>);

impl AsPin for Atom {
    type T = ffi::Atom;

    fn as_pin(&self) -> std::pin::Pin<&Self::T> {
        unsafe { Pin::new_unchecked(self.0.as_ref().unwrap()) }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AtomError {
    #[error(transparent)]
    InvalidElement(#[from] ElementError),
    #[error("invalid {0} coordinate: {1}")]
    InvalidCoordinate(String, ParseFloatError),
}

impl FromStr for Atom {
    type Err = AtomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let element = iter
            .next()
            .map(Element::from_str)
            .expect("Missing element")?;
        let x = iter
            .next()
            .map(str::parse::<f64>)
            .expect("missing x-coordinate")
            .map_err(|e| AtomError::InvalidCoordinate(String::from("x"), e))?;
        let y = iter
            .next()
            .map(str::parse::<f64>)
            .expect("missing y-coordinate")
            .map_err(|e| AtomError::InvalidCoordinate(String::from("y"), e))?;
        let z = iter
            .next()
            .map(str::parse::<f64>)
            .expect("missing z-coordinate")
            .map_err(|e| AtomError::InvalidCoordinate(String::from("z"), e))?;

        Ok(Atom::new(element.atomic_number(), x, y, z))
    }
}

impl Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Atom")
            .field("atomic_number", &self.atomic_number())
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .finish()
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.atomic_number() == other.atomic_number()
            && self.x() == other.x()
            && self.y() == other.y()
            && self.z() == other.z()
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

    pub(crate) fn as_ptr(&self) -> *const ffi::Atom {
        self.0.as_ptr()
    }
}

pub fn read_dotxyz(path: std::path::PathBuf) -> std::io::Result<Vec<Atom>> {
    read_dotxyz_str(&std::fs::read_to_string(path)?)
}

pub fn read_dotxyz_str(xyz: &str) -> std::io::Result<Vec<Atom>> {
    let mut lines = xyz.lines();

    let n = lines
        .next()
        .expect("missing number of atoms")
        .parse::<usize>()
        .expect("invalid number of atoms");
    let _ = lines.next(); // skip comment line

    let atoms = lines
        .filter(|l| !l.trim().is_empty())
        .map(|l| Atom::from_str(l).expect("invalid atom"))
        .collect::<Vec<_>>();
    if atoms.len() != n {
        return Err(std::io::Error::other(
            "number of atoms does not match number of coordinates",
        ));
    }

    Ok(atoms)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

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

    #[test]
    fn from_str() {
        let atom = Atom::from_str("Sn 0.0 1.0 2.0");
        let r = Atom::new(50, 0., 1., 2.);
        assert_eq!(atom.unwrap(), r);
    }

    #[test]
    fn read_dotxyz() {
        let xyz = r"3

O   0.0000   0.0000   0.0626
H  -0.7920   0.0000  -0.4973
H   0.7920   0.0000  -0.4973
    ";
        let atoms = super::read_dotxyz_str(xyz).unwrap();
        assert_eq!(atoms.len(), 3);
        assert_eq!(atoms[0], Atom::new(8, 0., 0., 0.0626));
        assert_eq!(atoms[1], Atom::new(1, -0.7920, 0., -0.4973));
        assert_eq!(atoms[2], Atom::new(1, 0.7920, 0., -0.4973));
    }
}
