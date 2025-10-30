---
name: Rfc minimal test.
description: >
    Reads CMDSTA and exits.
dumped_symbols:
  nothing: 1 words
configurations:
- { }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:radio_mode = "minimal" %}
{% extends "asm.s.tpl" %}

// Doorbell Doorbell Base Address
{% set RFC_DBELL_BASE = '0x40041000'|int(base=16) %}
// Doorbell Command Status Register
{% set RFC_DBELL_O_CMDSTA = '0x00000004'|int(base=16) %}

{% block code %}
    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    ldr.w r2, ={{RFC_DBELL_BASE}}
    ldr.w r1, [r2, #{{RFC_DBELL_O_CMDSTA}}]
    b.w end_label
{% endblock %}
