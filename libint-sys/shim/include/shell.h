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
  const Contraction& at_contraction(const Shell& shell, std::size_t i);
  std::array<double, 3> O(const Shell& shell);
  rust::Vec<double> max_ln_coeff(const Shell& shell);

  std::size_t cartesian_size_shell(const Shell& shell);
  std::size_t size_shell(const Shell& shell);
  std::size_t ncontr(const Shell& shell);
  std::size_t nprim(const Shell& shell);

  // -----------------------------------------------------
  // Contraction
  // -----------------------------------------------------
  
  // getters
  int l(const Contraction& contraction);   
  bool pure(const Contraction& contraction);   
  rust::Vec<double> coeff(const Contraction& contraction);   

  size_t cartesian_size_contraction(const Contraction& contraction);
  size_t size_contraction(const Contraction& contraction);
}

