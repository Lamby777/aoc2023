#
# Day 3
# (Part 2)
#
#  - &Cherry <3
#


# read input file as list of chars
with open("inputs/3.txt") as f:
    lines = f.readlines()

GEAR = "*"

print("\n\n\n")


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
        x
        for x in [
            try_get(lines, line - 1),
            lines[line],
            try_get(lines, line + 1),
        ]
        if x is not None
    ]

    slice_start = col if col == 0 else col - 1

    sliced = [line[slice_start : col + length + 1] for line in to_slice]
    return "".join(sliced)


partnums = []

# find part numbers
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
        is_part = any([is_symbol(ch) for ch in sliced])

        if is_part:
            partnum = int(line[start : start + numlen])
            partdata = [partnum, lineno, range(start - 1, start + numlen + 2)]
            partnums.append(partdata)


def get_gear_ratio(line: int, col: int):
    acceptable_lines = range(line - 1, line + 2)

    def is_gearnum(partdata: list):
        return partdata[1] in acceptable_lines and col in partdata[2]

    gearnums = list(filter(is_gearnum, partnums))

    if len(gearnums) != 2:
        return

    n1, n2 = gearnums[0][0], gearnums[1][0]
    print(n1, n2)
    return n1 * n2


ratios = []

for lineno, line in enumerate(lines):
    ratios.append(
        [get_gear_ratio(lineno, i) for i, ch in enumerate(line) if ch == GEAR]
    )

ratios = [[x for x in r if x is not None] for r in ratios if r]
ratios = [x for r in ratios for x in r]

print(ratios)
assert 166140 in ratios

# "20131086" is too low
# "39263106" is too low
# "43617617" is too low
# "82785737" is not correct (???)
# "77059864" is not correct (???)
print(sum(ratios))
