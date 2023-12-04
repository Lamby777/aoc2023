#
# Day 3
# (Part 1)
#
#  - &Cherry <3
#


# read input file as list of chars
with open("inputs/3.txt") as f:
    first_line = f.readline()
    LINE_LEN = len(first_line)

    chars = (first_line + f.read()).replace("\n", "")
    chars = list(chars)

# could probably be a loop but i'm lazy rn :P
OFFSETS = [
    -LINE_LEN - 1,
    -LINE_LEN,
    -LINE_LEN + 1,
    -1,
    # we don't count 0
    1,
    LINE_LEN - 1,
    LINE_LEN,
    LINE_LEN + 1,
]


def is_symbol(ch: str):
    return (not ch.isdigit()) and (ch != ".")


def is_partnum(i: int):
    return any(is_symbol(chars[i + offset]) for offset in OFFSETS)


for ch in chars:
    pass
