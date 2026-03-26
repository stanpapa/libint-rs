#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint-sys/shim/include/atom.h");

        type Atom;

        // fn atom_value() -> Atom;
        fn atom(atomic_number: i32, x: f64, y: f64, z: f64) -> UniquePtr<Atom>;

        // getters
        fn atomic_number(atom: Pin<&Atom>) -> i32;
        fn x(atom: Pin<&Atom>) -> f64;
        fn y(atom: Pin<&Atom>) -> f64;
        fn z(atom: Pin<&Atom>) -> f64;

        // fn read_dotxyz(is: &str, bohr_to_angstrom: f64) -> CxxVector<Atom>;
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    #[test]
    #[allow(clippy::float_cmp)]
    fn atom() {
        let atom = super::ffi::atom(1, 0., 1., 2.);
        assert_eq!(
            unsafe { super::ffi::atomic_number(Pin::new_unchecked(atom.as_ref().unwrap())) },
            1
        );
        assert_eq!(
            unsafe { super::ffi::x(Pin::new_unchecked(atom.as_ref().unwrap())) },
            0.
        );
        assert_eq!(
            unsafe { super::ffi::y(Pin::new_unchecked(atom.as_ref().unwrap())) },
            1.
        );
        assert_eq!(
            unsafe { super::ffi::z(Pin::new_unchecked(atom.as_ref().unwrap())) },
            2.
        );
    }
}
