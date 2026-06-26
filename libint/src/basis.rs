use std::ops::{Deref, DerefMut, Index};

use libint_sys::{UniquePtr, basis as ffi};

use crate::{Atom, Shell};

pub struct BasisSet(UniquePtr<ffi::BasisSet>);

impl Deref for BasisSet {
    type Target = UniquePtr<ffi::BasisSet>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BasisSet {
    pub fn new(name: &str, atoms: &[Atom]) -> Self {
        let ptrs = atoms.iter().map(Atom::as_ptr).collect::<Vec<_>>();
        Self(unsafe { ffi::basis(name, ptrs.as_ptr(), ptrs.len()) })
    }

    #[must_use]
    pub fn len(&self) -> usize {
        ffi::nshells(self)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn at(&self, i: usize) -> Shell {
        Shell::from(ffi::at(self, i))
    }

    pub fn set_pure(&mut self, solid: bool) {
        ffi::set_pure(self.0.pin_mut(), solid);
    }

    pub fn nbf(basis: &BasisSet) -> usize {
        ffi::nbf(basis)
    }

    pub fn max_nprim(basis: &BasisSet) -> usize {
        ffi::max_nprim(basis)
    }

    pub fn max_l(basis: &BasisSet) -> usize {
        ffi::max_l(basis)
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
        let basis = super::BasisSet::new("def2-SVP", &atoms);
        assert_eq!(basis.len(), 12);
    }
}
