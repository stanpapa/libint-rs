use std::ops::{Deref, Index};

use cxx::UniquePtr;
use libint_sys::basis as ffi;

use crate::{Atom, Shell};

pub struct BasisSet(UniquePtr<ffi::BasisSet>);

impl Deref for BasisSet {
    type Target = UniquePtr<ffi::BasisSet>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BasisSet {
    /// # Errors
    ///
    /// Returns an error if `name` does not match a known basis set.
    pub fn new(name: &str, atoms: &[Atom]) -> Result<Self, cxx::Exception> {
        let ptrs = atoms.iter().map(Atom::as_ptr).collect::<Vec<_>>();
        Ok(Self(unsafe {
            ffi::basis(name, ptrs.as_ptr(), ptrs.len())?
        }))
    }

    /// if `false` use Cartesian Gaussians
    pub fn set_pure(&mut self, solid: bool) {
        ffi::set_pure(self.0.pin_mut(), solid);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        ffi::nshells(self)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    pub fn at(&self, i: usize) -> Shell {
        Shell::from(ffi::at(self, i).unwrap_or_else(|e| panic!("shell index {i} out of bounds: {e}")))
    }

    pub fn shells(&self) -> Vec<Shell> {
        (0..self.len()).map(|i| self.at(i)).collect()
    }

    pub fn shell2bf(&self) -> Vec<usize> {
        ffi::shell2bf(self).iter().copied().collect()
    }

    pub fn nbf(&self) -> usize {
        ffi::nbf(self)
    }

    pub fn max_nprim(&self) -> usize {
        ffi::max_nprim(self)
    }

    pub fn max_l(&self) -> usize {
        ffi::max_l(self)
    }

    pub fn iter(&self) -> std::vec::IntoIter<Shell> {
        <&Self as IntoIterator>::into_iter(self)
    }
}

impl IntoIterator for BasisSet {
    type Item = Shell;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.shells().into_iter()
    }
}

impl IntoIterator for &BasisSet {
    type Item = Shell;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.shells().into_iter()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn def2_svp() {
        let xyz = r"3

O   0.0000   0.0000   0.0626
H  -0.7920   0.0000  -0.4973
H   0.7920   0.0000  -0.4973
    ";
        let atoms = crate::atom::read_dotxyz_str(xyz).unwrap();
        let basis = super::BasisSet::new("def2-SVP", &atoms).unwrap();
        assert_eq!(basis.len(), 12);
        assert_eq!(basis.nbf(), 24);
        assert_eq!(basis.max_nprim(), 5);
        assert_eq!(basis.max_l(), 2);
    }

    #[test]
    fn unknown_name_errors() {
        let atoms = vec![crate::Atom::new(1, 0., 0., 0.)];
        assert!(super::BasisSet::new("not-a-real-basis-set", &atoms).is_err());
    }

    #[test]
    #[should_panic(expected = "shell index 1000 out of bounds")]
    fn at_out_of_bounds_panics() {
        let atoms = vec![crate::Atom::new(1, 0., 0., 0.)];
        let basis = super::BasisSet::new("def2-SVP", &atoms).unwrap();
        basis.at(1000);
    }
}
