#include "libint-sys/shim/include/basis.h"

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

  const Shell* const* contr(const BasisSet& basis) {
    // convert vector<Shell> to vector<const* Shell>
    std::vector<const Shell*> ptrs;
    ptrs.reserve(basis.size());
    for (const auto& shell : basis.shells()) {
      ptrs.push_back(&shell);
    }

    // convert vector to pointer array
    const Shell* const* array = ptrs.data();
    return array;
  }
}
