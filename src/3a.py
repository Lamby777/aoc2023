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

    for i, ch in enumerate(line):
        vert_scan = vertical_symbol_scan(lineno, i)
        is_part = is_part or vert_scan

        if ch.isdigit():
            print(f"{is_part=}")
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
            is_part = is_part or ch != "."

            if is_part and number_so_far:
                partnums.append(int(number_so_far))
            else:
                print(f"not a part number: {number_so_far}")

            number_so_far = ""

    # for testing
    break

# should be 180, 218, 189, 507
print(partnums)
