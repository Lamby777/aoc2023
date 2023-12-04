#
# Day 3
# (Part 1)
#
#  - &Cherry <3
#

# read input file as list of chars
with open("inputs/3.txt") as f:
    chars = list(f.read())


def is_symbol(ch: str):
    return (not ch.isdigit()) and (ch != ".")


for ch in chars:
    pass
