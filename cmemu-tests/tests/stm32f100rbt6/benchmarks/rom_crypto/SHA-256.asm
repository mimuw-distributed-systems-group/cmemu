---
name: SHA-256 from ROM crypto
description: "SHA-256 from ROM crypto"
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
{% device:external_benchmark = "../../wc385540/benchmarks/rom_crypto/SHA-256" %}
{% set stack_region_end_addr = '0x20004F2C' %}
{% extends "bases/benchmark.asm.tpl" %}
