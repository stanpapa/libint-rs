use cxx::UniquePtr;

#[cxx::bridge(namespace = "libint2")]
pub mod ffi {
    unsafe extern "C++" {
        include!("libint-sys/shim/include/atom.h");

        type Atom;

        #[must_use]
        fn atom(atomic_number: i32, x: f64, y: f64, z: f64) -> UniquePtr<Atom>;

        // getters
        #[must_use]
        fn atomic_number(atom: &Atom) -> i32;
        #[must_use]
        fn x(atom: &Atom) -> f64;
        #[must_use]
        fn y(atom: &Atom) -> f64;
        #[must_use]
        fn z(atom: &Atom) -> f64;
    }
}

#[cfg(test)]
mod tests {
    use super::ffi;

    #[test]
    #[allow(clippy::float_cmp)]
    fn atom() {
        let atom = ffi::atom(1, 0., 1., 2.);
        assert_eq!(ffi::atomic_number(&atom), 1);
        assert_eq!(ffi::x(&atom), 0.);
        assert_eq!(ffi::y(&atom), 1.);
        assert_eq!(ffi::z(&atom), 2.);
    }
}
