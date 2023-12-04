#
# Day 3
# (Part 1)
#
#  - &Cherry <3
#


# read input file as list of chars
with open("inputs/3.txt") as f:
    lines = f.readlines()


def is_blank(ch: str):
    return ch not in [".", "\n"]


def is_symbol(ch: str | None):
    return ch is not None and (not ch.isdigit()) and not is_blank(ch)


def getch_safe(line: int, col: int):
    try:
        return lines[line][col]
    except IndexError:
        return ""


def vertical_symbol_scan(line: int, col: int):
    """
    If the character is a symbol OR if
    there's a symbol above/below it
    """

    return any([is_symbol(getch_safe(line + i, col)) for i in [-1, 0, 1]])


partnums = []

for lineno, line in enumerate(lines):
    is_part = False

    # lineno = 1
    # line = lines[1]

    num_indices = [i for i, ch in enumerate(line) if ch.isdigit()]
    num_indices.reverse()

    num_indices = [i for i in num_indices if i - 1 not in num_indices]
    num_indices.reverse()

    for start in num_indices:
        numlen = 0

        while line[start + numlen].isdigit():
            numlen += 1

        print(f"numlen: {numlen}")

    break

#
print(partnums)

# 546184 is "too high" :(
print(sum(partnums))
