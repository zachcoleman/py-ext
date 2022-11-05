# py-ext
A repo examining extending Python with various tools

- [py-ext](#py-ext)
  - [Installation](#installation)
    - [Python](#python)
    - [Cython](#cython)
    - [Rust](#rust)
    - [C++](#c)

## Installation 

### Python
I use pyenv to manage multiple Python versions and create and manage virtual enviornments within the respective repo:
```sh
pyenv shell 3.11.0
python -m venv .venv
```

For benchmarking, Python 3.11 is used to leverage the latest speedups.

### Cython
```sh
pip install Cython
```

### Rust
`maturin`:
```sh
pip install maturin
maturn init
```
or
`setuptools-rust`:
```sh
pip install setuptools-rust
```

`maturin` is build-backend though and as far as I can tell you must used `setuptools-rust` to build RustExtensions manually. Also, I opted for the pyo3 bindings.

### C++
```
pip install pybind11
```

The extension is added to

