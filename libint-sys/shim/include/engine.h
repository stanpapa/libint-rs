#pragma once

#include "libint2/engine.h"
#include "rust/cxx.h"

namespace libint2 {
std::unique_ptr<Engine> engine(int oper, std::size_t max_nprim,
                               std::size_t max_l, std::size_t deriv_order);

std::size_t nshellsets(const Engine &engine);

// rust::Slice<const double> iresult(const Engine& engine, std::size_t i);
rust::Vec<std::size_t> results(const Engine &engine);

void compute1(Engine &engine, const Shell &s1, const Shell &s2);
} // namespace libint2
