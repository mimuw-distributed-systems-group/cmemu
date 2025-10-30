#!/usr/bin/env python3

import sys

from shift_results import RESULTS, CYCLES


RNG_SEQUENCE = [0, 1, 3, 2, 1, 3, 3, 2, 0, 1, 2, 0, 0, 0, 0, 1, 2, 1, 2, 1, 3, 2, 1, 3, 2, 1, 2, 1, 3, 2, 0, 1, 2, 1, 3, 2, 0, 0, 0, 1, 3, 3, 3, 3, 2, 1, 3, 2, 1, 3, 3, 3, 2, 1, 2, 1, 3, 3, 2, 1, 2, 0, 0, 1, 2, 0, 0, 0, 1, 3, 2, 1, 3, 2, 0, 0, 1, 3, 3, 3, 2, 0, 1, 3, 3, 2, 0, 1, 3, 2, 0, 0, 1, 2, 1, 3, 2, 1, 2, 0, 1, 2, 0, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 1, 2, 0, 1, 3, 3, 2, 1, 3, 3, 2, 1, 3, 2, 0, 1, 3, 3, 3, 2, 1, 3, 3, 3, 3, 3, 2, 1, 2, 0, 1, 3, 2, 0, 1, 3, 2, 1, 2, 1, 2, 0, 0, 1, 3, 2, 0, 0, 0, 0, 1, 3, 3, 2, 1, 2, 1, 2, 1, 2, 1, 3, 3, 3, 3, 2, 0, 1, 2, 1, 2, 0, 0, 0, 1, 2, 0, 1, 3, 3, 3, 3, 3, 3, 3, 2, 0, 0, 0, 1, 2, 1, 3, 3, 3, 2, 0, 0, 1, 3, 2, 1, 2, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 3, 3, 2, 0, 0, 1, 2, 0, 1, 2, 1, 3, 3, 2, 0, 0, 0, 0, 0, 1, 3, 2, 0, 1, 2, 0, 1, 2]
RESULT_PART_SIZE = 7


def get_pos(subsequence):
    extended_rng_sequence = RNG_SEQUENCE + RNG_SEQUENCE[:(len(subsequence) - 1)]
    pos = None
    for i in range(len(RNG_SEQUENCE)):
        match = True
        for j in range(len(subsequence)):
            if extended_rng_sequence[i + j] != subsequence[j]:
                match = False
                break
        if match:
            if pos is not None:
                print("Error: RNG subsequence position not unique", file=sys.stderr)
                sys.exit(1)
            pos = i
    if pos is None:
        print("Error: RNG subsequence not found", file=sys.stderr)
        sys.exit(1)
    return pos


def get_rng_shift(result):
    first_part_pos = get_pos(result[:RESULT_PART_SIZE])
    second_part_pos = get_pos(result[RESULT_PART_SIZE:])
    return (second_part_pos - (first_part_pos + RESULT_PART_SIZE)) % len(RNG_SEQUENCE)


def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} out_file", file=sys.stderr)
        sys.exit(1)
    with open(sys.argv[1], "w") as file:
        print(f"prefetch, second_ldr_variant, between_ldrs, tag_hit, cache_hit, rng_shift, cycles", file=file)
        result_pos = 0
        result_idx = 0
        for prefetch in [False, True]:
            for second_ldr_variant in ["next_line", "next_next_line", "same_set_not_evicted", "same_set_evicted", "next_set", "unrelated"]:
                for between_ldrs in ["single", "empty", "nop", "add_1", "add_2", "add_3", "add_4"]:
                    for tag_hit in [False, True]:
                        for cache_hit in [False, True]:
                            rng_shift = get_rng_shift(RESULTS[result_pos:(result_pos + 2 * RESULT_PART_SIZE)])
                            # Subtract shift coming from preparing the TAG/cache hit/miss
                            if cache_hit:
                                rng_shift -= 2
                            elif tag_hit:
                                rng_shift -= 1
                            if second_ldr_variant not in ["same_set_not_evicted", "same_set_evicted"] and between_ldrs != "single":
                                rng_shift -= 1
                            cycles = CYCLES[result_idx]
                            if between_ldrs != "single":
                                print(
                                    f"{prefetch}, {second_ldr_variant}, {between_ldrs}, {tag_hit}, {cache_hit}, {rng_shift}, {cycles}",
                                    file=file
                                )
                            result_pos += 2 * RESULT_PART_SIZE
                            result_idx += 1
        if result_pos != len(RESULTS):
            print("Error: results size mismatch", file=sys.stderr)
            sys.exit(1)

if __name__ == '__main__':
    main()
