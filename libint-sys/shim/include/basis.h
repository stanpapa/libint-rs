#pragma once

#include "rust/cxx.h"
#include "libint2/basis.h"
#include "libint2/atom.h"
#include <memory>

namespace libint2 {
  std::unique_ptr<BasisSet> basis(rust::Str name, const Atom* const* atoms, std::size_t n);
}

