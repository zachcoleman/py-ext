from functools import cache


def hello_world():
    print("Hello World!")


def fib_no_cache(n: int):
    if n <= 1:
        return n
    return fib_no_cache(n - 2) + fib_no_cache(n - 1)


@cache
def fib_cache(n: int):
    if n <= 1:
        return n
    return fib_cache(n - 2) + fib_cache(n - 1)
