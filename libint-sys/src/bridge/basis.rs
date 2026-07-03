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

        fn set_pure(basis: Pin<&mut BasisSet>, solid: bool);

        /// Number of [`Shell`]s in the basis.
        #[must_use]
        fn nshells(basis: &BasisSet) -> usize;
        fn nbf(basis: &BasisSet) -> usize;
        fn max_nprim(basis: &BasisSet) -> usize;
        fn max_l(basis: &BasisSet) -> usize;
        fn shell2bf(basis: &BasisSet) -> &CxxVector<usize>;

        fn at(basis: &BasisSet, i: usize) -> UniquePtr<Shell>;
    }
}
