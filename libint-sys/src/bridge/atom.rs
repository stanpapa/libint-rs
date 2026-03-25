#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint-sys/shim/include/atom.h");

        type Atom;

        // fn atom_value() -> Atom;
        fn atom(atomic_number: i32, x: f64, y: f64, z: f64) -> UniquePtr<Atom>;

        // fn read_dotxyz(is: &str, bohr_to_angstrom: f64) -> CxxVector<Atom>;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn atom() {
        let atom = super::ffi::atom(1, 0., 1., 2.);
        assert!(false);
    }
}
