#include "libint-sys/shim/include/shell.h"
#include <iterator>
#include <memory>

namespace libint2 {
  // -----------------------------------------------------
  // Shell
  // -----------------------------------------------------
  std::unique_ptr<Shell> shell() {
    return std::make_unique<Shell>();
  }

  rust::Vec<double> alpha(const Shell& shell) {
    rust::Vec<double> v;
    std::copy(shell.alpha.begin(), shell.alpha.end(), std::back_inserter(v));
    return v;
  }

  const Contraction* const* contr(const Shell& shell) {
    // convert vector<Contraction> to vector<const* Contraction>
    std::vector<const Contraction*> ptrs;
    ptrs.reserve(shell.contr.size());
    for (const auto& c : shell.contr) {
      ptrs.push_back(&c);
    }

    // convert vector to pointer array
    const Contraction* const* array = ptrs.data();
    return array;
  }

  std::array<double, 3> O(const Shell& shell) {
    return shell.O;
  }

  rust::Vec<double> max_ln_coeff(const Shell& shell) {
    rust::Vec<double> v;
    std::copy(shell.max_ln_coeff.begin(), shell.max_ln_coeff.end(), std::back_inserter(v));
    return v;
  }

  // -----------------------------------------------------
  // Contraction
  // -----------------------------------------------------

  // getters
  
  int l(const Contraction& contraction) {
    return contraction.l;
  }

  bool pure(const Contraction& contraction) {
    return contraction.pure;
  }
  
  rust::Vec<double> coeff(const Contraction& contraction) {
    rust::Vec<double> v;
    std::copy(contraction.coeff.begin(), contraction.coeff.end(), std::back_inserter(v));
    return v;
  }

  size_t cartesian_size(const Contraction& contraction) {
    return contraction.cartesian_size();
  }

  size_t size(const Contraction& contraction) {
    return contraction.size();
  }
}
