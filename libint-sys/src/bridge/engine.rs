#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint-sys/shim/include/engine.h");

        type Engine;
        type Operator;

        #[must_use]
        fn engine(
            oper: i32,
            max_nprim: usize,
            max_l: usize,
            deriv_order: usize,
        ) -> UniquePtr<Engine>;
    }
}
