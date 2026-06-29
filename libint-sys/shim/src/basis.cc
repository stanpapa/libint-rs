#include "libint-sys/shim/include/basis.h"
#include <memory>

namespace libint2 {
  std::unique_ptr<BasisSet> basis(rust::Str name, const Atom* const* atoms, size_t n) {
    // convert rust &str to std::string
    std::string s(name.data(), name.size());

    // convert pointer array of `Atom`s to std::vector<Atom>
    std::vector<Atom> tmp;
    tmp.reserve(n);
    for (size_t i = 0; i < n; ++i) {
        const libint2::Atom* a = atoms[i];
        tmp.push_back(*a);
    }

    // construct `BasisSet` and return
    return std::make_unique<BasisSet>(s, tmp);
  }

  std::size_t nshells(const BasisSet& basis) {
    return basis.size();
  }

  void set_pure(BasisSet& basis, bool solid) {
    basis.set_pure(solid);
  }

  std::unique_ptr<Shell> at(const BasisSet& basis, std::size_t i) {
    return std::make_unique<Shell>(basis[i]);
  }

  std::size_t nbf(const BasisSet& basis) {
    return basis.nbf();
  }

  std::size_t max_nprim(const BasisSet& basis) {
    return basis.max_nprim();
  }

  std::size_t max_l(const BasisSet& basis) {
    return basis.max_l();
  }
}

