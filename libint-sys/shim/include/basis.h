#pragma once

#include "rust/cxx.h"
#include "libint2/basis.h"
#include "libint2/atom.h"
#include <memory>

namespace libint2 {
  std::unique_ptr<BasisSet> basis(rust::Str name, const Atom* const* atoms, std::size_t n);

  std::size_t nshells(const BasisSet& basis);

  std::unique_ptr<Shell> at(const BasisSet& basis, std::size_t i);

  void set_pure(BasisSet& basis, bool solid);
  
  std::unique_ptr<Shell> at(const BasisSet& basis, std::size_t i);

  std::size_t nbf(const BasisSet& basis);
  

  std::size_t max_nprim(const BasisSet& basis);
  

  std::size_t max_l(const BasisSet& basis);
  
}
