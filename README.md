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
I use pyenv to manage multiple Python versions and create and manage virtual environments within the respective repo:
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
Install Rust (pulled from https://www.rust-lang.org/tools/install):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

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
Install or check if a previous compliant compiler exists. See supported compilers [here](https://pybind11.readthedocs.io/en/stable/#supported-compilers).
```
pip install pybind11
```

The extension is added to the `setup.py` via the extenstion `Pybind11Extension` class. The code is built with `pybind11`'s `build_ext`. 

See `setup.py` for an example of jointly building a package with native Python and Cython, C++, and Rust extensions.
