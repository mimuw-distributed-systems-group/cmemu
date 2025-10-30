#!/usr/bin/env python3

import sys

def main():
    for line in sys.stdin:
        if not line.startswith("- {"):
            print(line, end="")
            continue

        prefix, values_str = line.split("registerValues: [")
        values_str = values_str.removesuffix("]] }\n")
        values = values_str.split("],")
        values_split_strs = ["],".join(values_part) for values_part in [values[:len(values) // 2], values[len(values) // 2:]]]

        for values_split_str in values_split_strs:
            print(f"{prefix}registerValues: [{values_split_str}]] }}\n", end="")

if __name__ == '__main__':
    main()
