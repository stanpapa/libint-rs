#pragma once

#include "rust/cxx.h"
#include "libint2/atom.h"
#include <memory>

namespace libint2 {
  std::unique_ptr<Atom> atom(const int atomic_number, const double x, const double y, const double z);

  // getters
  int atomic_number(const Atom& atom);  
  double x(const Atom& atom);  
  double y(const Atom& atom);  
  double z(const Atom& atom);  
}

