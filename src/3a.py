#
# Day 3
# (Part 1)
#
#  - &Cherry <3
#


# borrowed, not stolen. :3
# https://stackoverflow.com/a/5125712/8149876
class safelist(list):
    def get(self, index, default=None):
        try:
            return self.__getitem__(index)
        except IndexError:
            return default


# read input file as list of chars
with open("inputs/3.txt") as f:
    first_line = f.readline()
    LINE_LEN = len(first_line)

    _chars = (first_line + f.read()).replace("\n", "")
    chars = safelist(_chars)

OFFSETS = [(LINE_LEN * y) - x for x in range(-1, 2) for y in range(-1, 2)]
OFFSETS = [n for n in OFFSETS if n != 0]
OFFSETS.sort()
print(OFFSETS)


def is_symbol(ch: str | None):
    return ch is not None and (not ch.isdigit()) and (ch != ".")


def is_partnum(i: int):
    return any(is_symbol(chars.get(i + offset)) for offset in OFFSETS)


for ch in chars:
    number_so_far = ""
    is_part = False

    if ch.isdigit():
        pass
    else:
        pass
