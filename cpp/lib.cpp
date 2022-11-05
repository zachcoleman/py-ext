#include <pybind11/pybind11.h>

namespace py = pybind11;

void hello_world();
int fib_no_cache(int n);

PYBIND11_MODULE(cpp_ext, m) {
    m.def("hello_world", &hello_world);
    m.def("fib_no_cache", &fib_no_cache);
}