#include<iostream>
#include <pybind11/pybind11.h>

namespace py = pybind11;

void hello_world(){
    py::print("Hello World!");
}

PYBIND11_MODULE(cpp_ext, m) {
    m.def("hello_world", &hello_world, "Hello World!");
}