#pragma once

#include "rust/cxx.h"
#include "libint2/shell.h"
#include <memory>

namespace libint2 {
  using Contraction = Shell::Contraction;

  // -----------------------------------------------------
  // Shell
  // -----------------------------------------------------
  std::unique_ptr<Shell> shell();

  rust::Vec<double> alpha(const Shell& shell);
  const Contraction* const* contr(const Shell& shell);
  std::array<double, 3> O(const Shell& shell);
  rust::Vec<double> max_ln_coeff(const Shell& shell);

  // -----------------------------------------------------
  // Contraction
  // -----------------------------------------------------
  // constructor
  std::unique_ptr<Contraction> contraction();
  
  // getters
  int l(const Contraction& contraction);   
  bool pure(const Contraction& contraction);   
  rust::Vec<double> coeff(const Contraction& contraction);   

  size_t cartesian_size(const Contraction& contraction);
  size_t size(const Contraction& contraction);
}

