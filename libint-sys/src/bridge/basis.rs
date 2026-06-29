use cxx::CxxVector;

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

        /// Number of [`Shell`]s in the basis.
        #[must_use]
        fn nshells(basis: &BasisSet) -> usize;

        fn set_pure(basis: Pin<&mut BasisSet>, solid: bool);
        fn at(basis: &BasisSet, i: usize) -> UniquePtr<Shell>;
        fn nbf(basis: &BasisSet) -> usize;
        fn max_nprim(basis: &BasisSet) -> usize;
        fn max_l(basis: &BasisSet) -> usize;
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
        // assert_eq!(ffi::nshells(&basis), 12);
        // for i in ffi::shells_alt(&basis).iter() {
        //     println!("{i}");
        // }
        assert!(false);
    }
}
