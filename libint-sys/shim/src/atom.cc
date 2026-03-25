#include "libint-sys/shim/include/atom.h"

namespace libint2 {
  std::unique_ptr<Atom> atom(const int atomic_number, const double x, const double y, const double z) {
    return std::make_unique<Atom>();
  }
}
