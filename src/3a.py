#
# Day 3
# (Part 1)
#
#  - &Cherry <3
#


# read input file as list of chars
with open("inputs/3.txt") as f:
    lines = f.readlines()


def is_symbol(ch: str | None):
    return ch is not None and (not ch.isdigit()) and (ch != ".")


def getch_safe(i: int, j: int):
    try:
        return lines[i][j]
    except IndexError:
        return None


def is_partnum(i: int):
    pass


for lineno, line in enumerate(lines):
    number_so_far = ""
    is_part = False

    for i, ch in enumerate(line):
        print(i, ch)
        if ch.isdigit():
            pass
        else:
            pass

    exit()
