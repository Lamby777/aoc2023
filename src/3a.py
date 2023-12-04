#
# Day 3
# (Part 1)
#
#  - &Cherry <3
#

# read input file from inputs/3.txt
with open("inputs/3.txt") as f:
    lines = f.readlines()

# map each line to a list of chars
char_grid = [list(line) for line in lines]


def is_symbol(ch: str):
    return (not ch.isdigit()) and (ch != ".")


for line in char_grid:
    for ch in line:
        pass
