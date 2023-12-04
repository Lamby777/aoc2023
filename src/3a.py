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
    number_so_far = ""
    is_part = False

    # lineno = 1
    # line = lines[1]

    for i, ch in enumerate(line):
        vert_scan = vertical_symbol_scan(lineno, i)

        # if next or previous char are numbers, check
        # this column for symbols
        print(getch_safe(lineno, i - 1))
        if getch_safe(lineno, i - 1).isdigit() or getch_safe(lineno, i + 1).isdigit():
            print("vert_scan", vert_scan)
            is_part = True

        # if this line is a symbol and we parsed a number
        # and that number has been detected to be a part
        # number, push it to the list
        if ch.isdigit():
            number_so_far += ch
        if is_symbol(ch) and number_so_far and is_part:
            partnums.append(int(number_so_far))
            number_so_far = ""

            if not is_symbol(ch):
                is_part = False

    break

#
print(partnums)

# 546184 is "too high" :(
print(sum(partnums))
