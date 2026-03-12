import math

def round_half_up(n):
    return math.floor(n + 0.5)


def compress_mod16():
    c = []
    cur_y = -1
    for x in range(3330):
        y = round_half_up(x*16/3329)%16
        if y != cur_y:
            c.append((y, x))
            cur_y = y
    return c

c = compress_mod16()

0 0
1 208
2 416
3 624
4 832
5 1040
6 1248
7 1456
8 1664
9 1873
10 2081
11 2289
12 2497
13 2705
14 2913
15 3121

def compress(x):
    return round_half_up(x*16/3329)%16


def decompress(x):
    return round_half_up(x * 3329/16)

def decompression():
    d = [0]*16
    for y in range(16):
        d[y] = decompress(y)
        print(y, decompress(y))
    return d

d = decompression()

def compression10():
    c = []
    cur_y = -1
    for x in range(3330):
        y = round_half_up(x*(2**10)/3329)%(2**10)
        if y != cur_y:
            c.append((y, x))
            cur_y = y
    return c

c = compression10()

def compress10(x):
    return round_half_up((x*1024)/3329)%1024

def decompress10(y):
    return round_half_up((y*3329)/1024)


def decompression10():
    for y in range(1024):
        d = decompress10(y)
        print(f"{y} {d}")

# decompress10(y) returns the point c_y, which is the point in {0, 3, 6, ..., 3326}
# that is closer to x in balanced modular distance.
# In other words,
# y  decompress10(y)
# 0 0
# 1 3
# 2 7
# 3 10
# ...
# 1022 3323
# 1023 3326

# (3329/(2**(10+1))) = r(3329/2048) = 1.625


##

def compress1(x):
    return round_half_up((x*2)/3329)%2

def decompress1(y):
    return round_half_up((y*3329)/2)


def compression1():
    for x in range(3330):
        y = compress1(x)
        print(f"{x} {y}")

def decompression1():
    for y in range(2):
        d = decompress1(y)
        print(f"{y} {d}")

# compress1 maps all x to either 0 or 1, depending on which of the two points
# 0 or 1665 is closer to x in *balanced modular distance*, i.e., comparing
# |x - 0 mod^\pm q| and |x - 1665 mod^\pm q|
#
# Compress1(x) returns 0 if
# |x mod^\pm q| < |x - 1665 mod^\pm q|
# and returns 1 otherwise.

# Let 𝑞= 3329. In ML-KEM
# compress_1(x) = arg min{\under{{b \in {0, 1}}} |x - c_b mod^\pm q|
# where c_0 = 0 and c_1 = 1665.
# a mod^\pm q denotes the unique representative of a mod q in the interval [-q/2, q/2).

# decompress1(y) returns 0 or 1665.
# decompress1(0) = 0 and decompress1(1) = 1665.
