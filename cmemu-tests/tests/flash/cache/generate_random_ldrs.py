#!/usr/bin/env python3

import random

def main():
    for _ in range(50):
        base_reg = random.randint(1, 6)
        offset = random.randint(0, 12) * 8
        print(f"ldr.n r7, [r{base_reg}, #{offset}]")

if __name__ == '__main__':
    main()
