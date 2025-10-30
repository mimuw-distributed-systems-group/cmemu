---
name: Fred-generated test
description: 'Test flow: (conf. 0) label55 -> label271 -> label176 -> label305 ->
  label591 -> label80 -> label50 -> label390 -> label481'
dumped_symbols:
  counters: 7 words
  registers: 77 words
  stack: user-defined
  safeSpaceSram: user-defined
  safeSpaceGpramSram: user-defined
configurations:
- code_memory: flash
  cache_en: true
  lb_en: true
  wb_en: false
  jump_start: label55
  jump_label55: label271
  jump_label271: label176
  jump_label176: label305
  jump_label305: label591
  jump_label591: label80
  jump_label80: label50
  jump_label50: label390
  jump_label390: label481
  jump_label481: code_end
  code_end: code_end
  space_mod: 2048
...

{% device:cache_enabled = cache_en %}
{% device:line_buffer_enabled = lb_en %}
{% device:write_buffer_enabled = wb_en %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Save original sp
    ldr.w  r11, =original_sp
    str.w  sp, [r11]

    b.w    tested_code
.thumb_func
end_label:
    @ Restore original sp
    ldr.w  r11, =original_sp
    ldr.w  sp, [r11]
{% endblock %}
{% block after %}
{{section(code_memory)}}


.align  4
.thumb_func
tested_code:
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Reset line buffer
    mov.w  r7, #0
    ldr.w  r2, [r7]

    @ Randomize values of registers
	mov.w	r0, #23698
	mov.w	r1, #2519
	mov.w	r2, #2090
	mov.w	r3, #45224
	mov.w	r4, #12832
	mov.w	r5, #24149
	mov.w	r6, #21203
	mov.w	r7, #37300
	mov.w	r8, #5247
	mov.w	r9, #13772
	mov.w	r10, #33428

    @ Start the test
    b.w    start_test


.align  4
.thumb_func
start_test:
    @ Set custom stack
    ldr.w  r11, =stack
    add.w  r11, r11, #328
    mov.w  sp, r11

    @ Get counter address
    ldr.w  r11, =counter_idx
    ldr.w  r11, [r11]
    ldr.w  r12, =counters_to_test
    ldr.w  r11, [r12, r11]
    @ Get counter start value
    ldr.w  r12, [r11]
        @ r11 – counter address
        @ r12 – counter start value

    @ Jump to the 1st block
    b.w    {{jump_start}}



.align	1
.space 156 % {{space_mod|default("0x10000000")}}
end_func_1:
	bx	r14

.ltorg
.align	2
.space	0, 45
.space 399 % {{space_mod|default("0x10000000")}}

.align	1
.space 1279 % {{space_mod|default("0x10000000")}}

.align	1
.space 3458 % {{space_mod|default("0x10000000")}}

.align	1
.space 2314 % {{space_mod|default("0x10000000")}}
label50:
ldr	r9, cell_313  @ 4b
ldr	r6, =forward_label_88  @ 2b
mov	r7, #4  @ 4b
orr	r6, #1  @ 4b
mov	r1, #47759  @ 4b
ldr	r10, cell_312  @ 4b
mov	r3, #12  @ 4b
.space 2

.space 4
.space 4
.space 4
ldr	r4, cell_311  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r4, cell_310  @ 2b
.space 4
.space 4
.space 2
.space 4
.space 4

forward_label_88:
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_label50:
	b.w	{{jump_label50}}

.ltorg
.align	2
.global	cell_309
cell_309:	.quad	0x35d0d7627a38f9c9

.space	1, 45
.global	cell_311
cell_311:	.word	safeSpaceSram-95282

.space	0, 45
.global	cell_312
cell_312:	.word	safeSpaceFlash+775

.space	3, 45
.global	cell_310
cell_310:	.word	safeSpaceFlash+488

.space	3, 45
.global	cell_313
cell_313:	.word	safeSpaceFlash+244

.align	1
.space 352 % {{space_mod|default("0x10000000")}}
label55:
	sub	r13, #84  @ 2b
mov	r4, #51  @ 4b
ldr	r14, =post_branch_72  @ 4b
mov	r10, #8  @ 4b
ldr	r6, cell_333  @ 4b
	strb	r1, [r13, r10, LSL #2]        @ A7.7.164  @ 4b
	strb	r1, [r13, #20]                @ A7.7.163  @ 4b
.space 4
	ldr	r5, [r13, r4]                 @ A7.7.45  @ 4b
.space 4
orr	r14, #1  @ 4b
	and	r4, r4                        @ A7.7.9  @ 4b @ looks important!
	b	func_37                       @ A7.7.12  @ 4b @ looks important!
post_branch_72:


mov	r0, #1  @ 4b
	ldrsh	r8, [r13, r0, LSL #1]         @ A7.7.65  @ 4b
	nop.n  @ was .align 2  @ 2b @ looks important!
	ldrd	r4, r0, cell_332              @ A7.7.51  @ 4b @ looks important!
	str	r2, [r13, #84]!               @ A7.7.161  @ 4b
.space 4
ldr	r2, cell_330  @ 4b
.space 4
.space 4
.space 4
	ldm	r13, {r0-r10}                 @ A7.7.41  @ 4b
.space 4
end_label55:
	b.w	{{jump_label55}}

.ltorg
.align	2
.space	3, 45
.global	cell_330
cell_330:	.word	safeSpaceGpramSram+68

.space	3, 45
.global	cell_331
cell_331:	.short	0x4d50

.align	2
.global	cell_332
cell_332:	.quad	0x140bf01d1462dce6

.space	1, 45
.global	cell_333
cell_333:	.word	safeSpaceSram+60

.align	1
.space 1162 % {{space_mod|default("0x10000000")}}

.align	1
.space 2020 % {{space_mod|default("0x10000000")}}
func_11:
.space 2
ldr	r2, cell_465  @ 4b
mov	r1, #0  @ 4b
.space 2

.space 4
ldr	r0, cell_464  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r10, cell_462  @ 4b
.space 4
.space 4
.space 4
.space 4

forward_label_135:
.space 4
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 2
.space 10
mov	r3, #37  @ 4b
mov	r1, #43  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4
mov	r0, #29  @ 4b
.space 4
.space 2
.space 4
.space 4
ldr	r0, cell_458  @ 4b
.space 4
ldr	r3, cell_457  @ 4b
ldr	r9, cell_456  @ 4b
.space 2
.space 4
.space 4
.space 4
.space 2
end_func_11:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_459
cell_459:	.word	0xb49a9b6b

.space	1, 45
.global	cell_465
cell_465:	.word	safeSpaceSram+843

.space	1, 45
.global	cell_463
cell_463:	.byte	0xf4

.space	0, 45
.global	cell_464
cell_464:	.word	safeSpaceSram+501

.space	3, 45
.global	cell_456
cell_456:	.word	safeSpaceGpramSram+972

.space	2, 45
.global	cell_462
cell_462:	.word	safeSpaceSram+196

.space	3, 45
.global	cell_457
cell_457:	.word	safeSpaceGpramSram+128

.space	0, 45
.global	cell_458
cell_458:	.word	safeSpaceGpramSram+612

.space	2, 45
.global	cell_461
cell_461:	.byte	0x9c

.space	3, 45
.global	cell_460
cell_460:	.byte	0x8c

.align	1
.space 568 % {{space_mod|default("0x10000000")}}
label80:
ldr	r1, =forward_label_144  @ 2b
ldr	r5, =func_62  @ 2b
ldr	r9, cell_494  @ 4b
orr	r5, #1  @ 4b
ldr	r3, cell_493  @ 4b
orr	r1, #1  @ 4b
ldr	r6, cell_492  @ 2b
.space 2
ldr	r14, =post_branch_104  @ 4b

.space 4
orr	r14, #1  @ 4b
.space 4
ldr	r4, cell_490  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 2
post_branch_104:


ldr	r0, =func_75  @ 2b
.space 4
orr	r0, #1  @ 4b
.space 4
.space 2


.space 4
.space 4
.space 4

forward_label_144:
.space 4
.space 4
.space 4
end_label80:
	b.w	{{jump_label80}}

.ltorg
.align	2
.space	0, 45
.global	cell_491
cell_491:	.byte	0x4a

.space	0, 45
.global	cell_489
cell_489:	.byte	0xa6

.space	2, 45
.global	cell_488
cell_488:	.short	0xd10e

.space	0, 45
.global	cell_487
cell_487:	.byte	0x98

.space	0, 45
.global	cell_493
cell_493:	.word	safeSpaceGpramSram+824

.space	2, 45
.global	cell_490
cell_490:	.word	safeSpaceGpramSram+127

.space	3, 45
.global	cell_494
cell_494:	.word	safeSpaceGpramSram+760

.space	0, 45
.global	cell_492
cell_492:	.word	safeSpaceSram+527

.align	1
.space 702 % {{space_mod|default("0x10000000")}}

.align	1
.space 2141 % {{space_mod|default("0x10000000")}}

.align	1
.space 2784 % {{space_mod|default("0x10000000")}}

.align	1
.space 3628 % {{space_mod|default("0x10000000")}}

.align	1
.space 2095 % {{space_mod|default("0x10000000")}}

.align	1
.space 2094 % {{space_mod|default("0x10000000")}}

.align	1
.space 1979 % {{space_mod|default("0x10000000")}}

.align	1
.space 1370 % {{space_mod|default("0x10000000")}}
label176:
.space 2
ldr	r3, =forward_label_332  @ 2b
orr	r3, #1  @ 4b
ldr	r7, cell_1292  @ 2b
ldr	r9, =func_11  @ 4b
ldr	r2, cell_1291  @ 4b
mov	r0, #65366  @ 4b
mov	r1, #19792  @ 4b
mov	r5, #19  @ 4b
.space 2

.space 4
ldr	r3, cell_1290  @ 4b
.space 4
ldr	r4, cell_1289  @ 4b
.space 4
.space 4
.space 4

forward_label_332:
orr	r9, #1  @ 4b
ldr	r8, =func_32  @ 4b
.space 2
.space 4

.space 4
orr	r8, #1  @ 4b
.space 4
.space 4
.space 4
.space 2


.space 4
.space 2



forward_label_331:
.space 4
.space 4
.space 4
.space 4
mov	r6, #156  @ 4b
.space 2
end_label176:
	b.w	{{jump_label176}}

.ltorg
.align	2
.space	0, 45
.global	cell_1292
cell_1292:	.word	safeSpaceFlash-522270

.space	2, 45
.global	cell_1288
cell_1288:	.byte	0x64

.space	3, 45
.global	cell_1290
cell_1290:	.word	safeSpaceFlash+681

.space	3, 45
.global	cell_1291
cell_1291:	.word	safeSpaceSram-19078

.space	1, 45
.global	cell_1289
cell_1289:	.word	safeSpaceFlash+556

.align	1
.space 830 % {{space_mod|default("0x10000000")}}

.align	1
.space 1692 % {{space_mod|default("0x10000000")}}
func_32:
.space 2
ldr	r2, cell_1405  @ 4b
mov	r0, #3  @ 4b
.space 4
ldr	r3, cell_1404  @ 2b
.space 4
ldr	r1, cell_1403  @ 4b
mov	r9, #44155  @ 4b
.space 4
.space 4
.space 4
mov	r10, #28152  @ 4b
.space 4
ldr	r2, cell_1402  @ 4b
.space 2
.space 4
.space 4
.space 2
.space 4
.space 2
.space 4
ldr	r1, =forward_label_372  @ 2b
.space 4
orr	r1, #1  @ 4b
.space 4
.space 4
ldr	r3, =forward_label_371  @ 2b
.space 4
.space 4
.space 2
ldr	r2, cell_1398  @ 2b
.space 4
.space 4
.space 2


forward_label_372:
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r0, cell_1396  @ 4b
.space 4
mov	r2, #23627  @ 4b
.space 4
orr	r3, #1  @ 4b
.space 2

.space 4
.space 4
.space 4

forward_label_371:
ldr	r3, cell_1393  @ 2b
.space 4
.space 4
.space 4
.space 4
end_func_32:
	bx	r14

.ltorg
.align	2
.space	3, 45
.global	cell_1394
cell_1394:	.short	0x9fe4

.space	3, 45
.global	cell_1398
cell_1398:	.word	safeSpaceFlash-1532

.space	1, 45
.global	cell_1401
cell_1401:	.short	0x619b

.space	1, 45
.global	cell_1393
cell_1393:	.word	safeSpaceGpramSram-23558

.space	3, 45
.global	cell_1397
cell_1397:	.word	0x3fc303bf

.space	2, 45
.global	cell_1399
cell_1399:	.byte	0x72

.space	3, 45
.global	cell_1392
cell_1392:	.word	0xeba3d17a

.space	2, 45
.global	cell_1402
cell_1402:	.word	safeSpaceSram-352923

.space	1, 45
.global	cell_1400
cell_1400:	.word	0x1f4bca60

.space	0, 45
.global	cell_1404
cell_1404:	.word	safeSpaceFlash-27901

.space	1, 45
.global	cell_1403
cell_1403:	.word	safeSpaceGpramSram+292

.space	2, 45
.global	cell_1396
cell_1396:	.word	safeSpaceFlash+134

.space	2, 45
.global	cell_1395
cell_1395:	.word	0xb5c413b1

.space	0, 45
.global	cell_1405
cell_1405:	.word	safeSpaceFlash+427

.align	1
.space 3193 % {{space_mod|default("0x10000000")}}

.space	0, 45
.space 1283 % {{space_mod|default("0x10000000")}}
func_37:
.space 2
ldr	r10, =forward_label_428  @ 4b
ldr	r9, cell_1599  @ 4b
orr	r10, #1  @ 4b
ldr	r1, cell_1598  @ 4b
mov	r2, #1  @ 4b
.space 2
.space 4
.space 4
.space 4
mov	r3, #39507  @ 4b
.space 4
ldr	r0, cell_1596  @ 4b
.space 2
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r9, cell_1594  @ 4b
.space 4
.space 4
.space 4

forward_label_428:
.space 4
.space 4
.space 4
.space 2
.space 4
ldr	r1, cell_1593  @ 4b
.space 10
.space 4
end_func_37:
	bx	r14

.ltorg
.align	2
.space	2, 45
.global	cell_1593
cell_1593:	.word	safeSpaceGpramSram+313

.space	1, 45
.global	cell_1595
cell_1595:	.short	0x706c

.space	0, 45
.global	cell_1596
cell_1596:	.word	safeSpaceGpramSram+565

.space	2, 45
.global	cell_1599
cell_1599:	.word	safeSpaceGpramSram+160

.space	0, 45
.global	cell_1594
cell_1594:	.word	safeSpaceGpramSram-315384

.space	3, 45
.global	cell_1598
cell_1598:	.word	safeSpaceSram+559

.space	2, 45
.global	cell_1597
cell_1597:	.byte	0xc1

.space	1, 45
.global	cell_1592
cell_1592:	.short	0xbaba

.align	1
.space 827 % {{space_mod|default("0x10000000")}}

.align	1
.space 4274 % {{space_mod|default("0x10000000")}}
label271:
ldr	r8, cell_1791  @ 4b
mov	r3, #51  @ 4b
mov	r5, #10200  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r7, cell_1790  @ 4b
.space 4
.space 4
end_label271:
	b.w	{{jump_label271}}

.ltorg
.align	2
.space	2, 45
.global	cell_1790
cell_1790:	.word	safeSpaceSram-40426

.space	1, 45
.global	cell_1791
cell_1791:	.word	safeSpaceGpramSram+36

.align	1
.space 979 % {{space_mod|default("0x10000000")}}

.align	1
.space 1452 % {{space_mod|default("0x10000000")}}

.align	1
.space 1152 % {{space_mod|default("0x10000000")}}
label305:
ldr	r5, cell_1929  @ 4b
ldr	r1, cell_1928  @ 4b
ldr	r6, =forward_label_549  @ 2b
mov	r8, #0  @ 4b
ldr	r9, cell_1927  @ 4b
orr	r6, #1  @ 4b
.space 2

.space 4
.space 4
mov	r8, #58309  @ 4b
.space 4
.space 4
ldr	r3, cell_1926  @ 2b
.space 2
.space 4
.space 4
.space 2
.space 4
.space 4
.space 2
.space 4
.space 4

forward_label_549:
mov	r1, #8009  @ 4b
ldr	r6, cell_1923  @ 4b
.space 4
.space 4
.space 4
ldr	r5, cell_1922  @ 2b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_label305:
	b.w	{{jump_label305}}

.ltorg
.align	2
.space	2, 45
.global	cell_1921
cell_1921:	.byte	0x63

.space	1, 45
.global	cell_1922
cell_1922:	.word	safeSpaceGpramSram+832

.space	1, 45
.global	cell_1927
cell_1927:	.word	safeSpaceGpramSram+496

.align	2
.global	cell_1925
cell_1925:	.quad	0x18c034f6d68586bd

.space	1, 45
.global	cell_1929
cell_1929:	.word	safeSpaceFlash-57498

.space	1, 45
.global	cell_1928
cell_1928:	.word	safeSpaceGpramSram-2625

.space	2, 45
.global	cell_1924
cell_1924:	.byte	0x6e

.space	1, 45
.global	cell_1923
cell_1923:	.word	safeSpaceSram-7816

.space	2, 45
.global	cell_1926
cell_1926:	.word	safeSpaceGpramSram+493

.align	1
.space 1936 % {{space_mod|default("0x10000000")}}

.align	1
.space 5608 % {{space_mod|default("0x10000000")}}

.align	1
.space 3846 % {{space_mod|default("0x10000000")}}
label390:
mov	r0, #8  @ 4b
mov	r3, #16648  @ 4b
ldr	r4, cell_2384  @ 4b
ldr	r8, cell_2383  @ 4b
ldr	r5, cell_2382  @ 2b
.space 4
.space 18
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r4, cell_2379  @ 4b
.space 4
.space 4
.space 4
.space 2

.space 4
.space 4
.space 4
.space 4
.space 4
.space 4

forward_label_695:
ldr	r9, cell_2377  @ 4b
.space 4
ldr	r8, cell_2376  @ 4b
.space 4
.space 4
end_label390:
	b.w	{{jump_label390}}

.ltorg
.align	2
.space	1, 45
.global	cell_2384
cell_2384:	.word	safeSpaceSram+14

.space	3, 45
.global	cell_2382
cell_2382:	.word	safeSpaceSram+883

.space	0, 45
.global	cell_2376
cell_2376:	.word	safeSpaceSram-385

.space	2, 45
.global	cell_2377
cell_2377:	.word	safeSpaceFlash+223

.space	3, 45
.global	cell_2380
cell_2380:	.word	0x74b8f0e2

.space	0, 45
.global	cell_2381
cell_2381:	.word	0x697c97bc

.space	3, 45
.global	cell_2375
cell_2375:	.byte	0x36

.space	1, 45
.global	cell_2378
cell_2378:	.byte	0x77

.space	1, 45
.global	cell_2383
cell_2383:	.word	safeSpaceGpramSram-133118

.space	2, 45
.global	cell_2379
cell_2379:	.word	safeSpaceFlash+368

.align	1
.space 6482 % {{space_mod|default("0x10000000")}}

.align	1
.space 5452 % {{space_mod|default("0x10000000")}}
func_62:
.space 2
ldr	r2, cell_2866  @ 2b
.space 4
.space 4
.space 4
.space 4
.space 2
mov	r9, #62  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
mov	r10, #1  @ 4b
.space 4
.space 4
.space 4
ldr	r2, cell_2863  @ 4b
.space 2
.space 4
ldr	r1, =forward_label_840  @ 2b
orr	r1, #1  @ 4b
ldr	r0, cell_2862  @ 4b
.space 4
.space 2
mov	r10, #2501  @ 4b
.space 4
mov	r3, #25829  @ 4b
.space 4
.space 4
mov	r9, #54  @ 4b
.space 4
ldr	r0, cell_2860  @ 4b
.space 2

.space 4
.space 4
.space 4
ldr	r1, cell_2858  @ 4b
.space 4
.space 4
.space 4
.space 4
mov	r0, #53  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4

forward_label_840:
.space 4
.space 4
.space 2
end_func_62:
	bx	r14

.ltorg
.align	2
.space	2, 45
.global	cell_2858
cell_2858:	.word	safeSpaceFlash-102836

.space	0, 45
.global	cell_2863
cell_2863:	.word	safeSpaceSram+700

.space	1, 45
.global	cell_2865
cell_2865:	.short	0x9f39

.space	2, 45
.global	cell_2859
cell_2859:	.short	0x783d

.space	0, 45
.global	cell_2864
cell_2864:	.byte	0xea

.space	0, 45
.global	cell_2861
cell_2861:	.byte	0xb4

.space	0, 45
.global	cell_2860
cell_2860:	.word	safeSpaceGpramSram+292

.space	1, 45
.global	cell_2866
cell_2866:	.word	safeSpaceSram+769

.space	2, 45
.global	cell_2857
cell_2857:	.short	0x77de

.space	3, 45
.global	cell_2862
cell_2862:	.word	safeSpaceGpramSram-1808

.align	1
.space 888 % {{space_mod|default("0x10000000")}}

.align	1
.space 1094 % {{space_mod|default("0x10000000")}}
label481:
.space 2
ldr	r9, cell_2954  @ 4b
ldr	r0, cell_2953  @ 2b
.space 4
mov	r7, #2  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 2
.space 4
mov	r8, #16164  @ 4b
ldr	r7, cell_2952  @ 4b
.space 4
.space 4
.space 4
mov	r1, #25  @ 4b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
mov	r7, #5  @ 4b
.space 4
end_label481:
	b.w	{{jump_label481}}

.ltorg
.align	2
.space	0, 45
.global	cell_2951
cell_2951:	.byte	0xc0

.space	1, 45
.global	cell_2952
cell_2952:	.word	safeSpaceSram-15556

.space	3, 45
.global	cell_2954
cell_2954:	.word	safeSpaceGpramSram+142

.space	3, 45
.global	cell_2953
cell_2953:	.word	safeSpaceGpramSram+272

.align	1
.space 675 % {{space_mod|default("0x10000000")}}

.align	1
.space 2639 % {{space_mod|default("0x10000000")}}

.align	1
.space 1582 % {{space_mod|default("0x10000000")}}

.align	1
.space 1668 % {{space_mod|default("0x10000000")}}

.align	1
.space 1875 % {{space_mod|default("0x10000000")}}

.align	1
.space 4787 % {{space_mod|default("0x10000000")}}

.align	1
.space 1261 % {{space_mod|default("0x10000000")}}

.align	1
.space 1296 % {{space_mod|default("0x10000000")}}
label591:
mov	r10, #50856  @ 4b
ldr	r0, cell_3662  @ 4b
ldr	r9, cell_3661  @ 4b
ldr	r6, cell_3660  @ 4b
mov	r5, #11223  @ 4b
.space 2

.space 4
.space 4
.space 4
.space 2
ldr	r6, cell_3659  @ 4b
.space 4
ldr	r0, cell_3658  @ 2b
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r4, cell_3657  @ 4b
.space 4
.space 4

forward_label_1061:
.space 4
.space 4
.space 4
.space 2
.space 4
.space 4
.space 2
.space 4
.space 4
ldr	r5, cell_3655  @ 4b
.space 4
end_label591:
	b.w	{{jump_label591}}

.ltorg
.align	2
.space	3, 45
.global	cell_3657
cell_3657:	.word	safeSpaceSram+875

.space	1, 45
.global	cell_3658
cell_3658:	.word	safeSpaceGpramSram+584

.space	2, 45
.global	cell_3659
cell_3659:	.word	safeSpaceSram+628

.space	2, 45
.global	cell_3656
cell_3656:	.short	0xd505

.space	0, 45
.global	cell_3661
cell_3661:	.word	safeSpaceSram+260

.space	3, 45
.global	cell_3660
cell_3660:	.word	safeSpaceFlash-10383

.space	1, 45
.global	cell_3655
cell_3655:	.word	safeSpaceGpramSram-202601

.space	1, 45
.global	cell_3662
cell_3662:	.word	safeSpaceFlash-1237

.align	1
.space 2466 % {{space_mod|default("0x10000000")}}
func_75:
.space 2
ldr	r1, =forward_label_1097  @ 2b
orr	r1, #1  @ 4b
mov	r0, #2320  @ 4b
mov	r3, #17  @ 4b
ldr	r2, cell_3765  @ 4b
.space 2

.space 2
.space 2
.space 4
.space 4
.space 4

forward_label_1097:
.space 4
.space 2
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
ldr	r10, cell_3760  @ 4b
.space 4
mov	r2, #0  @ 4b
.space 4
.space 4
.space 4
ldr	r2, cell_3759  @ 4b
.space 4
ldr	r1, cell_3758  @ 4b
.space 4
.space 2
.space 4
.space 4
.space 2
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
.space 4
end_func_75:
	bx	r14

.ltorg
.align	2
.space	0, 45
.global	cell_3763
cell_3763:	.short	0xca8a

.space	3, 45
.global	cell_3764
cell_3764:	.byte	0x6c

.space	0, 45
.global	cell_3761
cell_3761:	.byte	0xf7

.space	0, 45
.global	cell_3762
cell_3762:	.byte	0xfe

.space	0, 45
.global	cell_3760
cell_3760:	.word	safeSpaceFlash-3317

.space	3, 45
.global	cell_3759
cell_3759:	.word	safeSpaceGpramSram+122

.space	2, 45
.global	cell_3758
cell_3758:	.word	safeSpaceSram+880

.space	2, 45
.global	cell_3765
cell_3765:	.word	safeSpaceSram-3765

.align	1
.space 847 % {{space_mod|default("0x10000000")}}

.align	1
.space 1493 % {{space_mod|default("0x10000000")}}



.align  4
.thumb_func
code_end:

    @ Get counter finish value
    ldr.w  r14, [r11]
    @ Calculate counter difference
    sub.w  r14, r14, r12
    @ Mask counter difference if this is not the 4-byte CYCCNT
    ldr.w  r12, cyccnt_addr
    cmp.w  r11, r12
    it.n ne
    andne.w  r14, r14, 0xFF
    @ Save counter difference
    {{saveValue("counters", r14, r11, r12)}}

    @ Save values of registers
	{{saveValue("registers", r0, r11, r12)}}
	{{saveValue("registers", r1, r11, r12)}}
	{{saveValue("registers", r2, r11, r12)}}
	{{saveValue("registers", r3, r11, r12)}}
	{{saveValue("registers", r4, r11, r12)}}
	{{saveValue("registers", r5, r11, r12)}}
	{{saveValue("registers", r6, r11, r12)}}
	{{saveValue("registers", r7, r11, r12)}}
	{{saveValue("registers", r8, r11, r12)}}
	{{saveValue("registers", r9, r11, r12)}}
	{{saveValue("registers", r10, r11, r12)}}

    @ Advance counter_idx and repeat or end the test
    ldr.w  r11, =counter_idx
    ldr.w  r12, [r11]
    add.w  r12, r12, #4
    str.w  r12, [r11]
    cmp.w  r12, end_counters_to_test-counters_to_test-4
    bls.w  start_test
    b.w    end_label
.align  2
cyccnt_addr:       .word   {{CYCCNT_ADDR}}



@ Global data:
{{section('flash')}}
.align  2
.space	2, 46
.space 203 % {{space_mod|default("0x10000000")}}

.space	0, 46
.space 125 % {{space_mod|default("0x10000000")}}


{{section('sram')}}
.align  2
.space	0, 46
.space 374 % {{space_mod|default("0x10000000")}}

.space	1, 46
.space 43 % {{space_mod|default("0x10000000")}}



@ safeSpaces:
{{section('flash')}}
.align  4
.global safeSpaceFlash
safeSpaceFlash:      .space  1024, 41       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceFlash, .-safeSpaceFlash

{{section('sram')}}
.align  4
.global safeSpaceSram
safeSpaceSram:      .space  1024, 42       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceSram, .-safeSpaceSram

{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  4
.global safeSpaceGpramSram
safeSpaceGpramSram: .space  1024, 43       @ See SafeAddrConstraint in instructions/constraints.py
.size               safeSpaceGpramSram, .-safeSpaceGpramSram


@ Stack:
{{section('sram')}}
.align  4
.global stack
stack:  .space  400, 44    @ 256B of stack + upper and lower safety offsets for ldm/stm
.size   stack, .-stack


@ Test's data:

{{section('flash')}}
.align 2
.global counters_to_test
counters_to_test:    .word {{CYCCNT_ADDR}}, {{CYCCNT_ADDR}}, {{CYCCNT_ADDR}}, {{CYCCNT_ADDR}}, {{CPICNT_ADDR}}, {{LSUCNT_ADDR}}, {{FOLDCNT_ADDR}}
end_counters_to_test:



{{section('sram')}}

.align  2
.global original_sp
original_sp:        .word   0x00000000

.align  2
.global counter_idx
counter_idx:     .word   0


{% endblock %}