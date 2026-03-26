#include "libint-sys/shim/include/atom.h"

namespace libint2 {
  std::unique_ptr<Atom> atom(const int atomic_number, const double x, const double y, const double z) {
    return std::make_unique<Atom>(Atom { atomic_number, x, y, z});
  }

  int atomic_number(const Atom& atom) {
    return atom.atomic_number;
  }

  double x(const Atom& atom) {
    return atom.x;
  }

  double y(const Atom& atom) {
    return atom.y;
  }  
  double z(const Atom& atom) {
    return atom.z;
  }  
}
