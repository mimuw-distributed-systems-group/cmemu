#!/usr/bin/env python3

import re
import json


def divide_val(val, lengths):
    assert sum(lengths) == 32
    
    result = []
    for bit in lengths:
        result.append(val % 2 ** bit)
        val //= 2 ** bit
    return result


def divide_arr(arr, lengths):
    assert sum(lengths) == 32
    
    result = [divide_val(val, lengths) for val in arr]
    result = zip(*result)
    result = list(map(list, result))
    
    return result


while True:
    try:
        line = input()
    except EOFError:
        break
    
    line = line.strip()
    
    symbol_name = re.findall(r"^symbol:(\s*)(\w+)$", line)
    
    if len(symbol_name) == 0:
        print(line)
        continue
    
    (symbol_spacing, symbol), = symbol_name
    line = input().strip()
    (address_spacing, address,), = re.findall(r"^address:(\s*)((?:0x)?[a-fA-Z0-9]+)$", line)
    line = input().strip()
    (expected_spacing, expected), = re.findall(r"^expected:(\s*)(\[\d+(?:,\s*\d+)*\])$", line)
    
    expected = json.loads(expected)
    
    values = [(symbol, expected)]
    
    division_scheme = re.findall(r"^COMB((?:_\w+?_LEN_\d+)+)$", symbol)
    
    if len(division_scheme) == 1:
        scheme = division_scheme[0]
        names, lengths = zip(*re.findall(r"_(\w+?)_LEN_(\d+)", scheme))
        values = divide_arr(expected, [int(val) for val in lengths])
        values = list(zip(names, values))
    else:
        pass
    
    for symbol, expected in reversed(values):
        print(f"symbol:{symbol_spacing}{symbol}")
        print(f"address:{address_spacing}{address}")
        print(f"expected:{expected_spacing}{expected}")
