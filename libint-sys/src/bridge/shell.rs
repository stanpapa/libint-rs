#[cxx::bridge(namespace = "libint2")]
pub mod ffi {

    unsafe extern "C++" {
        include!("libint2/shell.h");

        type Shell;

        #[Self = "Shell"]
        fn am_symbol(l: usize) -> c_char;
        #[Self = "Shell"]
        fn am_symbol_to_l(am_symbol: c_char) -> u16;
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
