---
name: TI MSP430 benchmark - 8-bit Switch Case
description: "TI MSP430 benchmark: 8-bit Switch Case"
dumped_symbols:
  cyccnt: 1 words
  lsucnt: 1 words
  cpicnt: 1 words
  foldcnt: 1 words
configurations: []
product:
- BENCHMARK_OPT_LEVEL: [-Os, -O0, -O3]
  lbEn: [False]
  wbEn: [True, False]
  cache_enabled: [False]
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = wbEn %}
{% device:cache_enabled = cache_enabled %}
{% device:external_benchmark = "../../wc385540/benchmarks/ti_msp430/8-bit_switch_case" %}
{% extends "bases/benchmark.asm.tpl" %}
