use cxx::UniquePtr;

#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint-sys/shim/include/shell.h");

        // -----------------------------------------------------
        // Shell
        // -----------------------------------------------------
        type Shell;

        fn shell() -> UniquePtr<Shell>;

        // rust::Vec<double> alpha(const Shell& shell);
        // // std::vector<Contraction> contr(const Shell& shell);
        // std::array<double, 3> O(const Shell& shell);
        // rust::Vec<double> max_ln_coeff(const Shell& shell);
        fn alpha(shell: Pin<&Shell>) -> Vec<f64>;
        fn contr(shell: Pin<&Shell>) -> *const *const Contraction;
        fn O(shell: Pin<&Shell>) -> [f64; 3];
        fn max_ln_coeff(shell: Pin<&Shell>) -> Vec<f64>;

        #[Self = "Shell"]
        #[must_use]
        fn am_symbol(l: usize) -> c_char;
        #[Self = "Shell"]
        #[must_use]
        fn am_symbol_to_l(am_symbol: c_char) -> u16;

        // -----------------------------------------------------
        // Contraction
        // -----------------------------------------------------
        type Contraction;

        fn contraction() -> UniquePtr<Contraction>;

        // getters
        #[must_use]
        fn l(contraction: Pin<&Contraction>) -> i32;
        #[must_use]
        fn pure(contraction: Pin<&Contraction>) -> bool;
        #[must_use]
        fn coeff(contraction: Pin<&Contraction>) -> Vec<f64>;

        #[must_use]
        fn cartesian_size(contraction: Pin<&Contraction>) -> usize;
        #[must_use]
        fn size(contraction: Pin<&Contraction>) -> usize;

    }
}

#[cfg(test)]
mod tests {
    use super::ffi::Shell;

    #[test]
    fn am_symbol() {
        assert_eq!(Shell::am_symbol(0), 115); // s
        assert_eq!(Shell::am_symbol(1), 112); // p
    }

    #[test]
    fn am_symbol_to_l() {
        assert_eq!(Shell::am_symbol_to_l(115), 0); // s
        assert_eq!(Shell::am_symbol_to_l(112), 1); // p
    }
}
