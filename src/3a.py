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
    return ch in [".", "\n"]


def is_symbol(ch: str):
    return (not ch.isdigit()) and not is_blank(ch)


def getch_safe(line: int, col: int):
    try:
        return lines[line][col]
    except IndexError:
        return ""


def try_get(lst: list, i: int):
    try:
        return lst[i]
    except IndexError:
        pass


def try_slice_around(line: int, col: int, length: int):
    """
    return a flattened list of all the characters
    "surrounding" a slice
    """
    to_slice = [
        try_get(lines, line - 1),
        lines[line],
        try_get(lines, line + 1),
    ]

    slice_start = col if col == 0 else col - 1

    sliced = [line[slice_start : col + length + 1] for line in to_slice]
    return "".join(sliced)


partnums = []

for lineno, line in enumerate(lines):
    num_indices = [i for i, ch in enumerate(line) if ch.isdigit()]
    num_indices.reverse()

    num_indices = [i for i in num_indices if i - 1 not in num_indices]
    num_indices.reverse()

    for start in num_indices:
        numlen = 0

        # find length of number
        while getch_safe(lineno, start + numlen).isdigit():
            numlen += 1

        # see if number is a part number
        sliced = try_slice_around(lineno, start, numlen)
        print(sliced)
        is_part = any([is_symbol(ch) for ch in sliced])
        print(f"{is_part=}")

        if is_part:
            partnum = int(line[start : start + numlen])
            partnums.append(partnum)

    break

#
print(partnums)

# 546184 is "too high" :(
print(sum(partnums))
