use libint_sys::{UniquePtr, basis as ffi};

use crate::Atom;

pub struct BasisSet(UniquePtr<ffi::BasisSet>);

impl BasisSet {
    pub fn new(name: &str, atoms: &[Atom]) -> Self {
        let ptrs = atoms.iter().map(Atom::as_ptr).collect::<Vec<_>>();
        Self(unsafe { ffi::basis(name, ptrs.as_ptr(), ptrs.len()) })
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
        assert!(false);
    }
}
