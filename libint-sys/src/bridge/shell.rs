use cxx::UniquePtr;

#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint-sys/shim/include/shell.h");

        // -----------------------------------------------------
        // Shell
        // -----------------------------------------------------
        type Shell;

        #[must_use]
        fn shell() -> UniquePtr<Shell>;

        #[must_use]
        fn alpha(shell: &Shell) -> Vec<f64>;
        #[must_use]
        fn contr(shell: &Shell) -> *const *const Contraction;
        #[must_use]
        fn O(shell: &Shell) -> [f64; 3];
        #[must_use]
        fn max_ln_coeff(shell: &Shell) -> Vec<f64>;

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

        #[must_use]
        fn contraction() -> UniquePtr<Contraction>;

        // getters
        #[must_use]
        fn l(contraction: &Contraction) -> i32;
        #[must_use]
        fn pure(contraction: &Contraction) -> bool;
        #[must_use]
        fn coeff(contraction: &Contraction) -> Vec<f64>;

        #[must_use]
        fn cartesian_size(contraction: &Contraction) -> usize;
        #[must_use]
        fn size(contraction: &Contraction) -> usize;

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
