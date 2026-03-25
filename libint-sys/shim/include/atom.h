#pragma once

#include "rust/cxx.h"
#include "libint2/atom.h"
#include <memory>

namespace libint2 {
  std::unique_ptr<Atom> atom(const int atomic_number, const double x, const double y, const double z);
}

