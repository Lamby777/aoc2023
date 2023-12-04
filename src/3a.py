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
        return None


def vertical_symbol_scan(line: int, col: int):
    """
    If the character is a symbol OR if
    there's a symbol above/below it
    """

    return any([is_symbol(getch_safe(line + i, col)) for i in [-1, 0, 1]])


partnums = []

for lineno, line in enumerate(lines):
    number_so_far = ""
    is_part = False

    # lineno = 1
    # line = lines[1]

    for i, ch in enumerate(line):
        vert_scan = vertical_symbol_scan(lineno, i)
        is_part = is_part or vert_scan

        if ch.isdigit():
            number_so_far += ch
        else:
            # number ended, do something with it

            if not number_so_far:
                # last ch was also not a number,
                # so we disregard its symbol-ness
                is_part = vert_scan

                # don't push empty strings
                continue

            # if the char after the number is a symbol,
            # then the number is still a part number
            if is_part or is_symbol(ch):
                partnums.append(int(number_so_far))

            number_so_far = ""

        # TODO handle newlines
    # break

#
print(partnums)

# 546184 is "too high" :(
print(sum(partnums))
