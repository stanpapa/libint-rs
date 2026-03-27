#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint2/initialize.h");

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
    }
}
