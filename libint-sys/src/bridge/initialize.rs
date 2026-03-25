#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint2/initialize.h");
        // include!("libint2/libint2_iface.h");
        // include!("libint2.hpp");
        // include!("libint-sys/shim/include/initialize.h");

        fn initialize(verbose: bool);
        fn finalize();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn initialize() {
        super::ffi::initialize(true);
        super::ffi::finalize();
        assert!(false);
    }
}
