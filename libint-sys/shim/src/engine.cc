#include "libint-sys/shim/include/engine.h"

namespace libint2 {
  std::unique_ptr<Engine> engine(int oper, std::size_t max_nprim, std::size_t max_l, std::size_t deriv_order) {
    return std::make_unique<Engine>(static_cast<Operator>(oper), max_nprim, max_l, deriv_order);
  }
}
