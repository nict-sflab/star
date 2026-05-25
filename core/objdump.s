=== A=0x401C544 F=0x1c544 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000001c4e4 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::derive_keypair+0x34>:
   1c4e4:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   1c4eb:	00 
   1c4ec:	0f 1f 40 00          	nopl   0x0(%rax)

000000000001c4f0 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap>:
   1c4f0:	55                   	push   %rbp
   1c4f1:	48 89 e5             	mov    %rsp,%rbp
   1c4f4:	41 57                	push   %r15
   1c4f6:	41 56                	push   %r14
   1c4f8:	41 55                	push   %r13
   1c4fa:	41 54                	push   %r12
   1c4fc:	53                   	push   %rbx
   1c4fd:	48 81 ec 68 05 00 00 	sub    $0x568,%rsp
   1c504:	49 89 cc             	mov    %rcx,%r12
   1c507:	49 89 d6             	mov    %rdx,%r14
   1c50a:	49 89 f7             	mov    %rsi,%r15
   1c50d:	48 89 fb             	mov    %rdi,%rbx
   1c510:	c6 45 d4 4d          	movb   $0x4d,-0x2c(%rbp)
   1c514:	66 c7 45 d2 4b 45    	movw   $0x454b,-0x2e(%rbp)
   1c51a:	66 c7 45 d5 00 20    	movw   $0x2000,-0x2b(%rbp)
   1c520:	4c 8d ad e0 fd ff ff 	lea    -0x220(%rbp),%r13
   1c527:	4c 89 ef             	mov    %r13,%rdi
   1c52a:	48 89 ca             	mov    %rcx,%rdx
   1c52d:	ff 15 45 53 07 00    	call   *0x75345(%rip)        # 91878 <_GLOBAL_OFFSET_TABLE_+0x360>
   1c533:	48 8d 35 ea 59 05 00 	lea    0x559ea(%rip),%rsi        # 71f24 <anon.39a2a9f1401edcf2cd24a86c78a26440.25.llvm.12169493792077094199>
   1c53a:	4c 89 ef             	mov    %r13,%rdi
   1c53d:	e8 ce e0 ff ff       	call   1a610 <<[T] as subtle::ConstantTimeEq>::ct_eq>
   1c542:	84 c0                	test   %al,%al
   1c544:	74 1d                	je     1c563 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x73>
   1c546:	48 8d bd e0 fd ff ff 	lea    -0x220(%rbp),%rdi
   1c54d:	ff 15 6d 5a 07 00    	call   *0x75a6d(%rip)        # 91fc0 <_GLOBAL_OFFSET_TABLE_+0xaa8>
   1c553:	48 c7 43 08 06 00 00 	movq   $0x6,0x8(%rbx)
   1c55a:	00 
   1c55b:	c6 03 01             	movb   $0x1,(%rbx)
   1c55e:	e9 9a 04 00 00       	jmp    1c9fd <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x50d>
   1c563:	8b 85 e0 fd ff ff    	mov    -0x220(%rbp),%eax
   1c569:	8b 8d e3 fd ff ff    	mov    -0x21d(%rbp),%ecx
   1c56f:	89 4d 93             	mov    %ecx,-0x6d(%rbp)
   1c572:	89 45 90             	mov    %eax,-0x70(%rbp)
   1c575:	48 8b 85 e7 fd ff ff 	mov    -0x219(%rbp),%rax
   1c57c:	0f 10 85 ef fd ff ff 	movups -0x211(%rbp),%xmm0
   1c583:	0f 11 45 9f          	movups %xmm0,-0x61(%rbp)
   1c587:	0f b6 8d ff fd ff ff 	movzbl -0x201(%rbp),%ecx
   1c58e:	48 89 45 97          	mov    %rax,-0x69(%rbp)
   1c592:	88 4d af             	mov    %cl,-0x51(%rbp)
   1c595:	48 8d bd c0 fd ff ff 	lea    -0x240(%rbp),%rdi
   1c59c:	4c 89 fe             	mov    %r15,%rsi
   1c59f:	ff 15 73 53 07 00    	call   *0x75373(%rip)        # 91918 <_GLOBAL_OFFSET_TABLE_+0x400>
   1c5a5:	4d 85 f6             	test   %r14,%r14
   1c5a8:	0f 84 00 01 00 00    	je     1c6ae <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x1be>
   1c5ae:	48 8d bd 40 fe ff ff 	lea    -0x1c0(%rbp),%rdi
   1c5b5:	ba 2f 01 00 00       	mov    $0x12f,%edx
   1c5ba:	31 f6                	xor    %esi,%esi
   1c5bc:	ff 15 1e 50 07 00    	call   *0x7501e(%rip)        # 915e0 <memset@GLIBC_2.2.5>
   1c5c2:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   1c5c9:	4c 89 e6             	mov    %r12,%rsi
   1c5cc:	ff 15 b6 59 07 00    	call   *0x759b6(%rip)        # 91f88 <_GLOBAL_OFFSET_TABLE_+0xa70>
   1c5d2:	0f 10 85 31 fc ff ff 	movups -0x3cf(%rbp),%xmm0
   1c5d9:	0f 10 8d 41 fc ff ff 	movups -0x3bf(%rbp),%xmm1
   1c5e0:	0f 29 8d f0 fd ff ff 	movaps %xmm1,-0x210(%rbp)
   1c5e7:	0f 29 85 e0 fd ff ff 	movaps %xmm0,-0x220(%rbp)
   1c5ee:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   1c5f5:	48 8d b5 c0 fd ff ff 	lea    -0x240(%rbp),%rsi
   1c5fc:	ff 15 86 59 07 00    	call   *0x75986(%rip)        # 91f88 <_GLOBAL_OFFSET_TABLE_+0xa70>
   1c602:	0f 10 85 31 fc ff ff 	movups -0x3cf(%rbp),%xmm0
   1c609:	0f 10 8d 41 fc ff ff 	movups -0x3bf(%rbp),%xmm1
   1c610:	0f 29 8d 10 fe ff ff 	movaps %xmm1,-0x1f0(%rbp)
   1c617:	0f 29 85 00 fe ff ff 	movaps %xmm0,-0x200(%rbp)
   1c61e:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   1c625:	4c 89 f6             	mov    %r14,%rsi
   1c628:	ff 15 5a 59 07 00    	call   *0x7595a(%rip)        # 91f88 <_GLOBAL_OFFSET_TABLE_+0xa70>
   1c62e:	0f 10 85 31 fc ff ff 	movups -0x3cf(%rbp),%xmm0
   1c635:	0f 10 8d 41 fc ff ff 	movups -0x3bf(%rbp),%xmm1
   1c63c:	0f 29 8d 30 fe ff ff 	movaps %xmm1,-0x1d0(%rbp)
   1c643:	0f 29 85 20 fe ff ff 	movaps %xmm0,-0x1e0(%rbp)
   1c64a:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   1c651:	48 8d b5 e0 fd ff ff 	lea    -0x220(%rbp),%rsi
   1c658:	ba 8f 01 00 00       	mov    $0x18f,%edx
   1c65d:	ff 15 15 59 07 00    	call   *0x75915(%rip)        # 91f78 <memcpy@GLIBC_2.14>
   1c663:	48 8d bd e0 fd ff ff 	lea    -0x220(%rbp),%rdi
   1c66a:	4c 89 fe             	mov    %r15,%rsi
   1c66d:	4c 89 f2             	mov    %r14,%rdx
   1c670:	ff 15 02 52 07 00    	call   *0x75202(%rip)        # 91878 <_GLOBAL_OFFSET_TABLE_+0x360>
   1c676:	48 8d 35 a7 58 05 00 	lea    0x558a7(%rip),%rsi        # 71f24 <anon.39a2a9f1401edcf2cd24a86c78a26440.25.llvm.12169493792077094199>
   1c67d:	48 8d bd e0 fd ff ff 	lea    -0x220(%rbp),%rdi
   1c684:	e8 87 df ff ff       	call   1a610 <<[T] as subtle::ConstantTimeEq>::ct_eq>
   1c689:	84 c0                	test   %al,%al
   1c68b:	0f 84 a7 01 00 00    	je     1c838 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x348>
   1c691:	48 8d bd e0 fd ff ff 	lea    -0x220(%rbp),%rdi
   1c698:	ff 15 22 59 07 00    	call   *0x75922(%rip)        # 91fc0 <_GLOBAL_OFFSET_TABLE_+0xaa8>
   1c69e:	48 c7 43 08 06 00 00 	movq   $0x6,0x8(%rbx)
   1c6a5:	00 
   1c6a6:	c6 03 01             	movb   $0x1,(%rbx)
   1c6a9:	e9 45 03 00 00       	jmp    1c9f3 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x503>
   1c6ae:	0f 57 c0             	xorps  %xmm0,%xmm0
   1c6b1:	0f 11 85 da fe ff ff 	movups %xmm0,-0x126(%rbp)
   1c6b8:	0f 11 85 d0 fe ff ff 	movups %xmm0,-0x130(%rbp)
   1c6bf:	0f 11 85 c0 fe ff ff 	movups %xmm0,-0x140(%rbp)
   1c6c6:	0f 11 85 b0 fe ff ff 	movups %xmm0,-0x150(%rbp)
   1c6cd:	0f 11 85 a0 fe ff ff 	movups %xmm0,-0x160(%rbp)
   1c6d4:	0f 11 85 90 fe ff ff 	movups %xmm0,-0x170(%rbp)
   1c6db:	0f 11 85 80 fe ff ff 	movups %xmm0,-0x180(%rbp)
   1c6e2:	0f 11 85 70 fe ff ff 	movups %xmm0,-0x190(%rbp)
   1c6e9:	0f 11 85 60 fe ff ff 	movups %xmm0,-0x1a0(%rbp)
   1c6f0:	0f 11 85 50 fe ff ff 	movups %xmm0,-0x1b0(%rbp)
   1c6f7:	0f 11 85 40 fe ff ff 	movups %xmm0,-0x1c0(%rbp)
   1c6fe:	0f 11 85 30 fe ff ff 	movups %xmm0,-0x1d0(%rbp)
   1c705:	0f 11 85 20 fe ff ff 	movups %xmm0,-0x1e0(%rbp)
   1c70c:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   1c713:	4c 89 e6             	mov    %r12,%rsi
   1c716:	ff 15 6c 58 07 00    	call   *0x7586c(%rip)        # 91f88 <_GLOBAL_OFFSET_TABLE_+0xa70>
   1c71c:	0f 10 85 31 fc ff ff 	movups -0x3cf(%rbp),%xmm0
   1c723:	0f 10 8d 41 fc ff ff 	movups -0x3bf(%rbp),%xmm1
   1c72a:	0f 29 8d f0 fd ff ff 	movaps %xmm1,-0x210(%rbp)
   1c731:	0f 29 85 e0 fd ff ff 	movaps %xmm0,-0x220(%rbp)
   1c738:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   1c73f:	48                   	rex.W
   1c740:	8d                   	.byte 0x8d
   1c741:	b5 c0                	mov    $0xc0,%ch
   1c743:	fd                   	std
=== A=0x4014DB5 F=0x14db5 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000014d55 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x4b5>:
   14d55:	67 c9                	addr32 leave
   14d57:	07                   	(bad)
   14d58:	00 0f                	add    %cl,(%rdi)
   14d5a:	b6 4c                	mov    $0x4c,%dh
   14d5c:	24 10                	and    $0x10,%al
   14d5e:	22 4c 24 0f          	and    0xf(%rsp),%cl
   14d62:	0f b6 54 24 0e       	movzbl 0xe(%rsp),%edx
   14d67:	22 54 24 0d          	and    0xd(%rsp),%dl
   14d6b:	20 ca                	and    %cl,%dl
   14d6d:	44 22 74 24 0c       	and    0xc(%rsp),%r14b
   14d72:	44 22 74 24 0b       	and    0xb(%rsp),%r14b
   14d77:	41 20 d6             	and    %dl,%r14b
   14d7a:	44 22 7c 24 0a       	and    0xa(%rsp),%r15b
   14d7f:	44 22 7c 24 09       	and    0x9(%rsp),%r15b
   14d84:	44 22 7c 24 08       	and    0x8(%rsp),%r15b
   14d89:	45 20 f7             	and    %r14b,%r15b
   14d8c:	44 22 6c 24 07       	and    0x7(%rsp),%r13b
   14d91:	44 22 6c 24 06       	and    0x6(%rsp),%r13b
   14d96:	44 22 6c 24 05       	and    0x5(%rsp),%r13b
   14d9b:	41 20 c5             	and    %al,%r13b
   14d9e:	45 20 fd             	and    %r15b,%r13b
   14da1:	41 80 e5 01          	and    $0x1,%r13b
   14da5:	41 0f b6 fd          	movzbl %r13b,%edi
   14da9:	ff 15 11 c9 07 00    	call   *0x7c911(%rip)        # 916c0 <_GLOBAL_OFFSET_TABLE_+0x1a8>
   14daf:	84 c0                	test   %al,%al
   14db1:	41 0f 94 c6          	sete   %r14b
   14db5:	74 16                	je     14dcd <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x52d>
   14db7:	48 8d bb 40 02 00 00 	lea    0x240(%rbx),%rdi
   14dbe:	48 8b 74 24 30       	mov    0x30(%rsp),%rsi
   14dc3:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   14dc8:	e8 13 04 00 00       	call   151e0 <cipher::stream::StreamCipher::apply_keystream>
   14dcd:	4c 8d bb 40 02 00 00 	lea    0x240(%rbx),%r15
   14dd4:	4c 89 ff             	mov    %r15,%rdi
   14dd7:	e8 54 1f 00 00       	call   16d30 <<cipher::stream_wrapper::StreamCipherCoreWrapper<T> as core::ops::drop::Drop>::drop>
   14ddc:	41 c7 07 00 00 00 00 	movl   $0x0,(%r15)
   14de3:	c7 83 44 02 00 00 00 	movl   $0x0,0x244(%rbx)
   14dea:	00 00 00 
   14ded:	c7 83 48 02 00 00 00 	movl   $0x0,0x248(%rbx)
   14df4:	00 00 00 
   14df7:	c7 83 4c 02 00 00 00 	movl   $0x0,0x24c(%rbx)
   14dfe:	00 00 00 
   14e01:	c7 83 50 02 00 00 00 	movl   $0x0,0x250(%rbx)
   14e08:	00 00 00 
   14e0b:	c7 83 54 02 00 00 00 	movl   $0x0,0x254(%rbx)
   14e12:	00 00 00 
   14e15:	c7 83 58 02 00 00 00 	movl   $0x0,0x258(%rbx)
   14e1c:	00 00 00 
   14e1f:	c7 83 5c 02 00 00 00 	movl   $0x0,0x25c(%rbx)
   14e26:	00 00 00 
   14e29:	c7 83 60 02 00 00 00 	movl   $0x0,0x260(%rbx)
   14e30:	00 00 00 
   14e33:	c7 83 64 02 00 00 00 	movl   $0x0,0x264(%rbx)
   14e3a:	00 00 00 
   14e3d:	c7 83 68 02 00 00 00 	movl   $0x0,0x268(%rbx)
   14e44:	00 00 00 
   14e47:	c7 83 6c 02 00 00 00 	movl   $0x0,0x26c(%rbx)
   14e4e:	00 00 00 
   14e51:	c7 83 70 02 00 00 00 	movl   $0x0,0x270(%rbx)
   14e58:	00 00 00 
   14e5b:	c7 83 74 02 00 00 00 	movl   $0x0,0x274(%rbx)
   14e62:	00 00 00 
   14e65:	c7 83 78 02 00 00 00 	movl   $0x0,0x278(%rbx)
   14e6c:	00 00 00 
   14e6f:	c7 83 7c 02 00 00 00 	movl   $0x0,0x27c(%rbx)
   14e76:	00 00 00 
   14e79:	44 89 f0             	mov    %r14d,%eax
   14e7c:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   14e80:	5b                   	pop    %rbx
   14e81:	41 5c                	pop    %r12
   14e83:	41 5d                	pop    %r13
   14e85:	41 5e                	pop    %r14
   14e87:	41 5f                	pop    %r15
   14e89:	5d                   	pop    %rbp
   14e8a:	c3                   	ret
   14e8b:	eb 3e                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14e8d:	eb 3c                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14e8f:	eb 3a                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14e91:	eb 38                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14e93:	eb 36                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14e95:	eb 34                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14e97:	49 89 c6             	mov    %rax,%r14
   14e9a:	4c 89 ff             	mov    %r15,%rdi
   14e9d:	e8 9e 2f 00 00       	call   17e40 <<chacha20::ChaChaCore<R> as core::ops::drop::Drop>::drop>
   14ea2:	eb 32                	jmp    14ed6 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x636>
   14ea4:	ff 15 4e d0 07 00    	call   *0x7d04e(%rip)        # 91ef8 <_GLOBAL_OFFSET_TABLE_+0x9e0>
   14eaa:	49 89 c6             	mov    %rax,%r14
   14ead:	4c 89 ff             	mov    %r15,%rdi
   14eb0:	e8 8b 2f 00 00       	call   17e40 <<chacha20::ChaChaCore<R> as core::ops::drop::Drop>::drop>
   14eb5:	eb 1f                	jmp    14ed6 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x636>
   14eb7:	ff 15 3b d0 07 00    	call   *0x7d03b(%rip)        # 91ef8 <_GLOBAL_OFFSET_TABLE_+0x9e0>
   14ebd:	eb 0c                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14ebf:	eb 0a                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14ec1:	eb 08                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14ec3:	eb 06                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14ec5:	eb 04                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14ec7:	eb 02                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14ec9:	eb 00                	jmp    14ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   14ecb:	49 89 c6             	mov    %rax,%r14
   14ece:	48 89 df             	mov    %rbx,%rdi
   14ed1:	e8 2a 02 00 00       	call   15100 <core::ptr::drop_in_place<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm,typenum::bit::B1>,typenum::bit::B0>,typenum::bit::B1>,typenum::bit::B0>>>>>>
   14ed6:	4c 89 f7             	mov    %r14,%rdi
   14ed9:	e8 72 31 ff ff       	call   8050 <_Unwind_Resume@plt>
   14ede:	ff 15 14 d0 07 00    	call   *0x7d014(%rip)        # 91ef8 <_GLOBAL_OFFSET_TABLE_+0x9e0>
   14ee4:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   14eeb:	00 00 00 
   14eee:	66 90                	xchg   %ax,%ax

0000000000014ef0 <chacha20poly1305::cipher::Cipher<C>::new>:
   14ef0:	55                   	push   %rbp
   14ef1:	48 89 e5             	mov    %rsp,%rbp
   14ef4:	41 57                	push   %r15
   14ef6:	41 56                	push   %r14
   14ef8:	53                   	push   %rbx
   14ef9:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   14efd:	48 81 ec 80 02 00 00 	sub    $0x280,%rsp
   14f04:	49 89 f6             	mov    %rsi,%r14
   14f07:	48 89 fb             	mov    %rdi,%rbx
   14f0a:	0f 57 c0             	xorps  %xmm0,%xmm0
   14f0d:	0f 29 44 24 10       	movaps %xmm0,0x10(%rsp)
   14f12:	0f 29 04 24          	movaps %xmm0,(%rsp)
   14f16:	48 89 e6             	mov    %rsp,%rsi
   14f19:	ba 20 00 00 00       	mov    $0x20,%edx
   14f1e:	4c 89 f7             	mov    %r14,%rdi
   14f21:	e8 ba 02 00 00       	call   151e0 <cipher::stream::StreamCipher::apply_keystream>
   14f26:	48 8d 7c 24 20       	lea    0x20(%rsp),%rdi
   14f2b:	48 89 e6             	mov    %rsp,%rsi
   14f2e:	ff 15 14 cb 07 00    	call   *0x7cb14(%rip)        # 91a48 <_GLOBAL_OFFSET_TABLE_+0x530>
   14f34:	c6 04 24 00          	movb   $0x0,(%rsp)
   14f38:	c6 44 24 01 00       	movb   $0x0,0x1(%rsp)
   14f3d:	c6 44 24 02 00       	movb   $0x0,0x2(%rsp)
   14f42:	c6 44 24 03 00       	movb   $0x0,0x3(%rsp)
   14f47:	c6 44 24 04 00       	movb   $0x0,0x4(%rsp)
   14f4c:	c6 44 24 05 00       	movb   $0x0,0x5(%rsp)
   14f51:	c6 44 24 06 00       	movb   $0x0,0x6(%rsp)
   14f56:	c6 44 24 07 00       	movb   $0x0,0x7(%rsp)
   14f5b:	c6 44 24 08 00       	movb   $0x0,0x8(%rsp)
   14f60:	c6 44 24 09 00       	movb   $0x0,0x9(%rsp)
   14f65:	c6 44 24 0a 00       	movb   $0x0,0xa(%rsp)
   14f6a:	c6 44 24 0b 00       	movb   $0x0,0xb(%rsp)
   14f6f:	c6 44 24 0c 00       	movb   $0x0,0xc(%rsp)
   14f74:	c6 44 24 0d 00       	movb   $0x0,0xd(%rsp)
   14f79:	c6 44 24 0e 00       	movb   $0x0,0xe(%rsp)
   14f7e:	c6 44 24 0f 00       	movb   $0x0,0xf(%rsp)
   14f83:	c6 44 24 10 00       	movb   $0x0,0x10(%rsp)
   14f88:	c6 44 24 11 00       	movb   $0x0,0x11(%rsp)
   14f8d:	c6 44 24 12 00       	movb   $0x0,0x12(%rsp)
   14f92:	c6 44 24 13 00       	movb   $0x0,0x13(%rsp)
   14f97:	c6 44 24 14 00       	movb   $0x0,0x14(%rsp)
   14f9c:	c6 44 24 15 00       	movb   $0x0,0x15(%rsp)
   14fa1:	c6 44 24 16 00       	movb   $0x0,0x16(%rsp)
   14fa6:	c6 44 24 17 00       	movb   $0x0,0x17(%rsp)
   14fab:	c6 44 24 18 00       	movb   $0x0,0x18(%rsp)
   14fb0:	c6 44 24 19 00       	movb   $0x0,0x19(%rsp)
=== A=0x4017C85 F=0x17c85 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000017c25 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x155>:
   17c25:	88 75 d5             	mov    %dh,-0x2b(%rbp)
   17c28:	44 88 45 d6          	mov    %r8b,-0x2a(%rbp)
   17c2c:	44 88 4d d7          	mov    %r9b,-0x29(%rbp)
   17c30:	c6 45 c0 00          	movb   $0x0,-0x40(%rbp)
   17c34:	c6 45 c1 00          	movb   $0x0,-0x3f(%rbp)
   17c38:	c6 45 c2 00          	movb   $0x0,-0x3e(%rbp)
   17c3c:	c6 45 c3 00          	movb   $0x0,-0x3d(%rbp)
   17c40:	c6 45 c4 00          	movb   $0x0,-0x3c(%rbp)
   17c44:	c6 45 c5 00          	movb   $0x0,-0x3b(%rbp)
   17c48:	c6 45 c6 00          	movb   $0x0,-0x3a(%rbp)
   17c4c:	c6 45 c7 00          	movb   $0x0,-0x39(%rbp)
   17c50:	c6 45 c8 00          	movb   $0x0,-0x38(%rbp)
   17c54:	c6 45 c9 00          	movb   $0x0,-0x37(%rbp)
   17c58:	c6 45 ca 00          	movb   $0x0,-0x36(%rbp)
   17c5c:	c6 45 cb 00          	movb   $0x0,-0x35(%rbp)
   17c60:	48 8d 45 90          	lea    -0x70(%rbp),%rax
   17c64:	48 89 04 24          	mov    %rax,(%rsp)
   17c68:	48 8d 75 cc          	lea    -0x34(%rbp),%rsi
   17c6c:	48 8b 55 a8          	mov    -0x58(%rbp),%rdx
   17c70:	48 8b 4d a0          	mov    -0x60(%rbp),%rcx
   17c74:	4d 89 f0             	mov    %r14,%r8
   17c77:	49 89 d9             	mov    %rbx,%r9
   17c7a:	e8 f1 22 00 00       	call   19f70 <<chacha20poly1305::ChaChaPoly1305<C,N> as aead::AeadInPlace>::decrypt_in_place_detached>
   17c7f:	84 c0                	test   %al,%al
   17c81:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   17c85:	74 63                	je     17cea <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x21a>
   17c87:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   17c8b:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   17c8f:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   17c93:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   17c97:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   17c9b:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   17c9f:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   17ca3:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   17ca7:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   17cab:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   17caf:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   17cb3:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   17cb7:	b8 01 00 00 00       	mov    $0x1,%eax
   17cbc:	48 89 47 08          	mov    %rax,0x8(%rdi)
   17cc0:	48 c7 07 01 00 00 00 	movq   $0x1,(%rdi)
   17cc7:	48 85 db             	test   %rbx,%rbx
   17cca:	0f 84 26 fe ff ff    	je     17af6 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x26>
   17cd0:	ba 01 00 00 00       	mov    $0x1,%edx
   17cd5:	4c 89 f7             	mov    %r14,%rdi
   17cd8:	48 89 de             	mov    %rbx,%rsi
   17cdb:	ff 15 e7 9b 07 00    	call   *0x79be7(%rip)        # 918c8 <_GLOBAL_OFFSET_TABLE_+0x3b0>
   17ce1:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   17ce5:	e9 0c fe ff ff       	jmp    17af6 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x26>
   17cea:	48 8b 45 b0          	mov    -0x50(%rbp),%rax
   17cee:	48 8b 48 40          	mov    0x40(%rax),%rcx
   17cf2:	48 83 f9 ff          	cmp    $0xffffffffffffffff,%rcx
   17cf6:	74 57                	je     17d4f <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x27f>
   17cf8:	48 ff c1             	inc    %rcx
   17cfb:	48 c7 40 40 00 00 00 	movq   $0x0,0x40(%rax)
   17d02:	00 
   17d03:	48 89 48 40          	mov    %rcx,0x40(%rax)
   17d07:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   17d0b:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   17d0f:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   17d13:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   17d17:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   17d1b:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   17d1f:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   17d23:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   17d27:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   17d2b:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   17d2f:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   17d33:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   17d37:	48 89 5f 08          	mov    %rbx,0x8(%rdi)
   17d3b:	4c 89 77 10          	mov    %r14,0x10(%rdi)
   17d3f:	48 89 5f 18          	mov    %rbx,0x18(%rdi)
   17d43:	48 c7 07 00 00 00 00 	movq   $0x0,(%rdi)
   17d4a:	e9 a7 fd ff ff       	jmp    17af6 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x26>
   17d4f:	c6 40 5e 01          	movb   $0x1,0x5e(%rax)
   17d53:	eb b2                	jmp    17d07 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x237>
   17d55:	49 89 c7             	mov    %rax,%r15
   17d58:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   17d5c:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   17d60:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   17d64:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   17d68:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   17d6c:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   17d70:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   17d74:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   17d78:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   17d7c:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   17d80:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   17d84:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   17d88:	48 85 db             	test   %rbx,%rbx
   17d8b:	74 11                	je     17d9e <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x2ce>
   17d8d:	ba 01 00 00 00       	mov    $0x1,%edx
   17d92:	4c 89 f7             	mov    %r14,%rdi
   17d95:	48 89 de             	mov    %rbx,%rsi
   17d98:	ff 15 2a 9b 07 00    	call   *0x79b2a(%rip)        # 918c8 <_GLOBAL_OFFSET_TABLE_+0x3b0>
   17d9e:	4c 89 ff             	mov    %r15,%rdi
   17da1:	e8 aa 02 ff ff       	call   8050 <_Unwind_Resume@plt>
   17da6:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   17dad:	00 00 00 

0000000000017db0 <<hpke::aead::AeadKey<A> as core::ops::drop::Drop>::drop>:
   17db0:	55                   	push   %rbp
   17db1:	48 89 e5             	mov    %rsp,%rbp
   17db4:	c6 07 00             	movb   $0x0,(%rdi)
   17db7:	c6 47 01 00          	movb   $0x0,0x1(%rdi)
   17dbb:	c6 47 02 00          	movb   $0x0,0x2(%rdi)
   17dbf:	c6 47 03 00          	movb   $0x0,0x3(%rdi)
   17dc3:	c6 47 04 00          	movb   $0x0,0x4(%rdi)
   17dc7:	c6 47 05 00          	movb   $0x0,0x5(%rdi)
   17dcb:	c6 47 06 00          	movb   $0x0,0x6(%rdi)
   17dcf:	c6 47 07 00          	movb   $0x0,0x7(%rdi)
   17dd3:	c6 47 08 00          	movb   $0x0,0x8(%rdi)
   17dd7:	c6 47 09 00          	movb   $0x0,0x9(%rdi)
   17ddb:	c6 47 0a 00          	movb   $0x0,0xa(%rdi)
   17ddf:	c6 47 0b 00          	movb   $0x0,0xb(%rdi)
   17de3:	c6 47 0c 00          	movb   $0x0,0xc(%rdi)
   17de7:	c6 47 0d 00          	movb   $0x0,0xd(%rdi)
   17deb:	c6 47 0e 00          	movb   $0x0,0xe(%rdi)
   17def:	c6 47 0f 00          	movb   $0x0,0xf(%rdi)
   17df3:	c6 47 10 00          	movb   $0x0,0x10(%rdi)
   17df7:	c6 47 11 00          	movb   $0x0,0x11(%rdi)
   17dfb:	c6 47 12 00          	movb   $0x0,0x12(%rdi)
   17dff:	c6 47 13 00          	movb   $0x0,0x13(%rdi)
   17e03:	c6 47 14 00          	movb   $0x0,0x14(%rdi)
   17e07:	c6 47 15 00          	movb   $0x0,0x15(%rdi)
   17e0b:	c6 47 16 00          	movb   $0x0,0x16(%rdi)
   17e0f:	c6 47 17 00          	movb   $0x0,0x17(%rdi)
   17e13:	c6 47 18 00          	movb   $0x0,0x18(%rdi)
   17e17:	c6 47 19 00          	movb   $0x0,0x19(%rdi)
   17e1b:	c6 47 1a 00          	movb   $0x0,0x1a(%rdi)
   17e1f:	c6 47 1b 00          	movb   $0x0,0x1b(%rdi)
   17e23:	c6 47 1c 00          	movb   $0x0,0x1c(%rdi)
   17e27:	c6 47 1d 00          	movb   $0x0,0x1d(%rdi)
   17e2b:	c6 47 1e 00          	movb   $0x0,0x1e(%rdi)
   17e2f:	c6 47 1f 00          	movb   $0x0,0x1f(%rdi)
   17e33:	5d                   	pop    %rbp
   17e34:	c3                   	ret
   17e35:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   17e3c:	00 00 00 
   17e3f:	90                   	nop

0000000000017e40 <<chacha20::ChaChaCore<R> as core::ops::drop::Drop>::drop>:
   17e40:	55                   	push   %rbp
   17e41:	48 89 e5             	mov    %rsp,%rbp
   17e44:	c7 07 00 00 00 00    	movl   $0x0,(%rdi)
   17e4a:	c7 47 04 00 00 00 00 	movl   $0x0,0x4(%rdi)
   17e51:	c7 47 08 00 00 00 00 	movl   $0x0,0x8(%rdi)
   17e58:	c7 47 0c 00 00 00 00 	movl   $0x0,0xc(%rdi)
   17e5f:	c7 47 10 00 00 00 00 	movl   $0x0,0x10(%rdi)
   17e66:	c7 47 14 00 00 00 00 	movl   $0x0,0x14(%rdi)
   17e6d:	c7 47 18 00 00 00 00 	movl   $0x0,0x18(%rdi)
   17e74:	c7 47 1c 00 00 00 00 	movl   $0x0,0x1c(%rdi)
   17e7b:	c7 47 20 00 00 00 00 	movl   $0x0,0x20(%rdi)
   17e82:	c7                   	.byte 0xc7
   17e83:	47                   	rex.RXB
   17e84:	24                   	.byte 0x24
=== A=0x40166C9 F=0x166c9 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000016669 <star_core::serv::core::cred::Credential::is_valid+0x489>:
   16669:	d0 66 0f             	shlb   $1,0xf(%rsi)
   1666c:	ef                   	out    %eax,(%dx)
   1666d:	db 66 0f             	(bad) 0xf(%rsi)
   16670:	68 d3 f2 0f 70       	push   $0x700ff2d3
   16675:	d2 1b                	rcrb   %cl,(%rbx)
   16677:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   1667c:	66 0f 60 c3          	punpcklbw %xmm3,%xmm0
   16680:	f2 0f 70 c0 1b       	pshuflw $0x1b,%xmm0,%xmm0
   16685:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   1668a:	66 0f 67 c2          	packuswb %xmm2,%xmm0
   1668e:	66 0f 6f d1          	movdqa %xmm1,%xmm2
   16692:	66 0f 68 d3          	punpckhbw %xmm3,%xmm2
   16696:	f2 0f 70 d2 1b       	pshuflw $0x1b,%xmm2,%xmm2
   1669b:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   166a0:	66 0f 60 cb          	punpcklbw %xmm3,%xmm1
   166a4:	f2 0f 70 c9 1b       	pshuflw $0x1b,%xmm1,%xmm1
   166a9:	f3 0f 70 c9 1b       	pshufhw $0x1b,%xmm1,%xmm1
   166ae:	66 0f 67 ca          	packuswb %xmm2,%xmm1
   166b2:	66 0f 7f 85 30 ff ff 	movdqa %xmm0,-0xd0(%rbp)
   166b9:	ff 
   166ba:	66 0f 7f 8d 40 ff ff 	movdqa %xmm1,-0xc0(%rbp)
   166c1:	ff 
   166c2:	41 80 f6 01          	xor    $0x1,%r14b
   166c6:	45 08 fe             	or     %r15b,%r14b
   166c9:	75 30                	jne    166fb <star_core::serv::core::cred::Credential::is_valid+0x51b>
   166cb:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   166cf:	f3 0f 6f 40 20       	movdqu 0x20(%rax),%xmm0
   166d4:	f3 0f 6f 48 30       	movdqu 0x30(%rax),%xmm1
   166d9:	66 0f 74 8d 40 ff ff 	pcmpeqb -0xc0(%rbp),%xmm1
   166e0:	ff 
   166e1:	66 0f 74 85 30 ff ff 	pcmpeqb -0xd0(%rbp),%xmm0
   166e8:	ff 
   166e9:	66 0f db c1          	pand   %xmm1,%xmm0
   166ed:	66 0f d7 c0          	pmovmskb %xmm0,%eax
   166f1:	3d ff ff 00 00       	cmp    $0xffff,%eax
   166f6:	0f 94 c0             	sete   %al
   166f9:	eb 02                	jmp    166fd <star_core::serv::core::cred::Credential::is_valid+0x51d>
   166fb:	31 c0                	xor    %eax,%eax
   166fd:	48 81 c4 e8 01 00 00 	add    $0x1e8,%rsp
   16704:	5b                   	pop    %rbx
   16705:	41 5c                	pop    %r12
   16707:	41 5d                	pop    %r13
   16709:	41 5e                	pop    %r14
   1670b:	41 5f                	pop    %r15
   1670d:	5d                   	pop    %rbp
   1670e:	c3                   	ret
   1670f:	90                   	nop

0000000000016710 <core::ptr::drop_in_place<hpke::aead::AeadCtxR<hpke::aead::chacha20_poly1305::ChaCha20Poly1305,hpke::kdf::HkdfSha256,hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256>>>:
   16710:	55                   	push   %rbp
   16711:	48 89 e5             	mov    %rsp,%rbp
   16714:	c6 07 00             	movb   $0x0,(%rdi)
   16717:	c6 47 01 00          	movb   $0x0,0x1(%rdi)
   1671b:	c6 47 02 00          	movb   $0x0,0x2(%rdi)
   1671f:	c6 47 03 00          	movb   $0x0,0x3(%rdi)
   16723:	c6 47 04 00          	movb   $0x0,0x4(%rdi)
   16727:	c6 47 05 00          	movb   $0x0,0x5(%rdi)
   1672b:	c6 47 06 00          	movb   $0x0,0x6(%rdi)
   1672f:	c6 47 07 00          	movb   $0x0,0x7(%rdi)
   16733:	c6 47 08 00          	movb   $0x0,0x8(%rdi)
   16737:	c6 47 09 00          	movb   $0x0,0x9(%rdi)
   1673b:	c6 47 0a 00          	movb   $0x0,0xa(%rdi)
   1673f:	c6 47 0b 00          	movb   $0x0,0xb(%rdi)
   16743:	c6 47 0c 00          	movb   $0x0,0xc(%rdi)
   16747:	c6 47 0d 00          	movb   $0x0,0xd(%rdi)
   1674b:	c6 47 0e 00          	movb   $0x0,0xe(%rdi)
   1674f:	c6 47 0f 00          	movb   $0x0,0xf(%rdi)
   16753:	c6 47 10 00          	movb   $0x0,0x10(%rdi)
   16757:	c6 47 11 00          	movb   $0x0,0x11(%rdi)
   1675b:	c6 47 12 00          	movb   $0x0,0x12(%rdi)
   1675f:	c6 47 13 00          	movb   $0x0,0x13(%rdi)
   16763:	c6 47 14 00          	movb   $0x0,0x14(%rdi)
   16767:	c6 47 15 00          	movb   $0x0,0x15(%rdi)
   1676b:	c6 47 16 00          	movb   $0x0,0x16(%rdi)
   1676f:	c6 47 17 00          	movb   $0x0,0x17(%rdi)
   16773:	c6 47 18 00          	movb   $0x0,0x18(%rdi)
   16777:	c6 47 19 00          	movb   $0x0,0x19(%rdi)
   1677b:	c6 47 1a 00          	movb   $0x0,0x1a(%rdi)
   1677f:	c6 47 1b 00          	movb   $0x0,0x1b(%rdi)
   16783:	c6 47 1c 00          	movb   $0x0,0x1c(%rdi)
   16787:	c6 47 1d 00          	movb   $0x0,0x1d(%rdi)
   1678b:	c6 47 1e 00          	movb   $0x0,0x1e(%rdi)
   1678f:	c6 47 1f 00          	movb   $0x0,0x1f(%rdi)
   16793:	c6 47 48 00          	movb   $0x0,0x48(%rdi)
   16797:	c6 47 49 00          	movb   $0x0,0x49(%rdi)
   1679b:	c6 47 4a 00          	movb   $0x0,0x4a(%rdi)
   1679f:	c6 47 4b 00          	movb   $0x0,0x4b(%rdi)
   167a3:	c6 47 4c 00          	movb   $0x0,0x4c(%rdi)
   167a7:	c6 47 4d 00          	movb   $0x0,0x4d(%rdi)
   167ab:	c6 47 4e 00          	movb   $0x0,0x4e(%rdi)
   167af:	c6 47 4f 00          	movb   $0x0,0x4f(%rdi)
   167b3:	c6 47 50 00          	movb   $0x0,0x50(%rdi)
   167b7:	c6 47 51 00          	movb   $0x0,0x51(%rdi)
   167bb:	c6 47 52 00          	movb   $0x0,0x52(%rdi)
   167bf:	c6 47 53 00          	movb   $0x0,0x53(%rdi)
   167c3:	c6 47 20 00          	movb   $0x0,0x20(%rdi)
   167c7:	c6 47 21 00          	movb   $0x0,0x21(%rdi)
   167cb:	c6 47 22 00          	movb   $0x0,0x22(%rdi)
   167cf:	c6 47 23 00          	movb   $0x0,0x23(%rdi)
   167d3:	c6 47 24 00          	movb   $0x0,0x24(%rdi)
   167d7:	c6 47 25 00          	movb   $0x0,0x25(%rdi)
   167db:	c6 47 26 00          	movb   $0x0,0x26(%rdi)
   167df:	c6 47 27 00          	movb   $0x0,0x27(%rdi)
   167e3:	c6 47 28 00          	movb   $0x0,0x28(%rdi)
   167e7:	c6 47 29 00          	movb   $0x0,0x29(%rdi)
   167eb:	c6 47 2a 00          	movb   $0x0,0x2a(%rdi)
   167ef:	c6 47 2b 00          	movb   $0x0,0x2b(%rdi)
   167f3:	c6 47 2c 00          	movb   $0x0,0x2c(%rdi)
   167f7:	c6 47 2d 00          	movb   $0x0,0x2d(%rdi)
   167fb:	c6 47 2e 00          	movb   $0x0,0x2e(%rdi)
   167ff:	c6 47 2f 00          	movb   $0x0,0x2f(%rdi)
   16803:	c6 47 30 00          	movb   $0x0,0x30(%rdi)
   16807:	c6 47 31 00          	movb   $0x0,0x31(%rdi)
   1680b:	c6 47 32 00          	movb   $0x0,0x32(%rdi)
   1680f:	c6 47 33 00          	movb   $0x0,0x33(%rdi)
   16813:	c6 47 34 00          	movb   $0x0,0x34(%rdi)
   16817:	c6 47 35 00          	movb   $0x0,0x35(%rdi)
   1681b:	c6 47 36 00          	movb   $0x0,0x36(%rdi)
   1681f:	c6 47 37 00          	movb   $0x0,0x37(%rdi)
   16823:	c6 47 38 00          	movb   $0x0,0x38(%rdi)
   16827:	c6 47 39 00          	movb   $0x0,0x39(%rdi)
   1682b:	c6 47 3a 00          	movb   $0x0,0x3a(%rdi)
   1682f:	c6 47 3b 00          	movb   $0x0,0x3b(%rdi)
   16833:	c6 47 3c 00          	movb   $0x0,0x3c(%rdi)
   16837:	c6 47 3d 00          	movb   $0x0,0x3d(%rdi)
   1683b:	c6 47 3e 00          	movb   $0x0,0x3e(%rdi)
   1683f:	c6 47 3f 00          	movb   $0x0,0x3f(%rdi)
   16843:	48 c7 47 40 00 00 00 	movq   $0x0,0x40(%rdi)
   1684a:	00 
   1684b:	5d                   	pop    %rbp
   1684c:	c3                   	ret
   1684d:	0f 1f 00             	nopl   (%rax)

0000000000016850 <core::ptr::drop_in_place<hpke::setup::ExporterSecret<hpke::kdf::HkdfSha256>>>:
   16850:	55                   	push   %rbp
   16851:	48 89 e5             	mov    %rsp,%rbp
   16854:	c6 07 00             	movb   $0x0,(%rdi)
   16857:	c6 47 01 00          	movb   $0x0,0x1(%rdi)
   1685b:	c6 47 02 00          	movb   $0x0,0x2(%rdi)
   1685f:	c6 47 03 00          	movb   $0x0,0x3(%rdi)
   16863:	c6 47 04 00          	movb   $0x0,0x4(%rdi)
   16867:	c6 47 05 00          	movb   $0x0,0x5(%rdi)
   1686b:	c6 47 06 00          	movb   $0x0,0x6(%rdi)
   1686f:	c6 47 07 00          	movb   $0x0,0x7(%rdi)
   16873:	c6 47 08 00          	movb   $0x0,0x8(%rdi)
   16877:	c6 47 09 00          	movb   $0x0,0x9(%rdi)
   1687b:	c6 47 0a 00          	movb   $0x0,0xa(%rdi)
   1687f:	c6 47 0b 00          	movb   $0x0,0xb(%rdi)
   16883:	c6 47 0c 00          	movb   $0x0,0xc(%rdi)
   16887:	c6 47 0d 00          	movb   $0x0,0xd(%rdi)
   1688b:	c6 47 0e 00          	movb   $0x0,0xe(%rdi)
   1688f:	c6 47 0f 00          	movb   $0x0,0xf(%rdi)
   16893:	c6 47 10 00          	movb   $0x0,0x10(%rdi)
   16897:	c6 47 11 00          	movb   $0x0,0x11(%rdi)
   1689b:	c6 47 12 00          	movb   $0x0,0x12(%rdi)
   1689f:	c6 47 13 00          	movb   $0x0,0x13(%rdi)
   168a3:	c6 47 14 00          	movb   $0x0,0x14(%rdi)
   168a7:	c6 47 15 00          	movb   $0x0,0x15(%rdi)
   168ab:	c6 47 16 00          	movb   $0x0,0x16(%rdi)
   168af:	c6 47 17 00          	movb   $0x0,0x17(%rdi)
   168b3:	c6 47 18 00          	movb   $0x0,0x18(%rdi)
   168b7:	c6 47 19 00          	movb   $0x0,0x19(%rdi)
   168bb:	c6 47 1a 00          	movb   $0x0,0x1a(%rdi)
   168bf:	c6 47 1b 00          	movb   $0x0,0x1b(%rdi)
   168c3:	c6 47 1c 00          	movb   $0x0,0x1c(%rdi)
   168c7:	c6                   	.byte 0xc6
   168c8:	47                   	rex.RXB
=== A=0x40158D5 F=0x158d5 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000015875 <star_core::serv::enclave::Enclave::issue_token_plain+0x45>:
   15875:	4d 80 0f 29          	rex.WRB orb $0x29,(%r15)
   15879:	85 70 ff             	test   %esi,-0x1(%rax)
   1587c:	ff                   	(bad)
   1587d:	ff 41 0f             	incl   0xf(%rcx)
   15880:	10 06                	adc    %al,(%rsi)
   15882:	41 0f 10 4e 10       	movups 0x10(%r14),%xmm1
   15887:	0f 29 85 50 ff ff ff 	movaps %xmm0,-0xb0(%rbp)
   1588e:	0f 29 8d 60 ff ff ff 	movaps %xmm1,-0xa0(%rbp)
   15895:	49 8b 46 40          	mov    0x40(%r14),%rax
   15899:	4d 8b 6e 48          	mov    0x48(%r14),%r13
   1589d:	4d 8b 44 24 60       	mov    0x60(%r12),%r8
   158a2:	41 0f 10 44 24 40    	movups 0x40(%r12),%xmm0
   158a8:	41 0f 10 4c 24 50    	movups 0x50(%r12),%xmm1
   158ae:	49 89 c4             	mov    %rax,%r12
   158b1:	0f 29 4d c0          	movaps %xmm1,-0x40(%rbp)
   158b5:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
   158b9:	48 8d bd 50 ff ff ff 	lea    -0xb0(%rbp),%rdi
   158c0:	4c 8d 4d b0          	lea    -0x50(%rbp),%r9
   158c4:	4c 89 ee             	mov    %r13,%rsi
   158c7:	48 89 c2             	mov    %rax,%rdx
   158ca:	4c 89 f9             	mov    %r15,%rcx
   158cd:	ff 15 8d c2 07 00    	call   *0x7c28d(%rip)        # 91b60 <_GLOBAL_OFFSET_TABLE_+0x648>
   158d3:	84 c0                	test   %al,%al
   158d5:	0f 84 86 00 00 00    	je     15961 <star_core::serv::enclave::Enclave::issue_token_plain+0x131>
   158db:	41 0f 10 06          	movups (%r14),%xmm0
   158df:	41 0f 10 4e 10       	movups 0x10(%r14),%xmm1
   158e4:	0f 29 4d c0          	movaps %xmm1,-0x40(%rbp)
   158e8:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
   158ec:	48 8d 7d 90          	lea    -0x70(%rbp),%rdi
   158f0:	48 8d 75 b0          	lea    -0x50(%rbp),%rsi
   158f4:	4c 89 ea             	mov    %r13,%rdx
   158f7:	4c 89 e1             	mov    %r12,%rcx
   158fa:	ff 15 a8 c0 07 00    	call   *0x7c0a8(%rip)        # 919a8 <_GLOBAL_OFFSET_TABLE_+0x490>
   15900:	48 c7 45 b0 00 00 00 	movq   $0x0,-0x50(%rbp)
   15907:	00 
   15908:	48 c7 45 b8 01 00 00 	movq   $0x1,-0x48(%rbp)
   1590f:	00 
   15910:	48 c7 45 c0 00 00 00 	movq   $0x0,-0x40(%rbp)
   15917:	00 
   15918:	48 8d 7d b0          	lea    -0x50(%rbp),%rdi
   1591c:	ba 20 00 00 00       	mov    $0x20,%edx
   15921:	b9 01 00 00 00       	mov    $0x1,%ecx
   15926:	41 b8 01 00 00 00    	mov    $0x1,%r8d
   1592c:	31 f6                	xor    %esi,%esi
   1592e:	e8 3d 2c ff ff       	call   8570 <alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle>
   15933:	48 8b 45 b8          	mov    -0x48(%rbp),%rax
   15937:	48 8b 4d c0          	mov    -0x40(%rbp),%rcx
   1593b:	0f 10 45 90          	movups -0x70(%rbp),%xmm0
   1593f:	0f 10 4d a0          	movups -0x60(%rbp),%xmm1
   15943:	0f 11 4c 08 10       	movups %xmm1,0x10(%rax,%rcx,1)
   15948:	0f 11 04 08          	movups %xmm0,(%rax,%rcx,1)
   1594c:	48 83 c1 20          	add    $0x20,%rcx
   15950:	48 89 4d c0          	mov    %rcx,-0x40(%rbp)
   15954:	48 89 4b 10          	mov    %rcx,0x10(%rbx)
   15958:	0f 10 45 b0          	movups -0x50(%rbp),%xmm0
   1595c:	0f 11 03             	movups %xmm0,(%rbx)
   1595f:	eb 31                	jmp    15992 <star_core::serv::enclave::Enclave::issue_token_plain+0x162>
   15961:	ff 15 89 bd 07 00    	call   *0x7bd89(%rip)        # 916f0 <_GLOBAL_OFFSET_TABLE_+0x1d8>
   15967:	bf 01 00 00 00       	mov    $0x1,%edi
   1596c:	be 01 00 00 00       	mov    $0x1,%esi
   15971:	ff 15 e9 c0 07 00    	call   *0x7c0e9(%rip)        # 91a60 <_GLOBAL_OFFSET_TABLE_+0x548>
   15977:	48 85 c0             	test   %rax,%rax
   1597a:	74 2b                	je     159a7 <star_core::serv::enclave::Enclave::issue_token_plain+0x177>
   1597c:	c6 00 00             	movb   $0x0,(%rax)
   1597f:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   15986:	48 89 43 08          	mov    %rax,0x8(%rbx)
   1598a:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   15991:	00 
   15992:	48 89 d8             	mov    %rbx,%rax
   15995:	48 81 c4 88 00 00 00 	add    $0x88,%rsp
   1599c:	5b                   	pop    %rbx
   1599d:	41 5c                	pop    %r12
   1599f:	41 5d                	pop    %r13
   159a1:	41 5e                	pop    %r14
   159a3:	41 5f                	pop    %r15
   159a5:	5d                   	pop    %rbp
   159a6:	c3                   	ret
   159a7:	bf 01 00 00 00       	mov    $0x1,%edi
   159ac:	be 01 00 00 00       	mov    $0x1,%esi
   159b1:	ff 15 11 c2 07 00    	call   *0x7c211(%rip)        # 91bc8 <_GLOBAL_OFFSET_TABLE_+0x6b0>
   159b7:	48 89 c3             	mov    %rax,%rbx
   159ba:	48 8b 75 b0          	mov    -0x50(%rbp),%rsi
   159be:	48 85 f6             	test   %rsi,%rsi
   159c1:	74 0f                	je     159d2 <star_core::serv::enclave::Enclave::issue_token_plain+0x1a2>
   159c3:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   159c7:	ba 01 00 00 00       	mov    $0x1,%edx
   159cc:	ff 15 f6 be 07 00    	call   *0x7bef6(%rip)        # 918c8 <_GLOBAL_OFFSET_TABLE_+0x3b0>
   159d2:	48 89 df             	mov    %rbx,%rdi
   159d5:	e8 76 26 ff ff       	call   8050 <_Unwind_Resume@plt>
   159da:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)

00000000000159e0 <star_core::serv::enclave::Enclave::new>:
   159e0:	55                   	push   %rbp
   159e1:	48 89 e5             	mov    %rsp,%rbp
   159e4:	41 57                	push   %r15
   159e6:	41 56                	push   %r14
   159e8:	53                   	push   %rbx
   159e9:	48 81 ec 08 01 00 00 	sub    $0x108,%rsp
   159f0:	49 89 d6             	mov    %rdx,%r14
   159f3:	41 89 f7             	mov    %esi,%r15d
   159f6:	48 89 fb             	mov    %rdi,%rbx
   159f9:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   15a00:	ba 20 00 00 00       	mov    $0x20,%edx
   15a05:	48 89 ce             	mov    %rcx,%rsi
   15a08:	ff 15 82 bb 07 00    	call   *0x7bb82(%rip)        # 91590 <_GLOBAL_OFFSET_TABLE_+0x78>
   15a0e:	80 bd 60 ff ff ff 00 	cmpb   $0x0,-0xa0(%rbp)
   15a15:	74 2b                	je     15a42 <star_core::serv::enclave::Enclave::new+0x62>
   15a17:	48 8b 85 78 ff ff ff 	mov    -0x88(%rbp),%rax
   15a1e:	0f 10 85 68 ff ff ff 	movups -0x98(%rbp),%xmm0
   15a25:	0f 11 85 27 ff ff ff 	movups %xmm0,-0xd9(%rbp)
   15a2c:	48 89 85 37 ff ff ff 	mov    %rax,-0xc9(%rbp)
   15a33:	48 89 43 18          	mov    %rax,0x18(%rbx)
   15a37:	0f 11 43 08          	movups %xmm0,0x8(%rbx)
   15a3b:	b0 01                	mov    $0x1,%al
   15a3d:	e9 a4 00 00 00       	jmp    15ae6 <star_core::serv::enclave::Enclave::new+0x106>
   15a42:	0f 10 85 61 ff ff ff 	movups -0x9f(%rbp),%xmm0
   15a49:	0f 10 8d 71 ff ff ff 	movups -0x8f(%rbp),%xmm1
   15a50:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
   15a54:	0f 29 4d d0          	movaps %xmm1,-0x30(%rbp)
   15a58:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   15a5f:	48 8d 75 c0          	lea    -0x40(%rbp),%rsi
   15a63:	ff 15 17 bd 07 00    	call   *0x7bd17(%rip)        # 91780 <_GLOBAL_OFFSET_TABLE_+0x268>
   15a69:	0f 28 45 c0          	movaps -0x40(%rbp),%xmm0
   15a6d:	0f 28 4d d0          	movaps -0x30(%rbp),%xmm1
   15a71:	0f 29 8d 30 ff ff ff 	movaps %xmm1,-0xd0(%rbp)
   15a78:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   15a7f:	0f 10 95 00 ff ff ff 	movups -0x100(%rbp),%xmm2
   15a86:	0f 10 9d 10 ff ff ff 	movups -0xf0(%rbp),%xmm3
   15a8d:	0f 29 95 40 ff ff ff 	movaps %xmm2,-0xc0(%rbp)
   15a94:	0f 29 9d 50 ff ff ff 	movaps %xmm3,-0xb0(%rbp)
   15a9b:	0f 29 5d 90          	movaps %xmm3,-0x70(%rbp)
   15a9f:	0f 29 55 80          	movaps %xmm2,-0x80(%rbp)
   15aa3:	0f 29 8d 70 ff ff ff 	movaps %xmm1,-0x90(%rbp)
   15aaa:	0f 29 85 60 ff ff ff 	movaps %xmm0,-0xa0(%rbp)
   15ab1:	44 89 f8             	mov    %r15d,%eax
   15ab4:	48 0f c8             	bswap  %rax
   15ab7:	41 0f 10 26          	movups (%r14),%xmm4
   15abb:	41 0f 10 6e 10       	movups 0x10(%r14),%xmm5
   15ac0:	0f 29 65 a0          	movaps %xmm4,-0x60(%rbp)
   15ac4:	0f 29 6d b0          	movaps %xmm5,-0x50(%rbp)
   15ac8:	0f 11 6b 51          	movups %xmm5,0x51(%rbx)
   15acc:	0f 11 63 41          	movups %xmm4,0x41(%rbx)
   15ad0:	0f 11 5b 31          	movups %xmm3,0x31(%rbx)
   15ad4:	0f                   	.byte 0xf
