use std::pin::Pin;

use libint_sys::{UniquePtr, basis as ffi};

use crate::{Atom, utils::AsPin};

pub struct BasisSet(UniquePtr<ffi::BasisSet>);

impl AsPin for BasisSet {
    type T = ffi::BasisSet;

    fn as_pin(&self) -> Pin<&Self::T> {
        unsafe { Pin::new_unchecked(self.0.as_ref().unwrap()) }
    }
}

impl BasisSet {
    pub fn new(name: &str, atoms: &[Atom]) -> Self {
        let ptrs = atoms.iter().map(Atom::as_ptr).collect::<Vec<_>>();
        Self(unsafe { ffi::basis(name, ptrs.as_ptr(), ptrs.len()) })
    }

    pub fn size(&self) -> usize {
        ffi::nshells(self.as_pin())
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
        assert_eq!(basis.size(), 12);
    }
}
