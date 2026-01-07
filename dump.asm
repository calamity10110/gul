
tests/simple_fn:     file format elf64-x86-64


Disassembly of section .init:

0000000000002000 <_init>:
    2000:	f3 0f 1e fa          	endbr64
    2004:	48 83 ec 08          	sub    $0x8,%rsp
    2008:	48 8b 05 d9 6f 00 00 	mov    0x6fd9(%rip),%rax        # 8fe8 <__gmon_start__@Base>
    200f:	48 85 c0             	test   %rax,%rax
    2012:	74 02                	je     2016 <_init+0x16>
    2014:	ff d0                	call   *%rax
    2016:	48 83 c4 08          	add    $0x8,%rsp
    201a:	c3                   	ret

Disassembly of section .plt:

0000000000002020 <free@plt-0x10>:
    2020:	ff 35 3a 6e 00 00    	push   0x6e3a(%rip)        # 8e60 <_GLOBAL_OFFSET_TABLE_+0x8>
    2026:	ff 25 3c 6e 00 00    	jmp    *0x6e3c(%rip)        # 8e68 <_GLOBAL_OFFSET_TABLE_+0x10>
    202c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000002030 <free@plt>:
    2030:	ff 25 3a 6e 00 00    	jmp    *0x6e3a(%rip)        # 8e70 <free@GLIBC_2.2.5>
    2036:	68 00 00 00 00       	push   $0x0
    203b:	e9 e0 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002040 <log2@plt>:
    2040:	ff 25 32 6e 00 00    	jmp    *0x6e32(%rip)        # 8e78 <log2@GLIBC_2.29>
    2046:	68 01 00 00 00       	push   $0x1
    204b:	e9 d0 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002050 <round@plt>:
    2050:	ff 25 2a 6e 00 00    	jmp    *0x6e2a(%rip)        # 8e80 <round@GLIBC_2.2.5>
    2056:	68 02 00 00 00       	push   $0x2
    205b:	e9 c0 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002060 <strncpy@plt>:
    2060:	ff 25 22 6e 00 00    	jmp    *0x6e22(%rip)        # 8e88 <strncpy@GLIBC_2.2.5>
    2066:	68 03 00 00 00       	push   $0x3
    206b:	e9 b0 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002070 <strcpy@plt>:
    2070:	ff 25 1a 6e 00 00    	jmp    *0x6e1a(%rip)        # 8e90 <strcpy@GLIBC_2.2.5>
    2076:	68 04 00 00 00       	push   $0x4
    207b:	e9 a0 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002080 <trunc@plt>:
    2080:	ff 25 12 6e 00 00    	jmp    *0x6e12(%rip)        # 8e98 <trunc@GLIBC_2.2.5>
    2086:	68 05 00 00 00       	push   $0x5
    208b:	e9 90 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002090 <puts@plt>:
    2090:	ff 25 0a 6e 00 00    	jmp    *0x6e0a(%rip)        # 8ea0 <puts@GLIBC_2.2.5>
    2096:	68 06 00 00 00       	push   $0x6
    209b:	e9 80 ff ff ff       	jmp    2020 <_init+0x20>

00000000000020a0 <atan@plt>:
    20a0:	ff 25 02 6e 00 00    	jmp    *0x6e02(%rip)        # 8ea8 <atan@GLIBC_2.2.5>
    20a6:	68 07 00 00 00       	push   $0x7
    20ab:	e9 70 ff ff ff       	jmp    2020 <_init+0x20>

00000000000020b0 <pow@plt>:
    20b0:	ff 25 fa 6d 00 00    	jmp    *0x6dfa(%rip)        # 8eb0 <pow@GLIBC_2.29>
    20b6:	68 08 00 00 00       	push   $0x8
    20bb:	e9 60 ff ff ff       	jmp    2020 <_init+0x20>

00000000000020c0 <fclose@plt>:
    20c0:	ff 25 f2 6d 00 00    	jmp    *0x6df2(%rip)        # 8eb8 <fclose@GLIBC_2.2.5>
    20c6:	68 09 00 00 00       	push   $0x9
    20cb:	e9 50 ff ff ff       	jmp    2020 <_init+0x20>

00000000000020d0 <strlen@plt>:
    20d0:	ff 25 ea 6d 00 00    	jmp    *0x6dea(%rip)        # 8ec0 <strlen@GLIBC_2.2.5>
    20d6:	68 0a 00 00 00       	push   $0xa
    20db:	e9 40 ff ff ff       	jmp    2020 <_init+0x20>

00000000000020e0 <__stack_chk_fail@plt>:
    20e0:	ff 25 e2 6d 00 00    	jmp    *0x6de2(%rip)        # 8ec8 <__stack_chk_fail@GLIBC_2.4>
    20e6:	68 0b 00 00 00       	push   $0xb
    20eb:	e9 30 ff ff ff       	jmp    2020 <_init+0x20>

00000000000020f0 <system@plt>:
    20f0:	ff 25 da 6d 00 00    	jmp    *0x6dda(%rip)        # 8ed0 <system@GLIBC_2.2.5>
    20f6:	68 0c 00 00 00       	push   $0xc
    20fb:	e9 20 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002100 <printf@plt>:
    2100:	ff 25 d2 6d 00 00    	jmp    *0x6dd2(%rip)        # 8ed8 <printf@GLIBC_2.2.5>
    2106:	68 0d 00 00 00       	push   $0xd
    210b:	e9 10 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002110 <snprintf@plt>:
    2110:	ff 25 ca 6d 00 00    	jmp    *0x6dca(%rip)        # 8ee0 <snprintf@GLIBC_2.2.5>
    2116:	68 0e 00 00 00       	push   $0xe
    211b:	e9 00 ff ff ff       	jmp    2020 <_init+0x20>

0000000000002120 <memset@plt>:
    2120:	ff 25 c2 6d 00 00    	jmp    *0x6dc2(%rip)        # 8ee8 <memset@GLIBC_2.2.5>
    2126:	68 0f 00 00 00       	push   $0xf
    212b:	e9 f0 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002130 <log@plt>:
    2130:	ff 25 ba 6d 00 00    	jmp    *0x6dba(%rip)        # 8ef0 <log@GLIBC_2.29>
    2136:	68 10 00 00 00       	push   $0x10
    213b:	e9 e0 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002140 <cos@plt>:
    2140:	ff 25 b2 6d 00 00    	jmp    *0x6db2(%rip)        # 8ef8 <cos@GLIBC_2.2.5>
    2146:	68 11 00 00 00       	push   $0x11
    214b:	e9 d0 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002150 <acos@plt>:
    2150:	ff 25 aa 6d 00 00    	jmp    *0x6daa(%rip)        # 8f00 <acos@GLIBC_2.2.5>
    2156:	68 12 00 00 00       	push   $0x12
    215b:	e9 c0 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002160 <fgets@plt>:
    2160:	ff 25 a2 6d 00 00    	jmp    *0x6da2(%rip)        # 8f08 <fgets@GLIBC_2.2.5>
    2166:	68 13 00 00 00       	push   $0x13
    216b:	e9 b0 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002170 <calloc@plt>:
    2170:	ff 25 9a 6d 00 00    	jmp    *0x6d9a(%rip)        # 8f10 <calloc@GLIBC_2.2.5>
    2176:	68 14 00 00 00       	push   $0x14
    217b:	e9 a0 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002180 <strcmp@plt>:
    2180:	ff 25 92 6d 00 00    	jmp    *0x6d92(%rip)        # 8f18 <strcmp@GLIBC_2.2.5>
    2186:	68 15 00 00 00       	push   $0x15
    218b:	e9 90 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002190 <getchar@plt>:
    2190:	ff 25 8a 6d 00 00    	jmp    *0x6d8a(%rip)        # 8f20 <getchar@GLIBC_2.2.5>
    2196:	68 16 00 00 00       	push   $0x16
    219b:	e9 80 fe ff ff       	jmp    2020 <_init+0x20>

00000000000021a0 <log10@plt>:
    21a0:	ff 25 82 6d 00 00    	jmp    *0x6d82(%rip)        # 8f28 <log10@GLIBC_2.2.5>
    21a6:	68 17 00 00 00       	push   $0x17
    21ab:	e9 70 fe ff ff       	jmp    2020 <_init+0x20>

00000000000021b0 <sqrtf@plt>:
    21b0:	ff 25 7a 6d 00 00    	jmp    *0x6d7a(%rip)        # 8f30 <sqrtf@GLIBC_2.2.5>
    21b6:	68 18 00 00 00       	push   $0x18
    21bb:	e9 60 fe ff ff       	jmp    2020 <_init+0x20>

00000000000021c0 <malloc@plt>:
    21c0:	ff 25 72 6d 00 00    	jmp    *0x6d72(%rip)        # 8f38 <malloc@GLIBC_2.2.5>
    21c6:	68 19 00 00 00       	push   $0x19
    21cb:	e9 50 fe ff ff       	jmp    2020 <_init+0x20>

00000000000021d0 <tan@plt>:
    21d0:	ff 25 6a 6d 00 00    	jmp    *0x6d6a(%rip)        # 8f40 <tan@GLIBC_2.2.5>
    21d6:	68 1a 00 00 00       	push   $0x1a
    21db:	e9 40 fe ff ff       	jmp    2020 <_init+0x20>

00000000000021e0 <atan2@plt>:
    21e0:	ff 25 62 6d 00 00    	jmp    *0x6d62(%rip)        # 8f48 <atan2@GLIBC_2.2.5>
    21e6:	68 1b 00 00 00       	push   $0x1b
    21eb:	e9 30 fe ff ff       	jmp    2020 <_init+0x20>

00000000000021f0 <realloc@plt>:
    21f0:	ff 25 5a 6d 00 00    	jmp    *0x6d5a(%rip)        # 8f50 <realloc@GLIBC_2.2.5>
    21f6:	68 1c 00 00 00       	push   $0x1c
    21fb:	e9 20 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002200 <memmove@plt>:
    2200:	ff 25 52 6d 00 00    	jmp    *0x6d52(%rip)        # 8f58 <memmove@GLIBC_2.2.5>
    2206:	68 1d 00 00 00       	push   $0x1d
    220b:	e9 10 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002210 <cbrt@plt>:
    2210:	ff 25 4a 6d 00 00    	jmp    *0x6d4a(%rip)        # 8f60 <cbrt@GLIBC_2.2.5>
    2216:	68 1e 00 00 00       	push   $0x1e
    221b:	e9 00 fe ff ff       	jmp    2020 <_init+0x20>

0000000000002220 <fopen@plt>:
    2220:	ff 25 42 6d 00 00    	jmp    *0x6d42(%rip)        # 8f68 <fopen@GLIBC_2.2.5>
    2226:	68 1f 00 00 00       	push   $0x1f
    222b:	e9 f0 fd ff ff       	jmp    2020 <_init+0x20>

0000000000002230 <sin@plt>:
    2230:	ff 25 3a 6d 00 00    	jmp    *0x6d3a(%rip)        # 8f70 <sin@GLIBC_2.2.5>
    2236:	68 20 00 00 00       	push   $0x20
    223b:	e9 e0 fd ff ff       	jmp    2020 <_init+0x20>

0000000000002240 <floor@plt>:
    2240:	ff 25 32 6d 00 00    	jmp    *0x6d32(%rip)        # 8f78 <floor@GLIBC_2.2.5>
    2246:	68 21 00 00 00       	push   $0x21
    224b:	e9 d0 fd ff ff       	jmp    2020 <_init+0x20>

0000000000002250 <__isoc99_scanf@plt>:
    2250:	ff 25 2a 6d 00 00    	jmp    *0x6d2a(%rip)        # 8f80 <__isoc99_scanf@GLIBC_2.7>
    2256:	68 22 00 00 00       	push   $0x22
    225b:	e9 c0 fd ff ff       	jmp    2020 <_init+0x20>

0000000000002260 <strcat@plt>:
    2260:	ff 25 22 6d 00 00    	jmp    *0x6d22(%rip)        # 8f88 <strcat@GLIBC_2.2.5>
    2266:	68 23 00 00 00       	push   $0x23
    226b:	e9 b0 fd ff ff       	jmp    2020 <_init+0x20>

0000000000002270 <tanh@plt>:
    2270:	ff 25 1a 6d 00 00    	jmp    *0x6d1a(%rip)        # 8f90 <tanh@GLIBC_2.2.5>
    2276:	68 24 00 00 00       	push   $0x24
    227b:	e9 a0 fd ff ff       	jmp    2020 <_init+0x20>

0000000000002280 <asin@plt>:
    2280:	ff 25 12 6d 00 00    	jmp    *0x6d12(%rip)        # 8f98 <asin@GLIBC_2.2.5>
    2286:	68 25 00 00 00       	push   $0x25
    228b:	e9 90 fd ff ff       	jmp    2020 <_init+0x20>

0000000000002290 <sprintf@plt>:
    2290:	ff 25 0a 6d 00 00    	jmp    *0x6d0a(%rip)        # 8fa0 <sprintf@GLIBC_2.2.5>
    2296:	68 26 00 00 00       	push   $0x26
    229b:	e9 80 fd ff ff       	jmp    2020 <_init+0x20>

00000000000022a0 <exit@plt>:
    22a0:	ff 25 02 6d 00 00    	jmp    *0x6d02(%rip)        # 8fa8 <exit@GLIBC_2.2.5>
    22a6:	68 27 00 00 00       	push   $0x27
    22ab:	e9 70 fd ff ff       	jmp    2020 <_init+0x20>

00000000000022b0 <fwrite@plt>:
    22b0:	ff 25 fa 6c 00 00    	jmp    *0x6cfa(%rip)        # 8fb0 <fwrite@GLIBC_2.2.5>
    22b6:	68 28 00 00 00       	push   $0x28
    22bb:	e9 60 fd ff ff       	jmp    2020 <_init+0x20>

00000000000022c0 <sqrt@plt>:
    22c0:	ff 25 f2 6c 00 00    	jmp    *0x6cf2(%rip)        # 8fb8 <sqrt@GLIBC_2.2.5>
    22c6:	68 29 00 00 00       	push   $0x29
    22cb:	e9 50 fd ff ff       	jmp    2020 <_init+0x20>

00000000000022d0 <strdup@plt>:
    22d0:	ff 25 ea 6c 00 00    	jmp    *0x6cea(%rip)        # 8fc0 <strdup@GLIBC_2.2.5>
    22d6:	68 2a 00 00 00       	push   $0x2a
    22db:	e9 40 fd ff ff       	jmp    2020 <_init+0x20>

00000000000022e0 <ceil@plt>:
    22e0:	ff 25 e2 6c 00 00    	jmp    *0x6ce2(%rip)        # 8fc8 <ceil@GLIBC_2.2.5>
    22e6:	68 2b 00 00 00       	push   $0x2b
    22eb:	e9 30 fd ff ff       	jmp    2020 <_init+0x20>

00000000000022f0 <exp@plt>:
    22f0:	ff 25 da 6c 00 00    	jmp    *0x6cda(%rip)        # 8fd0 <exp@GLIBC_2.29>
    22f6:	68 2c 00 00 00       	push   $0x2c
    22fb:	e9 20 fd ff ff       	jmp    2020 <_init+0x20>

Disassembly of section .plt.got:

0000000000002300 <__cxa_finalize@plt>:
    2300:	ff 25 f2 6c 00 00    	jmp    *0x6cf2(%rip)        # 8ff8 <__cxa_finalize@GLIBC_2.2.5>
    2306:	66 90                	xchg   %ax,%ax

Disassembly of section .text:

0000000000002310 <_start>:
    2310:	f3 0f 1e fa          	endbr64
    2314:	31 ed                	xor    %ebp,%ebp
    2316:	49 89 d1             	mov    %rdx,%r9
    2319:	5e                   	pop    %rsi
    231a:	48 89 e2             	mov    %rsp,%rdx
    231d:	48 83 e4 f0          	and    $0xfffffffffffffff0,%rsp
    2321:	50                   	push   %rax
    2322:	54                   	push   %rsp
    2323:	45 31 c0             	xor    %r8d,%r8d
    2326:	31 c9                	xor    %ecx,%ecx
    2328:	48 8d 3d ca 00 00 00 	lea    0xca(%rip),%rdi        # 23f9 <main>
    232f:	ff 15 a3 6c 00 00    	call   *0x6ca3(%rip)        # 8fd8 <__libc_start_main@GLIBC_2.34>
    2335:	f4                   	hlt
    2336:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
    233d:	00 00 00 

0000000000002340 <deregister_tm_clones>:
    2340:	48 8d 3d e1 6c 00 00 	lea    0x6ce1(%rip),%rdi        # 9028 <__TMC_END__>
    2347:	48 8d 05 da 6c 00 00 	lea    0x6cda(%rip),%rax        # 9028 <__TMC_END__>
    234e:	48 39 f8             	cmp    %rdi,%rax
    2351:	74 15                	je     2368 <deregister_tm_clones+0x28>
    2353:	48 8b 05 86 6c 00 00 	mov    0x6c86(%rip),%rax        # 8fe0 <_ITM_deregisterTMCloneTable@Base>
    235a:	48 85 c0             	test   %rax,%rax
    235d:	74 09                	je     2368 <deregister_tm_clones+0x28>
    235f:	ff e0                	jmp    *%rax
    2361:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
    2368:	c3                   	ret
    2369:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

0000000000002370 <register_tm_clones>:
    2370:	48 8d 3d b1 6c 00 00 	lea    0x6cb1(%rip),%rdi        # 9028 <__TMC_END__>
    2377:	48 8d 35 aa 6c 00 00 	lea    0x6caa(%rip),%rsi        # 9028 <__TMC_END__>
    237e:	48 29 fe             	sub    %rdi,%rsi
    2381:	48 89 f0             	mov    %rsi,%rax
    2384:	48 c1 ee 3f          	shr    $0x3f,%rsi
    2388:	48 c1 f8 03          	sar    $0x3,%rax
    238c:	48 01 c6             	add    %rax,%rsi
    238f:	48 d1 fe             	sar    $1,%rsi
    2392:	74 14                	je     23a8 <register_tm_clones+0x38>
    2394:	48 8b 05 55 6c 00 00 	mov    0x6c55(%rip),%rax        # 8ff0 <_ITM_registerTMCloneTable@Base>
    239b:	48 85 c0             	test   %rax,%rax
    239e:	74 08                	je     23a8 <register_tm_clones+0x38>
    23a0:	ff e0                	jmp    *%rax
    23a2:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
    23a8:	c3                   	ret
    23a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

00000000000023b0 <__do_global_dtors_aux>:
    23b0:	f3 0f 1e fa          	endbr64
    23b4:	80 3d ad 6c 00 00 00 	cmpb   $0x0,0x6cad(%rip)        # 9068 <completed.0>
    23bb:	75 2b                	jne    23e8 <__do_global_dtors_aux+0x38>
    23bd:	55                   	push   %rbp
    23be:	48 83 3d 32 6c 00 00 	cmpq   $0x0,0x6c32(%rip)        # 8ff8 <__cxa_finalize@GLIBC_2.2.5>
    23c5:	00 
    23c6:	48 89 e5             	mov    %rsp,%rbp
    23c9:	74 0c                	je     23d7 <__do_global_dtors_aux+0x27>
    23cb:	48 8b 3d 36 6c 00 00 	mov    0x6c36(%rip),%rdi        # 9008 <__dso_handle>
    23d2:	e8 29 ff ff ff       	call   2300 <__cxa_finalize@plt>
    23d7:	e8 64 ff ff ff       	call   2340 <deregister_tm_clones>
    23dc:	c6 05 85 6c 00 00 01 	movb   $0x1,0x6c85(%rip)        # 9068 <completed.0>
    23e3:	5d                   	pop    %rbp
    23e4:	c3                   	ret
    23e5:	0f 1f 00             	nopl   (%rax)
    23e8:	c3                   	ret
    23e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

00000000000023f0 <frame_dummy>:
    23f0:	f3 0f 1e fa          	endbr64
    23f4:	e9 77 ff ff ff       	jmp    2370 <register_tm_clones>

00000000000023f9 <main>:
    23f9:	55                   	push   %rbp
    23fa:	48 89 e5             	mov    %rsp,%rbp
    23fd:	48 83 ec 10          	sub    $0x10,%rsp
    2401:	48 89 1c 24          	mov    %rbx,(%rsp)
    2405:	48 31 f6             	xor    %rsi,%rsi
    2408:	48 89 f3             	mov    %rsi,%rbx
    240b:	ba 03 00 00 00       	mov    $0x3,%edx
    2410:	48 85 d2             	test   %rdx,%rdx
    2413:	0f 84 37 00 00 00    	je     2450 <main+0x57>
    2419:	48 8d 35 fc 6b 00 00 	lea    0x6bfc(%rip),%rsi        # 901c <str_1>
    2420:	48 8d 3d ee 6b 00 00 	lea    0x6bee(%rip),%rdi        # 9015 <fmt_str>
    2427:	49 b9 00 00 00 00 00 	movabs $0x0,%r9
    242e:	00 00 00 
    2431:	41 ff d1             	call   *%r9
    2434:	48 8d 3d d5 6b 00 00 	lea    0x6bd5(%rip),%rdi        # 9010 <fmt_int>
    243b:	49 b9 00 00 00 00 00 	movabs $0x0,%r9
    2442:	00 00 00 
    2445:	48 89 de             	mov    %rbx,%rsi
    2448:	41 ff d1             	call   *%r9
    244b:	e9 1b 00 00 00       	jmp    246b <main+0x72>
    2450:	48 8d 35 c2 6b 00 00 	lea    0x6bc2(%rip),%rsi        # 9019 <str_0>
    2457:	48 8d 3d b7 6b 00 00 	lea    0x6bb7(%rip),%rdi        # 9015 <fmt_str>
    245e:	49 ba 00 00 00 00 00 	movabs $0x0,%r10
    2465:	00 00 00 
    2468:	41 ff d2             	call   *%r10
    246b:	31 c0                	xor    %eax,%eax
    246d:	48 8b 1c 24          	mov    (%rsp),%rbx
    2471:	48 83 c4 10          	add    $0x10,%rsp
    2475:	48 89 ec             	mov    %rbp,%rsp
    2478:	5d                   	pop    %rbp
    2479:	c3                   	ret

000000000000247a <gul_string_concat>:
    247a:	f3 0f 1e fa          	endbr64
    247e:	55                   	push   %rbp
    247f:	48 89 e5             	mov    %rsp,%rbp
    2482:	48 83 ec 40          	sub    $0x40,%rsp
    2486:	48 89 7d c8          	mov    %rdi,-0x38(%rbp)
    248a:	48 89 75 c0          	mov    %rsi,-0x40(%rbp)
    248e:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    2492:	48 89 45 d8          	mov    %rax,-0x28(%rbp)
    2496:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
    249a:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    249e:	48 83 7d d8 00       	cmpq   $0x0,-0x28(%rbp)
    24a3:	75 0b                	jne    24b0 <gul_string_concat+0x36>
    24a5:	48 8d 05 64 3b 00 00 	lea    0x3b64(%rip),%rax        # 6010 <_IO_stdin_used+0x10>
    24ac:	48 89 45 d8          	mov    %rax,-0x28(%rbp)
    24b0:	48 83 7d e0 00       	cmpq   $0x0,-0x20(%rbp)
    24b5:	75 0b                	jne    24c2 <gul_string_concat+0x48>
    24b7:	48 8d 05 52 3b 00 00 	lea    0x3b52(%rip),%rax        # 6010 <_IO_stdin_used+0x10>
    24be:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    24c2:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    24c6:	48 89 c7             	mov    %rax,%rdi
    24c9:	e8 02 fc ff ff       	call   20d0 <strlen@plt>
    24ce:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    24d2:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    24d6:	48 89 c7             	mov    %rax,%rdi
    24d9:	e8 f2 fb ff ff       	call   20d0 <strlen@plt>
    24de:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    24e2:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    24e6:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    24ea:	48 01 d0             	add    %rdx,%rax
    24ed:	48 83 c0 01          	add    $0x1,%rax
    24f1:	48 89 c7             	mov    %rax,%rdi
    24f4:	e8 c7 fc ff ff       	call   21c0 <malloc@plt>
    24f9:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    24fd:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    2502:	75 2d                	jne    2531 <gul_string_concat+0xb7>
    2504:	48 8b 05 55 6b 00 00 	mov    0x6b55(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    250b:	48 89 c1             	mov    %rax,%rcx
    250e:	ba 2c 00 00 00       	mov    $0x2c,%edx
    2513:	be 01 00 00 00       	mov    $0x1,%esi
    2518:	48 8d 05 f9 3a 00 00 	lea    0x3af9(%rip),%rax        # 6018 <_IO_stdin_used+0x18>
    251f:	48 89 c7             	mov    %rax,%rdi
    2522:	e8 89 fd ff ff       	call   22b0 <fwrite@plt>
    2527:	bf 01 00 00 00       	mov    $0x1,%edi
    252c:	e8 6f fd ff ff       	call   22a0 <exit@plt>
    2531:	48 8b 55 d8          	mov    -0x28(%rbp),%rdx
    2535:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2539:	48 89 d6             	mov    %rdx,%rsi
    253c:	48 89 c7             	mov    %rax,%rdi
    253f:	e8 2c fb ff ff       	call   2070 <strcpy@plt>
    2544:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    2548:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    254c:	48 89 d6             	mov    %rdx,%rsi
    254f:	48 89 c7             	mov    %rax,%rdi
    2552:	e8 09 fd ff ff       	call   2260 <strcat@plt>
    2557:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    255b:	c9                   	leave
    255c:	c3                   	ret

000000000000255d <gul_int_to_string>:
    255d:	f3 0f 1e fa          	endbr64
    2561:	55                   	push   %rbp
    2562:	48 89 e5             	mov    %rsp,%rbp
    2565:	48 83 ec 20          	sub    $0x20,%rsp
    2569:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    256d:	bf 20 00 00 00       	mov    $0x20,%edi
    2572:	e8 49 fc ff ff       	call   21c0 <malloc@plt>
    2577:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    257b:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    2580:	75 2d                	jne    25af <gul_int_to_string+0x52>
    2582:	48 8b 05 d7 6a 00 00 	mov    0x6ad7(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    2589:	48 89 c1             	mov    %rax,%rcx
    258c:	ba 2c 00 00 00       	mov    $0x2c,%edx
    2591:	be 01 00 00 00       	mov    $0x1,%esi
    2596:	48 8d 05 ab 3a 00 00 	lea    0x3aab(%rip),%rax        # 6048 <_IO_stdin_used+0x48>
    259d:	48 89 c7             	mov    %rax,%rdi
    25a0:	e8 0b fd ff ff       	call   22b0 <fwrite@plt>
    25a5:	bf 01 00 00 00       	mov    $0x1,%edi
    25aa:	e8 f1 fc ff ff       	call   22a0 <exit@plt>
    25af:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    25b3:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    25b7:	48 8d 0d b7 3a 00 00 	lea    0x3ab7(%rip),%rcx        # 6075 <_IO_stdin_used+0x75>
    25be:	48 89 ce             	mov    %rcx,%rsi
    25c1:	48 89 c7             	mov    %rax,%rdi
    25c4:	b8 00 00 00 00       	mov    $0x0,%eax
    25c9:	e8 c2 fc ff ff       	call   2290 <sprintf@plt>
    25ce:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    25d2:	c9                   	leave
    25d3:	c3                   	ret

00000000000025d4 <gul_print_float>:
    25d4:	f3 0f 1e fa          	endbr64
    25d8:	55                   	push   %rbp
    25d9:	48 89 e5             	mov    %rsp,%rbp
    25dc:	48 83 ec 10          	sub    $0x10,%rsp
    25e0:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    25e5:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    25e9:	66 48 0f 6e c0       	movq   %rax,%xmm0
    25ee:	48 8d 05 84 3a 00 00 	lea    0x3a84(%rip),%rax        # 6079 <_IO_stdin_used+0x79>
    25f5:	48 89 c7             	mov    %rax,%rdi
    25f8:	b8 01 00 00 00       	mov    $0x1,%eax
    25fd:	e8 fe fa ff ff       	call   2100 <printf@plt>
    2602:	c9                   	leave
    2603:	c3                   	ret

0000000000002604 <gul_float_to_string>:
    2604:	f3 0f 1e fa          	endbr64
    2608:	55                   	push   %rbp
    2609:	48 89 e5             	mov    %rsp,%rbp
    260c:	48 83 ec 20          	sub    $0x20,%rsp
    2610:	f2 0f 11 45 e8       	movsd  %xmm0,-0x18(%rbp)
    2615:	bf 40 00 00 00       	mov    $0x40,%edi
    261a:	e8 a1 fb ff ff       	call   21c0 <malloc@plt>
    261f:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2623:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    2628:	75 2d                	jne    2657 <gul_float_to_string+0x53>
    262a:	48 8b 05 2f 6a 00 00 	mov    0x6a2f(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    2631:	48 89 c1             	mov    %rax,%rcx
    2634:	ba 2e 00 00 00       	mov    $0x2e,%edx
    2639:	be 01 00 00 00       	mov    $0x1,%esi
    263e:	48 8d 05 3b 3a 00 00 	lea    0x3a3b(%rip),%rax        # 6080 <_IO_stdin_used+0x80>
    2645:	48 89 c7             	mov    %rax,%rdi
    2648:	e8 63 fc ff ff       	call   22b0 <fwrite@plt>
    264d:	bf 01 00 00 00       	mov    $0x1,%edi
    2652:	e8 49 fc ff ff       	call   22a0 <exit@plt>
    2657:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    265b:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    265f:	66 48 0f 6e c2       	movq   %rdx,%xmm0
    2664:	48 8d 15 44 3a 00 00 	lea    0x3a44(%rip),%rdx        # 60af <_IO_stdin_used+0xaf>
    266b:	48 89 d6             	mov    %rdx,%rsi
    266e:	48 89 c7             	mov    %rax,%rdi
    2671:	b8 01 00 00 00       	mov    $0x1,%eax
    2676:	e8 15 fc ff ff       	call   2290 <sprintf@plt>
    267b:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    267f:	c9                   	leave
    2680:	c3                   	ret

0000000000002681 <gul_input_str>:
    2681:	f3 0f 1e fa          	endbr64
    2685:	55                   	push   %rbp
    2686:	48 89 e5             	mov    %rsp,%rbp
    2689:	48 83 ec 10          	sub    $0x10,%rsp
    268d:	bf 00 04 00 00       	mov    $0x400,%edi
    2692:	e8 29 fb ff ff       	call   21c0 <malloc@plt>
    2697:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    269b:	48 83 7d f0 00       	cmpq   $0x0,-0x10(%rbp)
    26a0:	75 2d                	jne    26cf <gul_input_str+0x4e>
    26a2:	48 8b 05 b7 69 00 00 	mov    0x69b7(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    26a9:	48 89 c1             	mov    %rax,%rcx
    26ac:	ba 28 00 00 00       	mov    $0x28,%edx
    26b1:	be 01 00 00 00       	mov    $0x1,%esi
    26b6:	48 8d 05 fb 39 00 00 	lea    0x39fb(%rip),%rax        # 60b8 <_IO_stdin_used+0xb8>
    26bd:	48 89 c7             	mov    %rax,%rdi
    26c0:	e8 eb fb ff ff       	call   22b0 <fwrite@plt>
    26c5:	bf 01 00 00 00       	mov    $0x1,%edi
    26ca:	e8 d1 fb ff ff       	call   22a0 <exit@plt>
    26cf:	48 8b 15 6a 69 00 00 	mov    0x696a(%rip),%rdx        # 9040 <stdin@GLIBC_2.2.5>
    26d6:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    26da:	be 00 04 00 00       	mov    $0x400,%esi
    26df:	48 89 c7             	mov    %rax,%rdi
    26e2:	e8 79 fa ff ff       	call   2160 <fgets@plt>
    26e7:	48 85 c0             	test   %rax,%rax
    26ea:	74 41                	je     272d <gul_input_str+0xac>
    26ec:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    26f0:	48 89 c7             	mov    %rax,%rdi
    26f3:	e8 d8 f9 ff ff       	call   20d0 <strlen@plt>
    26f8:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    26fc:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    2701:	74 31                	je     2734 <gul_input_str+0xb3>
    2703:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2707:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
    270b:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    270f:	48 01 d0             	add    %rdx,%rax
    2712:	0f b6 00             	movzbl (%rax),%eax
    2715:	3c 0a                	cmp    $0xa,%al
    2717:	75 1b                	jne    2734 <gul_input_str+0xb3>
    2719:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    271d:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
    2721:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2725:	48 01 d0             	add    %rdx,%rax
    2728:	c6 00 00             	movb   $0x0,(%rax)
    272b:	eb 07                	jmp    2734 <gul_input_str+0xb3>
    272d:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2731:	c6 00 00             	movb   $0x0,(%rax)
    2734:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2738:	c9                   	leave
    2739:	c3                   	ret

000000000000273a <gul_autograd_begin>:
    273a:	f3 0f 1e fa          	endbr64
    273e:	55                   	push   %rbp
    273f:	48 89 e5             	mov    %rsp,%rbp
    2742:	c7 05 b4 83 06 00 00 	movl   $0x0,0x683b4(%rip)        # 6ab00 <global_tape+0x61a80>
    2749:	00 00 00 
    274c:	c7 05 ae 83 06 00 01 	movl   $0x1,0x683ae(%rip)        # 6ab04 <global_tape+0x61a84>
    2753:	00 00 00 
    2756:	90                   	nop
    2757:	5d                   	pop    %rbp
    2758:	c3                   	ret

0000000000002759 <gul_autograd_end>:
    2759:	f3 0f 1e fa          	endbr64
    275d:	55                   	push   %rbp
    275e:	48 89 e5             	mov    %rsp,%rbp
    2761:	c7 05 99 83 06 00 00 	movl   $0x0,0x68399(%rip)        # 6ab04 <global_tape+0x61a84>
    2768:	00 00 00 
    276b:	90                   	nop
    276c:	5d                   	pop    %rbp
    276d:	c3                   	ret

000000000000276e <tape_add_node>:
    276e:	f3 0f 1e fa          	endbr64
    2772:	55                   	push   %rbp
    2773:	48 89 e5             	mov    %rsp,%rbp
    2776:	48 83 ec 30          	sub    $0x30,%rsp
    277a:	89 7d ec             	mov    %edi,-0x14(%rbp)
    277d:	f2 0f 11 45 e0       	movsd  %xmm0,-0x20(%rbp)
    2782:	89 75 e8             	mov    %esi,-0x18(%rbp)
    2785:	89 55 dc             	mov    %edx,-0x24(%rbp)
    2788:	8b 05 76 83 06 00    	mov    0x68376(%rip),%eax        # 6ab04 <global_tape+0x61a84>
    278e:	85 c0                	test   %eax,%eax
    2790:	75 0a                	jne    279c <tape_add_node+0x2e>
    2792:	b8 ff ff ff ff       	mov    $0xffffffff,%eax
    2797:	e9 a6 00 00 00       	jmp    2842 <tape_add_node+0xd4>
    279c:	8b 05 5e 83 06 00    	mov    0x6835e(%rip),%eax        # 6ab00 <global_tape+0x61a80>
    27a2:	3d 0f 27 00 00       	cmp    $0x270f,%eax
    27a7:	7e 2a                	jle    27d3 <tape_add_node+0x65>
    27a9:	48 8b 05 b0 68 00 00 	mov    0x68b0(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    27b0:	48 89 c1             	mov    %rax,%rcx
    27b3:	ba 24 00 00 00       	mov    $0x24,%edx
    27b8:	be 01 00 00 00       	mov    $0x1,%esi
    27bd:	48 8d 05 24 39 00 00 	lea    0x3924(%rip),%rax        # 60e8 <_IO_stdin_used+0xe8>
    27c4:	48 89 c7             	mov    %rax,%rdi
    27c7:	e8 e4 fa ff ff       	call   22b0 <fwrite@plt>
    27cc:	b8 ff ff ff ff       	mov    $0xffffffff,%eax
    27d1:	eb 6f                	jmp    2842 <tape_add_node+0xd4>
    27d3:	8b 05 27 83 06 00    	mov    0x68327(%rip),%eax        # 6ab00 <global_tape+0x61a80>
    27d9:	8d 50 01             	lea    0x1(%rax),%edx
    27dc:	89 15 1e 83 06 00    	mov    %edx,0x6831e(%rip)        # 6ab00 <global_tape+0x61a80>
    27e2:	89 45 f4             	mov    %eax,-0xc(%rbp)
    27e5:	8b 45 f4             	mov    -0xc(%rbp),%eax
    27e8:	48 63 d0             	movslq %eax,%rdx
    27eb:	48 89 d0             	mov    %rdx,%rax
    27ee:	48 c1 e0 02          	shl    $0x2,%rax
    27f2:	48 01 d0             	add    %rdx,%rax
    27f5:	48 c1 e0 03          	shl    $0x3,%rax
    27f9:	48 8d 15 80 68 00 00 	lea    0x6880(%rip),%rdx        # 9080 <global_tape>
    2800:	48 01 d0             	add    %rdx,%rax
    2803:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2807:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    280b:	8b 55 ec             	mov    -0x14(%rbp),%edx
    280e:	89 10                	mov    %edx,(%rax)
    2810:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2814:	f2 0f 10 45 e0       	movsd  -0x20(%rbp),%xmm0
    2819:	f2 0f 11 40 08       	movsd  %xmm0,0x8(%rax)
    281e:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2822:	66 0f ef c0          	pxor   %xmm0,%xmm0
    2826:	f2 0f 11 40 10       	movsd  %xmm0,0x10(%rax)
    282b:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    282f:	8b 55 e8             	mov    -0x18(%rbp),%edx
    2832:	89 50 18             	mov    %edx,0x18(%rax)
    2835:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2839:	8b 55 dc             	mov    -0x24(%rbp),%edx
    283c:	89 50 1c             	mov    %edx,0x1c(%rax)
    283f:	8b 45 f4             	mov    -0xc(%rbp),%eax
    2842:	c9                   	leave
    2843:	c3                   	ret

0000000000002844 <gul_grad_add>:
    2844:	f3 0f 1e fa          	endbr64
    2848:	55                   	push   %rbp
    2849:	48 89 e5             	mov    %rsp,%rbp
    284c:	f2 0f 11 45 e8       	movsd  %xmm0,-0x18(%rbp)
    2851:	f2 0f 11 4d e0       	movsd  %xmm1,-0x20(%rbp)
    2856:	89 7d dc             	mov    %edi,-0x24(%rbp)
    2859:	89 75 d8             	mov    %esi,-0x28(%rbp)
    285c:	f2 0f 10 45 e8       	movsd  -0x18(%rbp),%xmm0
    2861:	f2 0f 58 45 e0       	addsd  -0x20(%rbp),%xmm0
    2866:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    286b:	f2 0f 10 45 f8       	movsd  -0x8(%rbp),%xmm0
    2870:	5d                   	pop    %rbp
    2871:	c3                   	ret

0000000000002872 <gul_make_var>:
    2872:	f3 0f 1e fa          	endbr64
    2876:	55                   	push   %rbp
    2877:	48 89 e5             	mov    %rsp,%rbp
    287a:	48 83 ec 20          	sub    $0x20,%rsp
    287e:	f2 0f 11 45 e8       	movsd  %xmm0,-0x18(%rbp)
    2883:	bf 10 00 00 00       	mov    $0x10,%edi
    2888:	e8 33 f9 ff ff       	call   21c0 <malloc@plt>
    288d:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2891:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    2896:	75 2d                	jne    28c5 <gul_make_var+0x53>
    2898:	48 8b 05 c1 67 00 00 	mov    0x67c1(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    289f:	48 89 c1             	mov    %rax,%rcx
    28a2:	ba 2b 00 00 00       	mov    $0x2b,%edx
    28a7:	be 01 00 00 00       	mov    $0x1,%esi
    28ac:	48 8d 05 5d 38 00 00 	lea    0x385d(%rip),%rax        # 6110 <_IO_stdin_used+0x110>
    28b3:	48 89 c7             	mov    %rax,%rdi
    28b6:	e8 f5 f9 ff ff       	call   22b0 <fwrite@plt>
    28bb:	bf 01 00 00 00       	mov    $0x1,%edi
    28c0:	e8 db f9 ff ff       	call   22a0 <exit@plt>
    28c5:	8b 05 39 82 06 00    	mov    0x68239(%rip),%eax        # 6ab04 <global_tape+0x61a84>
    28cb:	85 c0                	test   %eax,%eax
    28cd:	74 26                	je     28f5 <gul_make_var+0x83>
    28cf:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    28d3:	ba ff ff ff ff       	mov    $0xffffffff,%edx
    28d8:	be ff ff ff ff       	mov    $0xffffffff,%esi
    28dd:	66 48 0f 6e c0       	movq   %rax,%xmm0
    28e2:	bf 00 00 00 00       	mov    $0x0,%edi
    28e7:	e8 82 fe ff ff       	call   276e <tape_add_node>
    28ec:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    28f0:	89 42 08             	mov    %eax,0x8(%rdx)
    28f3:	eb 0b                	jmp    2900 <gul_make_var+0x8e>
    28f5:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    28f9:	c7 40 08 ff ff ff ff 	movl   $0xffffffff,0x8(%rax)
    2900:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2904:	f2 0f 10 45 e8       	movsd  -0x18(%rbp),%xmm0
    2909:	f2 0f 11 00          	movsd  %xmm0,(%rax)
    290d:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2911:	c9                   	leave
    2912:	c3                   	ret

0000000000002913 <gul_var_val>:
    2913:	f3 0f 1e fa          	endbr64
    2917:	55                   	push   %rbp
    2918:	48 89 e5             	mov    %rsp,%rbp
    291b:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    291f:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    2924:	75 06                	jne    292c <gul_var_val+0x19>
    2926:	66 0f ef c0          	pxor   %xmm0,%xmm0
    292a:	eb 08                	jmp    2934 <gul_var_val+0x21>
    292c:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2930:	f2 0f 10 00          	movsd  (%rax),%xmm0
    2934:	5d                   	pop    %rbp
    2935:	c3                   	ret

0000000000002936 <gul_var_grad>:
    2936:	f3 0f 1e fa          	endbr64
    293a:	55                   	push   %rbp
    293b:	48 89 e5             	mov    %rsp,%rbp
    293e:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    2942:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    2947:	75 06                	jne    294f <gul_var_grad+0x19>
    2949:	66 0f ef c0          	pxor   %xmm0,%xmm0
    294d:	eb 51                	jmp    29a0 <gul_var_grad+0x6a>
    294f:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2953:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2957:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    295b:	8b 40 08             	mov    0x8(%rax),%eax
    295e:	85 c0                	test   %eax,%eax
    2960:	78 3a                	js     299c <gul_var_grad+0x66>
    2962:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2966:	8b 50 08             	mov    0x8(%rax),%edx
    2969:	8b 05 91 81 06 00    	mov    0x68191(%rip),%eax        # 6ab00 <global_tape+0x61a80>
    296f:	39 c2                	cmp    %eax,%edx
    2971:	7d 29                	jge    299c <gul_var_grad+0x66>
    2973:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2977:	8b 40 08             	mov    0x8(%rax),%eax
    297a:	48 63 d0             	movslq %eax,%rdx
    297d:	48 89 d0             	mov    %rdx,%rax
    2980:	48 c1 e0 02          	shl    $0x2,%rax
    2984:	48 01 d0             	add    %rdx,%rax
    2987:	48 c1 e0 03          	shl    $0x3,%rax
    298b:	48 89 c2             	mov    %rax,%rdx
    298e:	48 8d 05 fb 66 00 00 	lea    0x66fb(%rip),%rax        # 9090 <global_tape+0x10>
    2995:	f2 0f 10 04 02       	movsd  (%rdx,%rax,1),%xmm0
    299a:	eb 04                	jmp    29a0 <gul_var_grad+0x6a>
    299c:	66 0f ef c0          	pxor   %xmm0,%xmm0
    29a0:	5d                   	pop    %rbp
    29a1:	c3                   	ret

00000000000029a2 <gul_var_add>:
    29a2:	f3 0f 1e fa          	endbr64
    29a6:	55                   	push   %rbp
    29a7:	48 89 e5             	mov    %rsp,%rbp
    29aa:	48 83 ec 30          	sub    $0x30,%rsp
    29ae:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    29b2:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    29b6:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    29ba:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    29be:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    29c2:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    29c6:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    29ca:	f2 0f 10 08          	movsd  (%rax),%xmm1
    29ce:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    29d2:	f2 0f 10 00          	movsd  (%rax),%xmm0
    29d6:	f2 0f 58 c1          	addsd  %xmm1,%xmm0
    29da:	f2 0f 11 45 f0       	movsd  %xmm0,-0x10(%rbp)
    29df:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    29e3:	66 48 0f 6e c0       	movq   %rax,%xmm0
    29e8:	e8 85 fe ff ff       	call   2872 <gul_make_var>
    29ed:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    29f1:	8b 05 0d 81 06 00    	mov    0x6810d(%rip),%eax        # 6ab04 <global_tape+0x61a84>
    29f7:	85 c0                	test   %eax,%eax
    29f9:	74 40                	je     2a3b <gul_var_add+0x99>
    29fb:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    29ff:	8b 40 08             	mov    0x8(%rax),%eax
    2a02:	85 c0                	test   %eax,%eax
    2a04:	78 35                	js     2a3b <gul_var_add+0x99>
    2a06:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2a0a:	8b 40 08             	mov    0x8(%rax),%eax
    2a0d:	85 c0                	test   %eax,%eax
    2a0f:	78 2a                	js     2a3b <gul_var_add+0x99>
    2a11:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2a15:	8b 50 08             	mov    0x8(%rax),%edx
    2a18:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    2a1c:	8b 48 08             	mov    0x8(%rax),%ecx
    2a1f:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2a23:	89 ce                	mov    %ecx,%esi
    2a25:	66 48 0f 6e c0       	movq   %rax,%xmm0
    2a2a:	bf 01 00 00 00       	mov    $0x1,%edi
    2a2f:	e8 3a fd ff ff       	call   276e <tape_add_node>
    2a34:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    2a38:	89 42 08             	mov    %eax,0x8(%rdx)
    2a3b:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2a3f:	c9                   	leave
    2a40:	c3                   	ret

0000000000002a41 <gul_var_mul>:
    2a41:	f3 0f 1e fa          	endbr64
    2a45:	55                   	push   %rbp
    2a46:	48 89 e5             	mov    %rsp,%rbp
    2a49:	48 83 ec 30          	sub    $0x30,%rsp
    2a4d:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    2a51:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    2a55:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    2a59:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    2a5d:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    2a61:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    2a65:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    2a69:	f2 0f 10 08          	movsd  (%rax),%xmm1
    2a6d:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2a71:	f2 0f 10 00          	movsd  (%rax),%xmm0
    2a75:	f2 0f 59 c1          	mulsd  %xmm1,%xmm0
    2a79:	f2 0f 11 45 f0       	movsd  %xmm0,-0x10(%rbp)
    2a7e:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2a82:	66 48 0f 6e c0       	movq   %rax,%xmm0
    2a87:	e8 e6 fd ff ff       	call   2872 <gul_make_var>
    2a8c:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2a90:	8b 05 6e 80 06 00    	mov    0x6806e(%rip),%eax        # 6ab04 <global_tape+0x61a84>
    2a96:	85 c0                	test   %eax,%eax
    2a98:	74 40                	je     2ada <gul_var_mul+0x99>
    2a9a:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    2a9e:	8b 40 08             	mov    0x8(%rax),%eax
    2aa1:	85 c0                	test   %eax,%eax
    2aa3:	78 35                	js     2ada <gul_var_mul+0x99>
    2aa5:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2aa9:	8b 40 08             	mov    0x8(%rax),%eax
    2aac:	85 c0                	test   %eax,%eax
    2aae:	78 2a                	js     2ada <gul_var_mul+0x99>
    2ab0:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2ab4:	8b 50 08             	mov    0x8(%rax),%edx
    2ab7:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    2abb:	8b 48 08             	mov    0x8(%rax),%ecx
    2abe:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2ac2:	89 ce                	mov    %ecx,%esi
    2ac4:	66 48 0f 6e c0       	movq   %rax,%xmm0
    2ac9:	bf 03 00 00 00       	mov    $0x3,%edi
    2ace:	e8 9b fc ff ff       	call   276e <tape_add_node>
    2ad3:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    2ad7:	89 42 08             	mov    %eax,0x8(%rdx)
    2ada:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2ade:	c9                   	leave
    2adf:	c3                   	ret

0000000000002ae0 <gul_var_sin>:
    2ae0:	f3 0f 1e fa          	endbr64
    2ae4:	55                   	push   %rbp
    2ae5:	48 89 e5             	mov    %rsp,%rbp
    2ae8:	48 83 ec 30          	sub    $0x30,%rsp
    2aec:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    2af0:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    2af4:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    2af8:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2afc:	48 8b 00             	mov    (%rax),%rax
    2aff:	66 48 0f 6e c0       	movq   %rax,%xmm0
    2b04:	e8 27 f7 ff ff       	call   2230 <sin@plt>
    2b09:	66 48 0f 7e c0       	movq   %xmm0,%rax
    2b0e:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    2b12:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2b16:	66 48 0f 6e c0       	movq   %rax,%xmm0
    2b1b:	e8 52 fd ff ff       	call   2872 <gul_make_var>
    2b20:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2b24:	8b 05 da 7f 06 00    	mov    0x67fda(%rip),%eax        # 6ab04 <global_tape+0x61a84>
    2b2a:	85 c0                	test   %eax,%eax
    2b2c:	74 33                	je     2b61 <gul_var_sin+0x81>
    2b2e:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2b32:	8b 40 08             	mov    0x8(%rax),%eax
    2b35:	85 c0                	test   %eax,%eax
    2b37:	78 28                	js     2b61 <gul_var_sin+0x81>
    2b39:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2b3d:	8b 48 08             	mov    0x8(%rax),%ecx
    2b40:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2b44:	ba ff ff ff ff       	mov    $0xffffffff,%edx
    2b49:	89 ce                	mov    %ecx,%esi
    2b4b:	66 48 0f 6e c0       	movq   %rax,%xmm0
    2b50:	bf 05 00 00 00       	mov    $0x5,%edi
    2b55:	e8 14 fc ff ff       	call   276e <tape_add_node>
    2b5a:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    2b5e:	89 42 08             	mov    %eax,0x8(%rdx)
    2b61:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2b65:	c9                   	leave
    2b66:	c3                   	ret

0000000000002b67 <gul_backward>:
    2b67:	f3 0f 1e fa          	endbr64
    2b6b:	55                   	push   %rbp
    2b6c:	48 89 e5             	mov    %rsp,%rbp
    2b6f:	48 83 ec 20          	sub    $0x20,%rsp
    2b73:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    2b77:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    2b7b:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2b7f:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    2b84:	74 19                	je     2b9f <gul_backward+0x38>
    2b86:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2b8a:	8b 40 08             	mov    0x8(%rax),%eax
    2b8d:	85 c0                	test   %eax,%eax
    2b8f:	78 0e                	js     2b9f <gul_backward+0x38>
    2b91:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    2b95:	8b 40 08             	mov    0x8(%rax),%eax
    2b98:	89 c7                	mov    %eax,%edi
    2b9a:	e8 03 00 00 00       	call   2ba2 <gul_run_backward>
    2b9f:	90                   	nop
    2ba0:	c9                   	leave
    2ba1:	c3                   	ret

0000000000002ba2 <gul_run_backward>:
    2ba2:	f3 0f 1e fa          	endbr64
    2ba6:	55                   	push   %rbp
    2ba7:	48 89 e5             	mov    %rsp,%rbp
    2baa:	48 83 ec 30          	sub    $0x30,%rsp
    2bae:	89 7d dc             	mov    %edi,-0x24(%rbp)
    2bb1:	83 7d dc 00          	cmpl   $0x0,-0x24(%rbp)
    2bb5:	0f 88 45 03 00 00    	js     2f00 <gul_run_backward+0x35e>
    2bbb:	8b 05 3f 7f 06 00    	mov    0x67f3f(%rip),%eax        # 6ab00 <global_tape+0x61a80>
    2bc1:	39 45 dc             	cmp    %eax,-0x24(%rbp)
    2bc4:	0f 8d 36 03 00 00    	jge    2f00 <gul_run_backward+0x35e>
    2bca:	8b 45 dc             	mov    -0x24(%rbp),%eax
    2bcd:	48 63 d0             	movslq %eax,%rdx
    2bd0:	48 89 d0             	mov    %rdx,%rax
    2bd3:	48 c1 e0 02          	shl    $0x2,%rax
    2bd7:	48 01 d0             	add    %rdx,%rax
    2bda:	48 c1 e0 03          	shl    $0x3,%rax
    2bde:	48 89 c2             	mov    %rax,%rdx
    2be1:	48 8d 05 a8 64 00 00 	lea    0x64a8(%rip),%rax        # 9090 <global_tape+0x10>
    2be8:	f2 0f 10 05 80 36 00 	movsd  0x3680(%rip),%xmm0        # 6270 <_IO_stdin_used+0x270>
    2bef:	00 
    2bf0:	f2 0f 11 04 02       	movsd  %xmm0,(%rdx,%rax,1)
    2bf5:	8b 45 dc             	mov    -0x24(%rbp),%eax
    2bf8:	89 45 e4             	mov    %eax,-0x1c(%rbp)
    2bfb:	e9 f4 02 00 00       	jmp    2ef4 <gul_run_backward+0x352>
    2c00:	8b 45 e4             	mov    -0x1c(%rbp),%eax
    2c03:	48 63 d0             	movslq %eax,%rdx
    2c06:	48 89 d0             	mov    %rdx,%rax
    2c09:	48 c1 e0 02          	shl    $0x2,%rax
    2c0d:	48 01 d0             	add    %rdx,%rax
    2c10:	48 c1 e0 03          	shl    $0x3,%rax
    2c14:	48 8d 15 65 64 00 00 	lea    0x6465(%rip),%rdx        # 9080 <global_tape>
    2c1b:	48 01 d0             	add    %rdx,%rax
    2c1e:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    2c22:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2c26:	f2 0f 10 40 10       	movsd  0x10(%rax),%xmm0
    2c2b:	66 0f ef c9          	pxor   %xmm1,%xmm1
    2c2f:	66 0f 2e c1          	ucomisd %xmm1,%xmm0
    2c33:	7a 0e                	jp     2c43 <gul_run_backward+0xa1>
    2c35:	66 0f ef c9          	pxor   %xmm1,%xmm1
    2c39:	66 0f 2e c1          	ucomisd %xmm1,%xmm0
    2c3d:	0f 84 a0 02 00 00    	je     2ee3 <gul_run_backward+0x341>
    2c43:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2c47:	f2 0f 10 40 10       	movsd  0x10(%rax),%xmm0
    2c4c:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    2c51:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2c55:	8b 40 18             	mov    0x18(%rax),%eax
    2c58:	89 45 e8             	mov    %eax,-0x18(%rbp)
    2c5b:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2c5f:	8b 40 1c             	mov    0x1c(%rax),%eax
    2c62:	89 45 ec             	mov    %eax,-0x14(%rbp)
    2c65:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2c69:	8b 00                	mov    (%rax),%eax
    2c6b:	83 f8 05             	cmp    $0x5,%eax
    2c6e:	0f 84 d8 01 00 00    	je     2e4c <gul_run_backward+0x2aa>
    2c74:	83 f8 05             	cmp    $0x5,%eax
    2c77:	0f 87 69 02 00 00    	ja     2ee6 <gul_run_backward+0x344>
    2c7d:	83 f8 01             	cmp    $0x1,%eax
    2c80:	74 0e                	je     2c90 <gul_run_backward+0xee>
    2c82:	83 f8 03             	cmp    $0x3,%eax
    2c85:	0f 84 b0 00 00 00    	je     2d3b <gul_run_backward+0x199>
    2c8b:	e9 56 02 00 00       	jmp    2ee6 <gul_run_backward+0x344>
    2c90:	83 7d e8 00          	cmpl   $0x0,-0x18(%rbp)
    2c94:	78 4b                	js     2ce1 <gul_run_backward+0x13f>
    2c96:	8b 45 e8             	mov    -0x18(%rbp),%eax
    2c99:	48 63 d0             	movslq %eax,%rdx
    2c9c:	48 89 d0             	mov    %rdx,%rax
    2c9f:	48 c1 e0 02          	shl    $0x2,%rax
    2ca3:	48 01 d0             	add    %rdx,%rax
    2ca6:	48 c1 e0 03          	shl    $0x3,%rax
    2caa:	48 89 c2             	mov    %rax,%rdx
    2cad:	48 8d 05 dc 63 00 00 	lea    0x63dc(%rip),%rax        # 9090 <global_tape+0x10>
    2cb4:	f2 0f 10 04 02       	movsd  (%rdx,%rax,1),%xmm0
    2cb9:	f2 0f 58 45 f8       	addsd  -0x8(%rbp),%xmm0
    2cbe:	8b 45 e8             	mov    -0x18(%rbp),%eax
    2cc1:	48 63 d0             	movslq %eax,%rdx
    2cc4:	48 89 d0             	mov    %rdx,%rax
    2cc7:	48 c1 e0 02          	shl    $0x2,%rax
    2ccb:	48 01 d0             	add    %rdx,%rax
    2cce:	48 c1 e0 03          	shl    $0x3,%rax
    2cd2:	48 89 c2             	mov    %rax,%rdx
    2cd5:	48 8d 05 b4 63 00 00 	lea    0x63b4(%rip),%rax        # 9090 <global_tape+0x10>
    2cdc:	f2 0f 11 04 02       	movsd  %xmm0,(%rdx,%rax,1)
    2ce1:	83 7d ec 00          	cmpl   $0x0,-0x14(%rbp)
    2ce5:	0f 88 fe 01 00 00    	js     2ee9 <gul_run_backward+0x347>
    2ceb:	8b 45 ec             	mov    -0x14(%rbp),%eax
    2cee:	48 63 d0             	movslq %eax,%rdx
    2cf1:	48 89 d0             	mov    %rdx,%rax
    2cf4:	48 c1 e0 02          	shl    $0x2,%rax
    2cf8:	48 01 d0             	add    %rdx,%rax
    2cfb:	48 c1 e0 03          	shl    $0x3,%rax
    2cff:	48 89 c2             	mov    %rax,%rdx
    2d02:	48 8d 05 87 63 00 00 	lea    0x6387(%rip),%rax        # 9090 <global_tape+0x10>
    2d09:	f2 0f 10 04 02       	movsd  (%rdx,%rax,1),%xmm0
    2d0e:	f2 0f 58 45 f8       	addsd  -0x8(%rbp),%xmm0
    2d13:	8b 45 ec             	mov    -0x14(%rbp),%eax
    2d16:	48 63 d0             	movslq %eax,%rdx
    2d19:	48 89 d0             	mov    %rdx,%rax
    2d1c:	48 c1 e0 02          	shl    $0x2,%rax
    2d20:	48 01 d0             	add    %rdx,%rax
    2d23:	48 c1 e0 03          	shl    $0x3,%rax
    2d27:	48 89 c2             	mov    %rax,%rdx
    2d2a:	48 8d 05 5f 63 00 00 	lea    0x635f(%rip),%rax        # 9090 <global_tape+0x10>
    2d31:	f2 0f 11 04 02       	movsd  %xmm0,(%rdx,%rax,1)
    2d36:	e9 ae 01 00 00       	jmp    2ee9 <gul_run_backward+0x347>
    2d3b:	83 7d e8 00          	cmpl   $0x0,-0x18(%rbp)
    2d3f:	78 7e                	js     2dbf <gul_run_backward+0x21d>
    2d41:	8b 45 e8             	mov    -0x18(%rbp),%eax
    2d44:	48 63 d0             	movslq %eax,%rdx
    2d47:	48 89 d0             	mov    %rdx,%rax
    2d4a:	48 c1 e0 02          	shl    $0x2,%rax
    2d4e:	48 01 d0             	add    %rdx,%rax
    2d51:	48 c1 e0 03          	shl    $0x3,%rax
    2d55:	48 89 c2             	mov    %rax,%rdx
    2d58:	48 8d 05 31 63 00 00 	lea    0x6331(%rip),%rax        # 9090 <global_tape+0x10>
    2d5f:	f2 0f 10 0c 02       	movsd  (%rdx,%rax,1),%xmm1
    2d64:	83 7d ec 00          	cmpl   $0x0,-0x14(%rbp)
    2d68:	78 25                	js     2d8f <gul_run_backward+0x1ed>
    2d6a:	8b 45 ec             	mov    -0x14(%rbp),%eax
    2d6d:	48 63 d0             	movslq %eax,%rdx
    2d70:	48 89 d0             	mov    %rdx,%rax
    2d73:	48 c1 e0 02          	shl    $0x2,%rax
    2d77:	48 01 d0             	add    %rdx,%rax
    2d7a:	48 c1 e0 03          	shl    $0x3,%rax
    2d7e:	48 89 c2             	mov    %rax,%rdx
    2d81:	48 8d 05 00 63 00 00 	lea    0x6300(%rip),%rax        # 9088 <global_tape+0x8>
    2d88:	f2 0f 10 04 02       	movsd  (%rdx,%rax,1),%xmm0
    2d8d:	eb 04                	jmp    2d93 <gul_run_backward+0x1f1>
    2d8f:	66 0f ef c0          	pxor   %xmm0,%xmm0
    2d93:	f2 0f 59 45 f8       	mulsd  -0x8(%rbp),%xmm0
    2d98:	f2 0f 58 c1          	addsd  %xmm1,%xmm0
    2d9c:	8b 45 e8             	mov    -0x18(%rbp),%eax
    2d9f:	48 63 d0             	movslq %eax,%rdx
    2da2:	48 89 d0             	mov    %rdx,%rax
    2da5:	48 c1 e0 02          	shl    $0x2,%rax
    2da9:	48 01 d0             	add    %rdx,%rax
    2dac:	48 c1 e0 03          	shl    $0x3,%rax
    2db0:	48 89 c2             	mov    %rax,%rdx
    2db3:	48 8d 05 d6 62 00 00 	lea    0x62d6(%rip),%rax        # 9090 <global_tape+0x10>
    2dba:	f2 0f 11 04 02       	movsd  %xmm0,(%rdx,%rax,1)
    2dbf:	83 7d ec 00          	cmpl   $0x0,-0x14(%rbp)
    2dc3:	0f 88 23 01 00 00    	js     2eec <gul_run_backward+0x34a>
    2dc9:	8b 45 ec             	mov    -0x14(%rbp),%eax
    2dcc:	48 63 d0             	movslq %eax,%rdx
    2dcf:	48 89 d0             	mov    %rdx,%rax
    2dd2:	48 c1 e0 02          	shl    $0x2,%rax
    2dd6:	48 01 d0             	add    %rdx,%rax
    2dd9:	48 c1 e0 03          	shl    $0x3,%rax
    2ddd:	48 89 c2             	mov    %rax,%rdx
    2de0:	48 8d 05 a9 62 00 00 	lea    0x62a9(%rip),%rax        # 9090 <global_tape+0x10>
    2de7:	f2 0f 10 0c 02       	movsd  (%rdx,%rax,1),%xmm1
    2dec:	83 7d e8 00          	cmpl   $0x0,-0x18(%rbp)
    2df0:	78 25                	js     2e17 <gul_run_backward+0x275>
    2df2:	8b 45 e8             	mov    -0x18(%rbp),%eax
    2df5:	48 63 d0             	movslq %eax,%rdx
    2df8:	48 89 d0             	mov    %rdx,%rax
    2dfb:	48 c1 e0 02          	shl    $0x2,%rax
    2dff:	48 01 d0             	add    %rdx,%rax
    2e02:	48 c1 e0 03          	shl    $0x3,%rax
    2e06:	48 89 c2             	mov    %rax,%rdx
    2e09:	48 8d 05 78 62 00 00 	lea    0x6278(%rip),%rax        # 9088 <global_tape+0x8>
    2e10:	f2 0f 10 04 02       	movsd  (%rdx,%rax,1),%xmm0
    2e15:	eb 04                	jmp    2e1b <gul_run_backward+0x279>
    2e17:	66 0f ef c0          	pxor   %xmm0,%xmm0
    2e1b:	f2 0f 59 45 f8       	mulsd  -0x8(%rbp),%xmm0
    2e20:	f2 0f 58 c1          	addsd  %xmm1,%xmm0
    2e24:	8b 45 ec             	mov    -0x14(%rbp),%eax
    2e27:	48 63 d0             	movslq %eax,%rdx
    2e2a:	48 89 d0             	mov    %rdx,%rax
    2e2d:	48 c1 e0 02          	shl    $0x2,%rax
    2e31:	48 01 d0             	add    %rdx,%rax
    2e34:	48 c1 e0 03          	shl    $0x3,%rax
    2e38:	48 89 c2             	mov    %rax,%rdx
    2e3b:	48 8d 05 4e 62 00 00 	lea    0x624e(%rip),%rax        # 9090 <global_tape+0x10>
    2e42:	f2 0f 11 04 02       	movsd  %xmm0,(%rdx,%rax,1)
    2e47:	e9 a0 00 00 00       	jmp    2eec <gul_run_backward+0x34a>
    2e4c:	83 7d e8 00          	cmpl   $0x0,-0x18(%rbp)
    2e50:	0f 88 99 00 00 00    	js     2eef <gul_run_backward+0x34d>
    2e56:	8b 45 e8             	mov    -0x18(%rbp),%eax
    2e59:	48 63 d0             	movslq %eax,%rdx
    2e5c:	48 89 d0             	mov    %rdx,%rax
    2e5f:	48 c1 e0 02          	shl    $0x2,%rax
    2e63:	48 01 d0             	add    %rdx,%rax
    2e66:	48 c1 e0 03          	shl    $0x3,%rax
    2e6a:	48 89 c2             	mov    %rax,%rdx
    2e6d:	48 8d 05 1c 62 00 00 	lea    0x621c(%rip),%rax        # 9090 <global_tape+0x10>
    2e74:	f2 0f 10 14 02       	movsd  (%rdx,%rax,1),%xmm2
    2e79:	f2 0f 11 55 d0       	movsd  %xmm2,-0x30(%rbp)
    2e7e:	8b 45 e8             	mov    -0x18(%rbp),%eax
    2e81:	48 63 d0             	movslq %eax,%rdx
    2e84:	48 89 d0             	mov    %rdx,%rax
    2e87:	48 c1 e0 02          	shl    $0x2,%rax
    2e8b:	48 01 d0             	add    %rdx,%rax
    2e8e:	48 c1 e0 03          	shl    $0x3,%rax
    2e92:	48 89 c2             	mov    %rax,%rdx
    2e95:	48 8d 05 ec 61 00 00 	lea    0x61ec(%rip),%rax        # 9088 <global_tape+0x8>
    2e9c:	48 8b 04 02          	mov    (%rdx,%rax,1),%rax
    2ea0:	66 48 0f 6e c0       	movq   %rax,%xmm0
    2ea5:	e8 96 f2 ff ff       	call   2140 <cos@plt>
    2eaa:	66 48 0f 7e c0       	movq   %xmm0,%rax
    2eaf:	66 48 0f 6e c0       	movq   %rax,%xmm0
    2eb4:	f2 0f 59 45 f8       	mulsd  -0x8(%rbp),%xmm0
    2eb9:	f2 0f 58 45 d0       	addsd  -0x30(%rbp),%xmm0
    2ebe:	8b 45 e8             	mov    -0x18(%rbp),%eax
    2ec1:	48 63 d0             	movslq %eax,%rdx
    2ec4:	48 89 d0             	mov    %rdx,%rax
    2ec7:	48 c1 e0 02          	shl    $0x2,%rax
    2ecb:	48 01 d0             	add    %rdx,%rax
    2ece:	48 c1 e0 03          	shl    $0x3,%rax
    2ed2:	48 89 c2             	mov    %rax,%rdx
    2ed5:	48 8d 05 b4 61 00 00 	lea    0x61b4(%rip),%rax        # 9090 <global_tape+0x10>
    2edc:	f2 0f 11 04 02       	movsd  %xmm0,(%rdx,%rax,1)
    2ee1:	eb 0c                	jmp    2eef <gul_run_backward+0x34d>
    2ee3:	90                   	nop
    2ee4:	eb 0a                	jmp    2ef0 <gul_run_backward+0x34e>
    2ee6:	90                   	nop
    2ee7:	eb 07                	jmp    2ef0 <gul_run_backward+0x34e>
    2ee9:	90                   	nop
    2eea:	eb 04                	jmp    2ef0 <gul_run_backward+0x34e>
    2eec:	90                   	nop
    2eed:	eb 01                	jmp    2ef0 <gul_run_backward+0x34e>
    2eef:	90                   	nop
    2ef0:	83 6d e4 01          	subl   $0x1,-0x1c(%rbp)
    2ef4:	83 7d e4 00          	cmpl   $0x0,-0x1c(%rbp)
    2ef8:	0f 89 02 fd ff ff    	jns    2c00 <gul_run_backward+0x5e>
    2efe:	eb 01                	jmp    2f01 <gul_run_backward+0x35f>
    2f00:	90                   	nop
    2f01:	c9                   	leave
    2f02:	c3                   	ret

0000000000002f03 <gul_input_int>:
    2f03:	f3 0f 1e fa          	endbr64
    2f07:	55                   	push   %rbp
    2f08:	48 89 e5             	mov    %rsp,%rbp
    2f0b:	48 83 ec 20          	sub    $0x20,%rsp
    2f0f:	64 48 8b 04 25 28 00 	mov    %fs:0x28,%rax
    2f16:	00 00 
    2f18:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2f1c:	31 c0                	xor    %eax,%eax
    2f1e:	48 c7 45 f0 00 00 00 	movq   $0x0,-0x10(%rbp)
    2f25:	00 
    2f26:	48 8d 45 f0          	lea    -0x10(%rbp),%rax
    2f2a:	48 89 c6             	mov    %rax,%rsi
    2f2d:	48 8d 05 41 31 00 00 	lea    0x3141(%rip),%rax        # 6075 <_IO_stdin_used+0x75>
    2f34:	48 89 c7             	mov    %rax,%rdi
    2f37:	b8 00 00 00 00       	mov    $0x0,%eax
    2f3c:	e8 0f f3 ff ff       	call   2250 <__isoc99_scanf@plt>
    2f41:	83 f8 01             	cmp    $0x1,%eax
    2f44:	74 2b                	je     2f71 <gul_input_int+0x6e>
    2f46:	48 8b 05 13 61 00 00 	mov    0x6113(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    2f4d:	48 89 c1             	mov    %rax,%rcx
    2f50:	ba 24 00 00 00       	mov    $0x24,%edx
    2f55:	be 01 00 00 00       	mov    $0x1,%esi
    2f5a:	48 8d 05 df 31 00 00 	lea    0x31df(%rip),%rax        # 6140 <_IO_stdin_used+0x140>
    2f61:	48 89 c7             	mov    %rax,%rdi
    2f64:	e8 47 f3 ff ff       	call   22b0 <fwrite@plt>
    2f69:	48 c7 45 f0 00 00 00 	movq   $0x0,-0x10(%rbp)
    2f70:	00 
    2f71:	90                   	nop
    2f72:	e8 19 f2 ff ff       	call   2190 <getchar@plt>
    2f77:	89 45 ec             	mov    %eax,-0x14(%rbp)
    2f7a:	83 7d ec 0a          	cmpl   $0xa,-0x14(%rbp)
    2f7e:	74 06                	je     2f86 <gul_input_int+0x83>
    2f80:	83 7d ec ff          	cmpl   $0xffffffff,-0x14(%rbp)
    2f84:	75 ec                	jne    2f72 <gul_input_int+0x6f>
    2f86:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    2f8a:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    2f8e:	64 48 2b 14 25 28 00 	sub    %fs:0x28,%rdx
    2f95:	00 00 
    2f97:	74 05                	je     2f9e <gul_input_int+0x9b>
    2f99:	e8 42 f1 ff ff       	call   20e0 <__stack_chk_fail@plt>
    2f9e:	c9                   	leave
    2f9f:	c3                   	ret

0000000000002fa0 <gul_input_flt>:
    2fa0:	f3 0f 1e fa          	endbr64
    2fa4:	55                   	push   %rbp
    2fa5:	48 89 e5             	mov    %rsp,%rbp
    2fa8:	48 83 ec 20          	sub    $0x20,%rsp
    2fac:	64 48 8b 04 25 28 00 	mov    %fs:0x28,%rax
    2fb3:	00 00 
    2fb5:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    2fb9:	31 c0                	xor    %eax,%eax
    2fbb:	66 0f ef c0          	pxor   %xmm0,%xmm0
    2fbf:	f2 0f 11 45 f0       	movsd  %xmm0,-0x10(%rbp)
    2fc4:	48 8d 45 f0          	lea    -0x10(%rbp),%rax
    2fc8:	48 89 c6             	mov    %rax,%rsi
    2fcb:	48 8d 05 93 31 00 00 	lea    0x3193(%rip),%rax        # 6165 <_IO_stdin_used+0x165>
    2fd2:	48 89 c7             	mov    %rax,%rdi
    2fd5:	b8 00 00 00 00       	mov    $0x0,%eax
    2fda:	e8 71 f2 ff ff       	call   2250 <__isoc99_scanf@plt>
    2fdf:	83 f8 01             	cmp    $0x1,%eax
    2fe2:	74 2c                	je     3010 <gul_input_flt+0x70>
    2fe4:	48 8b 05 75 60 00 00 	mov    0x6075(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    2feb:	48 89 c1             	mov    %rax,%rcx
    2fee:	ba 22 00 00 00       	mov    $0x22,%edx
    2ff3:	be 01 00 00 00       	mov    $0x1,%esi
    2ff8:	48 8d 05 71 31 00 00 	lea    0x3171(%rip),%rax        # 6170 <_IO_stdin_used+0x170>
    2fff:	48 89 c7             	mov    %rax,%rdi
    3002:	e8 a9 f2 ff ff       	call   22b0 <fwrite@plt>
    3007:	66 0f ef c0          	pxor   %xmm0,%xmm0
    300b:	f2 0f 11 45 f0       	movsd  %xmm0,-0x10(%rbp)
    3010:	90                   	nop
    3011:	e8 7a f1 ff ff       	call   2190 <getchar@plt>
    3016:	89 45 ec             	mov    %eax,-0x14(%rbp)
    3019:	83 7d ec 0a          	cmpl   $0xa,-0x14(%rbp)
    301d:	74 06                	je     3025 <gul_input_flt+0x85>
    301f:	83 7d ec ff          	cmpl   $0xffffffff,-0x14(%rbp)
    3023:	75 ec                	jne    3011 <gul_input_flt+0x71>
    3025:	f2 0f 10 45 f0       	movsd  -0x10(%rbp),%xmm0
    302a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    302e:	64 48 2b 04 25 28 00 	sub    %fs:0x28,%rax
    3035:	00 00 
    3037:	74 05                	je     303e <gul_input_flt+0x9e>
    3039:	e8 a2 f0 ff ff       	call   20e0 <__stack_chk_fail@plt>
    303e:	c9                   	leave
    303f:	c3                   	ret

0000000000003040 <gul_file_open>:
    3040:	f3 0f 1e fa          	endbr64
    3044:	55                   	push   %rbp
    3045:	48 89 e5             	mov    %rsp,%rbp
    3048:	48 83 ec 30          	sub    $0x30,%rsp
    304c:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    3050:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    3054:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    3058:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    305c:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    3060:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    3064:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    3068:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    306c:	48 89 d6             	mov    %rdx,%rsi
    306f:	48 89 c7             	mov    %rax,%rdi
    3072:	e8 a9 f1 ff ff       	call   2220 <fopen@plt>
    3077:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    307b:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    3080:	75 07                	jne    3089 <gul_file_open+0x49>
    3082:	b8 00 00 00 00       	mov    $0x0,%eax
    3087:	eb 04                	jmp    308d <gul_file_open+0x4d>
    3089:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    308d:	c9                   	leave
    308e:	c3                   	ret

000000000000308f <gul_file_close>:
    308f:	f3 0f 1e fa          	endbr64
    3093:	55                   	push   %rbp
    3094:	48 89 e5             	mov    %rsp,%rbp
    3097:	48 83 ec 10          	sub    $0x10,%rsp
    309b:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    309f:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    30a4:	74 0c                	je     30b2 <gul_file_close+0x23>
    30a6:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    30aa:	48 89 c7             	mov    %rax,%rdi
    30ad:	e8 0e f0 ff ff       	call   20c0 <fclose@plt>
    30b2:	90                   	nop
    30b3:	c9                   	leave
    30b4:	c3                   	ret

00000000000030b5 <gul_file_read_line>:
    30b5:	f3 0f 1e fa          	endbr64
    30b9:	55                   	push   %rbp
    30ba:	48 89 e5             	mov    %rsp,%rbp
    30bd:	48 83 ec 20          	sub    $0x20,%rsp
    30c1:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    30c5:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    30ca:	75 07                	jne    30d3 <gul_file_read_line+0x1e>
    30cc:	b8 00 00 00 00       	mov    $0x0,%eax
    30d1:	eb 7e                	jmp    3151 <gul_file_read_line+0x9c>
    30d3:	bf 00 10 00 00       	mov    $0x1000,%edi
    30d8:	e8 e3 f0 ff ff       	call   21c0 <malloc@plt>
    30dd:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    30e1:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    30e5:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    30e9:	be 00 10 00 00       	mov    $0x1000,%esi
    30ee:	48 89 c7             	mov    %rax,%rdi
    30f1:	e8 6a f0 ff ff       	call   2160 <fgets@plt>
    30f6:	48 85 c0             	test   %rax,%rax
    30f9:	74 45                	je     3140 <gul_file_read_line+0x8b>
    30fb:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    30ff:	48 89 c7             	mov    %rax,%rdi
    3102:	e8 c9 ef ff ff       	call   20d0 <strlen@plt>
    3107:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    310b:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    3110:	74 28                	je     313a <gul_file_read_line+0x85>
    3112:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3116:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
    311a:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    311e:	48 01 d0             	add    %rdx,%rax
    3121:	0f b6 00             	movzbl (%rax),%eax
    3124:	3c 0a                	cmp    $0xa,%al
    3126:	75 12                	jne    313a <gul_file_read_line+0x85>
    3128:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    312c:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
    3130:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    3134:	48 01 d0             	add    %rdx,%rax
    3137:	c6 00 00             	movb   $0x0,(%rax)
    313a:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    313e:	eb 11                	jmp    3151 <gul_file_read_line+0x9c>
    3140:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    3144:	48 89 c7             	mov    %rax,%rdi
    3147:	e8 e4 ee ff ff       	call   2030 <free@plt>
    314c:	b8 00 00 00 00       	mov    $0x0,%eax
    3151:	c9                   	leave
    3152:	c3                   	ret

0000000000003153 <gul_math_sin>:
    3153:	f3 0f 1e fa          	endbr64
    3157:	55                   	push   %rbp
    3158:	48 89 e5             	mov    %rsp,%rbp
    315b:	48 83 ec 10          	sub    $0x10,%rsp
    315f:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    3164:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3168:	66 48 0f 6e c0       	movq   %rax,%xmm0
    316d:	e8 be f0 ff ff       	call   2230 <sin@plt>
    3172:	c9                   	leave
    3173:	c3                   	ret

0000000000003174 <gul_math_cos>:
    3174:	f3 0f 1e fa          	endbr64
    3178:	55                   	push   %rbp
    3179:	48 89 e5             	mov    %rsp,%rbp
    317c:	48 83 ec 10          	sub    $0x10,%rsp
    3180:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    3185:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3189:	66 48 0f 6e c0       	movq   %rax,%xmm0
    318e:	e8 ad ef ff ff       	call   2140 <cos@plt>
    3193:	c9                   	leave
    3194:	c3                   	ret

0000000000003195 <gul_math_tan>:
    3195:	f3 0f 1e fa          	endbr64
    3199:	55                   	push   %rbp
    319a:	48 89 e5             	mov    %rsp,%rbp
    319d:	48 83 ec 10          	sub    $0x10,%rsp
    31a1:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    31a6:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    31aa:	66 48 0f 6e c0       	movq   %rax,%xmm0
    31af:	e8 1c f0 ff ff       	call   21d0 <tan@plt>
    31b4:	c9                   	leave
    31b5:	c3                   	ret

00000000000031b6 <gul_math_asin>:
    31b6:	f3 0f 1e fa          	endbr64
    31ba:	55                   	push   %rbp
    31bb:	48 89 e5             	mov    %rsp,%rbp
    31be:	48 83 ec 10          	sub    $0x10,%rsp
    31c2:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    31c7:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    31cb:	66 48 0f 6e c0       	movq   %rax,%xmm0
    31d0:	e8 ab f0 ff ff       	call   2280 <asin@plt>
    31d5:	c9                   	leave
    31d6:	c3                   	ret

00000000000031d7 <gul_math_acos>:
    31d7:	f3 0f 1e fa          	endbr64
    31db:	55                   	push   %rbp
    31dc:	48 89 e5             	mov    %rsp,%rbp
    31df:	48 83 ec 10          	sub    $0x10,%rsp
    31e3:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    31e8:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    31ec:	66 48 0f 6e c0       	movq   %rax,%xmm0
    31f1:	e8 5a ef ff ff       	call   2150 <acos@plt>
    31f6:	c9                   	leave
    31f7:	c3                   	ret

00000000000031f8 <gul_math_atan>:
    31f8:	f3 0f 1e fa          	endbr64
    31fc:	55                   	push   %rbp
    31fd:	48 89 e5             	mov    %rsp,%rbp
    3200:	48 83 ec 10          	sub    $0x10,%rsp
    3204:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    3209:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    320d:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3212:	e8 89 ee ff ff       	call   20a0 <atan@plt>
    3217:	c9                   	leave
    3218:	c3                   	ret

0000000000003219 <gul_math_atan2>:
    3219:	f3 0f 1e fa          	endbr64
    321d:	55                   	push   %rbp
    321e:	48 89 e5             	mov    %rsp,%rbp
    3221:	48 83 ec 10          	sub    $0x10,%rsp
    3225:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    322a:	f2 0f 11 4d f0       	movsd  %xmm1,-0x10(%rbp)
    322f:	f2 0f 10 45 f0       	movsd  -0x10(%rbp),%xmm0
    3234:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3238:	66 0f 28 c8          	movapd %xmm0,%xmm1
    323c:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3241:	e8 9a ef ff ff       	call   21e0 <atan2@plt>
    3246:	c9                   	leave
    3247:	c3                   	ret

0000000000003248 <gul_math_exp>:
    3248:	f3 0f 1e fa          	endbr64
    324c:	55                   	push   %rbp
    324d:	48 89 e5             	mov    %rsp,%rbp
    3250:	48 83 ec 10          	sub    $0x10,%rsp
    3254:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    3259:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    325d:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3262:	e8 89 f0 ff ff       	call   22f0 <exp@plt>
    3267:	c9                   	leave
    3268:	c3                   	ret

0000000000003269 <gul_math_log>:
    3269:	f3 0f 1e fa          	endbr64
    326d:	55                   	push   %rbp
    326e:	48 89 e5             	mov    %rsp,%rbp
    3271:	48 83 ec 10          	sub    $0x10,%rsp
    3275:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    327a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    327e:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3283:	e8 a8 ee ff ff       	call   2130 <log@plt>
    3288:	c9                   	leave
    3289:	c3                   	ret

000000000000328a <gul_math_log10>:
    328a:	f3 0f 1e fa          	endbr64
    328e:	55                   	push   %rbp
    328f:	48 89 e5             	mov    %rsp,%rbp
    3292:	48 83 ec 10          	sub    $0x10,%rsp
    3296:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    329b:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    329f:	66 48 0f 6e c0       	movq   %rax,%xmm0
    32a4:	e8 f7 ee ff ff       	call   21a0 <log10@plt>
    32a9:	c9                   	leave
    32aa:	c3                   	ret

00000000000032ab <gul_math_log2>:
    32ab:	f3 0f 1e fa          	endbr64
    32af:	55                   	push   %rbp
    32b0:	48 89 e5             	mov    %rsp,%rbp
    32b3:	48 83 ec 10          	sub    $0x10,%rsp
    32b7:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    32bc:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    32c0:	66 48 0f 6e c0       	movq   %rax,%xmm0
    32c5:	e8 76 ed ff ff       	call   2040 <log2@plt>
    32ca:	c9                   	leave
    32cb:	c3                   	ret

00000000000032cc <gul_math_pow>:
    32cc:	f3 0f 1e fa          	endbr64
    32d0:	55                   	push   %rbp
    32d1:	48 89 e5             	mov    %rsp,%rbp
    32d4:	48 83 ec 10          	sub    $0x10,%rsp
    32d8:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    32dd:	f2 0f 11 4d f0       	movsd  %xmm1,-0x10(%rbp)
    32e2:	f2 0f 10 45 f0       	movsd  -0x10(%rbp),%xmm0
    32e7:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    32eb:	66 0f 28 c8          	movapd %xmm0,%xmm1
    32ef:	66 48 0f 6e c0       	movq   %rax,%xmm0
    32f4:	e8 b7 ed ff ff       	call   20b0 <pow@plt>
    32f9:	c9                   	leave
    32fa:	c3                   	ret

00000000000032fb <gul_math_sqrt>:
    32fb:	f3 0f 1e fa          	endbr64
    32ff:	55                   	push   %rbp
    3300:	48 89 e5             	mov    %rsp,%rbp
    3303:	48 83 ec 10          	sub    $0x10,%rsp
    3307:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    330c:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3310:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3315:	e8 a6 ef ff ff       	call   22c0 <sqrt@plt>
    331a:	c9                   	leave
    331b:	c3                   	ret

000000000000331c <gul_math_cbrt>:
    331c:	f3 0f 1e fa          	endbr64
    3320:	55                   	push   %rbp
    3321:	48 89 e5             	mov    %rsp,%rbp
    3324:	48 83 ec 10          	sub    $0x10,%rsp
    3328:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    332d:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3331:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3336:	e8 d5 ee ff ff       	call   2210 <cbrt@plt>
    333b:	c9                   	leave
    333c:	c3                   	ret

000000000000333d <gul_math_floor>:
    333d:	f3 0f 1e fa          	endbr64
    3341:	55                   	push   %rbp
    3342:	48 89 e5             	mov    %rsp,%rbp
    3345:	48 83 ec 10          	sub    $0x10,%rsp
    3349:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    334e:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3352:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3357:	e8 e4 ee ff ff       	call   2240 <floor@plt>
    335c:	c9                   	leave
    335d:	c3                   	ret

000000000000335e <gul_math_ceil>:
    335e:	f3 0f 1e fa          	endbr64
    3362:	55                   	push   %rbp
    3363:	48 89 e5             	mov    %rsp,%rbp
    3366:	48 83 ec 10          	sub    $0x10,%rsp
    336a:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    336f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3373:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3378:	e8 63 ef ff ff       	call   22e0 <ceil@plt>
    337d:	c9                   	leave
    337e:	c3                   	ret

000000000000337f <gul_math_round>:
    337f:	f3 0f 1e fa          	endbr64
    3383:	55                   	push   %rbp
    3384:	48 89 e5             	mov    %rsp,%rbp
    3387:	48 83 ec 10          	sub    $0x10,%rsp
    338b:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    3390:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3394:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3399:	e8 b2 ec ff ff       	call   2050 <round@plt>
    339e:	c9                   	leave
    339f:	c3                   	ret

00000000000033a0 <gul_math_trunc>:
    33a0:	f3 0f 1e fa          	endbr64
    33a4:	55                   	push   %rbp
    33a5:	48 89 e5             	mov    %rsp,%rbp
    33a8:	48 83 ec 10          	sub    $0x10,%rsp
    33ac:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    33b1:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    33b5:	66 48 0f 6e c0       	movq   %rax,%xmm0
    33ba:	e8 c1 ec ff ff       	call   2080 <trunc@plt>
    33bf:	c9                   	leave
    33c0:	c3                   	ret

00000000000033c1 <gul_math_abs>:
    33c1:	f3 0f 1e fa          	endbr64
    33c5:	55                   	push   %rbp
    33c6:	48 89 e5             	mov    %rsp,%rbp
    33c9:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    33ce:	f2 0f 10 45 f8       	movsd  -0x8(%rbp),%xmm0
    33d3:	f3 0f 7e 0d a5 2e 00 	movq   0x2ea5(%rip),%xmm1        # 6280 <_IO_stdin_used+0x280>
    33da:	00 
    33db:	66 0f 54 c1          	andpd  %xmm1,%xmm0
    33df:	5d                   	pop    %rbp
    33e0:	c3                   	ret

00000000000033e1 <gul_math_abs_int>:
    33e1:	f3 0f 1e fa          	endbr64
    33e5:	55                   	push   %rbp
    33e6:	48 89 e5             	mov    %rsp,%rbp
    33e9:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    33ed:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    33f1:	48 89 c2             	mov    %rax,%rdx
    33f4:	48 f7 da             	neg    %rdx
    33f7:	48 0f 49 c2          	cmovns %rdx,%rax
    33fb:	5d                   	pop    %rbp
    33fc:	c3                   	ret

00000000000033fd <gul_math_min>:
    33fd:	f3 0f 1e fa          	endbr64
    3401:	55                   	push   %rbp
    3402:	48 89 e5             	mov    %rsp,%rbp
    3405:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    340a:	f2 0f 11 4d f0       	movsd  %xmm1,-0x10(%rbp)
    340f:	f2 0f 10 45 f0       	movsd  -0x10(%rbp),%xmm0
    3414:	66 0f 2f 45 f8       	comisd -0x8(%rbp),%xmm0
    3419:	76 07                	jbe    3422 <gul_math_min+0x25>
    341b:	f2 0f 10 45 f8       	movsd  -0x8(%rbp),%xmm0
    3420:	eb 05                	jmp    3427 <gul_math_min+0x2a>
    3422:	f2 0f 10 45 f0       	movsd  -0x10(%rbp),%xmm0
    3427:	5d                   	pop    %rbp
    3428:	c3                   	ret

0000000000003429 <gul_math_max>:
    3429:	f3 0f 1e fa          	endbr64
    342d:	55                   	push   %rbp
    342e:	48 89 e5             	mov    %rsp,%rbp
    3431:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    3436:	f2 0f 11 4d f0       	movsd  %xmm1,-0x10(%rbp)
    343b:	f2 0f 10 45 f8       	movsd  -0x8(%rbp),%xmm0
    3440:	66 0f 2f 45 f0       	comisd -0x10(%rbp),%xmm0
    3445:	76 07                	jbe    344e <gul_math_max+0x25>
    3447:	f2 0f 10 45 f8       	movsd  -0x8(%rbp),%xmm0
    344c:	eb 05                	jmp    3453 <gul_math_max+0x2a>
    344e:	f2 0f 10 45 f0       	movsd  -0x10(%rbp),%xmm0
    3453:	5d                   	pop    %rbp
    3454:	c3                   	ret

0000000000003455 <gul_ml_sigmoid>:
    3455:	f3 0f 1e fa          	endbr64
    3459:	55                   	push   %rbp
    345a:	48 89 e5             	mov    %rsp,%rbp
    345d:	48 83 ec 10          	sub    $0x10,%rsp
    3461:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    3466:	f2 0f 10 45 f8       	movsd  -0x8(%rbp),%xmm0
    346b:	f3 0f 7e 0d 1d 2e 00 	movq   0x2e1d(%rip),%xmm1        # 6290 <_IO_stdin_used+0x290>
    3472:	00 
    3473:	66 0f 57 c1          	xorpd  %xmm1,%xmm0
    3477:	66 48 0f 7e c0       	movq   %xmm0,%rax
    347c:	66 48 0f 6e c0       	movq   %rax,%xmm0
    3481:	e8 6a ee ff ff       	call   22f0 <exp@plt>
    3486:	f2 0f 10 0d e2 2d 00 	movsd  0x2de2(%rip),%xmm1        # 6270 <_IO_stdin_used+0x270>
    348d:	00 
    348e:	f2 0f 58 c8          	addsd  %xmm0,%xmm1
    3492:	f2 0f 10 05 d6 2d 00 	movsd  0x2dd6(%rip),%xmm0        # 6270 <_IO_stdin_used+0x270>
    3499:	00 
    349a:	f2 0f 5e c1          	divsd  %xmm1,%xmm0
    349e:	c9                   	leave
    349f:	c3                   	ret

00000000000034a0 <gul_ml_tanh>:
    34a0:	f3 0f 1e fa          	endbr64
    34a4:	55                   	push   %rbp
    34a5:	48 89 e5             	mov    %rsp,%rbp
    34a8:	48 83 ec 10          	sub    $0x10,%rsp
    34ac:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    34b1:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    34b5:	66 48 0f 6e c0       	movq   %rax,%xmm0
    34ba:	e8 b1 ed ff ff       	call   2270 <tanh@plt>
    34bf:	c9                   	leave
    34c0:	c3                   	ret

00000000000034c1 <gul_ml_relu>:
    34c1:	f3 0f 1e fa          	endbr64
    34c5:	55                   	push   %rbp
    34c6:	48 89 e5             	mov    %rsp,%rbp
    34c9:	f2 0f 11 45 f8       	movsd  %xmm0,-0x8(%rbp)
    34ce:	f2 0f 10 45 f8       	movsd  -0x8(%rbp),%xmm0
    34d3:	66 0f ef c9          	pxor   %xmm1,%xmm1
    34d7:	66 0f 2f c1          	comisd %xmm1,%xmm0
    34db:	76 07                	jbe    34e4 <gul_ml_relu+0x23>
    34dd:	f2 0f 10 45 f8       	movsd  -0x8(%rbp),%xmm0
    34e2:	eb 04                	jmp    34e8 <gul_ml_relu+0x27>
    34e4:	66 0f ef c0          	pxor   %xmm0,%xmm0
    34e8:	5d                   	pop    %rbp
    34e9:	c3                   	ret

00000000000034ea <gul_tensor_alloc>:
    34ea:	f3 0f 1e fa          	endbr64
    34ee:	55                   	push   %rbp
    34ef:	48 89 e5             	mov    %rsp,%rbp
    34f2:	48 83 ec 20          	sub    $0x20,%rsp
    34f6:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    34fa:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    34fe:	48 c1 e0 03          	shl    $0x3,%rax
    3502:	48 89 c7             	mov    %rax,%rdi
    3505:	e8 b6 ec ff ff       	call   21c0 <malloc@plt>
    350a:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    350e:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    3513:	75 2d                	jne    3542 <gul_tensor_alloc+0x58>
    3515:	48 8b 05 44 5b 00 00 	mov    0x5b44(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    351c:	48 89 c1             	mov    %rax,%rcx
    351f:	ba 2d 00 00 00       	mov    $0x2d,%edx
    3524:	be 01 00 00 00       	mov    $0x1,%esi
    3529:	48 8d 05 68 2c 00 00 	lea    0x2c68(%rip),%rax        # 6198 <_IO_stdin_used+0x198>
    3530:	48 89 c7             	mov    %rax,%rdi
    3533:	e8 78 ed ff ff       	call   22b0 <fwrite@plt>
    3538:	bf 01 00 00 00       	mov    $0x1,%edi
    353d:	e8 5e ed ff ff       	call   22a0 <exit@plt>
    3542:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3546:	c9                   	leave
    3547:	c3                   	ret

0000000000003548 <gul_tensor_free>:
    3548:	f3 0f 1e fa          	endbr64
    354c:	55                   	push   %rbp
    354d:	48 89 e5             	mov    %rsp,%rbp
    3550:	48 83 ec 10          	sub    $0x10,%rsp
    3554:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    3558:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    355c:	48 89 c7             	mov    %rax,%rdi
    355f:	e8 cc ea ff ff       	call   2030 <free@plt>
    3564:	90                   	nop
    3565:	c9                   	leave
    3566:	c3                   	ret

0000000000003567 <gul_tensor_fill>:
    3567:	f3 0f 1e fa          	endbr64
    356b:	55                   	push   %rbp
    356c:	48 89 e5             	mov    %rsp,%rbp
    356f:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    3573:	48 89 75 e0          	mov    %rsi,-0x20(%rbp)
    3577:	f2 0f 11 45 d8       	movsd  %xmm0,-0x28(%rbp)
    357c:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    3580:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    3584:	48 c7 45 f0 00 00 00 	movq   $0x0,-0x10(%rbp)
    358b:	00 
    358c:	eb 21                	jmp    35af <gul_tensor_fill+0x48>
    358e:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    3592:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    3599:	00 
    359a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    359e:	48 01 d0             	add    %rdx,%rax
    35a1:	f2 0f 10 45 d8       	movsd  -0x28(%rbp),%xmm0
    35a6:	f2 0f 11 00          	movsd  %xmm0,(%rax)
    35aa:	48 83 45 f0 01       	addq   $0x1,-0x10(%rbp)
    35af:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    35b3:	48 3b 45 e0          	cmp    -0x20(%rbp),%rax
    35b7:	7c d5                	jl     358e <gul_tensor_fill+0x27>
    35b9:	90                   	nop
    35ba:	90                   	nop
    35bb:	5d                   	pop    %rbp
    35bc:	c3                   	ret

00000000000035bd <gul_tensor_add>:
    35bd:	f3 0f 1e fa          	endbr64
    35c1:	55                   	push   %rbp
    35c2:	48 89 e5             	mov    %rsp,%rbp
    35c5:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    35c9:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    35cd:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
    35d1:	48 89 4d c0          	mov    %rcx,-0x40(%rbp)
    35d5:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    35d9:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    35dd:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    35e1:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    35e5:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    35e9:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    35ed:	48 c7 45 e0 00 00 00 	movq   $0x0,-0x20(%rbp)
    35f4:	00 
    35f5:	eb 4e                	jmp    3645 <gul_tensor_add+0x88>
    35f7:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    35fb:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    3602:	00 
    3603:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    3607:	48 01 d0             	add    %rdx,%rax
    360a:	f2 0f 10 08          	movsd  (%rax),%xmm1
    360e:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    3612:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    3619:	00 
    361a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    361e:	48 01 d0             	add    %rdx,%rax
    3621:	f2 0f 10 00          	movsd  (%rax),%xmm0
    3625:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    3629:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    3630:	00 
    3631:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    3635:	48 01 d0             	add    %rdx,%rax
    3638:	f2 0f 58 c1          	addsd  %xmm1,%xmm0
    363c:	f2 0f 11 00          	movsd  %xmm0,(%rax)
    3640:	48 83 45 e0 01       	addq   $0x1,-0x20(%rbp)
    3645:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    3649:	48 3b 45 c0          	cmp    -0x40(%rbp),%rax
    364d:	7c a8                	jl     35f7 <gul_tensor_add+0x3a>
    364f:	90                   	nop
    3650:	90                   	nop
    3651:	5d                   	pop    %rbp
    3652:	c3                   	ret

0000000000003653 <gul_tensor_mul>:
    3653:	f3 0f 1e fa          	endbr64
    3657:	55                   	push   %rbp
    3658:	48 89 e5             	mov    %rsp,%rbp
    365b:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    365f:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    3663:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
    3667:	48 89 4d c0          	mov    %rcx,-0x40(%rbp)
    366b:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    366f:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    3673:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    3677:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    367b:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    367f:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    3683:	48 c7 45 e0 00 00 00 	movq   $0x0,-0x20(%rbp)
    368a:	00 
    368b:	eb 4e                	jmp    36db <gul_tensor_mul+0x88>
    368d:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    3691:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    3698:	00 
    3699:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    369d:	48 01 d0             	add    %rdx,%rax
    36a0:	f2 0f 10 08          	movsd  (%rax),%xmm1
    36a4:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    36a8:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    36af:	00 
    36b0:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    36b4:	48 01 d0             	add    %rdx,%rax
    36b7:	f2 0f 10 00          	movsd  (%rax),%xmm0
    36bb:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    36bf:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    36c6:	00 
    36c7:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    36cb:	48 01 d0             	add    %rdx,%rax
    36ce:	f2 0f 59 c1          	mulsd  %xmm1,%xmm0
    36d2:	f2 0f 11 00          	movsd  %xmm0,(%rax)
    36d6:	48 83 45 e0 01       	addq   $0x1,-0x20(%rbp)
    36db:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    36df:	48 3b 45 c0          	cmp    -0x40(%rbp),%rax
    36e3:	7c a8                	jl     368d <gul_tensor_mul+0x3a>
    36e5:	90                   	nop
    36e6:	90                   	nop
    36e7:	5d                   	pop    %rbp
    36e8:	c3                   	ret

00000000000036e9 <gul_tensor_matmul>:
    36e9:	f3 0f 1e fa          	endbr64
    36ed:	55                   	push   %rbp
    36ee:	48 89 e5             	mov    %rsp,%rbp
    36f1:	48 89 7d b8          	mov    %rdi,-0x48(%rbp)
    36f5:	48 89 75 b0          	mov    %rsi,-0x50(%rbp)
    36f9:	48 89 55 a8          	mov    %rdx,-0x58(%rbp)
    36fd:	48 89 4d a0          	mov    %rcx,-0x60(%rbp)
    3701:	4c 89 45 98          	mov    %r8,-0x68(%rbp)
    3705:	4c 89 4d 90          	mov    %r9,-0x70(%rbp)
    3709:	48 8b 45 b8          	mov    -0x48(%rbp),%rax
    370d:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    3711:	48 8b 45 b0          	mov    -0x50(%rbp),%rax
    3715:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    3719:	48 8b 45 a8          	mov    -0x58(%rbp),%rax
    371d:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    3721:	48 c7 45 c8 00 00 00 	movq   $0x0,-0x38(%rbp)
    3728:	00 
    3729:	e9 d0 00 00 00       	jmp    37fe <gul_tensor_matmul+0x115>
    372e:	48 c7 45 d0 00 00 00 	movq   $0x0,-0x30(%rbp)
    3735:	00 
    3736:	e9 b0 00 00 00       	jmp    37eb <gul_tensor_matmul+0x102>
    373b:	66 0f ef c0          	pxor   %xmm0,%xmm0
    373f:	f2 0f 11 45 d8       	movsd  %xmm0,-0x28(%rbp)
    3744:	48 c7 45 e0 00 00 00 	movq   $0x0,-0x20(%rbp)
    374b:	00 
    374c:	eb 63                	jmp    37b1 <gul_tensor_matmul+0xc8>
    374e:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    3752:	48 0f af 45 98       	imul   -0x68(%rbp),%rax
    3757:	48 89 c2             	mov    %rax,%rdx
    375a:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    375e:	48 01 d0             	add    %rdx,%rax
    3761:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    3768:	00 
    3769:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    376d:	48 01 d0             	add    %rdx,%rax
    3770:	f2 0f 10 08          	movsd  (%rax),%xmm1
    3774:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    3778:	48 0f af 45 90       	imul   -0x70(%rbp),%rax
    377d:	48 89 c2             	mov    %rax,%rdx
    3780:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    3784:	48 01 d0             	add    %rdx,%rax
    3787:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    378e:	00 
    378f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3793:	48 01 d0             	add    %rdx,%rax
    3796:	f2 0f 10 00          	movsd  (%rax),%xmm0
    379a:	f2 0f 59 c1          	mulsd  %xmm1,%xmm0
    379e:	f2 0f 10 4d d8       	movsd  -0x28(%rbp),%xmm1
    37a3:	f2 0f 58 c1          	addsd  %xmm1,%xmm0
    37a7:	f2 0f 11 45 d8       	movsd  %xmm0,-0x28(%rbp)
    37ac:	48 83 45 e0 01       	addq   $0x1,-0x20(%rbp)
    37b1:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    37b5:	48 3b 45 98          	cmp    -0x68(%rbp),%rax
    37b9:	7c 93                	jl     374e <gul_tensor_matmul+0x65>
    37bb:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    37bf:	48 0f af 45 90       	imul   -0x70(%rbp),%rax
    37c4:	48 89 c2             	mov    %rax,%rdx
    37c7:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    37cb:	48 01 d0             	add    %rdx,%rax
    37ce:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    37d5:	00 
    37d6:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    37da:	48 01 d0             	add    %rdx,%rax
    37dd:	f2 0f 10 45 d8       	movsd  -0x28(%rbp),%xmm0
    37e2:	f2 0f 11 00          	movsd  %xmm0,(%rax)
    37e6:	48 83 45 d0 01       	addq   $0x1,-0x30(%rbp)
    37eb:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    37ef:	48 3b 45 90          	cmp    -0x70(%rbp),%rax
    37f3:	0f 8c 42 ff ff ff    	jl     373b <gul_tensor_matmul+0x52>
    37f9:	48 83 45 c8 01       	addq   $0x1,-0x38(%rbp)
    37fe:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    3802:	48 3b 45 a0          	cmp    -0x60(%rbp),%rax
    3806:	0f 8c 22 ff ff ff    	jl     372e <gul_tensor_matmul+0x45>
    380c:	90                   	nop
    380d:	90                   	nop
    380e:	5d                   	pop    %rbp
    380f:	c3                   	ret

0000000000003810 <gul_tensor_sum>:
    3810:	f3 0f 1e fa          	endbr64
    3814:	55                   	push   %rbp
    3815:	48 89 e5             	mov    %rsp,%rbp
    3818:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    381c:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    3820:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    3824:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    3828:	66 0f ef c0          	pxor   %xmm0,%xmm0
    382c:	f2 0f 11 45 e8       	movsd  %xmm0,-0x18(%rbp)
    3831:	48 c7 45 f0 00 00 00 	movq   $0x0,-0x10(%rbp)
    3838:	00 
    3839:	eb 2a                	jmp    3865 <gul_tensor_sum+0x55>
    383b:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    383f:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    3846:	00 
    3847:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    384b:	48 01 d0             	add    %rdx,%rax
    384e:	f2 0f 10 00          	movsd  (%rax),%xmm0
    3852:	f2 0f 10 4d e8       	movsd  -0x18(%rbp),%xmm1
    3857:	f2 0f 58 c1          	addsd  %xmm1,%xmm0
    385b:	f2 0f 11 45 e8       	movsd  %xmm0,-0x18(%rbp)
    3860:	48 83 45 f0 01       	addq   $0x1,-0x10(%rbp)
    3865:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    3869:	48 3b 45 d0          	cmp    -0x30(%rbp),%rax
    386d:	7c cc                	jl     383b <gul_tensor_sum+0x2b>
    386f:	f2 0f 10 45 e8       	movsd  -0x18(%rbp),%xmm0
    3874:	5d                   	pop    %rbp
    3875:	c3                   	ret

0000000000003876 <gul_tensor_mean>:
    3876:	f3 0f 1e fa          	endbr64
    387a:	55                   	push   %rbp
    387b:	48 89 e5             	mov    %rsp,%rbp
    387e:	48 83 ec 10          	sub    $0x10,%rsp
    3882:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    3886:	48 89 75 f0          	mov    %rsi,-0x10(%rbp)
    388a:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    388e:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    3892:	48 89 d6             	mov    %rdx,%rsi
    3895:	48 89 c7             	mov    %rax,%rdi
    3898:	e8 73 ff ff ff       	call   3810 <gul_tensor_sum>
    389d:	66 48 0f 7e c0       	movq   %xmm0,%rax
    38a2:	66 0f ef c0          	pxor   %xmm0,%xmm0
    38a6:	f2 48 0f 2a 45 f0    	cvtsi2sdq -0x10(%rbp),%xmm0
    38ac:	66 48 0f 6e c8       	movq   %rax,%xmm1
    38b1:	f2 0f 5e c8          	divsd  %xmm0,%xmm1
    38b5:	66 0f 28 c1          	movapd %xmm1,%xmm0
    38b9:	c9                   	leave
    38ba:	c3                   	ret

00000000000038bb <gul_simd_vec4f>:
    38bb:	f3 0f 1e fa          	endbr64
    38bf:	55                   	push   %rbp
    38c0:	48 89 e5             	mov    %rsp,%rbp
    38c3:	53                   	push   %rbx
    38c4:	f3 0f 11 45 ec       	movss  %xmm0,-0x14(%rbp)
    38c9:	f3 0f 11 4d e8       	movss  %xmm1,-0x18(%rbp)
    38ce:	f3 0f 11 55 e4       	movss  %xmm2,-0x1c(%rbp)
    38d3:	f3 0f 11 5d e0       	movss  %xmm3,-0x20(%rbp)
    38d8:	8b 4d ec             	mov    -0x14(%rbp),%ecx
    38db:	89 ce                	mov    %ecx,%esi
    38dd:	48 89 c7             	mov    %rax,%rdi
    38e0:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    38e7:	ff ff ff 
    38ea:	48 21 f9             	and    %rdi,%rcx
    38ed:	48 09 f1             	or     %rsi,%rcx
    38f0:	48 89 c8             	mov    %rcx,%rax
    38f3:	8b 4d e8             	mov    -0x18(%rbp),%ecx
    38f6:	89 c9                	mov    %ecx,%ecx
    38f8:	48 c1 e1 20          	shl    $0x20,%rcx
    38fc:	48 89 c6             	mov    %rax,%rsi
    38ff:	89 f6                	mov    %esi,%esi
    3901:	48 09 f1             	or     %rsi,%rcx
    3904:	48 89 c8             	mov    %rcx,%rax
    3907:	8b 4d e4             	mov    -0x1c(%rbp),%ecx
    390a:	89 ce                	mov    %ecx,%esi
    390c:	48 89 d7             	mov    %rdx,%rdi
    390f:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    3916:	ff ff ff 
    3919:	48 21 f9             	and    %rdi,%rcx
    391c:	48 09 f1             	or     %rsi,%rcx
    391f:	48 89 ca             	mov    %rcx,%rdx
    3922:	8b 4d e0             	mov    -0x20(%rbp),%ecx
    3925:	89 c9                	mov    %ecx,%ecx
    3927:	48 c1 e1 20          	shl    $0x20,%rcx
    392b:	48 89 d6             	mov    %rdx,%rsi
    392e:	89 f6                	mov    %esi,%esi
    3930:	48 09 f1             	or     %rsi,%rcx
    3933:	48 89 ca             	mov    %rcx,%rdx
    3936:	48 89 c1             	mov    %rax,%rcx
    3939:	48 89 d3             	mov    %rdx,%rbx
    393c:	66 48 0f 6e ca       	movq   %rdx,%xmm1
    3941:	66 48 0f 6e c1       	movq   %rcx,%xmm0
    3946:	48 8b 5d f8          	mov    -0x8(%rbp),%rbx
    394a:	c9                   	leave
    394b:	c3                   	ret

000000000000394c <gul_simd_add>:
    394c:	f3 0f 1e fa          	endbr64
    3950:	55                   	push   %rbp
    3951:	48 89 e5             	mov    %rsp,%rbp
    3954:	53                   	push   %rbx
    3955:	48 81 ec e8 00 00 00 	sub    $0xe8,%rsp
    395c:	66 48 0f 7e c1       	movq   %xmm0,%rcx
    3961:	66 0f 6f c1          	movdqa %xmm1,%xmm0
    3965:	66 48 0f 7e c3       	movq   %xmm0,%rbx
    396a:	48 89 8d 30 ff ff ff 	mov    %rcx,-0xd0(%rbp)
    3971:	48 89 9d 38 ff ff ff 	mov    %rbx,-0xc8(%rbp)
    3978:	66 0f 6f c2          	movdqa %xmm2,%xmm0
    397c:	66 0f 6f cb          	movdqa %xmm3,%xmm1
    3980:	0f 29 85 10 ff ff ff 	movaps %xmm0,-0xf0(%rbp)
    3987:	48 8b 8d 10 ff ff ff 	mov    -0xf0(%rbp),%rcx
    398e:	48 8b 9d 18 ff ff ff 	mov    -0xe8(%rbp),%rbx
    3995:	66 48 0f 7e cb       	movq   %xmm1,%rbx
    399a:	48 89 8d 20 ff ff ff 	mov    %rcx,-0xe0(%rbp)
    39a1:	48 89 9d 28 ff ff ff 	mov    %rbx,-0xd8(%rbp)
    39a8:	64 48 8b 0c 25 28 00 	mov    %fs:0x28,%rcx
    39af:	00 00 
    39b1:	48 89 4d e8          	mov    %rcx,-0x18(%rbp)
    39b5:	31 c9                	xor    %ecx,%ecx
    39b7:	f3 0f 10 85 30 ff ff 	movss  -0xd0(%rbp),%xmm0
    39be:	ff 
    39bf:	f3 0f 10 8d 34 ff ff 	movss  -0xcc(%rbp),%xmm1
    39c6:	ff 
    39c7:	f3 0f 10 95 38 ff ff 	movss  -0xc8(%rbp),%xmm2
    39ce:	ff 
    39cf:	f3 0f 10 9d 3c ff ff 	movss  -0xc4(%rbp),%xmm3
    39d6:	ff 
    39d7:	f3 0f 11 9d 58 ff ff 	movss  %xmm3,-0xa8(%rbp)
    39de:	ff 
    39df:	f3 0f 11 95 5c ff ff 	movss  %xmm2,-0xa4(%rbp)
    39e6:	ff 
    39e7:	f3 0f 11 8d 60 ff ff 	movss  %xmm1,-0xa0(%rbp)
    39ee:	ff 
    39ef:	f3 0f 11 85 64 ff ff 	movss  %xmm0,-0x9c(%rbp)
    39f6:	ff 
    39f7:	f3 0f 10 85 58 ff ff 	movss  -0xa8(%rbp),%xmm0
    39fe:	ff 
    39ff:	f3 0f 10 8d 5c ff ff 	movss  -0xa4(%rbp),%xmm1
    3a06:	ff 
    3a07:	0f 28 d1             	movaps %xmm1,%xmm2
    3a0a:	0f 14 d0             	unpcklps %xmm0,%xmm2
    3a0d:	f3 0f 10 8d 60 ff ff 	movss  -0xa0(%rbp),%xmm1
    3a14:	ff 
    3a15:	f3 0f 10 85 64 ff ff 	movss  -0x9c(%rbp),%xmm0
    3a1c:	ff 
    3a1d:	0f 14 c1             	unpcklps %xmm1,%xmm0
    3a20:	0f 16 c2             	movlhps %xmm2,%xmm0
    3a23:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
    3a2a:	f3 0f 10 85 20 ff ff 	movss  -0xe0(%rbp),%xmm0
    3a31:	ff 
    3a32:	f3 0f 10 8d 24 ff ff 	movss  -0xdc(%rbp),%xmm1
    3a39:	ff 
    3a3a:	f3 0f 10 95 28 ff ff 	movss  -0xd8(%rbp),%xmm2
    3a41:	ff 
    3a42:	f3 0f 10 9d 2c ff ff 	movss  -0xd4(%rbp),%xmm3
    3a49:	ff 
    3a4a:	f3 0f 11 9d 48 ff ff 	movss  %xmm3,-0xb8(%rbp)
    3a51:	ff 
    3a52:	f3 0f 11 95 4c ff ff 	movss  %xmm2,-0xb4(%rbp)
    3a59:	ff 
    3a5a:	f3 0f 11 8d 50 ff ff 	movss  %xmm1,-0xb0(%rbp)
    3a61:	ff 
    3a62:	f3 0f 11 85 54 ff ff 	movss  %xmm0,-0xac(%rbp)
    3a69:	ff 
    3a6a:	f3 0f 10 85 48 ff ff 	movss  -0xb8(%rbp),%xmm0
    3a71:	ff 
    3a72:	f3 0f 10 8d 4c ff ff 	movss  -0xb4(%rbp),%xmm1
    3a79:	ff 
    3a7a:	0f 28 d1             	movaps %xmm1,%xmm2
    3a7d:	0f 14 d0             	unpcklps %xmm0,%xmm2
    3a80:	f3 0f 10 8d 50 ff ff 	movss  -0xb0(%rbp),%xmm1
    3a87:	ff 
    3a88:	f3 0f 10 85 54 ff ff 	movss  -0xac(%rbp),%xmm0
    3a8f:	ff 
    3a90:	0f 14 c1             	unpcklps %xmm1,%xmm0
    3a93:	0f 16 c2             	movlhps %xmm2,%xmm0
    3a96:	0f 29 45 80          	movaps %xmm0,-0x80(%rbp)
    3a9a:	0f 28 85 70 ff ff ff 	movaps -0x90(%rbp),%xmm0
    3aa1:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
    3aa5:	0f 28 45 80          	movaps -0x80(%rbp),%xmm0
    3aa9:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
    3aad:	0f 28 45 b0          	movaps -0x50(%rbp),%xmm0
    3ab1:	0f 58 45 c0          	addps  -0x40(%rbp),%xmm0
    3ab5:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
    3ab9:	48 8d 4d d0          	lea    -0x30(%rbp),%rcx
    3abd:	48 89 8d 68 ff ff ff 	mov    %rcx,-0x98(%rbp)
    3ac4:	0f 28 45 90          	movaps -0x70(%rbp),%xmm0
    3ac8:	0f 29 45 a0          	movaps %xmm0,-0x60(%rbp)
    3acc:	0f 28 45 a0          	movaps -0x60(%rbp),%xmm0
    3ad0:	48 8b 8d 68 ff ff ff 	mov    -0x98(%rbp),%rcx
    3ad7:	0f 11 01             	movups %xmm0,(%rcx)
    3ada:	90                   	nop
    3adb:	f3 0f 10 5d d0       	movss  -0x30(%rbp),%xmm3
    3ae0:	f3 0f 10 55 d4       	movss  -0x2c(%rbp),%xmm2
    3ae5:	f3 0f 10 4d d8       	movss  -0x28(%rbp),%xmm1
    3aea:	f3 0f 10 45 dc       	movss  -0x24(%rbp),%xmm0
    3aef:	66 0f 7e de          	movd   %xmm3,%esi
    3af3:	48 89 c7             	mov    %rax,%rdi
    3af6:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    3afd:	ff ff ff 
    3b00:	48 21 f9             	and    %rdi,%rcx
    3b03:	48 09 f1             	or     %rsi,%rcx
    3b06:	48 89 c8             	mov    %rcx,%rax
    3b09:	66 0f 7e d1          	movd   %xmm2,%ecx
    3b0d:	48 c1 e1 20          	shl    $0x20,%rcx
    3b11:	48 89 c6             	mov    %rax,%rsi
    3b14:	89 f6                	mov    %esi,%esi
    3b16:	48 09 f1             	or     %rsi,%rcx
    3b19:	48 89 c8             	mov    %rcx,%rax
    3b1c:	66 0f 7e ce          	movd   %xmm1,%esi
    3b20:	48 89 d7             	mov    %rdx,%rdi
    3b23:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    3b2a:	ff ff ff 
    3b2d:	48 21 f9             	and    %rdi,%rcx
    3b30:	48 09 f1             	or     %rsi,%rcx
    3b33:	48 89 ca             	mov    %rcx,%rdx
    3b36:	66 0f 7e c1          	movd   %xmm0,%ecx
    3b3a:	48 c1 e1 20          	shl    $0x20,%rcx
    3b3e:	48 89 d6             	mov    %rdx,%rsi
    3b41:	89 f6                	mov    %esi,%esi
    3b43:	48 09 f1             	or     %rsi,%rcx
    3b46:	48 89 ca             	mov    %rcx,%rdx
    3b49:	48 89 c1             	mov    %rax,%rcx
    3b4c:	48 89 d3             	mov    %rdx,%rbx
    3b4f:	66 48 0f 6e ca       	movq   %rdx,%xmm1
    3b54:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    3b58:	64 48 2b 04 25 28 00 	sub    %fs:0x28,%rax
    3b5f:	00 00 
    3b61:	74 05                	je     3b68 <gul_simd_add+0x21c>
    3b63:	e8 78 e5 ff ff       	call   20e0 <__stack_chk_fail@plt>
    3b68:	66 48 0f 6e c1       	movq   %rcx,%xmm0
    3b6d:	48 8b 5d f8          	mov    -0x8(%rbp),%rbx
    3b71:	c9                   	leave
    3b72:	c3                   	ret

0000000000003b73 <gul_simd_sub>:
    3b73:	f3 0f 1e fa          	endbr64
    3b77:	55                   	push   %rbp
    3b78:	48 89 e5             	mov    %rsp,%rbp
    3b7b:	53                   	push   %rbx
    3b7c:	48 81 ec e8 00 00 00 	sub    $0xe8,%rsp
    3b83:	66 48 0f 7e c1       	movq   %xmm0,%rcx
    3b88:	66 0f 6f c1          	movdqa %xmm1,%xmm0
    3b8c:	66 48 0f 7e c3       	movq   %xmm0,%rbx
    3b91:	48 89 8d 30 ff ff ff 	mov    %rcx,-0xd0(%rbp)
    3b98:	48 89 9d 38 ff ff ff 	mov    %rbx,-0xc8(%rbp)
    3b9f:	66 0f 6f c2          	movdqa %xmm2,%xmm0
    3ba3:	66 0f 6f cb          	movdqa %xmm3,%xmm1
    3ba7:	0f 29 85 10 ff ff ff 	movaps %xmm0,-0xf0(%rbp)
    3bae:	48 8b 8d 10 ff ff ff 	mov    -0xf0(%rbp),%rcx
    3bb5:	48 8b 9d 18 ff ff ff 	mov    -0xe8(%rbp),%rbx
    3bbc:	66 48 0f 7e cb       	movq   %xmm1,%rbx
    3bc1:	48 89 8d 20 ff ff ff 	mov    %rcx,-0xe0(%rbp)
    3bc8:	48 89 9d 28 ff ff ff 	mov    %rbx,-0xd8(%rbp)
    3bcf:	64 48 8b 0c 25 28 00 	mov    %fs:0x28,%rcx
    3bd6:	00 00 
    3bd8:	48 89 4d e8          	mov    %rcx,-0x18(%rbp)
    3bdc:	31 c9                	xor    %ecx,%ecx
    3bde:	f3 0f 10 85 30 ff ff 	movss  -0xd0(%rbp),%xmm0
    3be5:	ff 
    3be6:	f3 0f 10 8d 34 ff ff 	movss  -0xcc(%rbp),%xmm1
    3bed:	ff 
    3bee:	f3 0f 10 95 38 ff ff 	movss  -0xc8(%rbp),%xmm2
    3bf5:	ff 
    3bf6:	f3 0f 10 9d 3c ff ff 	movss  -0xc4(%rbp),%xmm3
    3bfd:	ff 
    3bfe:	f3 0f 11 9d 58 ff ff 	movss  %xmm3,-0xa8(%rbp)
    3c05:	ff 
    3c06:	f3 0f 11 95 5c ff ff 	movss  %xmm2,-0xa4(%rbp)
    3c0d:	ff 
    3c0e:	f3 0f 11 8d 60 ff ff 	movss  %xmm1,-0xa0(%rbp)
    3c15:	ff 
    3c16:	f3 0f 11 85 64 ff ff 	movss  %xmm0,-0x9c(%rbp)
    3c1d:	ff 
    3c1e:	f3 0f 10 85 58 ff ff 	movss  -0xa8(%rbp),%xmm0
    3c25:	ff 
    3c26:	f3 0f 10 8d 5c ff ff 	movss  -0xa4(%rbp),%xmm1
    3c2d:	ff 
    3c2e:	0f 28 d1             	movaps %xmm1,%xmm2
    3c31:	0f 14 d0             	unpcklps %xmm0,%xmm2
    3c34:	f3 0f 10 8d 60 ff ff 	movss  -0xa0(%rbp),%xmm1
    3c3b:	ff 
    3c3c:	f3 0f 10 85 64 ff ff 	movss  -0x9c(%rbp),%xmm0
    3c43:	ff 
    3c44:	0f 14 c1             	unpcklps %xmm1,%xmm0
    3c47:	0f 16 c2             	movlhps %xmm2,%xmm0
    3c4a:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
    3c51:	f3 0f 10 85 20 ff ff 	movss  -0xe0(%rbp),%xmm0
    3c58:	ff 
    3c59:	f3 0f 10 8d 24 ff ff 	movss  -0xdc(%rbp),%xmm1
    3c60:	ff 
    3c61:	f3 0f 10 95 28 ff ff 	movss  -0xd8(%rbp),%xmm2
    3c68:	ff 
    3c69:	f3 0f 10 9d 2c ff ff 	movss  -0xd4(%rbp),%xmm3
    3c70:	ff 
    3c71:	f3 0f 11 9d 48 ff ff 	movss  %xmm3,-0xb8(%rbp)
    3c78:	ff 
    3c79:	f3 0f 11 95 4c ff ff 	movss  %xmm2,-0xb4(%rbp)
    3c80:	ff 
    3c81:	f3 0f 11 8d 50 ff ff 	movss  %xmm1,-0xb0(%rbp)
    3c88:	ff 
    3c89:	f3 0f 11 85 54 ff ff 	movss  %xmm0,-0xac(%rbp)
    3c90:	ff 
    3c91:	f3 0f 10 85 48 ff ff 	movss  -0xb8(%rbp),%xmm0
    3c98:	ff 
    3c99:	f3 0f 10 8d 4c ff ff 	movss  -0xb4(%rbp),%xmm1
    3ca0:	ff 
    3ca1:	0f 28 d1             	movaps %xmm1,%xmm2
    3ca4:	0f 14 d0             	unpcklps %xmm0,%xmm2
    3ca7:	f3 0f 10 8d 50 ff ff 	movss  -0xb0(%rbp),%xmm1
    3cae:	ff 
    3caf:	f3 0f 10 85 54 ff ff 	movss  -0xac(%rbp),%xmm0
    3cb6:	ff 
    3cb7:	0f 14 c1             	unpcklps %xmm1,%xmm0
    3cba:	0f 16 c2             	movlhps %xmm2,%xmm0
    3cbd:	0f 29 45 80          	movaps %xmm0,-0x80(%rbp)
    3cc1:	0f 28 85 70 ff ff ff 	movaps -0x90(%rbp),%xmm0
    3cc8:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
    3ccc:	0f 28 45 80          	movaps -0x80(%rbp),%xmm0
    3cd0:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
    3cd4:	0f 28 45 b0          	movaps -0x50(%rbp),%xmm0
    3cd8:	0f 5c 45 c0          	subps  -0x40(%rbp),%xmm0
    3cdc:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
    3ce0:	48 8d 4d d0          	lea    -0x30(%rbp),%rcx
    3ce4:	48 89 8d 68 ff ff ff 	mov    %rcx,-0x98(%rbp)
    3ceb:	0f 28 45 90          	movaps -0x70(%rbp),%xmm0
    3cef:	0f 29 45 a0          	movaps %xmm0,-0x60(%rbp)
    3cf3:	0f 28 45 a0          	movaps -0x60(%rbp),%xmm0
    3cf7:	48 8b 8d 68 ff ff ff 	mov    -0x98(%rbp),%rcx
    3cfe:	0f 11 01             	movups %xmm0,(%rcx)
    3d01:	90                   	nop
    3d02:	f3 0f 10 5d d0       	movss  -0x30(%rbp),%xmm3
    3d07:	f3 0f 10 55 d4       	movss  -0x2c(%rbp),%xmm2
    3d0c:	f3 0f 10 4d d8       	movss  -0x28(%rbp),%xmm1
    3d11:	f3 0f 10 45 dc       	movss  -0x24(%rbp),%xmm0
    3d16:	66 0f 7e de          	movd   %xmm3,%esi
    3d1a:	48 89 c7             	mov    %rax,%rdi
    3d1d:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    3d24:	ff ff ff 
    3d27:	48 21 f9             	and    %rdi,%rcx
    3d2a:	48 09 f1             	or     %rsi,%rcx
    3d2d:	48 89 c8             	mov    %rcx,%rax
    3d30:	66 0f 7e d1          	movd   %xmm2,%ecx
    3d34:	48 c1 e1 20          	shl    $0x20,%rcx
    3d38:	48 89 c6             	mov    %rax,%rsi
    3d3b:	89 f6                	mov    %esi,%esi
    3d3d:	48 09 f1             	or     %rsi,%rcx
    3d40:	48 89 c8             	mov    %rcx,%rax
    3d43:	66 0f 7e ce          	movd   %xmm1,%esi
    3d47:	48 89 d7             	mov    %rdx,%rdi
    3d4a:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    3d51:	ff ff ff 
    3d54:	48 21 f9             	and    %rdi,%rcx
    3d57:	48 09 f1             	or     %rsi,%rcx
    3d5a:	48 89 ca             	mov    %rcx,%rdx
    3d5d:	66 0f 7e c1          	movd   %xmm0,%ecx
    3d61:	48 c1 e1 20          	shl    $0x20,%rcx
    3d65:	48 89 d6             	mov    %rdx,%rsi
    3d68:	89 f6                	mov    %esi,%esi
    3d6a:	48 09 f1             	or     %rsi,%rcx
    3d6d:	48 89 ca             	mov    %rcx,%rdx
    3d70:	48 89 c1             	mov    %rax,%rcx
    3d73:	48 89 d3             	mov    %rdx,%rbx
    3d76:	66 48 0f 6e ca       	movq   %rdx,%xmm1
    3d7b:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    3d7f:	64 48 2b 04 25 28 00 	sub    %fs:0x28,%rax
    3d86:	00 00 
    3d88:	74 05                	je     3d8f <gul_simd_sub+0x21c>
    3d8a:	e8 51 e3 ff ff       	call   20e0 <__stack_chk_fail@plt>
    3d8f:	66 48 0f 6e c1       	movq   %rcx,%xmm0
    3d94:	48 8b 5d f8          	mov    -0x8(%rbp),%rbx
    3d98:	c9                   	leave
    3d99:	c3                   	ret

0000000000003d9a <gul_simd_mul>:
    3d9a:	f3 0f 1e fa          	endbr64
    3d9e:	55                   	push   %rbp
    3d9f:	48 89 e5             	mov    %rsp,%rbp
    3da2:	53                   	push   %rbx
    3da3:	48 81 ec e8 00 00 00 	sub    $0xe8,%rsp
    3daa:	66 48 0f 7e c1       	movq   %xmm0,%rcx
    3daf:	66 0f 6f c1          	movdqa %xmm1,%xmm0
    3db3:	66 48 0f 7e c3       	movq   %xmm0,%rbx
    3db8:	48 89 8d 30 ff ff ff 	mov    %rcx,-0xd0(%rbp)
    3dbf:	48 89 9d 38 ff ff ff 	mov    %rbx,-0xc8(%rbp)
    3dc6:	66 0f 6f c2          	movdqa %xmm2,%xmm0
    3dca:	66 0f 6f cb          	movdqa %xmm3,%xmm1
    3dce:	0f 29 85 10 ff ff ff 	movaps %xmm0,-0xf0(%rbp)
    3dd5:	48 8b 8d 10 ff ff ff 	mov    -0xf0(%rbp),%rcx
    3ddc:	48 8b 9d 18 ff ff ff 	mov    -0xe8(%rbp),%rbx
    3de3:	66 48 0f 7e cb       	movq   %xmm1,%rbx
    3de8:	48 89 8d 20 ff ff ff 	mov    %rcx,-0xe0(%rbp)
    3def:	48 89 9d 28 ff ff ff 	mov    %rbx,-0xd8(%rbp)
    3df6:	64 48 8b 0c 25 28 00 	mov    %fs:0x28,%rcx
    3dfd:	00 00 
    3dff:	48 89 4d e8          	mov    %rcx,-0x18(%rbp)
    3e03:	31 c9                	xor    %ecx,%ecx
    3e05:	f3 0f 10 85 30 ff ff 	movss  -0xd0(%rbp),%xmm0
    3e0c:	ff 
    3e0d:	f3 0f 10 8d 34 ff ff 	movss  -0xcc(%rbp),%xmm1
    3e14:	ff 
    3e15:	f3 0f 10 95 38 ff ff 	movss  -0xc8(%rbp),%xmm2
    3e1c:	ff 
    3e1d:	f3 0f 10 9d 3c ff ff 	movss  -0xc4(%rbp),%xmm3
    3e24:	ff 
    3e25:	f3 0f 11 9d 58 ff ff 	movss  %xmm3,-0xa8(%rbp)
    3e2c:	ff 
    3e2d:	f3 0f 11 95 5c ff ff 	movss  %xmm2,-0xa4(%rbp)
    3e34:	ff 
    3e35:	f3 0f 11 8d 60 ff ff 	movss  %xmm1,-0xa0(%rbp)
    3e3c:	ff 
    3e3d:	f3 0f 11 85 64 ff ff 	movss  %xmm0,-0x9c(%rbp)
    3e44:	ff 
    3e45:	f3 0f 10 85 58 ff ff 	movss  -0xa8(%rbp),%xmm0
    3e4c:	ff 
    3e4d:	f3 0f 10 8d 5c ff ff 	movss  -0xa4(%rbp),%xmm1
    3e54:	ff 
    3e55:	0f 28 d1             	movaps %xmm1,%xmm2
    3e58:	0f 14 d0             	unpcklps %xmm0,%xmm2
    3e5b:	f3 0f 10 8d 60 ff ff 	movss  -0xa0(%rbp),%xmm1
    3e62:	ff 
    3e63:	f3 0f 10 85 64 ff ff 	movss  -0x9c(%rbp),%xmm0
    3e6a:	ff 
    3e6b:	0f 14 c1             	unpcklps %xmm1,%xmm0
    3e6e:	0f 16 c2             	movlhps %xmm2,%xmm0
    3e71:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
    3e78:	f3 0f 10 85 20 ff ff 	movss  -0xe0(%rbp),%xmm0
    3e7f:	ff 
    3e80:	f3 0f 10 8d 24 ff ff 	movss  -0xdc(%rbp),%xmm1
    3e87:	ff 
    3e88:	f3 0f 10 95 28 ff ff 	movss  -0xd8(%rbp),%xmm2
    3e8f:	ff 
    3e90:	f3 0f 10 9d 2c ff ff 	movss  -0xd4(%rbp),%xmm3
    3e97:	ff 
    3e98:	f3 0f 11 9d 48 ff ff 	movss  %xmm3,-0xb8(%rbp)
    3e9f:	ff 
    3ea0:	f3 0f 11 95 4c ff ff 	movss  %xmm2,-0xb4(%rbp)
    3ea7:	ff 
    3ea8:	f3 0f 11 8d 50 ff ff 	movss  %xmm1,-0xb0(%rbp)
    3eaf:	ff 
    3eb0:	f3 0f 11 85 54 ff ff 	movss  %xmm0,-0xac(%rbp)
    3eb7:	ff 
    3eb8:	f3 0f 10 85 48 ff ff 	movss  -0xb8(%rbp),%xmm0
    3ebf:	ff 
    3ec0:	f3 0f 10 8d 4c ff ff 	movss  -0xb4(%rbp),%xmm1
    3ec7:	ff 
    3ec8:	0f 28 d1             	movaps %xmm1,%xmm2
    3ecb:	0f 14 d0             	unpcklps %xmm0,%xmm2
    3ece:	f3 0f 10 8d 50 ff ff 	movss  -0xb0(%rbp),%xmm1
    3ed5:	ff 
    3ed6:	f3 0f 10 85 54 ff ff 	movss  -0xac(%rbp),%xmm0
    3edd:	ff 
    3ede:	0f 14 c1             	unpcklps %xmm1,%xmm0
    3ee1:	0f 16 c2             	movlhps %xmm2,%xmm0
    3ee4:	0f 29 45 80          	movaps %xmm0,-0x80(%rbp)
    3ee8:	0f 28 85 70 ff ff ff 	movaps -0x90(%rbp),%xmm0
    3eef:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
    3ef3:	0f 28 45 80          	movaps -0x80(%rbp),%xmm0
    3ef7:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
    3efb:	0f 28 45 b0          	movaps -0x50(%rbp),%xmm0
    3eff:	0f 59 45 c0          	mulps  -0x40(%rbp),%xmm0
    3f03:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
    3f07:	48 8d 4d d0          	lea    -0x30(%rbp),%rcx
    3f0b:	48 89 8d 68 ff ff ff 	mov    %rcx,-0x98(%rbp)
    3f12:	0f 28 45 90          	movaps -0x70(%rbp),%xmm0
    3f16:	0f 29 45 a0          	movaps %xmm0,-0x60(%rbp)
    3f1a:	0f 28 45 a0          	movaps -0x60(%rbp),%xmm0
    3f1e:	48 8b 8d 68 ff ff ff 	mov    -0x98(%rbp),%rcx
    3f25:	0f 11 01             	movups %xmm0,(%rcx)
    3f28:	90                   	nop
    3f29:	f3 0f 10 5d d0       	movss  -0x30(%rbp),%xmm3
    3f2e:	f3 0f 10 55 d4       	movss  -0x2c(%rbp),%xmm2
    3f33:	f3 0f 10 4d d8       	movss  -0x28(%rbp),%xmm1
    3f38:	f3 0f 10 45 dc       	movss  -0x24(%rbp),%xmm0
    3f3d:	66 0f 7e de          	movd   %xmm3,%esi
    3f41:	48 89 c7             	mov    %rax,%rdi
    3f44:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    3f4b:	ff ff ff 
    3f4e:	48 21 f9             	and    %rdi,%rcx
    3f51:	48 09 f1             	or     %rsi,%rcx
    3f54:	48 89 c8             	mov    %rcx,%rax
    3f57:	66 0f 7e d1          	movd   %xmm2,%ecx
    3f5b:	48 c1 e1 20          	shl    $0x20,%rcx
    3f5f:	48 89 c6             	mov    %rax,%rsi
    3f62:	89 f6                	mov    %esi,%esi
    3f64:	48 09 f1             	or     %rsi,%rcx
    3f67:	48 89 c8             	mov    %rcx,%rax
    3f6a:	66 0f 7e ce          	movd   %xmm1,%esi
    3f6e:	48 89 d7             	mov    %rdx,%rdi
    3f71:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    3f78:	ff ff ff 
    3f7b:	48 21 f9             	and    %rdi,%rcx
    3f7e:	48 09 f1             	or     %rsi,%rcx
    3f81:	48 89 ca             	mov    %rcx,%rdx
    3f84:	66 0f 7e c1          	movd   %xmm0,%ecx
    3f88:	48 c1 e1 20          	shl    $0x20,%rcx
    3f8c:	48 89 d6             	mov    %rdx,%rsi
    3f8f:	89 f6                	mov    %esi,%esi
    3f91:	48 09 f1             	or     %rsi,%rcx
    3f94:	48 89 ca             	mov    %rcx,%rdx
    3f97:	48 89 c1             	mov    %rax,%rcx
    3f9a:	48 89 d3             	mov    %rdx,%rbx
    3f9d:	66 48 0f 6e ca       	movq   %rdx,%xmm1
    3fa2:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    3fa6:	64 48 2b 04 25 28 00 	sub    %fs:0x28,%rax
    3fad:	00 00 
    3faf:	74 05                	je     3fb6 <gul_simd_mul+0x21c>
    3fb1:	e8 2a e1 ff ff       	call   20e0 <__stack_chk_fail@plt>
    3fb6:	66 48 0f 6e c1       	movq   %rcx,%xmm0
    3fbb:	48 8b 5d f8          	mov    -0x8(%rbp),%rbx
    3fbf:	c9                   	leave
    3fc0:	c3                   	ret

0000000000003fc1 <gul_simd_dot>:
    3fc1:	f3 0f 1e fa          	endbr64
    3fc5:	55                   	push   %rbp
    3fc6:	48 89 e5             	mov    %rsp,%rbp
    3fc9:	48 83 ec 40          	sub    $0x40,%rsp
    3fcd:	66 48 0f 7e c0       	movq   %xmm0,%rax
    3fd2:	66 0f 6f c1          	movdqa %xmm1,%xmm0
    3fd6:	66 48 0f 7e c2       	movq   %xmm0,%rdx
    3fdb:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    3fdf:	48 89 55 e8          	mov    %rdx,-0x18(%rbp)
    3fe3:	66 0f 6f c2          	movdqa %xmm2,%xmm0
    3fe7:	66 0f 6f cb          	movdqa %xmm3,%xmm1
    3feb:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
    3fef:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
    3ff3:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
    3ff7:	66 48 0f 7e ca       	movq   %xmm1,%rdx
    3ffc:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
    4000:	48 89 55 d8          	mov    %rdx,-0x28(%rbp)
    4004:	f3 0f 7e 55 d0       	movq   -0x30(%rbp),%xmm2
    4009:	f3 0f 7e 45 d8       	movq   -0x28(%rbp),%xmm0
    400e:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    4012:	f3 0f 7e 4d e8       	movq   -0x18(%rbp),%xmm1
    4017:	66 0f 6f d8          	movdqa %xmm0,%xmm3
    401b:	66 48 0f 6e c0       	movq   %rax,%xmm0
    4020:	e8 75 fd ff ff       	call   3d9a <gul_simd_mul>
    4025:	66 48 0f 7e c0       	movq   %xmm0,%rax
    402a:	66 0f 6f c1          	movdqa %xmm1,%xmm0
    402e:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    4032:	66 0f d6 45 f8       	movq   %xmm0,-0x8(%rbp)
    4037:	f3 0f 10 4d f0       	movss  -0x10(%rbp),%xmm1
    403c:	f3 0f 10 45 f4       	movss  -0xc(%rbp),%xmm0
    4041:	f3 0f 58 c8          	addss  %xmm0,%xmm1
    4045:	f3 0f 10 45 f8       	movss  -0x8(%rbp),%xmm0
    404a:	f3 0f 58 c8          	addss  %xmm0,%xmm1
    404e:	f3 0f 10 45 fc       	movss  -0x4(%rbp),%xmm0
    4053:	f3 0f 58 c1          	addss  %xmm1,%xmm0
    4057:	c9                   	leave
    4058:	c3                   	ret

0000000000004059 <gul_simd_magnitude>:
    4059:	f3 0f 1e fa          	endbr64
    405d:	55                   	push   %rbp
    405e:	48 89 e5             	mov    %rsp,%rbp
    4061:	48 83 ec 10          	sub    $0x10,%rsp
    4065:	66 48 0f 7e c0       	movq   %xmm0,%rax
    406a:	66 0f 6f c1          	movdqa %xmm1,%xmm0
    406e:	66 48 0f 7e c2       	movq   %xmm0,%rdx
    4073:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    4077:	48 89 55 f8          	mov    %rdx,-0x8(%rbp)
    407b:	f3 0f 7e 55 f0       	movq   -0x10(%rbp),%xmm2
    4080:	f3 0f 7e 45 f8       	movq   -0x8(%rbp),%xmm0
    4085:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    4089:	f3 0f 7e 4d f8       	movq   -0x8(%rbp),%xmm1
    408e:	66 0f 6f d8          	movdqa %xmm0,%xmm3
    4092:	66 48 0f 6e c0       	movq   %rax,%xmm0
    4097:	e8 25 ff ff ff       	call   3fc1 <gul_simd_dot>
    409c:	66 0f 7e c0          	movd   %xmm0,%eax
    40a0:	66 0f 6e c0          	movd   %eax,%xmm0
    40a4:	e8 07 e1 ff ff       	call   21b0 <sqrtf@plt>
    40a9:	c9                   	leave
    40aa:	c3                   	ret

00000000000040ab <gul_simd_normalize>:
    40ab:	f3 0f 1e fa          	endbr64
    40af:	55                   	push   %rbp
    40b0:	48 89 e5             	mov    %rsp,%rbp
    40b3:	41 55                	push   %r13
    40b5:	41 54                	push   %r12
    40b7:	53                   	push   %rbx
    40b8:	48 83 ec 28          	sub    $0x28,%rsp
    40bc:	66 48 0f 7e c0       	movq   %xmm0,%rax
    40c1:	66 0f 6f c1          	movdqa %xmm1,%xmm0
    40c5:	66 48 0f 7e c2       	movq   %xmm0,%rdx
    40ca:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
    40ce:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
    40d2:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
    40d6:	f3 0f 7e 4d c8       	movq   -0x38(%rbp),%xmm1
    40db:	66 48 0f 6e c0       	movq   %rax,%xmm0
    40e0:	e8 74 ff ff ff       	call   4059 <gul_simd_magnitude>
    40e5:	66 0f 7e c0          	movd   %xmm0,%eax
    40e9:	89 45 dc             	mov    %eax,-0x24(%rbp)
    40ec:	f3 0f 10 45 dc       	movss  -0x24(%rbp),%xmm0
    40f1:	66 0f ef c9          	pxor   %xmm1,%xmm1
    40f5:	0f 2f c1             	comiss %xmm1,%xmm0
    40f8:	0f 86 8d 00 00 00    	jbe    418b <gul_simd_normalize+0xe0>
    40fe:	f3 0f 10 45 c0       	movss  -0x40(%rbp),%xmm0
    4103:	0f 28 d8             	movaps %xmm0,%xmm3
    4106:	f3 0f 5e 5d dc       	divss  -0x24(%rbp),%xmm3
    410b:	f3 0f 10 45 c4       	movss  -0x3c(%rbp),%xmm0
    4110:	0f 28 d0             	movaps %xmm0,%xmm2
    4113:	f3 0f 5e 55 dc       	divss  -0x24(%rbp),%xmm2
    4118:	f3 0f 10 45 c8       	movss  -0x38(%rbp),%xmm0
    411d:	0f 28 c8             	movaps %xmm0,%xmm1
    4120:	f3 0f 5e 4d dc       	divss  -0x24(%rbp),%xmm1
    4125:	f3 0f 10 45 cc       	movss  -0x34(%rbp),%xmm0
    412a:	f3 0f 5e 45 dc       	divss  -0x24(%rbp),%xmm0
    412f:	66 0f 7e da          	movd   %xmm3,%edx
    4133:	4c 89 e1             	mov    %r12,%rcx
    4136:	48 b8 00 00 00 00 ff 	movabs $0xffffffff00000000,%rax
    413d:	ff ff ff 
    4140:	48 21 c8             	and    %rcx,%rax
    4143:	48 09 d0             	or     %rdx,%rax
    4146:	49 89 c4             	mov    %rax,%r12
    4149:	66 0f 7e d0          	movd   %xmm2,%eax
    414d:	48 c1 e0 20          	shl    $0x20,%rax
    4151:	4c 89 e2             	mov    %r12,%rdx
    4154:	89 d2                	mov    %edx,%edx
    4156:	48 09 d0             	or     %rdx,%rax
    4159:	49 89 c4             	mov    %rax,%r12
    415c:	66 0f 7e ca          	movd   %xmm1,%edx
    4160:	4c 89 e9             	mov    %r13,%rcx
    4163:	48 b8 00 00 00 00 ff 	movabs $0xffffffff00000000,%rax
    416a:	ff ff ff 
    416d:	48 21 c8             	and    %rcx,%rax
    4170:	48 09 d0             	or     %rdx,%rax
    4173:	49 89 c5             	mov    %rax,%r13
    4176:	66 0f 7e c0          	movd   %xmm0,%eax
    417a:	48 c1 e0 20          	shl    $0x20,%rax
    417e:	4c 89 ea             	mov    %r13,%rdx
    4181:	89 d2                	mov    %edx,%edx
    4183:	48 09 d0             	or     %rdx,%rax
    4186:	49 89 c5             	mov    %rax,%r13
    4189:	eb 08                	jmp    4193 <gul_simd_normalize+0xe8>
    418b:	4c 8b 65 c0          	mov    -0x40(%rbp),%r12
    418f:	4c 8b 6d c8          	mov    -0x38(%rbp),%r13
    4193:	4c 89 e1             	mov    %r12,%rcx
    4196:	4c 89 eb             	mov    %r13,%rbx
    4199:	4c 89 e0             	mov    %r12,%rax
    419c:	4c 89 ea             	mov    %r13,%rdx
    419f:	66 48 0f 6e ca       	movq   %rdx,%xmm1
    41a4:	66 48 0f 6e c1       	movq   %rcx,%xmm0
    41a9:	48 83 c4 28          	add    $0x28,%rsp
    41ad:	5b                   	pop    %rbx
    41ae:	41 5c                	pop    %r12
    41b0:	41 5d                	pop    %r13
    41b2:	5d                   	pop    %rbp
    41b3:	c3                   	ret

00000000000041b4 <gul_simd_cross>:
    41b4:	f3 0f 1e fa          	endbr64
    41b8:	55                   	push   %rbp
    41b9:	48 89 e5             	mov    %rsp,%rbp
    41bc:	53                   	push   %rbx
    41bd:	66 48 0f 7e c1       	movq   %xmm0,%rcx
    41c2:	66 0f 6f c1          	movdqa %xmm1,%xmm0
    41c6:	66 48 0f 7e c3       	movq   %xmm0,%rbx
    41cb:	48 89 4d e0          	mov    %rcx,-0x20(%rbp)
    41cf:	48 89 5d e8          	mov    %rbx,-0x18(%rbp)
    41d3:	66 0f 6f c2          	movdqa %xmm2,%xmm0
    41d7:	66 0f 6f cb          	movdqa %xmm3,%xmm1
    41db:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
    41df:	48 8b 4d c0          	mov    -0x40(%rbp),%rcx
    41e3:	48 8b 5d c8          	mov    -0x38(%rbp),%rbx
    41e7:	66 48 0f 7e cb       	movq   %xmm1,%rbx
    41ec:	48 89 4d d0          	mov    %rcx,-0x30(%rbp)
    41f0:	48 89 5d d8          	mov    %rbx,-0x28(%rbp)
    41f4:	f3 0f 10 4d e4       	movss  -0x1c(%rbp),%xmm1
    41f9:	f3 0f 10 45 d8       	movss  -0x28(%rbp),%xmm0
    41fe:	f3 0f 59 c1          	mulss  %xmm1,%xmm0
    4202:	f3 0f 10 55 e8       	movss  -0x18(%rbp),%xmm2
    4207:	f3 0f 10 4d d4       	movss  -0x2c(%rbp),%xmm1
    420c:	f3 0f 59 ca          	mulss  %xmm2,%xmm1
    4210:	0f 28 e0             	movaps %xmm0,%xmm4
    4213:	f3 0f 5c e1          	subss  %xmm1,%xmm4
    4217:	f3 0f 10 4d e8       	movss  -0x18(%rbp),%xmm1
    421c:	f3 0f 10 45 d0       	movss  -0x30(%rbp),%xmm0
    4221:	f3 0f 59 c1          	mulss  %xmm1,%xmm0
    4225:	f3 0f 10 55 e0       	movss  -0x20(%rbp),%xmm2
    422a:	f3 0f 10 4d d8       	movss  -0x28(%rbp),%xmm1
    422f:	f3 0f 59 ca          	mulss  %xmm2,%xmm1
    4233:	0f 28 d8             	movaps %xmm0,%xmm3
    4236:	f3 0f 5c d9          	subss  %xmm1,%xmm3
    423a:	f3 0f 10 4d e0       	movss  -0x20(%rbp),%xmm1
    423f:	f3 0f 10 45 d4       	movss  -0x2c(%rbp),%xmm0
    4244:	f3 0f 59 c1          	mulss  %xmm1,%xmm0
    4248:	f3 0f 10 55 e4       	movss  -0x1c(%rbp),%xmm2
    424d:	f3 0f 10 4d d0       	movss  -0x30(%rbp),%xmm1
    4252:	f3 0f 59 ca          	mulss  %xmm2,%xmm1
    4256:	f3 0f 5c c1          	subss  %xmm1,%xmm0
    425a:	0f 28 cc             	movaps %xmm4,%xmm1
    425d:	66 0f 7e ce          	movd   %xmm1,%esi
    4261:	48 89 c7             	mov    %rax,%rdi
    4264:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    426b:	ff ff ff 
    426e:	48 21 f9             	and    %rdi,%rcx
    4271:	48 09 f1             	or     %rsi,%rcx
    4274:	48 89 c8             	mov    %rcx,%rax
    4277:	0f 28 cb             	movaps %xmm3,%xmm1
    427a:	66 0f 7e c9          	movd   %xmm1,%ecx
    427e:	48 c1 e1 20          	shl    $0x20,%rcx
    4282:	48 89 c6             	mov    %rax,%rsi
    4285:	89 f6                	mov    %esi,%esi
    4287:	48 09 f1             	or     %rsi,%rcx
    428a:	48 89 c8             	mov    %rcx,%rax
    428d:	66 0f 7e c6          	movd   %xmm0,%esi
    4291:	48 89 d7             	mov    %rdx,%rdi
    4294:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
    429b:	ff ff ff 
    429e:	48 21 f9             	and    %rdi,%rcx
    42a1:	48 09 f1             	or     %rsi,%rcx
    42a4:	48 89 ca             	mov    %rcx,%rdx
    42a7:	b9 00 00 00 00       	mov    $0x0,%ecx
    42ac:	89 c9                	mov    %ecx,%ecx
    42ae:	48 c1 e1 20          	shl    $0x20,%rcx
    42b2:	48 89 d6             	mov    %rdx,%rsi
    42b5:	89 f6                	mov    %esi,%esi
    42b7:	48 09 f1             	or     %rsi,%rcx
    42ba:	48 89 ca             	mov    %rcx,%rdx
    42bd:	48 89 c1             	mov    %rax,%rcx
    42c0:	48 89 d3             	mov    %rdx,%rbx
    42c3:	66 48 0f 6e ca       	movq   %rdx,%xmm1
    42c8:	66 48 0f 6e c1       	movq   %rcx,%xmm0
    42cd:	48 8b 5d f8          	mov    -0x8(%rbp),%rbx
    42d1:	c9                   	leave
    42d2:	c3                   	ret

00000000000042d3 <gul_tensor_add_simd>:
    42d3:	f3 0f 1e fa          	endbr64
    42d7:	55                   	push   %rbp
    42d8:	48 89 e5             	mov    %rsp,%rbp
    42db:	48 83 ec 48          	sub    $0x48,%rsp
    42df:	48 89 bd 58 ff ff ff 	mov    %rdi,-0xa8(%rbp)
    42e6:	48 89 b5 50 ff ff ff 	mov    %rsi,-0xb0(%rbp)
    42ed:	48 89 95 48 ff ff ff 	mov    %rdx,-0xb8(%rbp)
    42f4:	48 89 8d 40 ff ff ff 	mov    %rcx,-0xc0(%rbp)
    42fb:	48 8b 85 58 ff ff ff 	mov    -0xa8(%rbp),%rax
    4302:	48 89 85 70 ff ff ff 	mov    %rax,-0x90(%rbp)
    4309:	48 8b 85 50 ff ff ff 	mov    -0xb0(%rbp),%rax
    4310:	48 89 85 78 ff ff ff 	mov    %rax,-0x88(%rbp)
    4317:	48 8b 85 48 ff ff ff 	mov    -0xb8(%rbp),%rax
    431e:	48 89 45 80          	mov    %rax,-0x80(%rbp)
    4322:	48 c7 85 68 ff ff ff 	movq   $0x0,-0x98(%rbp)
    4329:	00 00 00 00 
    432d:	e9 a2 00 00 00       	jmp    43d4 <gul_tensor_add_simd+0x101>
    4332:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    4339:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    4340:	00 
    4341:	48 8b 85 78 ff ff ff 	mov    -0x88(%rbp),%rax
    4348:	48 01 d0             	add    %rdx,%rax
    434b:	48 89 45 98          	mov    %rax,-0x68(%rbp)
    434f:	48 8b 45 98          	mov    -0x68(%rbp),%rax
    4353:	0f 10 00             	movups (%rax),%xmm0
    4356:	0f 29 45 a0          	movaps %xmm0,-0x60(%rbp)
    435a:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    4361:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    4368:	00 
    4369:	48 8b 45 80          	mov    -0x80(%rbp),%rax
    436d:	48 01 d0             	add    %rdx,%rax
    4370:	48 89 45 90          	mov    %rax,-0x70(%rbp)
    4374:	48 8b 45 90          	mov    -0x70(%rbp),%rax
    4378:	0f 10 00             	movups (%rax),%xmm0
    437b:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
    437f:	0f 28 45 a0          	movaps -0x60(%rbp),%xmm0
    4383:	0f 29 45 e0          	movaps %xmm0,-0x20(%rbp)
    4387:	0f 28 45 b0          	movaps -0x50(%rbp),%xmm0
    438b:	0f 29 45 f0          	movaps %xmm0,-0x10(%rbp)
    438f:	0f 28 45 e0          	movaps -0x20(%rbp),%xmm0
    4393:	0f 58 45 f0          	addps  -0x10(%rbp),%xmm0
    4397:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
    439b:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    43a2:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    43a9:	00 
    43aa:	48 8b 85 70 ff ff ff 	mov    -0x90(%rbp),%rax
    43b1:	48 01 d0             	add    %rdx,%rax
    43b4:	48 89 45 88          	mov    %rax,-0x78(%rbp)
    43b8:	0f 28 45 c0          	movaps -0x40(%rbp),%xmm0
    43bc:	0f 29 45 d0          	movaps %xmm0,-0x30(%rbp)
    43c0:	0f 28 45 d0          	movaps -0x30(%rbp),%xmm0
    43c4:	48 8b 45 88          	mov    -0x78(%rbp),%rax
    43c8:	0f 11 00             	movups %xmm0,(%rax)
    43cb:	90                   	nop
    43cc:	48 83 85 68 ff ff ff 	addq   $0x4,-0x98(%rbp)
    43d3:	04 
    43d4:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    43db:	48 83 c0 03          	add    $0x3,%rax
    43df:	48 39 85 40 ff ff ff 	cmp    %rax,-0xc0(%rbp)
    43e6:	0f 8f 46 ff ff ff    	jg     4332 <gul_tensor_add_simd+0x5f>
    43ec:	eb 60                	jmp    444e <gul_tensor_add_simd+0x17b>
    43ee:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    43f5:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    43fc:	00 
    43fd:	48 8b 85 78 ff ff ff 	mov    -0x88(%rbp),%rax
    4404:	48 01 d0             	add    %rdx,%rax
    4407:	f3 0f 10 08          	movss  (%rax),%xmm1
    440b:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    4412:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    4419:	00 
    441a:	48 8b 45 80          	mov    -0x80(%rbp),%rax
    441e:	48 01 d0             	add    %rdx,%rax
    4421:	f3 0f 10 00          	movss  (%rax),%xmm0
    4425:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    442c:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    4433:	00 
    4434:	48 8b 85 70 ff ff ff 	mov    -0x90(%rbp),%rax
    443b:	48 01 d0             	add    %rdx,%rax
    443e:	f3 0f 58 c1          	addss  %xmm1,%xmm0
    4442:	f3 0f 11 00          	movss  %xmm0,(%rax)
    4446:	48 83 85 68 ff ff ff 	addq   $0x1,-0x98(%rbp)
    444d:	01 
    444e:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    4455:	48 3b 85 40 ff ff ff 	cmp    -0xc0(%rbp),%rax
    445c:	7c 90                	jl     43ee <gul_tensor_add_simd+0x11b>
    445e:	90                   	nop
    445f:	90                   	nop
    4460:	c9                   	leave
    4461:	c3                   	ret

0000000000004462 <gul_tensor_mul_simd>:
    4462:	f3 0f 1e fa          	endbr64
    4466:	55                   	push   %rbp
    4467:	48 89 e5             	mov    %rsp,%rbp
    446a:	48 83 ec 48          	sub    $0x48,%rsp
    446e:	48 89 bd 58 ff ff ff 	mov    %rdi,-0xa8(%rbp)
    4475:	48 89 b5 50 ff ff ff 	mov    %rsi,-0xb0(%rbp)
    447c:	48 89 95 48 ff ff ff 	mov    %rdx,-0xb8(%rbp)
    4483:	48 89 8d 40 ff ff ff 	mov    %rcx,-0xc0(%rbp)
    448a:	48 8b 85 58 ff ff ff 	mov    -0xa8(%rbp),%rax
    4491:	48 89 85 70 ff ff ff 	mov    %rax,-0x90(%rbp)
    4498:	48 8b 85 50 ff ff ff 	mov    -0xb0(%rbp),%rax
    449f:	48 89 85 78 ff ff ff 	mov    %rax,-0x88(%rbp)
    44a6:	48 8b 85 48 ff ff ff 	mov    -0xb8(%rbp),%rax
    44ad:	48 89 45 80          	mov    %rax,-0x80(%rbp)
    44b1:	48 c7 85 68 ff ff ff 	movq   $0x0,-0x98(%rbp)
    44b8:	00 00 00 00 
    44bc:	e9 a2 00 00 00       	jmp    4563 <gul_tensor_mul_simd+0x101>
    44c1:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    44c8:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    44cf:	00 
    44d0:	48 8b 85 78 ff ff ff 	mov    -0x88(%rbp),%rax
    44d7:	48 01 d0             	add    %rdx,%rax
    44da:	48 89 45 98          	mov    %rax,-0x68(%rbp)
    44de:	48 8b 45 98          	mov    -0x68(%rbp),%rax
    44e2:	0f 10 00             	movups (%rax),%xmm0
    44e5:	0f 29 45 a0          	movaps %xmm0,-0x60(%rbp)
    44e9:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    44f0:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    44f7:	00 
    44f8:	48 8b 45 80          	mov    -0x80(%rbp),%rax
    44fc:	48 01 d0             	add    %rdx,%rax
    44ff:	48 89 45 90          	mov    %rax,-0x70(%rbp)
    4503:	48 8b 45 90          	mov    -0x70(%rbp),%rax
    4507:	0f 10 00             	movups (%rax),%xmm0
    450a:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
    450e:	0f 28 45 a0          	movaps -0x60(%rbp),%xmm0
    4512:	0f 29 45 e0          	movaps %xmm0,-0x20(%rbp)
    4516:	0f 28 45 b0          	movaps -0x50(%rbp),%xmm0
    451a:	0f 29 45 f0          	movaps %xmm0,-0x10(%rbp)
    451e:	0f 28 45 e0          	movaps -0x20(%rbp),%xmm0
    4522:	0f 59 45 f0          	mulps  -0x10(%rbp),%xmm0
    4526:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
    452a:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    4531:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    4538:	00 
    4539:	48 8b 85 70 ff ff ff 	mov    -0x90(%rbp),%rax
    4540:	48 01 d0             	add    %rdx,%rax
    4543:	48 89 45 88          	mov    %rax,-0x78(%rbp)
    4547:	0f 28 45 c0          	movaps -0x40(%rbp),%xmm0
    454b:	0f 29 45 d0          	movaps %xmm0,-0x30(%rbp)
    454f:	0f 28 45 d0          	movaps -0x30(%rbp),%xmm0
    4553:	48 8b 45 88          	mov    -0x78(%rbp),%rax
    4557:	0f 11 00             	movups %xmm0,(%rax)
    455a:	90                   	nop
    455b:	48 83 85 68 ff ff ff 	addq   $0x4,-0x98(%rbp)
    4562:	04 
    4563:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    456a:	48 83 c0 03          	add    $0x3,%rax
    456e:	48 39 85 40 ff ff ff 	cmp    %rax,-0xc0(%rbp)
    4575:	0f 8f 46 ff ff ff    	jg     44c1 <gul_tensor_mul_simd+0x5f>
    457b:	eb 60                	jmp    45dd <gul_tensor_mul_simd+0x17b>
    457d:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    4584:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    458b:	00 
    458c:	48 8b 85 78 ff ff ff 	mov    -0x88(%rbp),%rax
    4593:	48 01 d0             	add    %rdx,%rax
    4596:	f3 0f 10 08          	movss  (%rax),%xmm1
    459a:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    45a1:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    45a8:	00 
    45a9:	48 8b 45 80          	mov    -0x80(%rbp),%rax
    45ad:	48 01 d0             	add    %rdx,%rax
    45b0:	f3 0f 10 00          	movss  (%rax),%xmm0
    45b4:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    45bb:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    45c2:	00 
    45c3:	48 8b 85 70 ff ff ff 	mov    -0x90(%rbp),%rax
    45ca:	48 01 d0             	add    %rdx,%rax
    45cd:	f3 0f 59 c1          	mulss  %xmm1,%xmm0
    45d1:	f3 0f 11 00          	movss  %xmm0,(%rax)
    45d5:	48 83 85 68 ff ff ff 	addq   $0x1,-0x98(%rbp)
    45dc:	01 
    45dd:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
    45e4:	48 3b 85 40 ff ff ff 	cmp    -0xc0(%rbp),%rax
    45eb:	7c 90                	jl     457d <gul_tensor_mul_simd+0x11b>
    45ed:	90                   	nop
    45ee:	90                   	nop
    45ef:	c9                   	leave
    45f0:	c3                   	ret

00000000000045f1 <gul_string_len>:
    45f1:	f3 0f 1e fa          	endbr64
    45f5:	55                   	push   %rbp
    45f6:	48 89 e5             	mov    %rsp,%rbp
    45f9:	48 83 ec 20          	sub    $0x20,%rsp
    45fd:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4601:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4605:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4609:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    460e:	75 07                	jne    4617 <gul_string_len+0x26>
    4610:	b8 00 00 00 00       	mov    $0x0,%eax
    4615:	eb 0c                	jmp    4623 <gul_string_len+0x32>
    4617:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    461b:	48 89 c7             	mov    %rax,%rdi
    461e:	e8 ad da ff ff       	call   20d0 <strlen@plt>
    4623:	c9                   	leave
    4624:	c3                   	ret

0000000000004625 <gul_string_substr>:
    4625:	f3 0f 1e fa          	endbr64
    4629:	55                   	push   %rbp
    462a:	48 89 e5             	mov    %rsp,%rbp
    462d:	48 83 ec 40          	sub    $0x40,%rsp
    4631:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    4635:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    4639:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
    463d:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    4641:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    4645:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    464a:	75 0c                	jne    4658 <gul_string_substr+0x33>
    464c:	48 8d 05 bd 19 00 00 	lea    0x19bd(%rip),%rax        # 6010 <_IO_stdin_used+0x10>
    4653:	e9 ce 00 00 00       	jmp    4726 <gul_string_substr+0x101>
    4658:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    465c:	48 89 c7             	mov    %rax,%rdi
    465f:	e8 6c da ff ff       	call   20d0 <strlen@plt>
    4664:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    4668:	48 83 7d d0 00       	cmpq   $0x0,-0x30(%rbp)
    466d:	78 0a                	js     4679 <gul_string_substr+0x54>
    466f:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    4673:	48 3b 45 f0          	cmp    -0x10(%rbp),%rax
    4677:	72 0c                	jb     4685 <gul_string_substr+0x60>
    4679:	48 8d 05 90 19 00 00 	lea    0x1990(%rip),%rax        # 6010 <_IO_stdin_used+0x10>
    4680:	e9 a1 00 00 00       	jmp    4726 <gul_string_substr+0x101>
    4685:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    4689:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    468d:	48 8b 55 d0          	mov    -0x30(%rbp),%rdx
    4691:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    4695:	48 01 d0             	add    %rdx,%rax
    4698:	48 39 45 f0          	cmp    %rax,-0x10(%rbp)
    469c:	73 0f                	jae    46ad <gul_string_substr+0x88>
    469e:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    46a2:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    46a6:	48 29 c2             	sub    %rax,%rdx
    46a9:	48 89 55 e0          	mov    %rdx,-0x20(%rbp)
    46ad:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    46b1:	48 83 c0 01          	add    $0x1,%rax
    46b5:	48 89 c7             	mov    %rax,%rdi
    46b8:	e8 03 db ff ff       	call   21c0 <malloc@plt>
    46bd:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    46c1:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    46c6:	75 2d                	jne    46f5 <gul_string_substr+0xd0>
    46c8:	48 8b 05 91 49 00 00 	mov    0x4991(%rip),%rax        # 9060 <stderr@GLIBC_2.2.5>
    46cf:	48 89 c1             	mov    %rax,%rcx
    46d2:	ba 25 00 00 00       	mov    $0x25,%edx
    46d7:	be 01 00 00 00       	mov    $0x1,%esi
    46dc:	48 8d 05 e5 1a 00 00 	lea    0x1ae5(%rip),%rax        # 61c8 <_IO_stdin_used+0x1c8>
    46e3:	48 89 c7             	mov    %rax,%rdi
    46e6:	e8 c5 db ff ff       	call   22b0 <fwrite@plt>
    46eb:	bf 01 00 00 00       	mov    $0x1,%edi
    46f0:	e8 ab db ff ff       	call   22a0 <exit@plt>
    46f5:	48 8b 55 d0          	mov    -0x30(%rbp),%rdx
    46f9:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    46fd:	48 8d 0c 02          	lea    (%rdx,%rax,1),%rcx
    4701:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    4705:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4709:	48 89 ce             	mov    %rcx,%rsi
    470c:	48 89 c7             	mov    %rax,%rdi
    470f:	e8 4c d9 ff ff       	call   2060 <strncpy@plt>
    4714:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    4718:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    471c:	48 01 d0             	add    %rdx,%rax
    471f:	c6 00 00             	movb   $0x0,(%rax)
    4722:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4726:	c9                   	leave
    4727:	c3                   	ret

0000000000004728 <gul_string_get>:
    4728:	f3 0f 1e fa          	endbr64
    472c:	55                   	push   %rbp
    472d:	48 89 e5             	mov    %rsp,%rbp
    4730:	48 83 ec 10          	sub    $0x10,%rsp
    4734:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    4738:	48 89 75 f0          	mov    %rsi,-0x10(%rbp)
    473c:	48 8b 4d f0          	mov    -0x10(%rbp),%rcx
    4740:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4744:	ba 01 00 00 00       	mov    $0x1,%edx
    4749:	48 89 ce             	mov    %rcx,%rsi
    474c:	48 89 c7             	mov    %rax,%rdi
    474f:	e8 d1 fe ff ff       	call   4625 <gul_string_substr>
    4754:	c9                   	leave
    4755:	c3                   	ret

0000000000004756 <gul_exec_foreign>:
    4756:	f3 0f 1e fa          	endbr64
    475a:	55                   	push   %rbp
    475b:	48 89 e5             	mov    %rsp,%rbp
    475e:	48 81 ec 40 04 00 00 	sub    $0x440,%rsp
    4765:	48 89 bd c8 fb ff ff 	mov    %rdi,-0x438(%rbp)
    476c:	48 89 b5 c0 fb ff ff 	mov    %rsi,-0x440(%rbp)
    4773:	64 48 8b 04 25 28 00 	mov    %fs:0x28,%rax
    477a:	00 00 
    477c:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4780:	31 c0                	xor    %eax,%eax
    4782:	48 8b 85 c8 fb ff ff 	mov    -0x438(%rbp),%rax
    4789:	48 89 85 e0 fb ff ff 	mov    %rax,-0x420(%rbp)
    4790:	48 8b 85 c0 fb ff ff 	mov    -0x440(%rbp),%rax
    4797:	48 89 85 e8 fb ff ff 	mov    %rax,-0x418(%rbp)
    479e:	48 83 bd e0 fb ff ff 	cmpq   $0x0,-0x420(%rbp)
    47a5:	00 
    47a6:	0f 84 f1 00 00 00    	je     489d <gul_exec_foreign+0x147>
    47ac:	48 83 bd e8 fb ff ff 	cmpq   $0x0,-0x418(%rbp)
    47b3:	00 
    47b4:	0f 84 e3 00 00 00    	je     489d <gul_exec_foreign+0x147>
    47ba:	48 8b 85 e0 fb ff ff 	mov    -0x420(%rbp),%rax
    47c1:	48 89 c6             	mov    %rax,%rsi
    47c4:	48 8d 05 25 1a 00 00 	lea    0x1a25(%rip),%rax        # 61f0 <_IO_stdin_used+0x1f0>
    47cb:	48 89 c7             	mov    %rax,%rdi
    47ce:	b8 00 00 00 00       	mov    $0x0,%eax
    47d3:	e8 28 d9 ff ff       	call   2100 <printf@plt>
    47d8:	48 8b 85 e0 fb ff ff 	mov    -0x420(%rbp),%rax
    47df:	48 8d 15 2f 1a 00 00 	lea    0x1a2f(%rip),%rdx        # 6215 <_IO_stdin_used+0x215>
    47e6:	48 89 d6             	mov    %rdx,%rsi
    47e9:	48 89 c7             	mov    %rax,%rdi
    47ec:	e8 8f d9 ff ff       	call   2180 <strcmp@plt>
    47f1:	85 c0                	test   %eax,%eax
    47f3:	0f 85 84 00 00 00    	jne    487d <gul_exec_foreign+0x127>
    47f9:	48 8b 95 e8 fb ff ff 	mov    -0x418(%rbp),%rdx
    4800:	48 8d 85 f0 fb ff ff 	lea    -0x410(%rbp),%rax
    4807:	48 89 d1             	mov    %rdx,%rcx
    480a:	48 8d 15 0b 1a 00 00 	lea    0x1a0b(%rip),%rdx        # 621c <_IO_stdin_used+0x21c>
    4811:	be 00 04 00 00       	mov    $0x400,%esi
    4816:	48 89 c7             	mov    %rax,%rdi
    4819:	b8 00 00 00 00       	mov    $0x0,%eax
    481e:	e8 ed d8 ff ff       	call   2110 <snprintf@plt>
    4823:	48 8d 85 f0 fb ff ff 	lea    -0x410(%rbp),%rax
    482a:	48 89 c6             	mov    %rax,%rsi
    482d:	48 8d 05 f8 19 00 00 	lea    0x19f8(%rip),%rax        # 622c <_IO_stdin_used+0x22c>
    4834:	48 89 c7             	mov    %rax,%rdi
    4837:	b8 00 00 00 00       	mov    $0x0,%eax
    483c:	e8 bf d8 ff ff       	call   2100 <printf@plt>
    4841:	48 8d 85 f0 fb ff ff 	lea    -0x410(%rbp),%rax
    4848:	48 89 c7             	mov    %rax,%rdi
    484b:	e8 a0 d8 ff ff       	call   20f0 <system@plt>
    4850:	89 85 dc fb ff ff    	mov    %eax,-0x424(%rbp)
    4856:	83 bd dc fb ff ff 00 	cmpl   $0x0,-0x424(%rbp)
    485d:	74 2d                	je     488c <gul_exec_foreign+0x136>
    485f:	8b 85 dc fb ff ff    	mov    -0x424(%rbp),%eax
    4865:	89 c6                	mov    %eax,%esi
    4867:	48 8d 05 cb 19 00 00 	lea    0x19cb(%rip),%rax        # 6239 <_IO_stdin_used+0x239>
    486e:	48 89 c7             	mov    %rax,%rdi
    4871:	b8 00 00 00 00       	mov    $0x0,%eax
    4876:	e8 85 d8 ff ff       	call   2100 <printf@plt>
    487b:	eb 0f                	jmp    488c <gul_exec_foreign+0x136>
    487d:	48 8b 85 e8 fb ff ff 	mov    -0x418(%rbp),%rax
    4884:	48 89 c7             	mov    %rax,%rdi
    4887:	e8 04 d8 ff ff       	call   2090 <puts@plt>
    488c:	48 8d 05 c4 19 00 00 	lea    0x19c4(%rip),%rax        # 6257 <_IO_stdin_used+0x257>
    4893:	48 89 c7             	mov    %rax,%rdi
    4896:	e8 f5 d7 ff ff       	call   2090 <puts@plt>
    489b:	eb 01                	jmp    489e <gul_exec_foreign+0x148>
    489d:	90                   	nop
    489e:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    48a2:	64 48 2b 04 25 28 00 	sub    %fs:0x28,%rax
    48a9:	00 00 
    48ab:	74 05                	je     48b2 <gul_exec_foreign+0x15c>
    48ad:	e8 2e d8 ff ff       	call   20e0 <__stack_chk_fail@plt>
    48b2:	c9                   	leave
    48b3:	c3                   	ret

00000000000048b4 <gul_table_alloc>:
    48b4:	f3 0f 1e fa          	endbr64
    48b8:	55                   	push   %rbp
    48b9:	48 89 e5             	mov    %rsp,%rbp
    48bc:	48 83 ec 20          	sub    $0x20,%rsp
    48c0:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    48c4:	48 89 75 e0          	mov    %rsi,-0x20(%rbp)
    48c8:	bf 18 00 00 00       	mov    $0x18,%edi
    48cd:	e8 ee d8 ff ff       	call   21c0 <malloc@plt>
    48d2:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    48d6:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    48db:	75 0a                	jne    48e7 <gul_table_alloc+0x33>
    48dd:	bf 01 00 00 00       	mov    $0x1,%edi
    48e2:	e8 b9 d9 ff ff       	call   22a0 <exit@plt>
    48e7:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    48eb:	89 c2                	mov    %eax,%edx
    48ed:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    48f1:	89 10                	mov    %edx,(%rax)
    48f3:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    48f7:	89 c2                	mov    %eax,%edx
    48f9:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    48fd:	89 50 04             	mov    %edx,0x4(%rax)
    4900:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4904:	48 c1 e0 03          	shl    $0x3,%rax
    4908:	48 89 c7             	mov    %rax,%rdi
    490b:	e8 b0 d8 ff ff       	call   21c0 <malloc@plt>
    4910:	48 89 c2             	mov    %rax,%rdx
    4913:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4917:	48 89 50 08          	mov    %rdx,0x8(%rax)
    491b:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    491f:	48 c1 e0 04          	shl    $0x4,%rax
    4923:	48 89 c7             	mov    %rax,%rdi
    4926:	e8 95 d8 ff ff       	call   21c0 <malloc@plt>
    492b:	48 89 c2             	mov    %rax,%rdx
    492e:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4932:	48 89 50 10          	mov    %rdx,0x10(%rax)
    4936:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    493a:	48 8b 40 08          	mov    0x8(%rax),%rax
    493e:	48 85 c0             	test   %rax,%rax
    4941:	74 0d                	je     4950 <gul_table_alloc+0x9c>
    4943:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4947:	48 8b 40 10          	mov    0x10(%rax),%rax
    494b:	48 85 c0             	test   %rax,%rax
    494e:	75 0a                	jne    495a <gul_table_alloc+0xa6>
    4950:	bf 01 00 00 00       	mov    $0x1,%edi
    4955:	e8 46 d9 ff ff       	call   22a0 <exit@plt>
    495a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    495e:	c9                   	leave
    495f:	c3                   	ret

0000000000004960 <gul_table_set_col_name>:
    4960:	f3 0f 1e fa          	endbr64
    4964:	55                   	push   %rbp
    4965:	48 89 e5             	mov    %rsp,%rbp
    4968:	53                   	push   %rbx
    4969:	48 83 ec 38          	sub    $0x38,%rsp
    496d:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    4971:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    4975:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
    4979:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    497d:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    4981:	48 83 7d d0 00       	cmpq   $0x0,-0x30(%rbp)
    4986:	78 31                	js     49b9 <gul_table_set_col_name+0x59>
    4988:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    498c:	8b 00                	mov    (%rax),%eax
    498e:	48 98                	cltq
    4990:	48 39 45 d0          	cmp    %rax,-0x30(%rbp)
    4994:	7d 23                	jge    49b9 <gul_table_set_col_name+0x59>
    4996:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    499a:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    499e:	48 8b 52 08          	mov    0x8(%rdx),%rdx
    49a2:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
    49a6:	48 c1 e1 03          	shl    $0x3,%rcx
    49aa:	48 8d 1c 0a          	lea    (%rdx,%rcx,1),%rbx
    49ae:	48 89 c7             	mov    %rax,%rdi
    49b1:	e8 1a d9 ff ff       	call   22d0 <strdup@plt>
    49b6:	48 89 03             	mov    %rax,(%rbx)
    49b9:	90                   	nop
    49ba:	48 8b 5d f8          	mov    -0x8(%rbp),%rbx
    49be:	c9                   	leave
    49bf:	c3                   	ret

00000000000049c0 <gul_table_set_row>:
    49c0:	f3 0f 1e fa          	endbr64
    49c4:	55                   	push   %rbp
    49c5:	48 89 e5             	mov    %rsp,%rbp
    49c8:	53                   	push   %rbx
    49c9:	48 83 ec 38          	sub    $0x38,%rsp
    49cd:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    49d1:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    49d5:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
    49d9:	48 89 4d c0          	mov    %rcx,-0x40(%rbp)
    49dd:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    49e1:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    49e5:	48 83 7d d0 00       	cmpq   $0x0,-0x30(%rbp)
    49ea:	78 4d                	js     4a39 <gul_table_set_row+0x79>
    49ec:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    49f0:	8b 40 04             	mov    0x4(%rax),%eax
    49f3:	48 98                	cltq
    49f5:	48 39 45 d0          	cmp    %rax,-0x30(%rbp)
    49f9:	7d 3e                	jge    4a39 <gul_table_set_row+0x79>
    49fb:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    49ff:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    4a03:	48 8b 52 10          	mov    0x10(%rdx),%rdx
    4a07:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
    4a0b:	48 c1 e1 04          	shl    $0x4,%rcx
    4a0f:	48 8d 1c 0a          	lea    (%rdx,%rcx,1),%rbx
    4a13:	48 89 c7             	mov    %rax,%rdi
    4a16:	e8 b5 d8 ff ff       	call   22d0 <strdup@plt>
    4a1b:	48 89 03             	mov    %rax,(%rbx)
    4a1e:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4a22:	48 8b 40 10          	mov    0x10(%rax),%rax
    4a26:	48 8b 55 d0          	mov    -0x30(%rbp),%rdx
    4a2a:	48 c1 e2 04          	shl    $0x4,%rdx
    4a2e:	48 01 c2             	add    %rax,%rdx
    4a31:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
    4a35:	48 89 42 08          	mov    %rax,0x8(%rdx)
    4a39:	90                   	nop
    4a3a:	48 8b 5d f8          	mov    -0x8(%rbp),%rbx
    4a3e:	c9                   	leave
    4a3f:	c3                   	ret

0000000000004a40 <gul_table_get_cell>:
    4a40:	f3 0f 1e fa          	endbr64
    4a44:	55                   	push   %rbp
    4a45:	48 89 e5             	mov    %rsp,%rbp
    4a48:	48 83 ec 40          	sub    $0x40,%rsp
    4a4c:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    4a50:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    4a54:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
    4a58:	64 48 8b 04 25 28 00 	mov    %fs:0x28,%rax
    4a5f:	00 00 
    4a61:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4a65:	31 c0                	xor    %eax,%eax
    4a67:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    4a6b:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    4a6f:	48 83 7d d0 00       	cmpq   $0x0,-0x30(%rbp)
    4a74:	78 60                	js     4ad6 <gul_table_get_cell+0x96>
    4a76:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4a7a:	8b 40 04             	mov    0x4(%rax),%eax
    4a7d:	48 98                	cltq
    4a7f:	48 39 45 d0          	cmp    %rax,-0x30(%rbp)
    4a83:	7d 51                	jge    4ad6 <gul_table_get_cell+0x96>
    4a85:	48 83 7d c8 00       	cmpq   $0x0,-0x38(%rbp)
    4a8a:	78 4a                	js     4ad6 <gul_table_get_cell+0x96>
    4a8c:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4a90:	8b 00                	mov    (%rax),%eax
    4a92:	48 98                	cltq
    4a94:	48 39 45 c8          	cmp    %rax,-0x38(%rbp)
    4a98:	7d 3c                	jge    4ad6 <gul_table_get_cell+0x96>
    4a9a:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4a9e:	48 8b 40 10          	mov    0x10(%rax),%rax
    4aa2:	48 8b 55 d0          	mov    -0x30(%rbp),%rdx
    4aa6:	48 c1 e2 04          	shl    $0x4,%rdx
    4aaa:	48 01 d0             	add    %rdx,%rax
    4aad:	48 8b 40 08          	mov    0x8(%rax),%rax
    4ab1:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
    4ab5:	48 c1 e2 03          	shl    $0x3,%rdx
    4ab9:	48 01 d0             	add    %rdx,%rax
    4abc:	f2 0f 10 00          	movsd  (%rax),%xmm0
    4ac0:	f2 0f 11 45 e0       	movsd  %xmm0,-0x20(%rbp)
    4ac5:	48 8d 45 e0          	lea    -0x20(%rbp),%rax
    4ac9:	48 89 45 f0          	mov    %rax,-0x10(%rbp)
    4acd:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    4ad1:	48 8b 00             	mov    (%rax),%rax
    4ad4:	eb 05                	jmp    4adb <gul_table_get_cell+0x9b>
    4ad6:	b8 00 00 00 00       	mov    $0x0,%eax
    4adb:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    4adf:	64 48 2b 14 25 28 00 	sub    %fs:0x28,%rdx
    4ae6:	00 00 
    4ae8:	74 05                	je     4aef <gul_table_get_cell+0xaf>
    4aea:	e8 f1 d5 ff ff       	call   20e0 <__stack_chk_fail@plt>
    4aef:	c9                   	leave
    4af0:	c3                   	ret

0000000000004af1 <gul_table_free>:
    4af1:	f3 0f 1e fa          	endbr64
    4af5:	55                   	push   %rbp
    4af6:	48 89 e5             	mov    %rsp,%rbp
    4af9:	48 83 ec 20          	sub    $0x20,%rsp
    4afd:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4b01:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4b05:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4b09:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4b0e:	0f 84 c0 00 00 00    	je     4bd4 <gul_table_free+0xe3>
    4b14:	c7 45 f0 00 00 00 00 	movl   $0x0,-0x10(%rbp)
    4b1b:	eb 24                	jmp    4b41 <gul_table_free+0x50>
    4b1d:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4b21:	48 8b 40 08          	mov    0x8(%rax),%rax
    4b25:	8b 55 f0             	mov    -0x10(%rbp),%edx
    4b28:	48 63 d2             	movslq %edx,%rdx
    4b2b:	48 c1 e2 03          	shl    $0x3,%rdx
    4b2f:	48 01 d0             	add    %rdx,%rax
    4b32:	48 8b 00             	mov    (%rax),%rax
    4b35:	48 89 c7             	mov    %rax,%rdi
    4b38:	e8 f3 d4 ff ff       	call   2030 <free@plt>
    4b3d:	83 45 f0 01          	addl   $0x1,-0x10(%rbp)
    4b41:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4b45:	8b 00                	mov    (%rax),%eax
    4b47:	39 45 f0             	cmp    %eax,-0x10(%rbp)
    4b4a:	7c d1                	jl     4b1d <gul_table_free+0x2c>
    4b4c:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4b50:	48 8b 40 08          	mov    0x8(%rax),%rax
    4b54:	48 89 c7             	mov    %rax,%rdi
    4b57:	e8 d4 d4 ff ff       	call   2030 <free@plt>
    4b5c:	c7 45 f4 00 00 00 00 	movl   $0x0,-0xc(%rbp)
    4b63:	eb 45                	jmp    4baa <gul_table_free+0xb9>
    4b65:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4b69:	48 8b 40 10          	mov    0x10(%rax),%rax
    4b6d:	8b 55 f4             	mov    -0xc(%rbp),%edx
    4b70:	48 63 d2             	movslq %edx,%rdx
    4b73:	48 c1 e2 04          	shl    $0x4,%rdx
    4b77:	48 01 d0             	add    %rdx,%rax
    4b7a:	48 8b 00             	mov    (%rax),%rax
    4b7d:	48 89 c7             	mov    %rax,%rdi
    4b80:	e8 ab d4 ff ff       	call   2030 <free@plt>
    4b85:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4b89:	48 8b 40 10          	mov    0x10(%rax),%rax
    4b8d:	8b 55 f4             	mov    -0xc(%rbp),%edx
    4b90:	48 63 d2             	movslq %edx,%rdx
    4b93:	48 c1 e2 04          	shl    $0x4,%rdx
    4b97:	48 01 d0             	add    %rdx,%rax
    4b9a:	48 8b 40 08          	mov    0x8(%rax),%rax
    4b9e:	48 89 c7             	mov    %rax,%rdi
    4ba1:	e8 8a d4 ff ff       	call   2030 <free@plt>
    4ba6:	83 45 f4 01          	addl   $0x1,-0xc(%rbp)
    4baa:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4bae:	8b 40 04             	mov    0x4(%rax),%eax
    4bb1:	39 45 f4             	cmp    %eax,-0xc(%rbp)
    4bb4:	7c af                	jl     4b65 <gul_table_free+0x74>
    4bb6:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4bba:	48 8b 40 10          	mov    0x10(%rax),%rax
    4bbe:	48 89 c7             	mov    %rax,%rdi
    4bc1:	e8 6a d4 ff ff       	call   2030 <free@plt>
    4bc6:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4bca:	48 89 c7             	mov    %rax,%rdi
    4bcd:	e8 5e d4 ff ff       	call   2030 <free@plt>
    4bd2:	eb 01                	jmp    4bd5 <gul_table_free+0xe4>
    4bd4:	90                   	nop
    4bd5:	c9                   	leave
    4bd6:	c3                   	ret

0000000000004bd7 <gul_list_alloc>:
    4bd7:	f3 0f 1e fa          	endbr64
    4bdb:	55                   	push   %rbp
    4bdc:	48 89 e5             	mov    %rsp,%rbp
    4bdf:	48 83 ec 20          	sub    $0x20,%rsp
    4be3:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4be7:	bf 18 00 00 00       	mov    $0x18,%edi
    4bec:	e8 cf d5 ff ff       	call   21c0 <malloc@plt>
    4bf1:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4bf5:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4bfa:	75 0a                	jne    4c06 <gul_list_alloc+0x2f>
    4bfc:	bf 01 00 00 00       	mov    $0x1,%edi
    4c01:	e8 9a d6 ff ff       	call   22a0 <exit@plt>
    4c06:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    4c0b:	7e 06                	jle    4c13 <gul_list_alloc+0x3c>
    4c0d:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4c11:	eb 05                	jmp    4c18 <gul_list_alloc+0x41>
    4c13:	b8 08 00 00 00       	mov    $0x8,%eax
    4c18:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    4c1c:	48 89 42 10          	mov    %rax,0x10(%rdx)
    4c20:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4c24:	48 c7 40 08 00 00 00 	movq   $0x0,0x8(%rax)
    4c2b:	00 
    4c2c:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4c30:	48 8b 40 10          	mov    0x10(%rax),%rax
    4c34:	48 c1 e0 03          	shl    $0x3,%rax
    4c38:	48 89 c7             	mov    %rax,%rdi
    4c3b:	e8 80 d5 ff ff       	call   21c0 <malloc@plt>
    4c40:	48 89 c2             	mov    %rax,%rdx
    4c43:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4c47:	48 89 10             	mov    %rdx,(%rax)
    4c4a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4c4e:	48 8b 00             	mov    (%rax),%rax
    4c51:	48 85 c0             	test   %rax,%rax
    4c54:	75 0a                	jne    4c60 <gul_list_alloc+0x89>
    4c56:	bf 01 00 00 00       	mov    $0x1,%edi
    4c5b:	e8 40 d6 ff ff       	call   22a0 <exit@plt>
    4c60:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4c64:	c9                   	leave
    4c65:	c3                   	ret

0000000000004c66 <gul_list_free>:
    4c66:	f3 0f 1e fa          	endbr64
    4c6a:	55                   	push   %rbp
    4c6b:	48 89 e5             	mov    %rsp,%rbp
    4c6e:	48 83 ec 20          	sub    $0x20,%rsp
    4c72:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4c76:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4c7a:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4c7e:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4c83:	74 1b                	je     4ca0 <gul_list_free+0x3a>
    4c85:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4c89:	48 8b 00             	mov    (%rax),%rax
    4c8c:	48 89 c7             	mov    %rax,%rdi
    4c8f:	e8 9c d3 ff ff       	call   2030 <free@plt>
    4c94:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4c98:	48 89 c7             	mov    %rax,%rdi
    4c9b:	e8 90 d3 ff ff       	call   2030 <free@plt>
    4ca0:	90                   	nop
    4ca1:	c9                   	leave
    4ca2:	c3                   	ret

0000000000004ca3 <gul_list_len>:
    4ca3:	f3 0f 1e fa          	endbr64
    4ca7:	55                   	push   %rbp
    4ca8:	48 89 e5             	mov    %rsp,%rbp
    4cab:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4caf:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4cb3:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4cb7:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4cbc:	74 0a                	je     4cc8 <gul_list_len+0x25>
    4cbe:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4cc2:	48 8b 40 08          	mov    0x8(%rax),%rax
    4cc6:	eb 05                	jmp    4ccd <gul_list_len+0x2a>
    4cc8:	b8 00 00 00 00       	mov    $0x0,%eax
    4ccd:	5d                   	pop    %rbp
    4cce:	c3                   	ret

0000000000004ccf <gul_list_push>:
    4ccf:	f3 0f 1e fa          	endbr64
    4cd3:	55                   	push   %rbp
    4cd4:	48 89 e5             	mov    %rsp,%rbp
    4cd7:	48 83 ec 20          	sub    $0x20,%rsp
    4cdb:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4cdf:	48 89 75 e0          	mov    %rsi,-0x20(%rbp)
    4ce3:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4ce7:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4ceb:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4cf0:	0f 84 94 00 00 00    	je     4d8a <gul_list_push+0xbb>
    4cf6:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4cfa:	48 8b 50 08          	mov    0x8(%rax),%rdx
    4cfe:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4d02:	48 8b 40 10          	mov    0x10(%rax),%rax
    4d06:	48 39 c2             	cmp    %rax,%rdx
    4d09:	7c 53                	jl     4d5e <gul_list_push+0x8f>
    4d0b:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4d0f:	48 8b 40 10          	mov    0x10(%rax),%rax
    4d13:	48 8d 14 00          	lea    (%rax,%rax,1),%rdx
    4d17:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4d1b:	48 89 50 10          	mov    %rdx,0x10(%rax)
    4d1f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4d23:	48 8b 40 10          	mov    0x10(%rax),%rax
    4d27:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    4d2e:	00 
    4d2f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4d33:	48 8b 00             	mov    (%rax),%rax
    4d36:	48 89 d6             	mov    %rdx,%rsi
    4d39:	48 89 c7             	mov    %rax,%rdi
    4d3c:	e8 af d4 ff ff       	call   21f0 <realloc@plt>
    4d41:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    4d45:	48 89 02             	mov    %rax,(%rdx)
    4d48:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4d4c:	48 8b 00             	mov    (%rax),%rax
    4d4f:	48 85 c0             	test   %rax,%rax
    4d52:	75 0a                	jne    4d5e <gul_list_push+0x8f>
    4d54:	bf 01 00 00 00       	mov    $0x1,%edi
    4d59:	e8 42 d5 ff ff       	call   22a0 <exit@plt>
    4d5e:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4d62:	48 8b 30             	mov    (%rax),%rsi
    4d65:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4d69:	48 8b 40 08          	mov    0x8(%rax),%rax
    4d6d:	48 8d 48 01          	lea    0x1(%rax),%rcx
    4d71:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    4d75:	48 89 4a 08          	mov    %rcx,0x8(%rdx)
    4d79:	48 c1 e0 03          	shl    $0x3,%rax
    4d7d:	48 8d 14 06          	lea    (%rsi,%rax,1),%rdx
    4d81:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    4d85:	48 89 02             	mov    %rax,(%rdx)
    4d88:	eb 01                	jmp    4d8b <gul_list_push+0xbc>
    4d8a:	90                   	nop
    4d8b:	c9                   	leave
    4d8c:	c3                   	ret

0000000000004d8d <gul_list_pop>:
    4d8d:	f3 0f 1e fa          	endbr64
    4d91:	55                   	push   %rbp
    4d92:	48 89 e5             	mov    %rsp,%rbp
    4d95:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4d99:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4d9d:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4da1:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4da6:	74 0d                	je     4db5 <gul_list_pop+0x28>
    4da8:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4dac:	48 8b 40 08          	mov    0x8(%rax),%rax
    4db0:	48 85 c0             	test   %rax,%rax
    4db3:	75 07                	jne    4dbc <gul_list_pop+0x2f>
    4db5:	b8 00 00 00 00       	mov    $0x0,%eax
    4dba:	eb 2d                	jmp    4de9 <gul_list_pop+0x5c>
    4dbc:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4dc0:	48 8b 10             	mov    (%rax),%rdx
    4dc3:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4dc7:	48 8b 40 08          	mov    0x8(%rax),%rax
    4dcb:	48 8d 48 ff          	lea    -0x1(%rax),%rcx
    4dcf:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4dd3:	48 89 48 08          	mov    %rcx,0x8(%rax)
    4dd7:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4ddb:	48 8b 40 08          	mov    0x8(%rax),%rax
    4ddf:	48 c1 e0 03          	shl    $0x3,%rax
    4de3:	48 01 d0             	add    %rdx,%rax
    4de6:	48 8b 00             	mov    (%rax),%rax
    4de9:	5d                   	pop    %rbp
    4dea:	c3                   	ret

0000000000004deb <gul_list_get>:
    4deb:	f3 0f 1e fa          	endbr64
    4def:	55                   	push   %rbp
    4df0:	48 89 e5             	mov    %rsp,%rbp
    4df3:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4df7:	48 89 75 e0          	mov    %rsi,-0x20(%rbp)
    4dfb:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4dff:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4e03:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4e08:	74 15                	je     4e1f <gul_list_get+0x34>
    4e0a:	48 83 7d e0 00       	cmpq   $0x0,-0x20(%rbp)
    4e0f:	78 0e                	js     4e1f <gul_list_get+0x34>
    4e11:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4e15:	48 8b 40 08          	mov    0x8(%rax),%rax
    4e19:	48 39 45 e0          	cmp    %rax,-0x20(%rbp)
    4e1d:	7c 07                	jl     4e26 <gul_list_get+0x3b>
    4e1f:	b8 00 00 00 00       	mov    $0x0,%eax
    4e24:	eb 15                	jmp    4e3b <gul_list_get+0x50>
    4e26:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4e2a:	48 8b 00             	mov    (%rax),%rax
    4e2d:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    4e31:	48 c1 e2 03          	shl    $0x3,%rdx
    4e35:	48 01 d0             	add    %rdx,%rax
    4e38:	48 8b 00             	mov    (%rax),%rax
    4e3b:	5d                   	pop    %rbp
    4e3c:	c3                   	ret

0000000000004e3d <gul_list_set>:
    4e3d:	f3 0f 1e fa          	endbr64
    4e41:	55                   	push   %rbp
    4e42:	48 89 e5             	mov    %rsp,%rbp
    4e45:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4e49:	48 89 75 e0          	mov    %rsi,-0x20(%rbp)
    4e4d:	48 89 55 d8          	mov    %rdx,-0x28(%rbp)
    4e51:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4e55:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4e59:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4e5e:	74 30                	je     4e90 <gul_list_set+0x53>
    4e60:	48 83 7d e0 00       	cmpq   $0x0,-0x20(%rbp)
    4e65:	78 29                	js     4e90 <gul_list_set+0x53>
    4e67:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4e6b:	48 8b 40 08          	mov    0x8(%rax),%rax
    4e6f:	48 39 45 e0          	cmp    %rax,-0x20(%rbp)
    4e73:	7d 1b                	jge    4e90 <gul_list_set+0x53>
    4e75:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4e79:	48 8b 00             	mov    (%rax),%rax
    4e7c:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    4e80:	48 c1 e2 03          	shl    $0x3,%rdx
    4e84:	48 01 c2             	add    %rax,%rdx
    4e87:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    4e8b:	48 89 02             	mov    %rax,(%rdx)
    4e8e:	eb 01                	jmp    4e91 <gul_list_set+0x54>
    4e90:	90                   	nop
    4e91:	5d                   	pop    %rbp
    4e92:	c3                   	ret

0000000000004e93 <gul_list_clear>:
    4e93:	f3 0f 1e fa          	endbr64
    4e97:	55                   	push   %rbp
    4e98:	48 89 e5             	mov    %rsp,%rbp
    4e9b:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4e9f:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4ea3:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4ea7:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4eac:	74 0c                	je     4eba <gul_list_clear+0x27>
    4eae:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4eb2:	48 c7 40 08 00 00 00 	movq   $0x0,0x8(%rax)
    4eb9:	00 
    4eba:	90                   	nop
    4ebb:	5d                   	pop    %rbp
    4ebc:	c3                   	ret

0000000000004ebd <gul_list_contains>:
    4ebd:	f3 0f 1e fa          	endbr64
    4ec1:	55                   	push   %rbp
    4ec2:	48 89 e5             	mov    %rsp,%rbp
    4ec5:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4ec9:	48 89 75 e0          	mov    %rsi,-0x20(%rbp)
    4ecd:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4ed1:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4ed5:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4eda:	75 07                	jne    4ee3 <gul_list_contains+0x26>
    4edc:	b8 00 00 00 00       	mov    $0x0,%eax
    4ee1:	eb 44                	jmp    4f27 <gul_list_contains+0x6a>
    4ee3:	48 c7 45 f0 00 00 00 	movq   $0x0,-0x10(%rbp)
    4eea:	00 
    4eeb:	eb 27                	jmp    4f14 <gul_list_contains+0x57>
    4eed:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4ef1:	48 8b 00             	mov    (%rax),%rax
    4ef4:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    4ef8:	48 c1 e2 03          	shl    $0x3,%rdx
    4efc:	48 01 d0             	add    %rdx,%rax
    4eff:	48 8b 00             	mov    (%rax),%rax
    4f02:	48 39 45 e0          	cmp    %rax,-0x20(%rbp)
    4f06:	75 07                	jne    4f0f <gul_list_contains+0x52>
    4f08:	b8 01 00 00 00       	mov    $0x1,%eax
    4f0d:	eb 18                	jmp    4f27 <gul_list_contains+0x6a>
    4f0f:	48 83 45 f0 01       	addq   $0x1,-0x10(%rbp)
    4f14:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4f18:	48 8b 40 08          	mov    0x8(%rax),%rax
    4f1c:	48 39 45 f0          	cmp    %rax,-0x10(%rbp)
    4f20:	7c cb                	jl     4eed <gul_list_contains+0x30>
    4f22:	b8 00 00 00 00       	mov    $0x0,%eax
    4f27:	5d                   	pop    %rbp
    4f28:	c3                   	ret

0000000000004f29 <gul_list_insert_before>:
    4f29:	f3 0f 1e fa          	endbr64
    4f2d:	55                   	push   %rbp
    4f2e:	48 89 e5             	mov    %rsp,%rbp
    4f31:	48 83 ec 30          	sub    $0x30,%rsp
    4f35:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    4f39:	48 89 75 e0          	mov    %rsi,-0x20(%rbp)
    4f3d:	48 89 55 d8          	mov    %rdx,-0x28(%rbp)
    4f41:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    4f45:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    4f49:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    4f4e:	0f 84 e5 00 00 00    	je     5039 <gul_list_insert_before+0x110>
    4f54:	48 83 7d e0 00       	cmpq   $0x0,-0x20(%rbp)
    4f59:	0f 88 da 00 00 00    	js     5039 <gul_list_insert_before+0x110>
    4f5f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4f63:	48 8b 40 08          	mov    0x8(%rax),%rax
    4f67:	48 39 45 e0          	cmp    %rax,-0x20(%rbp)
    4f6b:	0f 8f c8 00 00 00    	jg     5039 <gul_list_insert_before+0x110>
    4f71:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4f75:	48 8b 50 08          	mov    0x8(%rax),%rdx
    4f79:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4f7d:	48 8b 40 10          	mov    0x10(%rax),%rax
    4f81:	48 39 c2             	cmp    %rax,%rdx
    4f84:	7c 3d                	jl     4fc3 <gul_list_insert_before+0x9a>
    4f86:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4f8a:	48 8b 40 10          	mov    0x10(%rax),%rax
    4f8e:	48 8d 14 00          	lea    (%rax,%rax,1),%rdx
    4f92:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4f96:	48 89 50 10          	mov    %rdx,0x10(%rax)
    4f9a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4f9e:	48 8b 40 10          	mov    0x10(%rax),%rax
    4fa2:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    4fa9:	00 
    4faa:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4fae:	48 8b 00             	mov    (%rax),%rax
    4fb1:	48 89 d6             	mov    %rdx,%rsi
    4fb4:	48 89 c7             	mov    %rax,%rdi
    4fb7:	e8 34 d2 ff ff       	call   21f0 <realloc@plt>
    4fbc:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    4fc0:	48 89 02             	mov    %rax,(%rdx)
    4fc3:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4fc7:	48 8b 40 08          	mov    0x8(%rax),%rax
    4fcb:	48 2b 45 e0          	sub    -0x20(%rbp),%rax
    4fcf:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    4fd6:	00 
    4fd7:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4fdb:	48 8b 00             	mov    (%rax),%rax
    4fde:	48 8b 4d e0          	mov    -0x20(%rbp),%rcx
    4fe2:	48 c1 e1 03          	shl    $0x3,%rcx
    4fe6:	48 01 c1             	add    %rax,%rcx
    4fe9:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    4fed:	48 8b 00             	mov    (%rax),%rax
    4ff0:	48 8b 75 e0          	mov    -0x20(%rbp),%rsi
    4ff4:	48 83 c6 01          	add    $0x1,%rsi
    4ff8:	48 c1 e6 03          	shl    $0x3,%rsi
    4ffc:	48 01 f0             	add    %rsi,%rax
    4fff:	48 89 ce             	mov    %rcx,%rsi
    5002:	48 89 c7             	mov    %rax,%rdi
    5005:	e8 f6 d1 ff ff       	call   2200 <memmove@plt>
    500a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    500e:	48 8b 00             	mov    (%rax),%rax
    5011:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    5015:	48 c1 e2 03          	shl    $0x3,%rdx
    5019:	48 01 c2             	add    %rax,%rdx
    501c:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    5020:	48 89 02             	mov    %rax,(%rdx)
    5023:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5027:	48 8b 40 08          	mov    0x8(%rax),%rax
    502b:	48 8d 50 01          	lea    0x1(%rax),%rdx
    502f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5033:	48 89 50 08          	mov    %rdx,0x8(%rax)
    5037:	eb 01                	jmp    503a <gul_list_insert_before+0x111>
    5039:	90                   	nop
    503a:	c9                   	leave
    503b:	c3                   	ret

000000000000503c <gul_list_insert_after>:
    503c:	f3 0f 1e fa          	endbr64
    5040:	55                   	push   %rbp
    5041:	48 89 e5             	mov    %rsp,%rbp
    5044:	48 83 ec 20          	sub    $0x20,%rsp
    5048:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    504c:	48 89 75 f0          	mov    %rsi,-0x10(%rbp)
    5050:	48 89 55 e8          	mov    %rdx,-0x18(%rbp)
    5054:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    5058:	48 8d 48 01          	lea    0x1(%rax),%rcx
    505c:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5060:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5064:	48 89 ce             	mov    %rcx,%rsi
    5067:	48 89 c7             	mov    %rax,%rdi
    506a:	e8 ba fe ff ff       	call   4f29 <gul_list_insert_before>
    506f:	90                   	nop
    5070:	c9                   	leave
    5071:	c3                   	ret

0000000000005072 <gul_list_remove>:
    5072:	f3 0f 1e fa          	endbr64
    5076:	55                   	push   %rbp
    5077:	48 89 e5             	mov    %rsp,%rbp
    507a:	48 83 ec 20          	sub    $0x20,%rsp
    507e:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    5082:	48 89 75 e0          	mov    %rsi,-0x20(%rbp)
    5086:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    508a:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    508e:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    5093:	74 76                	je     510b <gul_list_remove+0x99>
    5095:	48 83 7d e0 00       	cmpq   $0x0,-0x20(%rbp)
    509a:	78 6f                	js     510b <gul_list_remove+0x99>
    509c:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    50a0:	48 8b 40 08          	mov    0x8(%rax),%rax
    50a4:	48 39 45 e0          	cmp    %rax,-0x20(%rbp)
    50a8:	7d 61                	jge    510b <gul_list_remove+0x99>
    50aa:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    50ae:	48 8b 40 08          	mov    0x8(%rax),%rax
    50b2:	48 2b 45 e0          	sub    -0x20(%rbp),%rax
    50b6:	48 83 e8 01          	sub    $0x1,%rax
    50ba:	48 8d 14 c5 00 00 00 	lea    0x0(,%rax,8),%rdx
    50c1:	00 
    50c2:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    50c6:	48 8b 00             	mov    (%rax),%rax
    50c9:	48 8b 4d e0          	mov    -0x20(%rbp),%rcx
    50cd:	48 83 c1 01          	add    $0x1,%rcx
    50d1:	48 c1 e1 03          	shl    $0x3,%rcx
    50d5:	48 01 c1             	add    %rax,%rcx
    50d8:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    50dc:	48 8b 00             	mov    (%rax),%rax
    50df:	48 8b 75 e0          	mov    -0x20(%rbp),%rsi
    50e3:	48 c1 e6 03          	shl    $0x3,%rsi
    50e7:	48 01 f0             	add    %rsi,%rax
    50ea:	48 89 ce             	mov    %rcx,%rsi
    50ed:	48 89 c7             	mov    %rax,%rdi
    50f0:	e8 0b d1 ff ff       	call   2200 <memmove@plt>
    50f5:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    50f9:	48 8b 40 08          	mov    0x8(%rax),%rax
    50fd:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
    5101:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5105:	48 89 50 08          	mov    %rdx,0x8(%rax)
    5109:	eb 01                	jmp    510c <gul_list_remove+0x9a>
    510b:	90                   	nop
    510c:	c9                   	leave
    510d:	c3                   	ret

000000000000510e <dict_hash>:
    510e:	f3 0f 1e fa          	endbr64
    5112:	55                   	push   %rbp
    5113:	48 89 e5             	mov    %rsp,%rbp
    5116:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    511a:	48 c7 45 f8 05 15 00 	movq   $0x1505,-0x8(%rbp)
    5121:	00 
    5122:	eb 2d                	jmp    5151 <dict_hash+0x43>
    5124:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5128:	48 c1 e0 05          	shl    $0x5,%rax
    512c:	48 89 c2             	mov    %rax,%rdx
    512f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5133:	48 8d 0c 02          	lea    (%rdx,%rax,1),%rcx
    5137:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    513b:	48 8d 50 01          	lea    0x1(%rax),%rdx
    513f:	48 89 55 e8          	mov    %rdx,-0x18(%rbp)
    5143:	0f b6 00             	movzbl (%rax),%eax
    5146:	48 0f be c0          	movsbq %al,%rax
    514a:	48 01 c8             	add    %rcx,%rax
    514d:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5151:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5155:	0f b6 00             	movzbl (%rax),%eax
    5158:	84 c0                	test   %al,%al
    515a:	75 c8                	jne    5124 <dict_hash+0x16>
    515c:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5160:	5d                   	pop    %rbp
    5161:	c3                   	ret

0000000000005162 <gul_dict_alloc>:
    5162:	f3 0f 1e fa          	endbr64
    5166:	55                   	push   %rbp
    5167:	48 89 e5             	mov    %rsp,%rbp
    516a:	48 83 ec 20          	sub    $0x20,%rsp
    516e:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    5172:	bf 18 00 00 00       	mov    $0x18,%edi
    5177:	e8 44 d0 ff ff       	call   21c0 <malloc@plt>
    517c:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5180:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    5185:	75 0a                	jne    5191 <gul_dict_alloc+0x2f>
    5187:	bf 01 00 00 00       	mov    $0x1,%edi
    518c:	e8 0f d1 ff ff       	call   22a0 <exit@plt>
    5191:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    5196:	7e 06                	jle    519e <gul_dict_alloc+0x3c>
    5198:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    519c:	eb 05                	jmp    51a3 <gul_dict_alloc+0x41>
    519e:	b8 10 00 00 00       	mov    $0x10,%eax
    51a3:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    51a7:	48 89 42 08          	mov    %rax,0x8(%rdx)
    51ab:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    51af:	48 c7 40 10 00 00 00 	movq   $0x0,0x10(%rax)
    51b6:	00 
    51b7:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    51bb:	48 8b 40 08          	mov    0x8(%rax),%rax
    51bf:	be 18 00 00 00       	mov    $0x18,%esi
    51c4:	48 89 c7             	mov    %rax,%rdi
    51c7:	e8 a4 cf ff ff       	call   2170 <calloc@plt>
    51cc:	48 89 c2             	mov    %rax,%rdx
    51cf:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    51d3:	48 89 10             	mov    %rdx,(%rax)
    51d6:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    51da:	48 8b 00             	mov    (%rax),%rax
    51dd:	48 85 c0             	test   %rax,%rax
    51e0:	75 0a                	jne    51ec <gul_dict_alloc+0x8a>
    51e2:	bf 01 00 00 00       	mov    $0x1,%edi
    51e7:	e8 b4 d0 ff ff       	call   22a0 <exit@plt>
    51ec:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    51f0:	c9                   	leave
    51f1:	c3                   	ret

00000000000051f2 <gul_dict_free>:
    51f2:	f3 0f 1e fa          	endbr64
    51f6:	55                   	push   %rbp
    51f7:	48 89 e5             	mov    %rsp,%rbp
    51fa:	48 83 ec 20          	sub    $0x20,%rsp
    51fe:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    5202:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5206:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    520a:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    520f:	0f 84 80 00 00 00    	je     5295 <gul_dict_free+0xa3>
    5215:	48 c7 45 f0 00 00 00 	movq   $0x0,-0x10(%rbp)
    521c:	00 
    521d:	eb 4d                	jmp    526c <gul_dict_free+0x7a>
    521f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5223:	48 8b 08             	mov    (%rax),%rcx
    5226:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    522a:	48 89 d0             	mov    %rdx,%rax
    522d:	48 01 c0             	add    %rax,%rax
    5230:	48 01 d0             	add    %rdx,%rax
    5233:	48 c1 e0 03          	shl    $0x3,%rax
    5237:	48 01 c8             	add    %rcx,%rax
    523a:	8b 40 10             	mov    0x10(%rax),%eax
    523d:	85 c0                	test   %eax,%eax
    523f:	74 26                	je     5267 <gul_dict_free+0x75>
    5241:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5245:	48 8b 08             	mov    (%rax),%rcx
    5248:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    524c:	48 89 d0             	mov    %rdx,%rax
    524f:	48 01 c0             	add    %rax,%rax
    5252:	48 01 d0             	add    %rdx,%rax
    5255:	48 c1 e0 03          	shl    $0x3,%rax
    5259:	48 01 c8             	add    %rcx,%rax
    525c:	48 8b 00             	mov    (%rax),%rax
    525f:	48 89 c7             	mov    %rax,%rdi
    5262:	e8 c9 cd ff ff       	call   2030 <free@plt>
    5267:	48 83 45 f0 01       	addq   $0x1,-0x10(%rbp)
    526c:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5270:	48 8b 40 08          	mov    0x8(%rax),%rax
    5274:	48 39 45 f0          	cmp    %rax,-0x10(%rbp)
    5278:	7c a5                	jl     521f <gul_dict_free+0x2d>
    527a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    527e:	48 8b 00             	mov    (%rax),%rax
    5281:	48 89 c7             	mov    %rax,%rdi
    5284:	e8 a7 cd ff ff       	call   2030 <free@plt>
    5289:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    528d:	48 89 c7             	mov    %rax,%rdi
    5290:	e8 9b cd ff ff       	call   2030 <free@plt>
    5295:	90                   	nop
    5296:	c9                   	leave
    5297:	c3                   	ret

0000000000005298 <gul_dict_len>:
    5298:	f3 0f 1e fa          	endbr64
    529c:	55                   	push   %rbp
    529d:	48 89 e5             	mov    %rsp,%rbp
    52a0:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    52a4:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    52a8:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    52ac:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    52b1:	74 0a                	je     52bd <gul_dict_len+0x25>
    52b3:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    52b7:	48 8b 40 10          	mov    0x10(%rax),%rax
    52bb:	eb 05                	jmp    52c2 <gul_dict_len+0x2a>
    52bd:	b8 00 00 00 00       	mov    $0x0,%eax
    52c2:	5d                   	pop    %rbp
    52c3:	c3                   	ret

00000000000052c4 <gul_dict_set>:
    52c4:	f3 0f 1e fa          	endbr64
    52c8:	55                   	push   %rbp
    52c9:	48 89 e5             	mov    %rsp,%rbp
    52cc:	53                   	push   %rbx
    52cd:	48 83 ec 58          	sub    $0x58,%rsp
    52d1:	48 89 7d b8          	mov    %rdi,-0x48(%rbp)
    52d5:	48 89 75 b0          	mov    %rsi,-0x50(%rbp)
    52d9:	48 89 55 a8          	mov    %rdx,-0x58(%rbp)
    52dd:	48 8b 45 b8          	mov    -0x48(%rbp),%rax
    52e1:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
    52e5:	48 83 7d d0 00       	cmpq   $0x0,-0x30(%rbp)
    52ea:	0f 84 7d 01 00 00    	je     546d <gul_dict_set+0x1a9>
    52f0:	48 8b 45 b0          	mov    -0x50(%rbp),%rax
    52f4:	48 89 45 d8          	mov    %rax,-0x28(%rbp)
    52f8:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    52fc:	48 89 c7             	mov    %rax,%rdi
    52ff:	e8 0a fe ff ff       	call   510e <dict_hash>
    5304:	48 8b 55 d0          	mov    -0x30(%rbp),%rdx
    5308:	48 8b 52 08          	mov    0x8(%rdx),%rdx
    530c:	48 89 d1             	mov    %rdx,%rcx
    530f:	ba 00 00 00 00       	mov    $0x0,%edx
    5314:	48 f7 f1             	div    %rcx
    5317:	48 89 55 e0          	mov    %rdx,-0x20(%rbp)
    531b:	48 c7 45 c8 00 00 00 	movq   $0x0,-0x38(%rbp)
    5322:	00 
    5323:	e9 31 01 00 00       	jmp    5459 <gul_dict_set+0x195>
    5328:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
    532c:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5330:	48 01 c2             	add    %rax,%rdx
    5333:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    5337:	48 8b 40 08          	mov    0x8(%rax),%rax
    533b:	48 89 c1             	mov    %rax,%rcx
    533e:	48 89 d0             	mov    %rdx,%rax
    5341:	ba 00 00 00 00       	mov    $0x0,%edx
    5346:	48 f7 f1             	div    %rcx
    5349:	48 89 d0             	mov    %rdx,%rax
    534c:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    5350:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    5354:	48 8b 08             	mov    (%rax),%rcx
    5357:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    535b:	48 89 d0             	mov    %rdx,%rax
    535e:	48 01 c0             	add    %rax,%rax
    5361:	48 01 d0             	add    %rdx,%rax
    5364:	48 c1 e0 03          	shl    $0x3,%rax
    5368:	48 01 c8             	add    %rcx,%rax
    536b:	8b 40 10             	mov    0x10(%rax),%eax
    536e:	85 c0                	test   %eax,%eax
    5370:	0f 85 87 00 00 00    	jne    53fd <gul_dict_set+0x139>
    5376:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    537a:	48 8b 08             	mov    (%rax),%rcx
    537d:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5381:	48 89 d0             	mov    %rdx,%rax
    5384:	48 01 c0             	add    %rax,%rax
    5387:	48 01 d0             	add    %rdx,%rax
    538a:	48 c1 e0 03          	shl    $0x3,%rax
    538e:	48 8d 1c 01          	lea    (%rcx,%rax,1),%rbx
    5392:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    5396:	48 89 c7             	mov    %rax,%rdi
    5399:	e8 32 cf ff ff       	call   22d0 <strdup@plt>
    539e:	48 89 03             	mov    %rax,(%rbx)
    53a1:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    53a5:	48 8b 08             	mov    (%rax),%rcx
    53a8:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    53ac:	48 89 d0             	mov    %rdx,%rax
    53af:	48 01 c0             	add    %rax,%rax
    53b2:	48 01 d0             	add    %rdx,%rax
    53b5:	48 c1 e0 03          	shl    $0x3,%rax
    53b9:	48 8d 14 01          	lea    (%rcx,%rax,1),%rdx
    53bd:	48 8b 45 a8          	mov    -0x58(%rbp),%rax
    53c1:	48 89 42 08          	mov    %rax,0x8(%rdx)
    53c5:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    53c9:	48 8b 08             	mov    (%rax),%rcx
    53cc:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    53d0:	48 89 d0             	mov    %rdx,%rax
    53d3:	48 01 c0             	add    %rax,%rax
    53d6:	48 01 d0             	add    %rdx,%rax
    53d9:	48 c1 e0 03          	shl    $0x3,%rax
    53dd:	48 01 c8             	add    %rcx,%rax
    53e0:	c7 40 10 01 00 00 00 	movl   $0x1,0x10(%rax)
    53e7:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    53eb:	48 8b 40 10          	mov    0x10(%rax),%rax
    53ef:	48 8d 50 01          	lea    0x1(%rax),%rdx
    53f3:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    53f7:	48 89 50 10          	mov    %rdx,0x10(%rax)
    53fb:	eb 71                	jmp    546e <gul_dict_set+0x1aa>
    53fd:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    5401:	48 8b 08             	mov    (%rax),%rcx
    5404:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5408:	48 89 d0             	mov    %rdx,%rax
    540b:	48 01 c0             	add    %rax,%rax
    540e:	48 01 d0             	add    %rdx,%rax
    5411:	48 c1 e0 03          	shl    $0x3,%rax
    5415:	48 01 c8             	add    %rcx,%rax
    5418:	48 8b 00             	mov    (%rax),%rax
    541b:	48 8b 55 d8          	mov    -0x28(%rbp),%rdx
    541f:	48 89 d6             	mov    %rdx,%rsi
    5422:	48 89 c7             	mov    %rax,%rdi
    5425:	e8 56 cd ff ff       	call   2180 <strcmp@plt>
    542a:	85 c0                	test   %eax,%eax
    542c:	75 26                	jne    5454 <gul_dict_set+0x190>
    542e:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    5432:	48 8b 08             	mov    (%rax),%rcx
    5435:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5439:	48 89 d0             	mov    %rdx,%rax
    543c:	48 01 c0             	add    %rax,%rax
    543f:	48 01 d0             	add    %rdx,%rax
    5442:	48 c1 e0 03          	shl    $0x3,%rax
    5446:	48 8d 14 01          	lea    (%rcx,%rax,1),%rdx
    544a:	48 8b 45 a8          	mov    -0x58(%rbp),%rax
    544e:	48 89 42 08          	mov    %rax,0x8(%rdx)
    5452:	eb 1a                	jmp    546e <gul_dict_set+0x1aa>
    5454:	48 83 45 c8 01       	addq   $0x1,-0x38(%rbp)
    5459:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    545d:	48 8b 40 08          	mov    0x8(%rax),%rax
    5461:	48 39 45 c8          	cmp    %rax,-0x38(%rbp)
    5465:	0f 8c bd fe ff ff    	jl     5328 <gul_dict_set+0x64>
    546b:	eb 01                	jmp    546e <gul_dict_set+0x1aa>
    546d:	90                   	nop
    546e:	48 8b 5d f8          	mov    -0x8(%rbp),%rbx
    5472:	c9                   	leave
    5473:	c3                   	ret

0000000000005474 <gul_dict_get>:
    5474:	f3 0f 1e fa          	endbr64
    5478:	55                   	push   %rbp
    5479:	48 89 e5             	mov    %rsp,%rbp
    547c:	48 83 ec 40          	sub    $0x40,%rsp
    5480:	48 89 7d c8          	mov    %rdi,-0x38(%rbp)
    5484:	48 89 75 c0          	mov    %rsi,-0x40(%rbp)
    5488:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    548c:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    5490:	48 83 7d e0 00       	cmpq   $0x0,-0x20(%rbp)
    5495:	75 0a                	jne    54a1 <gul_dict_get+0x2d>
    5497:	b8 00 00 00 00       	mov    $0x0,%eax
    549c:	e9 f7 00 00 00       	jmp    5598 <gul_dict_get+0x124>
    54a1:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
    54a5:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    54a9:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    54ad:	48 89 c7             	mov    %rax,%rdi
    54b0:	e8 59 fc ff ff       	call   510e <dict_hash>
    54b5:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    54b9:	48 8b 52 08          	mov    0x8(%rdx),%rdx
    54bd:	48 89 d1             	mov    %rdx,%rcx
    54c0:	ba 00 00 00 00       	mov    $0x0,%edx
    54c5:	48 f7 f1             	div    %rcx
    54c8:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
    54cc:	48 c7 45 d8 00 00 00 	movq   $0x0,-0x28(%rbp)
    54d3:	00 
    54d4:	e9 a8 00 00 00       	jmp    5581 <gul_dict_get+0x10d>
    54d9:	48 8b 55 d8          	mov    -0x28(%rbp),%rdx
    54dd:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    54e1:	48 01 c2             	add    %rax,%rdx
    54e4:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    54e8:	48 8b 40 08          	mov    0x8(%rax),%rax
    54ec:	48 89 c1             	mov    %rax,%rcx
    54ef:	48 89 d0             	mov    %rdx,%rax
    54f2:	ba 00 00 00 00       	mov    $0x0,%edx
    54f7:	48 f7 f1             	div    %rcx
    54fa:	48 89 d0             	mov    %rdx,%rax
    54fd:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5501:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5505:	48 8b 08             	mov    (%rax),%rcx
    5508:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    550c:	48 89 d0             	mov    %rdx,%rax
    550f:	48 01 c0             	add    %rax,%rax
    5512:	48 01 d0             	add    %rdx,%rax
    5515:	48 c1 e0 03          	shl    $0x3,%rax
    5519:	48 01 c8             	add    %rcx,%rax
    551c:	8b 40 10             	mov    0x10(%rax),%eax
    551f:	85 c0                	test   %eax,%eax
    5521:	75 07                	jne    552a <gul_dict_get+0xb6>
    5523:	b8 00 00 00 00       	mov    $0x0,%eax
    5528:	eb 6e                	jmp    5598 <gul_dict_get+0x124>
    552a:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    552e:	48 8b 08             	mov    (%rax),%rcx
    5531:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5535:	48 89 d0             	mov    %rdx,%rax
    5538:	48 01 c0             	add    %rax,%rax
    553b:	48 01 d0             	add    %rdx,%rax
    553e:	48 c1 e0 03          	shl    $0x3,%rax
    5542:	48 01 c8             	add    %rcx,%rax
    5545:	48 8b 00             	mov    (%rax),%rax
    5548:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    554c:	48 89 d6             	mov    %rdx,%rsi
    554f:	48 89 c7             	mov    %rax,%rdi
    5552:	e8 29 cc ff ff       	call   2180 <strcmp@plt>
    5557:	85 c0                	test   %eax,%eax
    5559:	75 21                	jne    557c <gul_dict_get+0x108>
    555b:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    555f:	48 8b 08             	mov    (%rax),%rcx
    5562:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5566:	48 89 d0             	mov    %rdx,%rax
    5569:	48 01 c0             	add    %rax,%rax
    556c:	48 01 d0             	add    %rdx,%rax
    556f:	48 c1 e0 03          	shl    $0x3,%rax
    5573:	48 01 c8             	add    %rcx,%rax
    5576:	48 8b 40 08          	mov    0x8(%rax),%rax
    557a:	eb 1c                	jmp    5598 <gul_dict_get+0x124>
    557c:	48 83 45 d8 01       	addq   $0x1,-0x28(%rbp)
    5581:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5585:	48 8b 40 08          	mov    0x8(%rax),%rax
    5589:	48 39 45 d8          	cmp    %rax,-0x28(%rbp)
    558d:	0f 8c 46 ff ff ff    	jl     54d9 <gul_dict_get+0x65>
    5593:	b8 00 00 00 00       	mov    $0x0,%eax
    5598:	c9                   	leave
    5599:	c3                   	ret

000000000000559a <gul_dict_contains>:
    559a:	f3 0f 1e fa          	endbr64
    559e:	55                   	push   %rbp
    559f:	48 89 e5             	mov    %rsp,%rbp
    55a2:	48 83 ec 40          	sub    $0x40,%rsp
    55a6:	48 89 7d c8          	mov    %rdi,-0x38(%rbp)
    55aa:	48 89 75 c0          	mov    %rsi,-0x40(%rbp)
    55ae:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    55b2:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    55b6:	48 83 7d e0 00       	cmpq   $0x0,-0x20(%rbp)
    55bb:	75 0a                	jne    55c7 <gul_dict_contains+0x2d>
    55bd:	b8 00 00 00 00       	mov    $0x0,%eax
    55c2:	e9 dd 00 00 00       	jmp    56a4 <gul_dict_contains+0x10a>
    55c7:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
    55cb:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    55cf:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    55d3:	48 89 c7             	mov    %rax,%rdi
    55d6:	e8 33 fb ff ff       	call   510e <dict_hash>
    55db:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    55df:	48 8b 52 08          	mov    0x8(%rdx),%rdx
    55e3:	48 89 d1             	mov    %rdx,%rcx
    55e6:	ba 00 00 00 00       	mov    $0x0,%edx
    55eb:	48 f7 f1             	div    %rcx
    55ee:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
    55f2:	48 c7 45 d8 00 00 00 	movq   $0x0,-0x28(%rbp)
    55f9:	00 
    55fa:	e9 8e 00 00 00       	jmp    568d <gul_dict_contains+0xf3>
    55ff:	48 8b 55 d8          	mov    -0x28(%rbp),%rdx
    5603:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    5607:	48 01 c2             	add    %rax,%rdx
    560a:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    560e:	48 8b 40 08          	mov    0x8(%rax),%rax
    5612:	48 89 c1             	mov    %rax,%rcx
    5615:	48 89 d0             	mov    %rdx,%rax
    5618:	ba 00 00 00 00       	mov    $0x0,%edx
    561d:	48 f7 f1             	div    %rcx
    5620:	48 89 d0             	mov    %rdx,%rax
    5623:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5627:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    562b:	48 8b 08             	mov    (%rax),%rcx
    562e:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5632:	48 89 d0             	mov    %rdx,%rax
    5635:	48 01 c0             	add    %rax,%rax
    5638:	48 01 d0             	add    %rdx,%rax
    563b:	48 c1 e0 03          	shl    $0x3,%rax
    563f:	48 01 c8             	add    %rcx,%rax
    5642:	8b 40 10             	mov    0x10(%rax),%eax
    5645:	85 c0                	test   %eax,%eax
    5647:	75 07                	jne    5650 <gul_dict_contains+0xb6>
    5649:	b8 00 00 00 00       	mov    $0x0,%eax
    564e:	eb 54                	jmp    56a4 <gul_dict_contains+0x10a>
    5650:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5654:	48 8b 08             	mov    (%rax),%rcx
    5657:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    565b:	48 89 d0             	mov    %rdx,%rax
    565e:	48 01 c0             	add    %rax,%rax
    5661:	48 01 d0             	add    %rdx,%rax
    5664:	48 c1 e0 03          	shl    $0x3,%rax
    5668:	48 01 c8             	add    %rcx,%rax
    566b:	48 8b 00             	mov    (%rax),%rax
    566e:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5672:	48 89 d6             	mov    %rdx,%rsi
    5675:	48 89 c7             	mov    %rax,%rdi
    5678:	e8 03 cb ff ff       	call   2180 <strcmp@plt>
    567d:	85 c0                	test   %eax,%eax
    567f:	75 07                	jne    5688 <gul_dict_contains+0xee>
    5681:	b8 01 00 00 00       	mov    $0x1,%eax
    5686:	eb 1c                	jmp    56a4 <gul_dict_contains+0x10a>
    5688:	48 83 45 d8 01       	addq   $0x1,-0x28(%rbp)
    568d:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5691:	48 8b 40 08          	mov    0x8(%rax),%rax
    5695:	48 39 45 d8          	cmp    %rax,-0x28(%rbp)
    5699:	0f 8c 60 ff ff ff    	jl     55ff <gul_dict_contains+0x65>
    569f:	b8 00 00 00 00       	mov    $0x0,%eax
    56a4:	c9                   	leave
    56a5:	c3                   	ret

00000000000056a6 <gul_dict_remove>:
    56a6:	f3 0f 1e fa          	endbr64
    56aa:	55                   	push   %rbp
    56ab:	48 89 e5             	mov    %rsp,%rbp
    56ae:	48 83 ec 40          	sub    $0x40,%rsp
    56b2:	48 89 7d c8          	mov    %rdi,-0x38(%rbp)
    56b6:	48 89 75 c0          	mov    %rsi,-0x40(%rbp)
    56ba:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
    56be:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
    56c2:	48 83 7d e0 00       	cmpq   $0x0,-0x20(%rbp)
    56c7:	0f 84 2e 01 00 00    	je     57fb <gul_dict_remove+0x155>
    56cd:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
    56d1:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    56d5:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    56d9:	48 89 c7             	mov    %rax,%rdi
    56dc:	e8 2d fa ff ff       	call   510e <dict_hash>
    56e1:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    56e5:	48 8b 52 08          	mov    0x8(%rdx),%rdx
    56e9:	48 89 d1             	mov    %rdx,%rcx
    56ec:	ba 00 00 00 00       	mov    $0x0,%edx
    56f1:	48 f7 f1             	div    %rcx
    56f4:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
    56f8:	48 c7 45 d8 00 00 00 	movq   $0x0,-0x28(%rbp)
    56ff:	00 
    5700:	e9 e2 00 00 00       	jmp    57e7 <gul_dict_remove+0x141>
    5705:	48 8b 55 d8          	mov    -0x28(%rbp),%rdx
    5709:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    570d:	48 01 c2             	add    %rax,%rdx
    5710:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5714:	48 8b 40 08          	mov    0x8(%rax),%rax
    5718:	48 89 c1             	mov    %rax,%rcx
    571b:	48 89 d0             	mov    %rdx,%rax
    571e:	ba 00 00 00 00       	mov    $0x0,%edx
    5723:	48 f7 f1             	div    %rcx
    5726:	48 89 d0             	mov    %rdx,%rax
    5729:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    572d:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5731:	48 8b 08             	mov    (%rax),%rcx
    5734:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5738:	48 89 d0             	mov    %rdx,%rax
    573b:	48 01 c0             	add    %rax,%rax
    573e:	48 01 d0             	add    %rdx,%rax
    5741:	48 c1 e0 03          	shl    $0x3,%rax
    5745:	48 01 c8             	add    %rcx,%rax
    5748:	8b 40 10             	mov    0x10(%rax),%eax
    574b:	85 c0                	test   %eax,%eax
    574d:	0f 84 ab 00 00 00    	je     57fe <gul_dict_remove+0x158>
    5753:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5757:	48 8b 08             	mov    (%rax),%rcx
    575a:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    575e:	48 89 d0             	mov    %rdx,%rax
    5761:	48 01 c0             	add    %rax,%rax
    5764:	48 01 d0             	add    %rdx,%rax
    5767:	48 c1 e0 03          	shl    $0x3,%rax
    576b:	48 01 c8             	add    %rcx,%rax
    576e:	48 8b 00             	mov    (%rax),%rax
    5771:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5775:	48 89 d6             	mov    %rdx,%rsi
    5778:	48 89 c7             	mov    %rax,%rdi
    577b:	e8 00 ca ff ff       	call   2180 <strcmp@plt>
    5780:	85 c0                	test   %eax,%eax
    5782:	75 5e                	jne    57e2 <gul_dict_remove+0x13c>
    5784:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    5788:	48 8b 08             	mov    (%rax),%rcx
    578b:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    578f:	48 89 d0             	mov    %rdx,%rax
    5792:	48 01 c0             	add    %rax,%rax
    5795:	48 01 d0             	add    %rdx,%rax
    5798:	48 c1 e0 03          	shl    $0x3,%rax
    579c:	48 01 c8             	add    %rcx,%rax
    579f:	48 8b 00             	mov    (%rax),%rax
    57a2:	48 89 c7             	mov    %rax,%rdi
    57a5:	e8 86 c8 ff ff       	call   2030 <free@plt>
    57aa:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    57ae:	48 8b 08             	mov    (%rax),%rcx
    57b1:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    57b5:	48 89 d0             	mov    %rdx,%rax
    57b8:	48 01 c0             	add    %rax,%rax
    57bb:	48 01 d0             	add    %rdx,%rax
    57be:	48 c1 e0 03          	shl    $0x3,%rax
    57c2:	48 01 c8             	add    %rcx,%rax
    57c5:	c7 40 10 00 00 00 00 	movl   $0x0,0x10(%rax)
    57cc:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    57d0:	48 8b 40 10          	mov    0x10(%rax),%rax
    57d4:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
    57d8:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    57dc:	48 89 50 10          	mov    %rdx,0x10(%rax)
    57e0:	eb 1d                	jmp    57ff <gul_dict_remove+0x159>
    57e2:	48 83 45 d8 01       	addq   $0x1,-0x28(%rbp)
    57e7:	48 8b 45 e0          	mov    -0x20(%rbp),%rax
    57eb:	48 8b 40 08          	mov    0x8(%rax),%rax
    57ef:	48 39 45 d8          	cmp    %rax,-0x28(%rbp)
    57f3:	0f 8c 0c ff ff ff    	jl     5705 <gul_dict_remove+0x5f>
    57f9:	eb 04                	jmp    57ff <gul_dict_remove+0x159>
    57fb:	90                   	nop
    57fc:	eb 01                	jmp    57ff <gul_dict_remove+0x159>
    57fe:	90                   	nop
    57ff:	c9                   	leave
    5800:	c3                   	ret

0000000000005801 <gul_dict_clear>:
    5801:	f3 0f 1e fa          	endbr64
    5805:	55                   	push   %rbp
    5806:	48 89 e5             	mov    %rsp,%rbp
    5809:	48 83 ec 20          	sub    $0x20,%rsp
    580d:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    5811:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5815:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5819:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    581e:	0f 84 95 00 00 00    	je     58b9 <gul_dict_clear+0xb8>
    5824:	48 c7 45 f0 00 00 00 	movq   $0x0,-0x10(%rbp)
    582b:	00 
    582c:	eb 6f                	jmp    589d <gul_dict_clear+0x9c>
    582e:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5832:	48 8b 08             	mov    (%rax),%rcx
    5835:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    5839:	48 89 d0             	mov    %rdx,%rax
    583c:	48 01 c0             	add    %rax,%rax
    583f:	48 01 d0             	add    %rdx,%rax
    5842:	48 c1 e0 03          	shl    $0x3,%rax
    5846:	48 01 c8             	add    %rcx,%rax
    5849:	8b 40 10             	mov    0x10(%rax),%eax
    584c:	85 c0                	test   %eax,%eax
    584e:	74 48                	je     5898 <gul_dict_clear+0x97>
    5850:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5854:	48 8b 08             	mov    (%rax),%rcx
    5857:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    585b:	48 89 d0             	mov    %rdx,%rax
    585e:	48 01 c0             	add    %rax,%rax
    5861:	48 01 d0             	add    %rdx,%rax
    5864:	48 c1 e0 03          	shl    $0x3,%rax
    5868:	48 01 c8             	add    %rcx,%rax
    586b:	48 8b 00             	mov    (%rax),%rax
    586e:	48 89 c7             	mov    %rax,%rdi
    5871:	e8 ba c7 ff ff       	call   2030 <free@plt>
    5876:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    587a:	48 8b 08             	mov    (%rax),%rcx
    587d:	48 8b 55 f0          	mov    -0x10(%rbp),%rdx
    5881:	48 89 d0             	mov    %rdx,%rax
    5884:	48 01 c0             	add    %rax,%rax
    5887:	48 01 d0             	add    %rdx,%rax
    588a:	48 c1 e0 03          	shl    $0x3,%rax
    588e:	48 01 c8             	add    %rcx,%rax
    5891:	c7 40 10 00 00 00 00 	movl   $0x0,0x10(%rax)
    5898:	48 83 45 f0 01       	addq   $0x1,-0x10(%rbp)
    589d:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    58a1:	48 8b 40 08          	mov    0x8(%rax),%rax
    58a5:	48 39 45 f0          	cmp    %rax,-0x10(%rbp)
    58a9:	7c 83                	jl     582e <gul_dict_clear+0x2d>
    58ab:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    58af:	48 c7 40 10 00 00 00 	movq   $0x0,0x10(%rax)
    58b6:	00 
    58b7:	eb 01                	jmp    58ba <gul_dict_clear+0xb9>
    58b9:	90                   	nop
    58ba:	c9                   	leave
    58bb:	c3                   	ret

00000000000058bc <gul_set_alloc>:
    58bc:	f3 0f 1e fa          	endbr64
    58c0:	55                   	push   %rbp
    58c1:	48 89 e5             	mov    %rsp,%rbp
    58c4:	48 83 ec 20          	sub    $0x20,%rsp
    58c8:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    58cc:	bf 20 00 00 00       	mov    $0x20,%edi
    58d1:	e8 ea c8 ff ff       	call   21c0 <malloc@plt>
    58d6:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    58da:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    58df:	75 0a                	jne    58eb <gul_set_alloc+0x2f>
    58e1:	bf 01 00 00 00       	mov    $0x1,%edi
    58e6:	e8 b5 c9 ff ff       	call   22a0 <exit@plt>
    58eb:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    58f0:	7e 06                	jle    58f8 <gul_set_alloc+0x3c>
    58f2:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    58f6:	eb 05                	jmp    58fd <gul_set_alloc+0x41>
    58f8:	b8 10 00 00 00       	mov    $0x10,%eax
    58fd:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5901:	48 89 42 10          	mov    %rax,0x10(%rdx)
    5905:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5909:	48 c7 40 18 00 00 00 	movq   $0x0,0x18(%rax)
    5910:	00 
    5911:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5915:	48 8b 40 10          	mov    0x10(%rax),%rax
    5919:	48 c1 e0 03          	shl    $0x3,%rax
    591d:	48 89 c7             	mov    %rax,%rdi
    5920:	e8 9b c8 ff ff       	call   21c0 <malloc@plt>
    5925:	48 89 c2             	mov    %rax,%rdx
    5928:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    592c:	48 89 10             	mov    %rdx,(%rax)
    592f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5933:	48 8b 40 10          	mov    0x10(%rax),%rax
    5937:	be 04 00 00 00       	mov    $0x4,%esi
    593c:	48 89 c7             	mov    %rax,%rdi
    593f:	e8 2c c8 ff ff       	call   2170 <calloc@plt>
    5944:	48 89 c2             	mov    %rax,%rdx
    5947:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    594b:	48 89 50 08          	mov    %rdx,0x8(%rax)
    594f:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5953:	48 8b 00             	mov    (%rax),%rax
    5956:	48 85 c0             	test   %rax,%rax
    5959:	74 0d                	je     5968 <gul_set_alloc+0xac>
    595b:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    595f:	48 8b 40 08          	mov    0x8(%rax),%rax
    5963:	48 85 c0             	test   %rax,%rax
    5966:	75 0a                	jne    5972 <gul_set_alloc+0xb6>
    5968:	bf 01 00 00 00       	mov    $0x1,%edi
    596d:	e8 2e c9 ff ff       	call   22a0 <exit@plt>
    5972:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5976:	c9                   	leave
    5977:	c3                   	ret

0000000000005978 <gul_set_free>:
    5978:	f3 0f 1e fa          	endbr64
    597c:	55                   	push   %rbp
    597d:	48 89 e5             	mov    %rsp,%rbp
    5980:	48 83 ec 20          	sub    $0x20,%rsp
    5984:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    5988:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    598c:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5990:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    5995:	74 2b                	je     59c2 <gul_set_free+0x4a>
    5997:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    599b:	48 8b 00             	mov    (%rax),%rax
    599e:	48 89 c7             	mov    %rax,%rdi
    59a1:	e8 8a c6 ff ff       	call   2030 <free@plt>
    59a6:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    59aa:	48 8b 40 08          	mov    0x8(%rax),%rax
    59ae:	48 89 c7             	mov    %rax,%rdi
    59b1:	e8 7a c6 ff ff       	call   2030 <free@plt>
    59b6:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    59ba:	48 89 c7             	mov    %rax,%rdi
    59bd:	e8 6e c6 ff ff       	call   2030 <free@plt>
    59c2:	90                   	nop
    59c3:	c9                   	leave
    59c4:	c3                   	ret

00000000000059c5 <gul_set_len>:
    59c5:	f3 0f 1e fa          	endbr64
    59c9:	55                   	push   %rbp
    59ca:	48 89 e5             	mov    %rsp,%rbp
    59cd:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    59d1:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    59d5:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    59d9:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    59de:	74 0a                	je     59ea <gul_set_len+0x25>
    59e0:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    59e4:	48 8b 40 18          	mov    0x18(%rax),%rax
    59e8:	eb 05                	jmp    59ef <gul_set_len+0x2a>
    59ea:	b8 00 00 00 00       	mov    $0x0,%eax
    59ef:	5d                   	pop    %rbp
    59f0:	c3                   	ret

00000000000059f1 <gul_set_add>:
    59f1:	f3 0f 1e fa          	endbr64
    59f5:	55                   	push   %rbp
    59f6:	48 89 e5             	mov    %rsp,%rbp
    59f9:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    59fd:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    5a01:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    5a05:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    5a09:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    5a0e:	0f 84 e5 00 00 00    	je     5af9 <gul_set_add+0x108>
    5a14:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    5a18:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5a1c:	48 8b 52 10          	mov    0x10(%rdx),%rdx
    5a20:	48 89 d1             	mov    %rdx,%rcx
    5a23:	ba 00 00 00 00       	mov    $0x0,%edx
    5a28:	48 f7 f1             	div    %rcx
    5a2b:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
    5a2f:	48 c7 45 e0 00 00 00 	movq   $0x0,-0x20(%rbp)
    5a36:	00 
    5a37:	e9 a9 00 00 00       	jmp    5ae5 <gul_set_add+0xf4>
    5a3c:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    5a40:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    5a44:	48 01 c2             	add    %rax,%rdx
    5a47:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5a4b:	48 8b 40 10          	mov    0x10(%rax),%rax
    5a4f:	48 89 c1             	mov    %rax,%rcx
    5a52:	48 89 d0             	mov    %rdx,%rax
    5a55:	ba 00 00 00 00       	mov    $0x0,%edx
    5a5a:	48 f7 f1             	div    %rcx
    5a5d:	48 89 d0             	mov    %rdx,%rax
    5a60:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5a64:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5a68:	48 8b 40 08          	mov    0x8(%rax),%rax
    5a6c:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5a70:	48 c1 e2 02          	shl    $0x2,%rdx
    5a74:	48 01 d0             	add    %rdx,%rax
    5a77:	8b 00                	mov    (%rax),%eax
    5a79:	85 c0                	test   %eax,%eax
    5a7b:	75 48                	jne    5ac5 <gul_set_add+0xd4>
    5a7d:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5a81:	48 8b 00             	mov    (%rax),%rax
    5a84:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5a88:	48 c1 e2 03          	shl    $0x3,%rdx
    5a8c:	48 01 c2             	add    %rax,%rdx
    5a8f:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    5a93:	48 89 02             	mov    %rax,(%rdx)
    5a96:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5a9a:	48 8b 40 08          	mov    0x8(%rax),%rax
    5a9e:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5aa2:	48 c1 e2 02          	shl    $0x2,%rdx
    5aa6:	48 01 d0             	add    %rdx,%rax
    5aa9:	c7 00 01 00 00 00    	movl   $0x1,(%rax)
    5aaf:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5ab3:	48 8b 40 18          	mov    0x18(%rax),%rax
    5ab7:	48 8d 50 01          	lea    0x1(%rax),%rdx
    5abb:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5abf:	48 89 50 18          	mov    %rdx,0x18(%rax)
    5ac3:	eb 38                	jmp    5afd <gul_set_add+0x10c>
    5ac5:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5ac9:	48 8b 00             	mov    (%rax),%rax
    5acc:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5ad0:	48 c1 e2 03          	shl    $0x3,%rdx
    5ad4:	48 01 d0             	add    %rdx,%rax
    5ad7:	48 8b 00             	mov    (%rax),%rax
    5ada:	48 39 45 d0          	cmp    %rax,-0x30(%rbp)
    5ade:	74 1c                	je     5afc <gul_set_add+0x10b>
    5ae0:	48 83 45 e0 01       	addq   $0x1,-0x20(%rbp)
    5ae5:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5ae9:	48 8b 40 10          	mov    0x10(%rax),%rax
    5aed:	48 39 45 e0          	cmp    %rax,-0x20(%rbp)
    5af1:	0f 8c 45 ff ff ff    	jl     5a3c <gul_set_add+0x4b>
    5af7:	eb 04                	jmp    5afd <gul_set_add+0x10c>
    5af9:	90                   	nop
    5afa:	eb 01                	jmp    5afd <gul_set_add+0x10c>
    5afc:	90                   	nop
    5afd:	5d                   	pop    %rbp
    5afe:	c3                   	ret

0000000000005aff <gul_set_contains>:
    5aff:	f3 0f 1e fa          	endbr64
    5b03:	55                   	push   %rbp
    5b04:	48 89 e5             	mov    %rsp,%rbp
    5b07:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    5b0b:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    5b0f:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    5b13:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    5b17:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    5b1c:	75 0a                	jne    5b28 <gul_set_contains+0x29>
    5b1e:	b8 00 00 00 00       	mov    $0x0,%eax
    5b23:	e9 a7 00 00 00       	jmp    5bcf <gul_set_contains+0xd0>
    5b28:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    5b2c:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5b30:	48 8b 52 10          	mov    0x10(%rdx),%rdx
    5b34:	48 89 d1             	mov    %rdx,%rcx
    5b37:	ba 00 00 00 00       	mov    $0x0,%edx
    5b3c:	48 f7 f1             	div    %rcx
    5b3f:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
    5b43:	48 c7 45 e0 00 00 00 	movq   $0x0,-0x20(%rbp)
    5b4a:	00 
    5b4b:	eb 6f                	jmp    5bbc <gul_set_contains+0xbd>
    5b4d:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    5b51:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    5b55:	48 01 c2             	add    %rax,%rdx
    5b58:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5b5c:	48 8b 40 10          	mov    0x10(%rax),%rax
    5b60:	48 89 c1             	mov    %rax,%rcx
    5b63:	48 89 d0             	mov    %rdx,%rax
    5b66:	ba 00 00 00 00       	mov    $0x0,%edx
    5b6b:	48 f7 f1             	div    %rcx
    5b6e:	48 89 d0             	mov    %rdx,%rax
    5b71:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5b75:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5b79:	48 8b 40 08          	mov    0x8(%rax),%rax
    5b7d:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5b81:	48 c1 e2 02          	shl    $0x2,%rdx
    5b85:	48 01 d0             	add    %rdx,%rax
    5b88:	8b 00                	mov    (%rax),%eax
    5b8a:	85 c0                	test   %eax,%eax
    5b8c:	75 07                	jne    5b95 <gul_set_contains+0x96>
    5b8e:	b8 00 00 00 00       	mov    $0x0,%eax
    5b93:	eb 3a                	jmp    5bcf <gul_set_contains+0xd0>
    5b95:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5b99:	48 8b 00             	mov    (%rax),%rax
    5b9c:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5ba0:	48 c1 e2 03          	shl    $0x3,%rdx
    5ba4:	48 01 d0             	add    %rdx,%rax
    5ba7:	48 8b 00             	mov    (%rax),%rax
    5baa:	48 39 45 d0          	cmp    %rax,-0x30(%rbp)
    5bae:	75 07                	jne    5bb7 <gul_set_contains+0xb8>
    5bb0:	b8 01 00 00 00       	mov    $0x1,%eax
    5bb5:	eb 18                	jmp    5bcf <gul_set_contains+0xd0>
    5bb7:	48 83 45 e0 01       	addq   $0x1,-0x20(%rbp)
    5bbc:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5bc0:	48 8b 40 10          	mov    0x10(%rax),%rax
    5bc4:	48 39 45 e0          	cmp    %rax,-0x20(%rbp)
    5bc8:	7c 83                	jl     5b4d <gul_set_contains+0x4e>
    5bca:	b8 00 00 00 00       	mov    $0x0,%eax
    5bcf:	5d                   	pop    %rbp
    5bd0:	c3                   	ret

0000000000005bd1 <gul_set_remove>:
    5bd1:	f3 0f 1e fa          	endbr64
    5bd5:	55                   	push   %rbp
    5bd6:	48 89 e5             	mov    %rsp,%rbp
    5bd9:	48 89 7d d8          	mov    %rdi,-0x28(%rbp)
    5bdd:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
    5be1:	48 8b 45 d8          	mov    -0x28(%rbp),%rax
    5be5:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
    5be9:	48 83 7d e8 00       	cmpq   $0x0,-0x18(%rbp)
    5bee:	0f 84 cc 00 00 00    	je     5cc0 <gul_set_remove+0xef>
    5bf4:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
    5bf8:	48 8b 55 e8          	mov    -0x18(%rbp),%rdx
    5bfc:	48 8b 52 10          	mov    0x10(%rdx),%rdx
    5c00:	48 89 d1             	mov    %rdx,%rcx
    5c03:	ba 00 00 00 00       	mov    $0x0,%edx
    5c08:	48 f7 f1             	div    %rcx
    5c0b:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
    5c0f:	48 c7 45 e0 00 00 00 	movq   $0x0,-0x20(%rbp)
    5c16:	00 
    5c17:	e9 90 00 00 00       	jmp    5cac <gul_set_remove+0xdb>
    5c1c:	48 8b 55 e0          	mov    -0x20(%rbp),%rdx
    5c20:	48 8b 45 f0          	mov    -0x10(%rbp),%rax
    5c24:	48 01 c2             	add    %rax,%rdx
    5c27:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5c2b:	48 8b 40 10          	mov    0x10(%rax),%rax
    5c2f:	48 89 c1             	mov    %rax,%rcx
    5c32:	48 89 d0             	mov    %rdx,%rax
    5c35:	ba 00 00 00 00       	mov    $0x0,%edx
    5c3a:	48 f7 f1             	div    %rcx
    5c3d:	48 89 d0             	mov    %rdx,%rax
    5c40:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5c44:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5c48:	48 8b 40 08          	mov    0x8(%rax),%rax
    5c4c:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5c50:	48 c1 e2 02          	shl    $0x2,%rdx
    5c54:	48 01 d0             	add    %rdx,%rax
    5c57:	8b 00                	mov    (%rax),%eax
    5c59:	85 c0                	test   %eax,%eax
    5c5b:	74 66                	je     5cc3 <gul_set_remove+0xf2>
    5c5d:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5c61:	48 8b 00             	mov    (%rax),%rax
    5c64:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5c68:	48 c1 e2 03          	shl    $0x3,%rdx
    5c6c:	48 01 d0             	add    %rdx,%rax
    5c6f:	48 8b 00             	mov    (%rax),%rax
    5c72:	48 39 45 d0          	cmp    %rax,-0x30(%rbp)
    5c76:	75 2f                	jne    5ca7 <gul_set_remove+0xd6>
    5c78:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5c7c:	48 8b 40 08          	mov    0x8(%rax),%rax
    5c80:	48 8b 55 f8          	mov    -0x8(%rbp),%rdx
    5c84:	48 c1 e2 02          	shl    $0x2,%rdx
    5c88:	48 01 d0             	add    %rdx,%rax
    5c8b:	c7 00 00 00 00 00    	movl   $0x0,(%rax)
    5c91:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5c95:	48 8b 40 18          	mov    0x18(%rax),%rax
    5c99:	48 8d 50 ff          	lea    -0x1(%rax),%rdx
    5c9d:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5ca1:	48 89 50 18          	mov    %rdx,0x18(%rax)
    5ca5:	eb 1d                	jmp    5cc4 <gul_set_remove+0xf3>
    5ca7:	48 83 45 e0 01       	addq   $0x1,-0x20(%rbp)
    5cac:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5cb0:	48 8b 40 10          	mov    0x10(%rax),%rax
    5cb4:	48 39 45 e0          	cmp    %rax,-0x20(%rbp)
    5cb8:	0f 8c 5e ff ff ff    	jl     5c1c <gul_set_remove+0x4b>
    5cbe:	eb 04                	jmp    5cc4 <gul_set_remove+0xf3>
    5cc0:	90                   	nop
    5cc1:	eb 01                	jmp    5cc4 <gul_set_remove+0xf3>
    5cc3:	90                   	nop
    5cc4:	5d                   	pop    %rbp
    5cc5:	c3                   	ret

0000000000005cc6 <gul_set_clear>:
    5cc6:	f3 0f 1e fa          	endbr64
    5cca:	55                   	push   %rbp
    5ccb:	48 89 e5             	mov    %rsp,%rbp
    5cce:	48 83 ec 20          	sub    $0x20,%rsp
    5cd2:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
    5cd6:	48 8b 45 e8          	mov    -0x18(%rbp),%rax
    5cda:	48 89 45 f8          	mov    %rax,-0x8(%rbp)
    5cde:	48 83 7d f8 00       	cmpq   $0x0,-0x8(%rbp)
    5ce3:	74 33                	je     5d18 <gul_set_clear+0x52>
    5ce5:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5ce9:	48 8b 40 10          	mov    0x10(%rax),%rax
    5ced:	48 8d 14 85 00 00 00 	lea    0x0(,%rax,4),%rdx
    5cf4:	00 
    5cf5:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5cf9:	48 8b 40 08          	mov    0x8(%rax),%rax
    5cfd:	be 00 00 00 00       	mov    $0x0,%esi
    5d02:	48 89 c7             	mov    %rax,%rdi
    5d05:	e8 16 c4 ff ff       	call   2120 <memset@plt>
    5d0a:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5d0e:	48 c7 40 18 00 00 00 	movq   $0x0,0x18(%rax)
    5d15:	00 
    5d16:	eb 01                	jmp    5d19 <gul_set_clear+0x53>
    5d18:	90                   	nop
    5d19:	c9                   	leave
    5d1a:	c3                   	ret

0000000000005d1b <gul_malloc>:
    5d1b:	f3 0f 1e fa          	endbr64
    5d1f:	55                   	push   %rbp
    5d20:	48 89 e5             	mov    %rsp,%rbp
    5d23:	48 83 ec 10          	sub    $0x10,%rsp
    5d27:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    5d2b:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5d2f:	48 89 c7             	mov    %rax,%rdi
    5d32:	e8 89 c4 ff ff       	call   21c0 <malloc@plt>
    5d37:	c9                   	leave
    5d38:	c3                   	ret

0000000000005d39 <gul_free>:
    5d39:	f3 0f 1e fa          	endbr64
    5d3d:	55                   	push   %rbp
    5d3e:	48 89 e5             	mov    %rsp,%rbp
    5d41:	48 83 ec 10          	sub    $0x10,%rsp
    5d45:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
    5d49:	48 8b 45 f8          	mov    -0x8(%rbp),%rax
    5d4d:	48 89 c7             	mov    %rax,%rdi
    5d50:	e8 db c2 ff ff       	call   2030 <free@plt>
    5d55:	90                   	nop
    5d56:	c9                   	leave
    5d57:	c3                   	ret

Disassembly of section .fini:

0000000000005d58 <_fini>:
    5d58:	f3 0f 1e fa          	endbr64
    5d5c:	48 83 ec 08          	sub    $0x8,%rsp
    5d60:	48 83 c4 08          	add    $0x8,%rsp
    5d64:	c3                   	ret
