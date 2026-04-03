#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint-sys/shim/include/basis.h");

        type BasisSet;
        type Atom = crate::atom::Atom;
        type Shell = crate::shell::Shell;

        /// # Safety
        ///
        /// Make sure that `atoms` is not constructed with dangling pointers.
        /// This leads to C++ interpreting the pointers as garbage.
        #[must_use]
        unsafe fn basis(name: &str, atoms: *const *const Atom, n: usize) -> UniquePtr<BasisSet>;

        fn nshells(basis: Pin<&BasisSet>) -> usize;
        fn shells(basis: Pin<&BasisSet>) -> *const *const Shell;
    }
}

#[cfg(test)]
mod tests {
    use crate::bridge::atom::ffi::atom;

    use super::ffi;

    #[test]
    #[allow(clippy::float_cmp)]
    fn def2_svp() {
        let atoms = [
            atom(8, 0., 0., 0.0626),
            atom(1, -0.7920, 0.0000, -0.4973),
            atom(1, 0.7920, 0.0000, -0.4973),
        ];
        let ptrs = atoms.iter().map(cxx::UniquePtr::as_ptr).collect::<Vec<_>>();

        let basis = unsafe { ffi::basis("def2-SVP", ptrs.as_ptr(), ptrs.len()) };
        assert!(false);
    }
}
