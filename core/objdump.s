=== A=0x40100F5 F=0x100f5 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000010095 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x4d5>:
   10095:	ff                   	(bad)
   10096:	38 07                	cmp    %al,(%rdi)
   10098:	00 0f                	add    %cl,(%rdi)
   1009a:	b6 4c                	mov    $0x4c,%dh
   1009c:	24 10                	and    $0x10,%al
   1009e:	22 4c 24 0f          	and    0xf(%rsp),%cl
   100a2:	0f b6 54 24 0e       	movzbl 0xe(%rsp),%edx
   100a7:	22 54 24 0d          	and    0xd(%rsp),%dl
   100ab:	20 ca                	and    %cl,%dl
   100ad:	44 22 74 24 0c       	and    0xc(%rsp),%r14b
   100b2:	44 22 74 24 0b       	and    0xb(%rsp),%r14b
   100b7:	41 20 d6             	and    %dl,%r14b
   100ba:	44 22 7c 24 0a       	and    0xa(%rsp),%r15b
   100bf:	44 22 7c 24 09       	and    0x9(%rsp),%r15b
   100c4:	44 22 7c 24 08       	and    0x8(%rsp),%r15b
   100c9:	45 20 f7             	and    %r14b,%r15b
   100cc:	44 22 6c 24 07       	and    0x7(%rsp),%r13b
   100d1:	44 22 6c 24 06       	and    0x6(%rsp),%r13b
   100d6:	44 22 6c 24 05       	and    0x5(%rsp),%r13b
   100db:	41 20 c5             	and    %al,%r13b
   100de:	45 20 fd             	and    %r15b,%r13b
   100e1:	41 80 e5 01          	and    $0x1,%r13b
   100e5:	41 0f b6 fd          	movzbl %r13b,%edi
   100e9:	ff 15 a9 38 07 00    	call   *0x738a9(%rip)        # 83998 <_GLOBAL_OFFSET_TABLE_+0x418>
   100ef:	84 c0                	test   %al,%al
   100f1:	41 0f 94 c6          	sete   %r14b
   100f5:	74 16                	je     1010d <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x54d>
   100f7:	48 8d bb 40 02 00 00 	lea    0x240(%rbx),%rdi
   100fe:	48 8b 74 24 30       	mov    0x30(%rsp),%rsi
   10103:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   10108:	e8 e3 03 00 00       	call   104f0 <cipher::stream::StreamCipher::apply_keystream>
   1010d:	4c 8d bb 40 02 00 00 	lea    0x240(%rbx),%r15
   10114:	4c 89 ff             	mov    %r15,%rdi
   10117:	e8 34 26 00 00       	call   12750 <<cipher::stream_wrapper::StreamCipherCoreWrapper<T> as core::ops::drop::Drop>::drop>
   1011c:	41 c7 07 00 00 00 00 	movl   $0x0,(%r15)
   10123:	c7 83 44 02 00 00 00 	movl   $0x0,0x244(%rbx)
   1012a:	00 00 00 
   1012d:	c7 83 48 02 00 00 00 	movl   $0x0,0x248(%rbx)
   10134:	00 00 00 
   10137:	c7 83 4c 02 00 00 00 	movl   $0x0,0x24c(%rbx)
   1013e:	00 00 00 
   10141:	c7 83 50 02 00 00 00 	movl   $0x0,0x250(%rbx)
   10148:	00 00 00 
   1014b:	c7 83 54 02 00 00 00 	movl   $0x0,0x254(%rbx)
   10152:	00 00 00 
   10155:	c7 83 58 02 00 00 00 	movl   $0x0,0x258(%rbx)
   1015c:	00 00 00 
   1015f:	c7 83 5c 02 00 00 00 	movl   $0x0,0x25c(%rbx)
   10166:	00 00 00 
   10169:	c7 83 60 02 00 00 00 	movl   $0x0,0x260(%rbx)
   10170:	00 00 00 
   10173:	c7 83 64 02 00 00 00 	movl   $0x0,0x264(%rbx)
   1017a:	00 00 00 
   1017d:	c7 83 68 02 00 00 00 	movl   $0x0,0x268(%rbx)
   10184:	00 00 00 
   10187:	c7 83 6c 02 00 00 00 	movl   $0x0,0x26c(%rbx)
   1018e:	00 00 00 
   10191:	c7 83 70 02 00 00 00 	movl   $0x0,0x270(%rbx)
   10198:	00 00 00 
   1019b:	c7 83 74 02 00 00 00 	movl   $0x0,0x274(%rbx)
   101a2:	00 00 00 
   101a5:	c7 83 78 02 00 00 00 	movl   $0x0,0x278(%rbx)
   101ac:	00 00 00 
   101af:	c7 83 7c 02 00 00 00 	movl   $0x0,0x27c(%rbx)
   101b6:	00 00 00 
   101b9:	44 89 f0             	mov    %r14d,%eax
   101bc:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   101c0:	5b                   	pop    %rbx
   101c1:	41 5c                	pop    %r12
   101c3:	41 5d                	pop    %r13
   101c5:	41 5e                	pop    %r14
   101c7:	41 5f                	pop    %r15
   101c9:	5d                   	pop    %rbp
   101ca:	c3                   	ret
   101cb:	eb 3a                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   101cd:	eb 38                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   101cf:	eb 36                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   101d1:	eb 34                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   101d3:	49 89 c6             	mov    %rax,%r14
   101d6:	4c 89 ff             	mov    %r15,%rdi
   101d9:	e8 c2 42 00 00       	call   144a0 <<chacha20::ChaChaCore<R> as core::ops::drop::Drop>::drop>
   101de:	eb 32                	jmp    10212 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x652>
   101e0:	ff 15 f2 35 07 00    	call   *0x735f2(%rip)        # 837d8 <_GLOBAL_OFFSET_TABLE_+0x258>
   101e6:	49 89 c6             	mov    %rax,%r14
   101e9:	4c 89 ff             	mov    %r15,%rdi
   101ec:	e8 af 42 00 00       	call   144a0 <<chacha20::ChaChaCore<R> as core::ops::drop::Drop>::drop>
   101f1:	eb 1f                	jmp    10212 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x652>
   101f3:	ff 15 df 35 07 00    	call   *0x735df(%rip)        # 837d8 <_GLOBAL_OFFSET_TABLE_+0x258>
   101f9:	eb 0c                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   101fb:	eb 0a                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   101fd:	eb 08                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   101ff:	eb 06                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   10201:	eb 04                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   10203:	eb 02                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   10205:	eb 00                	jmp    10207 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x647>
   10207:	49 89 c6             	mov    %rax,%r14
   1020a:	48 89 df             	mov    %rbx,%rdi
   1020d:	e8 fe 01 00 00       	call   10410 <core::ptr::drop_in_place<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm,typenum::bit::B1>,typenum::bit::B0>,typenum::bit::B1>,typenum::bit::B0>>>>>>
   10212:	4c 89 f7             	mov    %r14,%rdi
   10215:	e8 36 5e ff ff       	call   6050 <_Unwind_Resume@plt>
   1021a:	ff 15 b8 35 07 00    	call   *0x735b8(%rip)        # 837d8 <_GLOBAL_OFFSET_TABLE_+0x258>

0000000000010220 <chacha20poly1305::cipher::Cipher<C>::new>:
   10220:	55                   	push   %rbp
   10221:	48 89 e5             	mov    %rsp,%rbp
   10224:	41 57                	push   %r15
   10226:	41 56                	push   %r14
   10228:	53                   	push   %rbx
   10229:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   1022d:	48 81 ec 80 02 00 00 	sub    $0x280,%rsp
   10234:	49 89 f6             	mov    %rsi,%r14
   10237:	48 89 fb             	mov    %rdi,%rbx
   1023a:	0f 57 c0             	xorps  %xmm0,%xmm0
   1023d:	0f 29 44 24 10       	movaps %xmm0,0x10(%rsp)
   10242:	0f 29 04 24          	movaps %xmm0,(%rsp)
   10246:	48 89 e6             	mov    %rsp,%rsi
   10249:	ba 20 00 00 00       	mov    $0x20,%edx
   1024e:	4c 89 f7             	mov    %r14,%rdi
   10251:	e8 9a 02 00 00       	call   104f0 <cipher::stream::StreamCipher::apply_keystream>
   10256:	48 8d 7c 24 20       	lea    0x20(%rsp),%rdi
   1025b:	48 89 e6             	mov    %rsp,%rsi
   1025e:	ff 15 e4 38 07 00    	call   *0x738e4(%rip)        # 83b48 <_GLOBAL_OFFSET_TABLE_+0x5c8>
   10264:	c6 04 24 00          	movb   $0x0,(%rsp)
   10268:	c6 44 24 01 00       	movb   $0x0,0x1(%rsp)
   1026d:	c6 44 24 02 00       	movb   $0x0,0x2(%rsp)
   10272:	c6 44 24 03 00       	movb   $0x0,0x3(%rsp)
   10277:	c6 44 24 04 00       	movb   $0x0,0x4(%rsp)
   1027c:	c6 44 24 05 00       	movb   $0x0,0x5(%rsp)
   10281:	c6 44 24 06 00       	movb   $0x0,0x6(%rsp)
   10286:	c6 44 24 07 00       	movb   $0x0,0x7(%rsp)
   1028b:	c6 44 24 08 00       	movb   $0x0,0x8(%rsp)
   10290:	c6 44 24 09 00       	movb   $0x0,0x9(%rsp)
   10295:	c6 44 24 0a 00       	movb   $0x0,0xa(%rsp)
   1029a:	c6 44 24 0b 00       	movb   $0x0,0xb(%rsp)
   1029f:	c6 44 24 0c 00       	movb   $0x0,0xc(%rsp)
   102a4:	c6 44 24 0d 00       	movb   $0x0,0xd(%rsp)
   102a9:	c6 44 24 0e 00       	movb   $0x0,0xe(%rsp)
   102ae:	c6 44 24 0f 00       	movb   $0x0,0xf(%rsp)
   102b3:	c6 44 24 10 00       	movb   $0x0,0x10(%rsp)
   102b8:	c6 44 24 11 00       	movb   $0x0,0x11(%rsp)
   102bd:	c6 44 24 12 00       	movb   $0x0,0x12(%rsp)
   102c2:	c6 44 24 13 00       	movb   $0x0,0x13(%rsp)
   102c7:	c6 44 24 14 00       	movb   $0x0,0x14(%rsp)
   102cc:	c6 44 24 15 00       	movb   $0x0,0x15(%rsp)
   102d1:	c6 44 24 16 00       	movb   $0x0,0x16(%rsp)
   102d6:	c6 44 24 17 00       	movb   $0x0,0x17(%rsp)
   102db:	c6 44 24 18 00       	movb   $0x0,0x18(%rsp)
   102e0:	c6 44 24 19 00       	movb   $0x0,0x19(%rsp)
   102e5:	c6 44 24 1a 00       	movb   $0x0,0x1a(%rsp)
   102ea:	c6 44 24 1b 00       	movb   $0x0,0x1b(%rsp)
   102ef:	c6 44 24 1c 00       	movb   $0x0,0x1c(%rsp)
   102f4:	c6                   	.byte 0xc6
=== A=0x401256F F=0x1256f ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000001250f <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x13f>:
   1250f:	40 88 7d d6          	mov    %dil,-0x2a(%rbp)
   12513:	44 88 45 d7          	mov    %r8b,-0x29(%rbp)
   12517:	c6 45 c0 00          	movb   $0x0,-0x40(%rbp)
   1251b:	c6 45 c1 00          	movb   $0x0,-0x3f(%rbp)
   1251f:	c6 45 c2 00          	movb   $0x0,-0x3e(%rbp)
   12523:	c6 45 c3 00          	movb   $0x0,-0x3d(%rbp)
   12527:	c6 45 c4 00          	movb   $0x0,-0x3c(%rbp)
   1252b:	c6 45 c5 00          	movb   $0x0,-0x3b(%rbp)
   1252f:	c6 45 c6 00          	movb   $0x0,-0x3a(%rbp)
   12533:	c6 45 c7 00          	movb   $0x0,-0x39(%rbp)
   12537:	c6 45 c8 00          	movb   $0x0,-0x38(%rbp)
   1253b:	c6 45 c9 00          	movb   $0x0,-0x37(%rbp)
   1253f:	c6 45 ca 00          	movb   $0x0,-0x36(%rbp)
   12543:	c6 45 cb 00          	movb   $0x0,-0x35(%rbp)
   12547:	48 8d 45 90          	lea    -0x70(%rbp),%rax
   1254b:	48 89 04 24          	mov    %rax,(%rsp)
   1254f:	48 8d 75 cc          	lea    -0x34(%rbp),%rsi
   12553:	4c 89 e7             	mov    %r12,%rdi
   12556:	48 8b 55 a8          	mov    -0x58(%rbp),%rdx
   1255a:	48 8b 4d b8          	mov    -0x48(%rbp),%rcx
   1255e:	4d 89 f0             	mov    %r14,%r8
   12561:	4d 89 e9             	mov    %r13,%r9
   12564:	e8 87 1d 00 00       	call   142f0 <<chacha20poly1305::ChaChaPoly1305<C,N> as aead::AeadInPlace>::decrypt_in_place_detached>
   12569:	4c 8b 7d b0          	mov    -0x50(%rbp),%r15
   1256d:	84 c0                	test   %al,%al
   1256f:	74 58                	je     125c9 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x1f9>
   12571:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   12575:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   12579:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   1257d:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   12581:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   12585:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   12589:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   1258d:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   12591:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   12595:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   12599:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   1259d:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   125a1:	b8 01 00 00 00       	mov    $0x1,%eax
   125a6:	49 89 47 08          	mov    %rax,0x8(%r15)
   125aa:	49 c7 07 01 00 00 00 	movq   $0x1,(%r15)
   125b1:	4d 85 ed             	test   %r13,%r13
   125b4:	74 77                	je     1262d <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x25d>
   125b6:	ba 01 00 00 00       	mov    $0x1,%edx
   125bb:	4c 89 f7             	mov    %r14,%rdi
   125be:	4c 89 ee             	mov    %r13,%rsi
   125c1:	ff 15 b9 12 07 00    	call   *0x712b9(%rip)        # 83880 <_GLOBAL_OFFSET_TABLE_+0x300>
   125c7:	eb 64                	jmp    1262d <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x25d>
   125c9:	4c 89 6d b8          	mov    %r13,-0x48(%rbp)
   125cd:	49 8d 5c 24 40       	lea    0x40(%r12),%rbx
   125d2:	4c 8b 2b             	mov    (%rbx),%r13
   125d5:	49 ff c5             	inc    %r13
   125d8:	74 65                	je     1263f <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x26f>
   125da:	48 89 df             	mov    %rbx,%rdi
   125dd:	ff 15 e5 0f 07 00    	call   *0x70fe5(%rip)        # 835c8 <_GLOBAL_OFFSET_TABLE_+0x48>
   125e3:	4c 89 2b             	mov    %r13,(%rbx)
   125e6:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   125ea:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   125ee:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   125f2:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   125f6:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   125fa:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   125fe:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   12602:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   12606:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   1260a:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   1260e:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   12612:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   12616:	48 8b 45 b8          	mov    -0x48(%rbp),%rax
   1261a:	49 89 47 08          	mov    %rax,0x8(%r15)
   1261e:	4d 89 77 10          	mov    %r14,0x10(%r15)
   12622:	49 89 47 18          	mov    %rax,0x18(%r15)
   12626:	49 c7 07 00 00 00 00 	movq   $0x0,(%r15)
   1262d:	4c 89 f8             	mov    %r15,%rax
   12630:	48 83 c4 58          	add    $0x58,%rsp
   12634:	5b                   	pop    %rbx
   12635:	41 5c                	pop    %r12
   12637:	41 5d                	pop    %r13
   12639:	41 5e                	pop    %r14
   1263b:	41 5f                	pop    %r15
   1263d:	5d                   	pop    %rbp
   1263e:	c3                   	ret
   1263f:	41 c6 44 24 5e 01    	movb   $0x1,0x5e(%r12)
   12645:	eb 9f                	jmp    125e6 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x216>
   12647:	bf 01 00 00 00       	mov    $0x1,%edi
   1264c:	4c 89 fe             	mov    %r15,%rsi
   1264f:	ff 15 7b 14 07 00    	call   *0x7147b(%rip)        # 83ad0 <_GLOBAL_OFFSET_TABLE_+0x550>
   12655:	49 89 c7             	mov    %rax,%r15
   12658:	4c 89 2b             	mov    %r13,(%rbx)
   1265b:	eb 07                	jmp    12664 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x294>
   1265d:	4c 89 6d b8          	mov    %r13,-0x48(%rbp)
   12661:	49 89 c7             	mov    %rax,%r15
   12664:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   12668:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   1266c:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   12670:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   12674:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   12678:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   1267c:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   12680:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   12684:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   12688:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   1268c:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   12690:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   12694:	48 83 7d b8 00       	cmpq   $0x0,-0x48(%rbp)
   12699:	74 12                	je     126ad <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x2dd>
   1269b:	ba 01 00 00 00       	mov    $0x1,%edx
   126a0:	4c 89 f7             	mov    %r14,%rdi
   126a3:	48 8b 75 b8          	mov    -0x48(%rbp),%rsi
   126a7:	ff 15 d3 11 07 00    	call   *0x711d3(%rip)        # 83880 <_GLOBAL_OFFSET_TABLE_+0x300>
   126ad:	4c 89 ff             	mov    %r15,%rdi
   126b0:	e8 9b 39 ff ff       	call   6050 <_Unwind_Resume@plt>
   126b5:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   126bc:	00 00 00 
   126bf:	90                   	nop

00000000000126c0 <<hpke::aead::AeadKey<A> as core::ops::drop::Drop>::drop>:
   126c0:	55                   	push   %rbp
   126c1:	48 89 e5             	mov    %rsp,%rbp
   126c4:	c6 07 00             	movb   $0x0,(%rdi)
   126c7:	c6 47 01 00          	movb   $0x0,0x1(%rdi)
   126cb:	c6 47 02 00          	movb   $0x0,0x2(%rdi)
   126cf:	c6 47 03 00          	movb   $0x0,0x3(%rdi)
   126d3:	c6 47 04 00          	movb   $0x0,0x4(%rdi)
   126d7:	c6 47 05 00          	movb   $0x0,0x5(%rdi)
   126db:	c6 47 06 00          	movb   $0x0,0x6(%rdi)
   126df:	c6 47 07 00          	movb   $0x0,0x7(%rdi)
   126e3:	c6 47 08 00          	movb   $0x0,0x8(%rdi)
   126e7:	c6 47 09 00          	movb   $0x0,0x9(%rdi)
   126eb:	c6 47 0a 00          	movb   $0x0,0xa(%rdi)
   126ef:	c6 47 0b 00          	movb   $0x0,0xb(%rdi)
   126f3:	c6 47 0c 00          	movb   $0x0,0xc(%rdi)
   126f7:	c6 47 0d 00          	movb   $0x0,0xd(%rdi)
   126fb:	c6 47 0e 00          	movb   $0x0,0xe(%rdi)
   126ff:	c6 47 0f 00          	movb   $0x0,0xf(%rdi)
   12703:	c6 47 10 00          	movb   $0x0,0x10(%rdi)
   12707:	c6 47 11 00          	movb   $0x0,0x11(%rdi)
   1270b:	c6 47 12 00          	movb   $0x0,0x12(%rdi)
   1270f:	c6 47 13 00          	movb   $0x0,0x13(%rdi)
   12713:	c6 47 14 00          	movb   $0x0,0x14(%rdi)
   12717:	c6 47 15 00          	movb   $0x0,0x15(%rdi)
   1271b:	c6 47 16 00          	movb   $0x0,0x16(%rdi)
   1271f:	c6 47 17 00          	movb   $0x0,0x17(%rdi)
   12723:	c6 47 18 00          	movb   $0x0,0x18(%rdi)
   12727:	c6 47 19 00          	movb   $0x0,0x19(%rdi)
   1272b:	c6 47 1a 00          	movb   $0x0,0x1a(%rdi)
   1272f:	c6 47 1b 00          	movb   $0x0,0x1b(%rdi)
   12733:	c6 47 1c 00          	movb   $0x0,0x1c(%rdi)
   12737:	c6 47 1d 00          	movb   $0x0,0x1d(%rdi)
   1273b:	c6 47 1e 00          	movb   $0x0,0x1e(%rdi)
   1273f:	c6 47 1f 00          	movb   $0x0,0x1f(%rdi)
   12743:	5d                   	pop    %rbp
   12744:	c3                   	ret
   12745:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   1274c:	00 00 00 
   1274f:	90                   	nop

0000000000012750 <<cipher::stream_wrapper::StreamCipherCoreWrapper<T> as core::ops::drop::Drop>::drop>:
   12750:	55                   	push   %rbp
   12751:	48 89 e5             	mov    %rsp,%rbp
   12754:	c6 47 40 00          	movb   $0x0,0x40(%rdi)
   12758:	c6 47 41 00          	movb   $0x0,0x41(%rdi)
   1275c:	c6 47 42 00          	movb   $0x0,0x42(%rdi)
   12760:	c6 47 43 00          	movb   $0x0,0x43(%rdi)
   12764:	c6 47 44 00          	movb   $0x0,0x44(%rdi)
   12768:	c6 47 45 00          	movb   $0x0,0x45(%rdi)
   1276c:	c6                   	.byte 0xc6
   1276d:	47                   	rex.RXB
   1276e:	46                   	rex.RX
=== A=0x4012A33 F=0x12a33 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

00000000000129d3 <star_core::serv::enclave::Enclave::issue_token_plain+0x43>:
   129d3:	0f 29 4d 80          	movaps %xmm1,-0x80(%rbp)
   129d7:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
   129de:	41 0f 10 07          	movups (%r15),%xmm0
   129e2:	41 0f 10 4f 10       	movups 0x10(%r15),%xmm1
   129e7:	0f 29 85 50 ff ff ff 	movaps %xmm0,-0xb0(%rbp)
   129ee:	0f 29 8d 60 ff ff ff 	movaps %xmm1,-0xa0(%rbp)
   129f5:	49 8b 47 40          	mov    0x40(%r15),%rax
   129f9:	49 8b 77 48          	mov    0x48(%r15),%rsi
   129fd:	4d 8b 45 60          	mov    0x60(%r13),%r8
   12a01:	41 0f 10 45 40       	movups 0x40(%r13),%xmm0
   12a06:	41 0f 10 4d 50       	movups 0x50(%r13),%xmm1
   12a0b:	49 89 c5             	mov    %rax,%r13
   12a0e:	0f 29 4d c0          	movaps %xmm1,-0x40(%rbp)
   12a12:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
   12a16:	48 8d bd 50 ff ff ff 	lea    -0xb0(%rbp),%rdi
   12a1d:	4c 8d 4d b0          	lea    -0x50(%rbp),%r9
   12a21:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
   12a25:	48 89 c2             	mov    %rax,%rdx
   12a28:	4c 89 e1             	mov    %r12,%rcx
   12a2b:	ff 15 d7 12 07 00    	call   *0x712d7(%rip)        # 83d08 <_GLOBAL_OFFSET_TABLE_+0x788>
   12a31:	84 c0                	test   %al,%al
   12a33:	0f 84 9d 00 00 00    	je     12ad6 <star_core::serv::enclave::Enclave::issue_token_plain+0x146>
   12a39:	41 0f 10 07          	movups (%r15),%xmm0
   12a3d:	41 0f 10 4f 10       	movups 0x10(%r15),%xmm1
   12a42:	0f 29 4d c0          	movaps %xmm1,-0x40(%rbp)
   12a46:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
   12a4a:	48 8d 7d 90          	lea    -0x70(%rbp),%rdi
   12a4e:	48 8d 75 b0          	lea    -0x50(%rbp),%rsi
   12a52:	48 8b 55 d0          	mov    -0x30(%rbp),%rdx
   12a56:	4c 89 e9             	mov    %r13,%rcx
   12a59:	ff 15 b9 10 07 00    	call   *0x710b9(%rip)        # 83b18 <_GLOBAL_OFFSET_TABLE_+0x598>
   12a5f:	48 c7 45 b0 00 00 00 	movq   $0x0,-0x50(%rbp)
   12a66:	00 
   12a67:	48 c7 45 b8 01 00 00 	movq   $0x1,-0x48(%rbp)
   12a6e:	00 
   12a6f:	48 c7 45 c0 00 00 00 	movq   $0x0,-0x40(%rbp)
   12a76:	00 
   12a77:	48 8d 7d b0          	lea    -0x50(%rbp),%rdi
   12a7b:	ba 20 00 00 00       	mov    $0x20,%edx
   12a80:	b9 01 00 00 00       	mov    $0x1,%ecx
   12a85:	41 b8 01 00 00 00    	mov    $0x1,%r8d
   12a8b:	31 f6                	xor    %esi,%esi
   12a8d:	e8 8e 37 ff ff       	call   6220 <alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle>
   12a92:	4c 8b 65 b8          	mov    -0x48(%rbp),%r12
   12a96:	4c 8b 6d c0          	mov    -0x40(%rbp),%r13
   12a9a:	0f 10 45 90          	movups -0x70(%rbp),%xmm0
   12a9e:	0f 10 4d a0          	movups -0x60(%rbp),%xmm1
   12aa2:	43 0f 11 4c 2c 10    	movups %xmm1,0x10(%r12,%r13,1)
   12aa8:	43 0f 11 04 2c       	movups %xmm0,(%r12,%r13,1)
   12aad:	48 8b 45 b0          	mov    -0x50(%rbp),%rax
   12ab1:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   12ab5:	4c 89 ff             	mov    %r15,%rdi
   12ab8:	4c 89 f6             	mov    %r14,%rsi
   12abb:	ff 15 d7 0d 07 00    	call   *0x70dd7(%rip)        # 83898 <_GLOBAL_OFFSET_TABLE_+0x318>
   12ac1:	49 83 c5 20          	add    $0x20,%r13
   12ac5:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   12ac9:	48 89 03             	mov    %rax,(%rbx)
   12acc:	4c 89 63 08          	mov    %r12,0x8(%rbx)
   12ad0:	4c 89 6b 10          	mov    %r13,0x10(%rbx)
   12ad4:	eb 31                	jmp    12b07 <star_core::serv::enclave::Enclave::issue_token_plain+0x177>
   12ad6:	ff 15 b4 0d 07 00    	call   *0x70db4(%rip)        # 83890 <_GLOBAL_OFFSET_TABLE_+0x310>
   12adc:	bf 01 00 00 00       	mov    $0x1,%edi
   12ae1:	be 01 00 00 00       	mov    $0x1,%esi
   12ae6:	ff 15 c4 10 07 00    	call   *0x710c4(%rip)        # 83bb0 <_GLOBAL_OFFSET_TABLE_+0x630>
   12aec:	48 85 c0             	test   %rax,%rax
   12aef:	74 2b                	je     12b1c <star_core::serv::enclave::Enclave::issue_token_plain+0x18c>
   12af1:	c6 00 00             	movb   $0x0,(%rax)
   12af4:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   12afb:	48 89 43 08          	mov    %rax,0x8(%rbx)
   12aff:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   12b06:	00 
   12b07:	48 89 d8             	mov    %rbx,%rax
   12b0a:	48 81 c4 88 00 00 00 	add    $0x88,%rsp
   12b11:	5b                   	pop    %rbx
   12b12:	41 5c                	pop    %r12
   12b14:	41 5d                	pop    %r13
   12b16:	41 5e                	pop    %r14
   12b18:	41 5f                	pop    %r15
   12b1a:	5d                   	pop    %rbp
   12b1b:	c3                   	ret
   12b1c:	bf 01 00 00 00       	mov    $0x1,%edi
   12b21:	be 01 00 00 00       	mov    $0x1,%esi
   12b26:	ff 15 a4 0a 07 00    	call   *0x70aa4(%rip)        # 835d0 <_GLOBAL_OFFSET_TABLE_+0x50>
   12b2c:	48 89 c3             	mov    %rax,%rbx
   12b2f:	48 83 7d d0 00       	cmpq   $0x0,-0x30(%rbp)
   12b34:	74 29                	je     12b5f <star_core::serv::enclave::Enclave::issue_token_plain+0x1cf>
   12b36:	ba 01 00 00 00       	mov    $0x1,%edx
   12b3b:	4c 89 e7             	mov    %r12,%rdi
   12b3e:	48 8b 75 d0          	mov    -0x30(%rbp),%rsi
   12b42:	eb 15                	jmp    12b59 <star_core::serv::enclave::Enclave::issue_token_plain+0x1c9>
   12b44:	48 89 c3             	mov    %rax,%rbx
   12b47:	48 8b 75 b0          	mov    -0x50(%rbp),%rsi
   12b4b:	48 85 f6             	test   %rsi,%rsi
   12b4e:	74 0f                	je     12b5f <star_core::serv::enclave::Enclave::issue_token_plain+0x1cf>
   12b50:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   12b54:	ba 01 00 00 00       	mov    $0x1,%edx
   12b59:	ff 15 21 0d 07 00    	call   *0x70d21(%rip)        # 83880 <_GLOBAL_OFFSET_TABLE_+0x300>
   12b5f:	48 89 df             	mov    %rbx,%rdi
   12b62:	e8 e9 34 ff ff       	call   6050 <_Unwind_Resume@plt>
   12b67:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   12b6e:	00 00 

0000000000012b70 <star_core::serv::enclave::Enclave::new>:
   12b70:	55                   	push   %rbp
   12b71:	48 89 e5             	mov    %rsp,%rbp
   12b74:	41 57                	push   %r15
   12b76:	41 56                	push   %r14
   12b78:	53                   	push   %rbx
   12b79:	48 81 ec 08 01 00 00 	sub    $0x108,%rsp
   12b80:	49 89 d6             	mov    %rdx,%r14
   12b83:	41 89 f7             	mov    %esi,%r15d
   12b86:	48 89 fb             	mov    %rdi,%rbx
   12b89:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   12b90:	ba 20 00 00 00       	mov    $0x20,%edx
   12b95:	48 89 ce             	mov    %rcx,%rsi
   12b98:	ff 15 32 11 07 00    	call   *0x71132(%rip)        # 83cd0 <_GLOBAL_OFFSET_TABLE_+0x750>
   12b9e:	80 bd 60 ff ff ff 00 	cmpb   $0x0,-0xa0(%rbp)
   12ba5:	74 2b                	je     12bd2 <star_core::serv::enclave::Enclave::new+0x62>
   12ba7:	48 8b 85 78 ff ff ff 	mov    -0x88(%rbp),%rax
   12bae:	0f 10 85 68 ff ff ff 	movups -0x98(%rbp),%xmm0
   12bb5:	0f 11 85 27 ff ff ff 	movups %xmm0,-0xd9(%rbp)
   12bbc:	48 89 85 37 ff ff ff 	mov    %rax,-0xc9(%rbp)
   12bc3:	48 89 43 18          	mov    %rax,0x18(%rbx)
   12bc7:	0f 11 43 08          	movups %xmm0,0x8(%rbx)
   12bcb:	b0 01                	mov    $0x1,%al
   12bcd:	e9 a4 00 00 00       	jmp    12c76 <star_core::serv::enclave::Enclave::new+0x106>
   12bd2:	0f 10 85 61 ff ff ff 	movups -0x9f(%rbp),%xmm0
   12bd9:	0f 10 8d 71 ff ff ff 	movups -0x8f(%rbp),%xmm1
   12be0:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
   12be4:	0f 29 4d d0          	movaps %xmm1,-0x30(%rbp)
   12be8:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   12bef:	48 8d 75 c0          	lea    -0x40(%rbp),%rsi
   12bf3:	ff 15 a7 0e 07 00    	call   *0x70ea7(%rip)        # 83aa0 <_GLOBAL_OFFSET_TABLE_+0x520>
   12bf9:	0f 28 45 c0          	movaps -0x40(%rbp),%xmm0
   12bfd:	0f 28 4d d0          	movaps -0x30(%rbp),%xmm1
   12c01:	0f 29 8d 30 ff ff ff 	movaps %xmm1,-0xd0(%rbp)
   12c08:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   12c0f:	0f 10 95 00 ff ff ff 	movups -0x100(%rbp),%xmm2
   12c16:	0f 10 9d 10 ff ff ff 	movups -0xf0(%rbp),%xmm3
   12c1d:	0f 29 95 40 ff ff ff 	movaps %xmm2,-0xc0(%rbp)
   12c24:	0f 29 9d 50 ff ff ff 	movaps %xmm3,-0xb0(%rbp)
   12c2b:	0f 29 5d 90          	movaps %xmm3,-0x70(%rbp)
   12c2f:	0f 29 55 80          	movaps %xmm2,-0x80(%rbp)
=== A=0x40178F4 F=0x178f4 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000017894 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::derive_keypair+0x154>:
   17894:	00 66 2e             	add    %ah,0x2e(%rsi)
   17897:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   1789e:	00 
   1789f:	90                   	nop

00000000000178a0 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap>:
   178a0:	55                   	push   %rbp
   178a1:	48 89 e5             	mov    %rsp,%rbp
   178a4:	41 57                	push   %r15
   178a6:	41 56                	push   %r14
   178a8:	41 55                	push   %r13
   178aa:	41 54                	push   %r12
   178ac:	53                   	push   %rbx
   178ad:	48 81 ec c8 04 00 00 	sub    $0x4c8,%rsp
   178b4:	49 89 cc             	mov    %rcx,%r12
   178b7:	49 89 d6             	mov    %rdx,%r14
   178ba:	49 89 f7             	mov    %rsi,%r15
   178bd:	48 89 fb             	mov    %rdi,%rbx
   178c0:	c6 45 d4 4d          	movb   $0x4d,-0x2c(%rbp)
   178c4:	66 c7 45 d2 4b 45    	movw   $0x454b,-0x2e(%rbp)
   178ca:	66 c7 45 d5 00 20    	movw   $0x2000,-0x2b(%rbp)
   178d0:	4c 8d ad c1 fb ff ff 	lea    -0x43f(%rbp),%r13
   178d7:	4c 89 ef             	mov    %r13,%rdi
   178da:	48 89 ca             	mov    %rcx,%rdx
   178dd:	ff 15 9d bd 06 00    	call   *0x6bd9d(%rip)        # 83680 <_GLOBAL_OFFSET_TABLE_+0x100>
   178e3:	48 8d 35 da eb 04 00 	lea    0x4ebda(%rip),%rsi        # 664c4 <anon.dc9528e0958c4f8879bba0a5869e5a6f.18.llvm.3815164543114662682+0x7c>
   178ea:	4c 89 ef             	mov    %r13,%rdi
   178ed:	e8 3e fa ff ff       	call   17330 <<[T] as subtle::ConstantTimeEq>::ct_eq>
   178f2:	84 c0                	test   %al,%al
   178f4:	74 1d                	je     17913 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x73>
   178f6:	48 8d bd c1 fb ff ff 	lea    -0x43f(%rbp),%rdi
   178fd:	ff 15 75 bf 06 00    	call   *0x6bf75(%rip)        # 83878 <_GLOBAL_OFFSET_TABLE_+0x2f8>
   17903:	48 c7 43 08 06 00 00 	movq   $0x6,0x8(%rbx)
   1790a:	00 
   1790b:	c6 03 01             	movb   $0x1,(%rbx)
   1790e:	e9 55 04 00 00       	jmp    17d68 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x4c8>
   17913:	8b 85 c1 fb ff ff    	mov    -0x43f(%rbp),%eax
   17919:	8b 8d c4 fb ff ff    	mov    -0x43c(%rbp),%ecx
   1791f:	89 8d 73 ff ff ff    	mov    %ecx,-0x8d(%rbp)
   17925:	89 85 70 ff ff ff    	mov    %eax,-0x90(%rbp)
   1792b:	48 8b 85 c8 fb ff ff 	mov    -0x438(%rbp),%rax
   17932:	0f 10 85 d0 fb ff ff 	movups -0x430(%rbp),%xmm0
   17939:	0f 11 85 7f ff ff ff 	movups %xmm0,-0x81(%rbp)
   17940:	0f b6 8d e0 fb ff ff 	movzbl -0x420(%rbp),%ecx
   17947:	48 89 85 77 ff ff ff 	mov    %rax,-0x89(%rbp)
   1794e:	88 4d 8f             	mov    %cl,-0x71(%rbp)
   17951:	48 8d bd 50 ff ff ff 	lea    -0xb0(%rbp),%rdi
   17958:	4c 89 fe             	mov    %r15,%rsi
   1795b:	ff 15 47 bf 06 00    	call   *0x6bf47(%rip)        # 838a8 <_GLOBAL_OFFSET_TABLE_+0x328>
   17961:	4d 85 f6             	test   %r14,%r14
   17964:	0f 84 c6 00 00 00    	je     17a30 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x190>
   1796a:	4c 8d ad 20 fe ff ff 	lea    -0x1e0(%rbp),%r13
   17971:	ba 2f 01 00 00       	mov    $0x12f,%edx
   17976:	4c 89 ef             	mov    %r13,%rdi
   17979:	31 f6                	xor    %esi,%esi
   1797b:	ff 15 af bc 06 00    	call   *0x6bcaf(%rip)        # 83630 <memset@GLIBC_2.2.5>
   17981:	41 0f 10 04 24       	movups (%r12),%xmm0
   17986:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   1798c:	0f 10 95 50 ff ff ff 	movups -0xb0(%rbp),%xmm2
   17993:	0f 10 9d 60 ff ff ff 	movups -0xa0(%rbp),%xmm3
   1799a:	41 0f 10 26          	movups (%r14),%xmm4
   1799e:	41 0f 10 6e 10       	movups 0x10(%r14),%xmm5
   179a3:	0f 11 85 c1 fb ff ff 	movups %xmm0,-0x43f(%rbp)
   179aa:	0f 11 8d d1 fb ff ff 	movups %xmm1,-0x42f(%rbp)
   179b1:	0f 11 95 e1 fb ff ff 	movups %xmm2,-0x41f(%rbp)
   179b8:	0f 11 9d f1 fb ff ff 	movups %xmm3,-0x40f(%rbp)
   179bf:	0f 11 a5 01 fc ff ff 	movups %xmm4,-0x3ff(%rbp)
   179c6:	0f 11 ad 11 fc ff ff 	movups %xmm5,-0x3ef(%rbp)
   179cd:	48 8d bd 21 fc ff ff 	lea    -0x3df(%rbp),%rdi
   179d4:	ba 2f 01 00 00       	mov    $0x12f,%edx
   179d9:	4c 89 ee             	mov    %r13,%rsi
   179dc:	ff 15 86 c5 06 00    	call   *0x6c586(%rip)        # 83f68 <memcpy@GLIBC_2.14>
   179e2:	48 8d bd 20 fe ff ff 	lea    -0x1e0(%rbp),%rdi
   179e9:	4c 89 fe             	mov    %r15,%rsi
   179ec:	4c 89 f2             	mov    %r14,%rdx
   179ef:	ff 15 8b bc 06 00    	call   *0x6bc8b(%rip)        # 83680 <_GLOBAL_OFFSET_TABLE_+0x100>
   179f5:	48 8d 35 c8 ea 04 00 	lea    0x4eac8(%rip),%rsi        # 664c4 <anon.dc9528e0958c4f8879bba0a5869e5a6f.18.llvm.3815164543114662682+0x7c>
   179fc:	4c 8d b5 20 fe ff ff 	lea    -0x1e0(%rbp),%r14
   17a03:	4c 89 f7             	mov    %r14,%rdi
   17a06:	e8 25 f9 ff ff       	call   17330 <<[T] as subtle::ConstantTimeEq>::ct_eq>
   17a0b:	84 c0                	test   %al,%al
   17a0d:	0f 84 97 01 00 00    	je     17baa <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x30a>
   17a13:	48 8d bd 20 fe ff ff 	lea    -0x1e0(%rbp),%rdi
   17a1a:	ff 15 58 be 06 00    	call   *0x6be58(%rip)        # 83878 <_GLOBAL_OFFSET_TABLE_+0x2f8>
   17a20:	48 c7 43 08 06 00 00 	movq   $0x6,0x8(%rbx)
   17a27:	00 
   17a28:	c6 03 01             	movb   $0x1,(%rbx)
   17a2b:	e9 2b 03 00 00       	jmp    17d5b <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x4bb>
   17a30:	0f 57 c0             	xorps  %xmm0,%xmm0
   17a33:	0f 11 85 da fe ff ff 	movups %xmm0,-0x126(%rbp)
   17a3a:	0f 29 85 d0 fe ff ff 	movaps %xmm0,-0x130(%rbp)
   17a41:	0f 29 85 c0 fe ff ff 	movaps %xmm0,-0x140(%rbp)
   17a48:	0f 29 85 b0 fe ff ff 	movaps %xmm0,-0x150(%rbp)
   17a4f:	0f 29 85 a0 fe ff ff 	movaps %xmm0,-0x160(%rbp)
   17a56:	0f 29 85 90 fe ff ff 	movaps %xmm0,-0x170(%rbp)
   17a5d:	0f 29 85 80 fe ff ff 	movaps %xmm0,-0x180(%rbp)
   17a64:	0f 29 85 70 fe ff ff 	movaps %xmm0,-0x190(%rbp)
   17a6b:	0f 29 85 60 fe ff ff 	movaps %xmm0,-0x1a0(%rbp)
   17a72:	0f 29 85 50 fe ff ff 	movaps %xmm0,-0x1b0(%rbp)
   17a79:	0f 29 85 40 fe ff ff 	movaps %xmm0,-0x1c0(%rbp)
   17a80:	0f 29 85 30 fe ff ff 	movaps %xmm0,-0x1d0(%rbp)
   17a87:	0f 29 85 20 fe ff ff 	movaps %xmm0,-0x1e0(%rbp)
   17a8e:	41 0f 10 04 24       	movups (%r12),%xmm0
   17a93:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   17a99:	0f 10 95 50 ff ff ff 	movups -0xb0(%rbp),%xmm2
   17aa0:	0f 10 9d 60 ff ff ff 	movups -0xa0(%rbp),%xmm3
   17aa7:	0f 11 85 c1 fb ff ff 	movups %xmm0,-0x43f(%rbp)
   17aae:	0f 11 8d d1 fb ff ff 	movups %xmm1,-0x42f(%rbp)
   17ab5:	0f 11 95 e1 fb ff ff 	movups %xmm2,-0x41f(%rbp)
   17abc:	0f 11 9d f1 fb ff ff 	movups %xmm3,-0x40f(%rbp)
   17ac3:	48 8d bd 01 fc ff ff 	lea    -0x3ff(%rbp),%rdi
   17aca:	48 8d b5 20 fe ff ff 	lea    -0x1e0(%rbp),%rsi
   17ad1:	ba ca 00 00 00       	mov    $0xca,%edx
   17ad6:	ff 15 8c c4 06 00    	call   *0x6c48c(%rip)        # 83f68 <memcpy@GLIBC_2.14>
   17adc:	0f 57 c0             	xorps  %xmm0,%xmm0
   17adf:	0f 29 45 a0          	movaps %xmm0,-0x60(%rbp)
   17ae3:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   17ae7:	0f 10 85 70 ff ff ff 	movups -0x90(%rbp),%xmm0
   17aee:	0f 10 4d 80          	movups -0x80(%rbp),%xmm1
   17af2:	0f                   	.byte 0xf
   17af3:	11                   	.byte 0x11
