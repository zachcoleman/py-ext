# cython: language_level=3

from functools import cache


def hello_world():
    print("Hello World!")

def fib_no_cache(int n):
    if n <= 1:
        return n
    return fib_no_cache(n-2) + fib_no_cache(n-1)

@cache
def fib_cache(int n):
    if n <= 1:
        return n
    return fib_cache(n-2) + fib_cache(n-1)

cdef class ComplexNumber:
    cdef public float x
    cdef public float y

    def __init__(self, float x, float y):
        self.x = x
        self.y = y

    def add(self, other: "ComplexNumber") -> "ComplexNumber":
        return ComplexNumber(self.x + other.x, self.y + other.y)

    def square(self) -> "ComplexNumber":
        return ComplexNumber(
            self.x**2 - self.y**2,
            2 * self.x * self.y,
        )

    def abs(self) -> float:
        return self.x**2 + self.y**2
    
def mandelbrot_set(float delta = 0.01, int max_iter = 20):
    def test_num(ComplexNumber c) -> bool:
        z = ComplexNumber(0, 0)
        for _ in range(max_iter):
            z = z.square().add(c)
            if z.abs() >= 4:
                return False
        return True

    cdef list res = []
    cdef float x = -2.0
    cdef float y = -1.5
    while x <= 0.5:
        y = -1.5
        while y <= 1.5:
            c = ComplexNumber(x, y)
            if test_num(c):
                res.append(c)
            y = y + delta
        x = x + delta
    return [(c.x, c.y) for c in res] 