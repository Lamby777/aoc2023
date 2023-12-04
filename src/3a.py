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
