from glob import glob

from Cython.Build import cythonize
from pybind11.setup_helpers import Pybind11Extension, build_ext
from setuptools import Extension, find_packages, setup
from setuptools_rust import Binding, RustExtension

ext_modules = [
    # cpp
    Pybind11Extension(
        "py_ext.cpp_ext",
        sorted(glob("./cpp/*.cpp")),
    ),
    # cython
    Extension("py_ext.cython_ext", sorted(glob("./cython/*.pyx"))),
]

setup(
    name="py-ext",
    cmdclass={"build_ext": build_ext},
    ext_modules=cythonize(ext_modules),  # add cpp and cython exts
    rust_extensions=[
        RustExtension("py_ext.rust_ext", binding=Binding.PyO3)
    ],  # add rust ext
    packages=find_packages(include=["py_ext", "py_ext.*"]),
    zip_safe=False,
)
