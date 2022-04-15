#!/usr/bin/python

import sys


def compareHexValues(hex1, hex2):
    """
    Compare if hex1 is half as big as hex2.
    """
    return int(hex1, 16) < (int(hex2, 16)/2)

print(compareHexValues(sys.argv[1], sys.argv[2]))
# print(compareHexValues('0x1', '0x3'))
