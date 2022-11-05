# py-ext
A repo examining extending Python with various tools

## Installation 

### Python
I use pyenv to manage multiple Python versions:
```sh
pyenv shell 3.11.0
python -m venv .venv
```

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

I opted for the pyo3 bindings.

### C++
```
pip install pybind11
```

