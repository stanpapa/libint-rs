//! Execute make of OpenBLAS, and its options

use std::{fmt::Display, path::Path, str::FromStr};

// use glob::glob;

// use crate::error::Error;

#[derive(Debug)]
pub struct Compiler(pub String);

impl Default for Compiler {
    /// Check the env variable `CXX`, otherwise default to `clang++`.
    fn default() -> Self {
        Self(std::env::var("CXX").unwrap_or("clang++".to_string()))
    }
}

// #[derive(Debug)]
// pub struct OptLevel(pub u32);

// impl Default for OptLevel {
//     fn default() -> Self {
//         Self(u32::from_str(&std::env::var("OPT_LEVEL").unwrap_or_default()).unwrap_or(3))
//     }
// }

#[derive(Debug, Default)]
pub enum CartGaussOrdering {
    #[default]
    Standard,
    Intv3,
    Gamess,
    Orca,
    Bagel,
}

impl Display for CartGaussOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                CartGaussOrdering::Standard => "standard",
                CartGaussOrdering::Intv3 => "intv3",
                CartGaussOrdering::Gamess => "gamess",
                CartGaussOrdering::Orca => "orca",
                CartGaussOrdering::Bagel => "bagel",
            }
        )
    }
}

/// Spherical harmonics shell ordering.
#[derive(Debug, Default)]
pub enum ShGaussOrdering {
    /// [`-l`,`l`]
    #[default]
    Standard,
    /// 0, 1, -1, 2, -2, ...
    Gaussian,
}

impl Display for ShGaussOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                ShGaussOrdering::Standard => "standard",
                ShGaussOrdering::Gaussian => "gaussian",
            }
        )
    }
}

#[derive(Debug, Default)]
pub enum ShellSet {
    #[default]
    Standard,
    Orca,
}

impl Display for ShellSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                ShellSet::Standard => "standard",
                ShellSet::Orca => "orca",
            }
        )
    }
}
/// make option generator
#[derive(Debug)]
pub struct Configure {
    /// maximum angular momentum of basis functions
    pub max_am: Vec<u8>,
    /// angular momentum for derivatives of 4c-2e integrals
    pub eri_max_am: Vec<u8>,
    /// angular momentum for derivatives of 2c-2e integrals
    pub eri2_max_am: Vec<u8>,
    /// angular momentum for derivatives of 3c-2e integrals
    pub eri3_max_am: Vec<u8>,
    /// angular momentum for derivatives of 4c-2e integrals (optimised)
    pub eri_opt_am: Vec<u8>,
    /// angular momentum for derivatives of 2c-2e integrals (optimised)
    pub eri2_opt_am: Vec<u8>,
    /// angular momentum for derivatives of 3c-2e integrals (optimised)
    pub eri3_opt_am: Vec<u8>,
    pub one_body: Option<u8>,
    pub multipole_max_order: Option<u8>,
    pub eri: Option<u8>,
    pub eri2: Option<u8>,
    pub eri3: Option<u8>,
    /// Cartesian shell ordering
    pub cartesian_ordering: CartGaussOrdering,
    pub spherical_ordering: ShGaussOrdering,
    pub shell_set: ShellSet,
}

impl Default for Configure {
    fn default() -> Self {
        Configure {
            max_am: vec![2, 2],
            eri_max_am: vec![2, 2],
            one_body: Some(1),
            multipole_max_order: Some(2),
            eri: Some(1),
            eri2_max_am: vec![],
            eri3_max_am: vec![],
            eri_opt_am: vec![],
            eri2_opt_am: vec![],
            eri3_opt_am: vec![],
            eri2: None,
            eri3: None,
            cartesian_ordering: CartGaussOrdering::default(),
            spherical_ordering: ShGaussOrdering::default(),
            shell_set: ShellSet::default(),
        }
    }
}

fn libint_list(v: Vec<u8>) -> String {
    v.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(";")
}

impl Configure {
    /// Build libint.
    ///
    /// Build directory will be `libint_root/build`. Library and header files will be installed in the build directory as well.
    ///
    /// # Errors
    ///
    /// Errors if the compilation fails. Stderr can be found in `libint_root/err.log`.
    ///
    /// # Panics
    ///
    /// Panics if log files cannot be created.
    pub fn build<P: AsRef<Path>>(self, libint_root: P) {
        let root = libint_root.as_ref();
        let build_dir = root.join("build");

        let mut config = cmake::Config::new(build_dir);

        let config = config
            .define("LIBINT2_MAX_AM", libint_list(self.max_am))
            .define("LIBINT2_ERI_MAX_AM", libint_list(self.eri_max_am))
            .define(
                "LIBINT2_CARTGAUSS_ORDERING",
                self.cartesian_ordering.to_string(),
            )
            .define(
                "LIBINT2_SHGAUSS_ORDERING",
                self.spherical_ordering.to_string(),
            )
            .define("LIBINT2_SHELL_SET", self.shell_set.to_string());

        let config = if let Some(one_body) = self.one_body {
            config.define("LIBINT2_ENABLE_ONEBODY", one_body.to_string())
        } else {
            config
        };

        let config = if let Some(multipole_max_order) = self.multipole_max_order {
            config.define(
                "LIBINT2_MULTIPLE_MAX_ORDER",
                multipole_max_order.to_string(),
            )
        } else {
            config
        };

        config.build();
    }
}

// #[cfg(test)]
// mod tests {
//     use std::{
//         fs,
//         path::{Path, PathBuf},
//     };

//     use crate::Configure;

//     /// Create test directory.
//     fn get_libint_source<P: AsRef<Path>>(out_dir: P) -> PathBuf {
//         // don't create the build directory if it already exists
//         let out_dir = out_dir.as_ref();
//         if out_dir.exists() {
//             return out_dir.to_path_buf();
//         }

//         // git clone Libint
//         let libint_src_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../libint-src/");
//         let source = crate::download(&libint_src_root).unwrap();

//         // copy files to the target directory
//         fs::create_dir_all(out_dir).unwrap();
//         for entry in walkdir::WalkDir::new(&source) {
//             let entry = entry.unwrap();
//             let src = entry.path();
//             let dest = out_dir.join(src.strip_prefix(&source).unwrap());
//             if entry.file_type().is_dir() {
//                 fs::create_dir_all(&dest).unwrap();
//             } else {
//                 fs::copy(src, dest).unwrap();
//             }
//         }

//         out_dir.to_path_buf()
//     }

//     #[test]
//     fn build_default() {
//         let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         let out_dir = root.join("test_build/build_default");
//         let opt = Configure::default();
//         opt.build(get_libint_source(&out_dir)).unwrap();
//     }
// }
