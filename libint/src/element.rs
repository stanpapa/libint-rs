use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum ElementError {
    #[error("invalid number {0}")]
    InvalidNumber(u8),
    #[error("invalid symbol {0}")]
    InvalidSymbol(String),
    #[error("invalid name {0}")]
    InvalidName(String),
}

macro_rules! elements {
    (
        $(
            ($variant:ident, $atomic_number:expr, $symbol:expr, $name:expr),
        )+
    ) => {
        /// Chemical element
        #[allow(missing_docs)]
        #[derive(Clone, Copy, Debug)]
        pub enum Element {
            $(
                $variant = $atomic_number,
            )+
        }

        impl Element {
            /// Return the atomic number of the element
            pub fn atomic_number(&self) -> u8 {
                *self as u8
            }

            /// Return the symbol of the element
            pub fn symbol(&self) -> &'static str {
                match self {
                    $(
                        Element::$variant => $symbol,
                    )+
                }
            }

            /// Return the name of the element
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        Element::$variant => $name,
                    )+
                }
            }

            /// Construct an element from its atomic number
            fn from_atomic_number(atomic_number: u8) -> Result<Self, ElementError> {
                match atomic_number {
                    $(
                        $atomic_number => Ok(Self::$variant),
                    )+
                    _ => Err(ElementError::InvalidNumber(atomic_number)),
                }
            }

            /// Construct an element from its symbol
            pub fn from_symbol(symbol: &str) -> Result<Self, ElementError> {
                match symbol {
                    $(
                        $symbol => Ok(Self::$variant),
                    )+
                    _ => Err(ElementError::InvalidSymbol(symbol.to_string())),
                }
            }

            /// Construct an element from its name
            pub fn from_name(name: &str) -> Result<Self, ElementError> {
                match name {
                    $(
                        $name => Ok(Self::$variant),
                    )+
                    _ => Err(ElementError::InvalidName(name.to_string())),
                }
            }
        }

        /// Construct an element from either its name or its symbol
        impl FromStr for Element {
            type Err = ElementError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_name(s)
                .or_else(|_| Self::from_symbol(s))
            }
        }

        impl std::fmt::Display for Element {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.symbol())
            }
        }
    };
}

elements! {
    (H , 1  , "H" , "Hydrogen"     )   ,
    (He, 2  , "He", "Helium"       )   ,
    (Li, 3  , "Li", "Lithium"      )   ,
    (Be, 4  , "Be", "Beryllium"    )   ,
    (B , 5  , "B" , "Boron"        )  ,
    (C , 6  , "C" , "Carbon"       )  ,
    (N , 7  , "N" , "Nitrogen"     )  ,
    (O , 8  , "O" , "Oxygen"       )  ,
    (F , 9  , "F" , "Fluorine"     )  ,
    (Ne, 10 , "Ne", "Neon"         )  ,
    (Na, 11 , "Na", "Sodium"       )  ,
    (Mg, 12 , "Mg", "Magnesium"    )  ,
    (Al, 13 , "Al", "Aluminium"    )  ,
    (Si, 14 , "Si", "Silicon"      )  ,
    (P , 15 , "P" , "Phosphorus"   )  ,
    (S , 16 , "S" , "Sulfur"       )  ,
    (Cl, 17 , "Cl", "Chlorine"     )  ,
    (Ar, 18 , "Ar", "Argon"        )  ,
    (K , 19 , "K" , "Potassium"    )  ,
    (Ca, 20 , "Ca", "Calcium"      )  ,
    (Sc, 21 , "Sc", "Scandium"     )  ,
    (Ti, 22 , "Ti", "Titanium"     )  ,
    (V , 23 , "V" , "Vanadium"     )  ,
    (Cr, 24 , "Cr", "Chromium"     )  ,
    (Mn, 25 , "Mn", "Manganese"    )  ,
    (Fe, 26 , "Fe", "Iron"         )  ,
    (Co, 27 , "Co", "Cobalt"       )  ,
    (Ni, 28 , "Ni", "Nickel"       )  ,
    (Cu, 29 , "Cu", "Copper"       )  ,
    (Zn, 30 , "Zn", "Zinc"         )  ,
    (Ga, 31 , "Ga", "Gallium"      )  ,
    (Ge, 32 , "Ge", "Germanium"    )  ,
    (As, 33 , "As", "Arsenic"      )  ,
    (Se, 34 , "Se", "Selenium"     )  ,
    (Br, 35 , "Br", "Bromine"      )  ,
    (Kr, 36 , "Kr", "Krypton"      )  ,
    (Rb, 37 , "Rb", "Rubidium"     )  ,
    (Sr, 38 , "Sr", "Strontium"    )  ,
    (Y , 39 , "Y" , "Yttrium"      )  ,
    (Zr, 40 , "Zr", "Zirconium"    )  ,
    (Nb, 41 , "Nb", "Niobium"      )  ,
    (Mo, 42 , "Mo", "Molybdenum"   )  ,
    (Tc, 43 , "Tc", "Technetium"   )  ,
    (Ru, 44 , "Ru", "Ruthenium"    ) ,
    (Rh, 45 , "Rh", "Rhodium"      ) ,
    (Pd, 46 , "Pd", "Palladium"    ) ,
    (Ag, 47 , "Ag", "Silver"       ) ,
    (Cd, 48 , "Cd", "Cadmium"      ) ,
    (In, 49 , "In", "Indium"       ) ,
    (Sn, 50 , "Sn", "Tin"          ) ,
    (Sb, 51 , "Sb", "Antimony"     ) ,
    (Te, 52 , "Te", "Tellurium"    ) ,
    (I , 53 , "I" , "Iodine"       ) ,
    (Xe, 54 , "Xe", "Xenon"        ) ,
    (Cs, 55 , "Cs", "Caesium"      ),
    (Ba, 56 , "Ba", "Barium"       ),
    (La, 57 , "La", "Lanthanum"    ),
    (Ce, 58 , "Ce", "Cerium"       ),
    (Pr, 59 , "Pr", "Praseodymium" ),
    (Nd, 60 , "Nd", "Neodymium"    ),
    (Pm, 61 , "Pm", "Promethium"   ),
    (Sm, 62 , "Sm", "Samarium"     ),
    (Eu, 63 , "Eu", "Europium"     ),
    (Gd, 64 , "Gd", "Gadolinium"   ),
    (Tb, 65 , "Tb", "Terbium"      ),
    (Dy, 66 , "Dy", "Dysprosium"   ),
    (Ho, 67 , "Ho", "Holmium"      ),
    (Er, 68 , "Er", "Erbium"       ),
    (Tm, 69 , "Tm", "Thulium"      ),
    (Yb, 70 , "Yb", "Ytterbium"    ),
    (Lu, 71 , "Lu", "Lutetium"     ),
    (Hf, 72 , "Hf", "Hafnium"      ),
    (Ta, 73 , "Ta", "Tantalum"     ),
    (W , 74 , "W" , "Tungsten"     ),
    (Re, 75 , "Re", "Rhenium"      ),
    (Os, 76 , "Os", "Osmium"       ),
    (Ir, 77 , "Ir", "Iridium"      ),
    (Pt, 78 , "Pt", "Platinum"     ),
    (Au, 79 , "Au", "Gold"         ),
    (Hg, 80 , "Hg", "Mercury"      ),
    (Tl, 81 , "Tl", "Thallium"     ),
    (Pb, 82 , "Pb", "Lead"         ),
    (Bi, 83 , "Bi", "Bismuth"      ),
    (Po, 84 , "Po", "Polonium"     ),
    (At, 85 , "At", "Astatine"     ),
    (Rn, 86 , "Rn", "Radon"        ),
    (Fr, 87 , "Fr", "Francium"     ),
    (Ra, 88 , "Ra", "Radium"       ),
    (Ac, 89 , "Ac", "Actinium"     ),
    (Th, 90 , "Th", "Thorium"      ),
    (Pa, 91 , "Pa", "Protactinium" ),
    (U , 92 , "U" , "Uranium"      ),
    (Np, 93 , "Np", "Neptunium"    ),
    (Pu, 94 , "Pu", "Plutonium"    ),
    (Am, 95 , "Am", "Americium"    ),
    (Cm, 96 , "Cm", "Curium"       ),
    (Bk, 97 , "Bk", "Berkelium"    ),
    (Cf, 98 , "Cf", "Californium"  ),
    (Es, 99 , "Es", "Einsteinium"  ),
    (Fm, 100, "Fm", "Fermium"      ),
    (Md, 101, "Md", "Mendelevium"  ),
    (No, 102, "No", "Nobelium"     ),
    (Lr, 103, "Lr", "Lawrencium"   ),
    (Rf, 104, "Rf", "Rutherfordium"),
    (Db, 105, "Db", "Dubnium"      ),
    (Sg, 106, "Sg", "Seaborgium"   ),
    (Bh, 107, "Bh", "Bohrium"      ),
    (Hs, 108, "Hs", "Hassium"      ),
    (Mt, 109, "Mt", "Meitnerium"   ),
    (Ds, 110, "Ds", "Darmstadtium" ),
    (Rg, 111, "Rg", "Roentgenium"  ),
    (Cn, 112, "Cn", "Copernicium"  ),
    (Nh, 113, "Nh", "Nihonium"     ),
    (Fl, 114, "Fl", "Flerovium"    ),
    (Mc, 115, "Mc", "Moscovium"    ),
    (Lv, 116, "Lv", "Livermorium"  ),
    (Ts, 117, "Ts", "Tennessine"   ),
    (Og, 118, "Og", "Oganesson"    ),
}
