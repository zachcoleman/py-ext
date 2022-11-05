from glob import glob
from setuptools import setup, find_packages, Extension
from setuptools_rust import Binding, RustExtension
from Cython.Build import cythonize
from pybind11.setup_helpers import Pybind11Extension, build_ext

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
    # add cpp and cython exts
    ext_modules=cythonize(ext_modules),
    # add rust ext
    rust_extensions=[
        RustExtension("py_ext.rust_ext", binding=Binding.PyO3)
    ],
    packages=find_packages(include=["py_ext", "py_ext.*"]),
    zip_safe=False,
)
