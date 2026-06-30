#pragma once

#include "libint2/engine.h"
#include "rust/cxx.h"

namespace libint2 {
  std::unique_ptr<Engine> engine(int oper, std::size_t max_nprim, std::size_t max_l, std::size_t deriv_order);
}

