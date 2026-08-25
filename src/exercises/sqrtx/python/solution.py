def mySqrt(x: int) -> int:
    r = x
    while r * r > x:
        r = (r + x // r) >> 1
    return r
