---
name: strtol libc test
description: "strtol libc test"
dumped_symbols:
  cyccnt: 1 words
  lsucnt: 1 words
  cpicnt: 1 words
  foldcnt: 1 words
  stdout_content: 256 B
  stderr_content: 256 B
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
{% device:external_benchmark = "../../mm319369/whip6_tests/ExtStdlib/toolie_benchmark_proxy/strtol" %}

{% set libc_stdout_buf_size = 256 %}
{% set libc_stderr_buf_size = 256 %}
{% extends "bases/libc_benchmark.asm.tpl" %}
