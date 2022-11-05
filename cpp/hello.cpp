#include <pybind11/pybind11.h>

namespace py = pybind11;

void hello_world(){
    py::print("Hello World!");
}