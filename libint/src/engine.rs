use cxx::UniquePtr;
use libint_sys::engine as ffi;

pub struct Engine(UniquePtr<ffi::Engine>);

#[repr(i32)]
pub enum Operator {
    Overlap = 0,
    Kinetic = 1,
    Nuclear = 2,
    // erf_nuclear = 3,
    // erfc_nuclear = 4,
    // emultipole1 = 5,
    // emultipole2 = 6,
    // emultipole3 = 7,
    // sphemultipole = 8,
    // opVop = 9,
    // delta = 10,
    // coulomb = 11,
    // r12_m1 = 11, // alias
    // cgtg = 12,
    // cgtg_x_coulomb = 13,
    // delcgtg2 = 14,
    // r12 = 15,
    // r12_1 = 16,
    // erf_coulomb = 17,
    // erfc_coulomb = 18,
    // stg = 19,
    Invalid = -1,
}

impl Engine {
    pub fn new(operator: Operator, max_nprim: usize, max_l: usize, deriv_order: usize) -> Engine {
        Engine(ffi::engine(operator as i32, max_nprim, max_l, deriv_order))
    }
}

#[cfg(test)]
mod tests {
    use libint_sys::initialize::{finalize, initialize};

    use crate::{BasisSet, Engine, engine::Operator};

    #[test]
    fn overlap() {
        let xyz = r"3

O   0.0000   0.0000   0.0626
H  -0.7920   0.0000  -0.4973
H   0.7920   0.0000  -0.4973
    ";
        let atoms = crate::atom::read_dotxyz_str(xyz).unwrap();
        let basis = BasisSet::new("def2-SVP", &atoms);
        initialize(true);
        let engine = Engine::new(Operator::Overlap, basis.max_nprim(), basis.max_l(), 0);
        finalize();
        assert!(false);
    }
}
