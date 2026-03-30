use cxx::{CxxString, CxxVector, UniquePtr};

#[cxx::bridge(namespace = "libint2")]
pub mod ffi {
    unsafe extern "C++" {
        include!("libint-sys/shim/include/atom.h");

        type Atom;

        #[must_use]
        fn atom(atomic_number: i32, x: f64, y: f64, z: f64) -> UniquePtr<Atom>;

        // getters
        #[must_use]
        fn atomic_number(atom: Pin<&Atom>) -> i32;
        #[must_use]
        fn x(atom: Pin<&Atom>) -> f64;
        #[must_use]
        fn y(atom: Pin<&Atom>) -> f64;
        #[must_use]
        fn z(atom: Pin<&Atom>) -> f64;
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use cxx::let_cxx_string;

    use super::ffi;

    #[test]
    #[allow(clippy::float_cmp)]
    fn atom() {
        let atom = ffi::atom(1, 0., 1., 2.);
        assert_eq!(
            unsafe { ffi::atomic_number(Pin::new_unchecked(atom.as_ref().unwrap())) },
            1
        );
        assert_eq!(
            unsafe { ffi::x(Pin::new_unchecked(atom.as_ref().unwrap())) },
            0.
        );
        assert_eq!(
            unsafe { ffi::y(Pin::new_unchecked(atom.as_ref().unwrap())) },
            1.
        );
        assert_eq!(
            unsafe { ffi::z(Pin::new_unchecked(atom.as_ref().unwrap())) },
            2.
        );
    }
}
