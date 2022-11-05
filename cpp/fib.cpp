#include <pybind11/pybind11.h>

namespace py = pybind11;

int fib_no_cache(int n){
    if (n <= 1){
        return n;
    }
    return fib_no_cache(n-2) + fib_no_cache(n-1);
}

int fib_cache(int n){
    // not implemented yet
    return 0;
}