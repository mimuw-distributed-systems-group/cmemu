---
name: OpenSSL - sha256_raw
description: "OpenSSL - sha256_raw"
dumped_symbols:
  cyccnt: 1 words
  lsucnt: 1 words
  cpicnt: 1 words
  foldcnt: 1 words
  stdout_content: 64 B
  stderr_content: 1 B
configurations: []
product:
- BENCHMARK_OPT_LEVEL: [-Os, -O3] # -O0 uses too much memory
  lbEn: [True, False]
  wbEn: [True, False]
  cache_enabled: [True, False]
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = wbEn %}
{% device:cache_enabled = cache_enabled %}
{% device:external_benchmark = "../../wc385540/benchmarks/openssl/sha256_raw" %}

{% set libc_stdout_buf_size = 64 %}
{% set libc_stderr_buf_size = 1 %}
{% extends "bases/libc_benchmark.asm.tpl" %}
