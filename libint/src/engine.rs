use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index},
};

use cxx::UniquePtr;
use libint_sys::engine as ffi;

use crate::Shell;

pub struct Engine(UniquePtr<ffi::Engine>);

impl Deref for Engine {
    type Target = UniquePtr<ffi::Engine>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Engine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(i32)]
pub enum Operator {
    /// Overlap
    Overlap = 0,
    /// Electronic kinetic energy, i.e. \f$ -\frac{1}{2} \nabla^2 \f$
    Kinetic = 1,
    /// Coulomb potential due to point charges
    Nuclear = 2,
    /// erf-attenuated point-charge Coulomb operator,
    /// \f$ \mathrm{erf}(\omega r)/r \f$
    ErfNuclear = 3,
    /// erfc-attenuated point-charge Coulomb operator,
    /// \f$ \mathrm{erfc}(\omega r)/r \f$
    ErfcNuclear = 4,
    /// overlap + (Cartesian) electric dipole moment,
    /// \f$ x_O, y_O, z_O \f$, where
    /// \f$ x_O \equiv x - O_x \f$ is relative to
    /// origin \f$ \vec{O} \f$
    Emultipole1 = 5,
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

    pub fn nshellsets(&self) -> usize {
        ffi::nshellsets(self)
    }

    /// Vector of pointers to libint2 buffer
    fn results(&self) -> Vec<usize> {
        ffi::results(self)
    }

    /// Compute 1-electron integral for shell-pair
    pub fn compute1(&mut self, s1: &Shell, s2: &Shell) -> Buffer<'_> {
        ffi::compute1(self.pin_mut(), s1, s2);
        let ptrs = self.results();
        Buffer {
            ptrs,
            nbf: s1.len() * s2.len(),
            _marker: PhantomData,
        }
    }
}

pub struct Buffer<'a> {
    ptrs: Vec<usize>,
    nbf: usize,
    _marker: PhantomData<&'a Engine>,
}

impl Buffer<'_> {
    pub fn len(&self) -> usize {
        self.ptrs.len()
    }

    pub fn get(&self, i: usize) -> Option<&[f64]> {
        // reinterpret pointers as floats
        let ptr = self.ptrs[i] as *const f64;

        // libint2 returns null for shell-sets that screen to zero
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { std::slice::from_raw_parts(ptr, self.nbf) })
        }
    }
}

// impl Index<usize> for Buffer<'_> {
//     type Output = [f64];

//     fn index(&self, index: usize) -> &Self::Output {
//         // reinterpret pointers as floats
//         let ptr = self.ptrs[index] as *const f64;

//         // libint2 returns null for shell-sets that screen to zero
//         if ptr.is_null() {
//             &[]
//         } else {
//             unsafe { std::slice::from_raw_parts(ptr, self.nbf) }
//         }
//     }
// }

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
        let mut engine = Engine::new(Operator::Overlap, basis.max_nprim(), basis.max_l(), 0);
        let shell2bf = basis.shell2bf();

        for s1 in 0..basis.len() {
            for s2 in 0..basis.len() {
                let buffer = engine.compute1(&basis.at(s1), &basis.at(s2));
                let Some(ints) = buffer.get(0) else {
                    continue;
                };
                // println!("{}", buffer.len());
                // println!("{:?}", buffer.get(0));

                let bf1 = shell2bf[s1];
                let n1 = basis.at(s1).len();
                let bf2 = shell2bf[s2];
                let n2 = basis.at(s2).len();

                for f1 in 0..n1 {
                    for f2 in 0..n2 {
                        println!("{} {} {}", bf1 + f1, bf2 + f2, ints[f1 * n2 + f2]);
                    }
                }
            }
        }

        finalize();
        assert!(false);
    }

    #[test]
    fn dipole() {
        let xyz = r"3

O   0.0000   0.0000   0.0626
H  -0.7920   0.0000  -0.4973
H   0.7920   0.0000  -0.4973
    ";
        let atoms = crate::atom::read_dotxyz_str(xyz).unwrap();
        let basis = BasisSet::new("def2-SVP", &atoms);
        initialize(true);
        let mut engine = Engine::new(Operator::Emultipole1, basis.max_nprim(), basis.max_l(), 0);
        let shell2bf = basis.shell2bf();

        for s1 in 0..basis.len() {
            for s2 in 0..basis.len() {
                let buffer = engine.compute1(&basis.at(s1), &basis.at(s2));
                // println!("{}", buffer.len());
                // println!("{:?}", buffer.get(0));
                // println!("{:?}", buffer.get(1));
                // println!("{:?}", buffer.get(2));
                // println!("{:?}", buffer.get(3));
            }
        }

        finalize();
        assert!(false);
    }
}
