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

  const Contraction& at_contraction(const Shell& shell, std::size_t i) {
    return shell.contr[i];
  }

  std::array<double, 3> O(const Shell& shell) {
    return shell.O;
  }

  rust::Vec<double> max_ln_coeff(const Shell& shell) {
    rust::Vec<double> v;
    std::copy(shell.max_ln_coeff.begin(), shell.max_ln_coeff.end(), std::back_inserter(v));
    return v;
  }

  std::size_t cartesian_size(const Shell& shell) {
    return shell.cartesian_size();
  }
  std::size_t size(const Shell& shell) {
    return shell.size();
  }
  std::size_t ncontr(const Shell& shell) {
    return shell.ncontr();
  }
  std::size_t nprim(const Shell& shell) {
    return shell.nprim();
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
