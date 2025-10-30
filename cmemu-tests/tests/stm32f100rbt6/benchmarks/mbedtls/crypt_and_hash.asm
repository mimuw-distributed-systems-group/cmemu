---
name: Mbed TLS - crypt and hash
description: "Mbed TLS - crypt and hash"
dumped_symbols:
  cyccnt: 1 words
  lsucnt: 1 words
  cpicnt: 1 words
  foldcnt: 1 words
  stdout_content: 1 B
  stderr_content: 128 B
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
{% device:external_benchmark = "../../wc385540/benchmarks/mbedtls/crypt_and_hash" %}

{% set libc_stdout_buf_size = 1 %}
{% set libc_stderr_buf_size = 128 %}
{% extends "bases/libc_benchmark.asm.tpl" %}
