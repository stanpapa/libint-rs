#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint-sys/shim/include/engine.h");

        type Engine;
        type Operator;
        type Shell = crate::shell::Shell;

        #[must_use]
        fn engine(
            oper: i32,
            max_nprim: usize,
            max_l: usize,
            deriv_order: usize,
        ) -> UniquePtr<Engine>;

        fn nshellsets(engine: &Engine) -> usize;
        fn results(engine: &Engine) -> Vec<usize>;
        fn compute1(engine: Pin<&mut Engine>, s1: &Shell, s2: &Shell);
    }
}
