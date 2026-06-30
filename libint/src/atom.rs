use std::{
    fmt::Debug,
    num::ParseFloatError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use cxx::UniquePtr;
use libint_sys::atom as ffi;

const BOHR_TO_ANGSTROM: f64 = 0.529_177_210_903;
const ANGSTROM_TO_BOHR: f64 = 1. / BOHR_TO_ANGSTROM;

use crate::element::{Element, ElementError};

pub struct Atom(UniquePtr<ffi::Atom>);

impl Deref for Atom {
    type Target = UniquePtr<ffi::Atom>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Atom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
impl approx::AbsDiffEq for Atom {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.atomic_number() == other.atomic_number()
            && approx::abs_diff_eq!(self.x(), other.x(), epsilon = epsilon)
            && approx::abs_diff_eq!(self.y(), other.y(), epsilon = epsilon)
            && approx::abs_diff_eq!(self.z(), other.z(), epsilon = epsilon)
    }
}

#[cfg(test)]
impl approx::RelativeEq for Atom {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.atomic_number() == other.atomic_number()
            && approx::relative_eq!(
                self.x(),
                other.x(),
                epsilon = epsilon,
                max_relative = max_relative
            )
            && approx::relative_eq!(
                self.y(),
                other.y(),
                epsilon = epsilon,
                max_relative = max_relative
            )
            && approx::relative_eq!(
                self.z(),
                other.z(),
                epsilon = epsilon,
                max_relative = max_relative
            )
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
        u8::try_from(ffi::atomic_number(self)).expect("invalid atomic number")
    }

    /// Returns x coordinate.
    #[must_use]
    pub fn x(&self) -> f64 {
        ffi::x(self)
    }

    /// Returns y coordinate.
    #[must_use]
    pub fn y(&self) -> f64 {
        ffi::y(self)
    }

    /// Returns z coordinate.
    #[must_use]
    pub fn z(&self) -> f64 {
        ffi::z(self)
    }

    pub(crate) fn as_ptr(&self) -> *const ffi::Atom {
        self.0.as_ptr()
    }

    fn scale(&mut self, factor: f64) {
        ffi::scale(self.pin_mut(), factor);
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
        .map(|l| {
            let mut atom = Atom::from_str(l).expect("invalid atom");
            atom.scale(ANGSTROM_TO_BOHR);
            atom
        })
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

    use approx::assert_relative_eq;

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
        assert_relative_eq!(atoms[0], Atom::new(8, 0., 0., 0.11830), epsilon = 1e-5);
        assert_relative_eq!(
            atoms[1],
            Atom::new(1, -1.49667, 0., -0.93976),
            epsilon = 1e-5
        );
        assert_relative_eq!(
            atoms[2],
            Atom::new(1, 1.49667, 0., -0.93976),
            epsilon = 1e-5
        );
    }
}
