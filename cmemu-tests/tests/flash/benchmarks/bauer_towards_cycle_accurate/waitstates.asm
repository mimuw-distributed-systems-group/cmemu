---
name: The test from Bauer 'Towards Cycle-Accurate Emulation...' demonstrating waitstates
description: "The test from Bauer 'Towards Cycle-Accurate Emulation...' demonstrating waitstates"
dumped_symbols:
  cyccnt: 1 words
  lsucnt: 1 words
  cpicnt: 1 words
  foldcnt: 1 words
configurations: []
product:
- BENCHMARK_OPT_LEVEL: [-Os, -O0, -O3]
  lbEn: [True, False]
  wbEn: [True, False]
  cache_enabled: [True, False]
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = wbEn %}
{% device:cache_enabled = cache_enabled %}
{% device:external_benchmark = "../../wc385540/benchmarks/bauer_towards_cycle_accurate/waitstates" %}
{% extends "bases/benchmark.asm.tpl" %}
