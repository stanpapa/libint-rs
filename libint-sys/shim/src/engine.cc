#include "libint-sys/shim/include/engine.h"

namespace libint2 {
std::unique_ptr<Engine> engine(int oper, std::size_t max_nprim,
                               std::size_t max_l, std::size_t deriv_order) {
  return std::make_unique<Engine>(static_cast<Operator>(oper), max_nprim, max_l,
                                  deriv_order);
}

std::size_t nshellsets(const Engine &engine) { return engine.nshellsets(); }

rust::Vec<std::size_t> results(const Engine &engine) {
  const auto &results = engine.results();
  rust::Vec<std::size_t> out;
  out.reserve(results.size());
  for (const double *p : results) {
    out.push_back(reinterpret_cast<std::size_t>(p));
  }
  return out;
}

void compute1(Engine &engine, const Shell &s1, const Shell &s2) {
  engine.compute1(s1, s2);
}
} // namespace libint2
