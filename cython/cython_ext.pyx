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