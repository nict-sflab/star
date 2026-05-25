=== A=0x4019884 F=0x19884 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000019824 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::derive_keypair+0x34>:
   19824:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   1982b:	00 
   1982c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000019830 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap>:
   19830:	55                   	push   %rbp
   19831:	48 89 e5             	mov    %rsp,%rbp
   19834:	41 57                	push   %r15
   19836:	41 56                	push   %r14
   19838:	41 55                	push   %r13
   1983a:	41 54                	push   %r12
   1983c:	53                   	push   %rbx
   1983d:	48 81 ec 68 05 00 00 	sub    $0x568,%rsp
   19844:	49 89 cc             	mov    %rcx,%r12
   19847:	49 89 d6             	mov    %rdx,%r14
   1984a:	49 89 f7             	mov    %rsi,%r15
   1984d:	48 89 fb             	mov    %rdi,%rbx
   19850:	c6 45 d4 4d          	movb   $0x4d,-0x2c(%rbp)
   19854:	66 c7 45 d2 4b 45    	movw   $0x454b,-0x2e(%rbp)
   1985a:	66 c7 45 d5 00 20    	movw   $0x2000,-0x2b(%rbp)
   19860:	4c 8d ad e0 fd ff ff 	lea    -0x220(%rbp),%r13
   19867:	4c 89 ef             	mov    %r13,%rdi
   1986a:	48 89 ca             	mov    %rcx,%rdx
   1986d:	ff 15 1d f0 06 00    	call   *0x6f01d(%rip)        # 88890 <_GLOBAL_OFFSET_TABLE_+0x360>
   19873:	48 8d 35 56 02 05 00 	lea    0x50256(%rip),%rsi        # 69ad0 <anon.39a2a9f1401edcf2cd24a86c78a26440.25.llvm.12169493792077094199>
   1987a:	4c 89 ef             	mov    %r13,%rdi
   1987d:	e8 ce e0 ff ff       	call   17950 <<[T] as subtle::ConstantTimeEq>::ct_eq>
   19882:	84 c0                	test   %al,%al
   19884:	74 1d                	je     198a3 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x73>
   19886:	48 8d bd e0 fd ff ff 	lea    -0x220(%rbp),%rdi
   1988d:	ff 15 2d f7 06 00    	call   *0x6f72d(%rip)        # 88fc0 <_GLOBAL_OFFSET_TABLE_+0xa90>
   19893:	48 c7 43 08 06 00 00 	movq   $0x6,0x8(%rbx)
   1989a:	00 
   1989b:	c6 03 01             	movb   $0x1,(%rbx)
   1989e:	e9 9a 04 00 00       	jmp    19d3d <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x50d>
   198a3:	8b 85 e0 fd ff ff    	mov    -0x220(%rbp),%eax
   198a9:	8b 8d e3 fd ff ff    	mov    -0x21d(%rbp),%ecx
   198af:	89 4d 93             	mov    %ecx,-0x6d(%rbp)
   198b2:	89 45 90             	mov    %eax,-0x70(%rbp)
   198b5:	48 8b 85 e7 fd ff ff 	mov    -0x219(%rbp),%rax
   198bc:	0f 10 85 ef fd ff ff 	movups -0x211(%rbp),%xmm0
   198c3:	0f 11 45 9f          	movups %xmm0,-0x61(%rbp)
   198c7:	0f b6 8d ff fd ff ff 	movzbl -0x201(%rbp),%ecx
   198ce:	48 89 45 97          	mov    %rax,-0x69(%rbp)
   198d2:	88 4d af             	mov    %cl,-0x51(%rbp)
   198d5:	48 8d bd c0 fd ff ff 	lea    -0x240(%rbp),%rdi
   198dc:	4c 89 fe             	mov    %r15,%rsi
   198df:	ff 15 4b f0 06 00    	call   *0x6f04b(%rip)        # 88930 <_GLOBAL_OFFSET_TABLE_+0x400>
   198e5:	4d 85 f6             	test   %r14,%r14
   198e8:	0f 84 00 01 00 00    	je     199ee <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x1be>
   198ee:	48 8d bd 40 fe ff ff 	lea    -0x1c0(%rbp),%rdi
   198f5:	ba 2f 01 00 00       	mov    $0x12f,%edx
   198fa:	31 f6                	xor    %esi,%esi
   198fc:	ff 15 f6 ec 06 00    	call   *0x6ecf6(%rip)        # 885f8 <memset@GLIBC_2.2.5>
   19902:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   19909:	4c 89 e6             	mov    %r12,%rsi
   1990c:	ff 15 76 f6 06 00    	call   *0x6f676(%rip)        # 88f88 <_GLOBAL_OFFSET_TABLE_+0xa58>
   19912:	0f 10 85 31 fc ff ff 	movups -0x3cf(%rbp),%xmm0
   19919:	0f 10 8d 41 fc ff ff 	movups -0x3bf(%rbp),%xmm1
   19920:	0f 29 8d f0 fd ff ff 	movaps %xmm1,-0x210(%rbp)
   19927:	0f 29 85 e0 fd ff ff 	movaps %xmm0,-0x220(%rbp)
   1992e:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   19935:	48 8d b5 c0 fd ff ff 	lea    -0x240(%rbp),%rsi
   1993c:	ff 15 46 f6 06 00    	call   *0x6f646(%rip)        # 88f88 <_GLOBAL_OFFSET_TABLE_+0xa58>
   19942:	0f 10 85 31 fc ff ff 	movups -0x3cf(%rbp),%xmm0
   19949:	0f 10 8d 41 fc ff ff 	movups -0x3bf(%rbp),%xmm1
   19950:	0f 29 8d 10 fe ff ff 	movaps %xmm1,-0x1f0(%rbp)
   19957:	0f 29 85 00 fe ff ff 	movaps %xmm0,-0x200(%rbp)
   1995e:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   19965:	4c 89 f6             	mov    %r14,%rsi
   19968:	ff 15 1a f6 06 00    	call   *0x6f61a(%rip)        # 88f88 <_GLOBAL_OFFSET_TABLE_+0xa58>
   1996e:	0f 10 85 31 fc ff ff 	movups -0x3cf(%rbp),%xmm0
   19975:	0f 10 8d 41 fc ff ff 	movups -0x3bf(%rbp),%xmm1
   1997c:	0f 29 8d 30 fe ff ff 	movaps %xmm1,-0x1d0(%rbp)
   19983:	0f 29 85 20 fe ff ff 	movaps %xmm0,-0x1e0(%rbp)
   1998a:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   19991:	48 8d b5 e0 fd ff ff 	lea    -0x220(%rbp),%rsi
   19998:	ba 8f 01 00 00       	mov    $0x18f,%edx
   1999d:	ff 15 d5 f5 06 00    	call   *0x6f5d5(%rip)        # 88f78 <memcpy@GLIBC_2.14>
   199a3:	48 8d bd e0 fd ff ff 	lea    -0x220(%rbp),%rdi
   199aa:	4c 89 fe             	mov    %r15,%rsi
   199ad:	4c 89 f2             	mov    %r14,%rdx
   199b0:	ff 15 da ee 06 00    	call   *0x6eeda(%rip)        # 88890 <_GLOBAL_OFFSET_TABLE_+0x360>
   199b6:	48 8d 35 13 01 05 00 	lea    0x50113(%rip),%rsi        # 69ad0 <anon.39a2a9f1401edcf2cd24a86c78a26440.25.llvm.12169493792077094199>
   199bd:	48 8d bd e0 fd ff ff 	lea    -0x220(%rbp),%rdi
   199c4:	e8 87 df ff ff       	call   17950 <<[T] as subtle::ConstantTimeEq>::ct_eq>
   199c9:	84 c0                	test   %al,%al
   199cb:	0f 84 a7 01 00 00    	je     19b78 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x348>
   199d1:	48 8d bd e0 fd ff ff 	lea    -0x220(%rbp),%rdi
   199d8:	ff 15 e2 f5 06 00    	call   *0x6f5e2(%rip)        # 88fc0 <_GLOBAL_OFFSET_TABLE_+0xa90>
   199de:	48 c7 43 08 06 00 00 	movq   $0x6,0x8(%rbx)
   199e5:	00 
   199e6:	c6 03 01             	movb   $0x1,(%rbx)
   199e9:	e9 45 03 00 00       	jmp    19d33 <<hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256 as hpke::kem::Kem>::decap+0x503>
   199ee:	0f 57 c0             	xorps  %xmm0,%xmm0
   199f1:	0f 11 85 da fe ff ff 	movups %xmm0,-0x126(%rbp)
   199f8:	0f 11 85 d0 fe ff ff 	movups %xmm0,-0x130(%rbp)
   199ff:	0f 11 85 c0 fe ff ff 	movups %xmm0,-0x140(%rbp)
   19a06:	0f 11 85 b0 fe ff ff 	movups %xmm0,-0x150(%rbp)
   19a0d:	0f 11 85 a0 fe ff ff 	movups %xmm0,-0x160(%rbp)
   19a14:	0f 11 85 90 fe ff ff 	movups %xmm0,-0x170(%rbp)
   19a1b:	0f 11 85 80 fe ff ff 	movups %xmm0,-0x180(%rbp)
   19a22:	0f 11 85 70 fe ff ff 	movups %xmm0,-0x190(%rbp)
   19a29:	0f 11 85 60 fe ff ff 	movups %xmm0,-0x1a0(%rbp)
   19a30:	0f 11 85 50 fe ff ff 	movups %xmm0,-0x1b0(%rbp)
   19a37:	0f 11 85 40 fe ff ff 	movups %xmm0,-0x1c0(%rbp)
   19a3e:	0f 11 85 30 fe ff ff 	movups %xmm0,-0x1d0(%rbp)
   19a45:	0f 11 85 20 fe ff ff 	movups %xmm0,-0x1e0(%rbp)
   19a4c:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   19a53:	4c 89 e6             	mov    %r12,%rsi
   19a56:	ff 15 2c f5 06 00    	call   *0x6f52c(%rip)        # 88f88 <_GLOBAL_OFFSET_TABLE_+0xa58>
   19a5c:	0f 10 85 31 fc ff ff 	movups -0x3cf(%rbp),%xmm0
   19a63:	0f 10 8d 41 fc ff ff 	movups -0x3bf(%rbp),%xmm1
   19a6a:	0f 29 8d f0 fd ff ff 	movaps %xmm1,-0x210(%rbp)
   19a71:	0f 29 85 e0 fd ff ff 	movaps %xmm0,-0x220(%rbp)
   19a78:	48 8d bd 31 fc ff ff 	lea    -0x3cf(%rbp),%rdi
   19a7f:	48                   	rex.W
   19a80:	8d                   	.byte 0x8d
   19a81:	b5 c0                	mov    $0xc0,%ch
   19a83:	fd                   	std
=== A=0x4014fc5 F=0x14fc5 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000014f65 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x155>:
   14f65:	88 75 d5             	mov    %dh,-0x2b(%rbp)
   14f68:	44 88 45 d6          	mov    %r8b,-0x2a(%rbp)
   14f6c:	44 88 4d d7          	mov    %r9b,-0x29(%rbp)
   14f70:	c6 45 c0 00          	movb   $0x0,-0x40(%rbp)
   14f74:	c6 45 c1 00          	movb   $0x0,-0x3f(%rbp)
   14f78:	c6 45 c2 00          	movb   $0x0,-0x3e(%rbp)
   14f7c:	c6 45 c3 00          	movb   $0x0,-0x3d(%rbp)
   14f80:	c6 45 c4 00          	movb   $0x0,-0x3c(%rbp)
   14f84:	c6 45 c5 00          	movb   $0x0,-0x3b(%rbp)
   14f88:	c6 45 c6 00          	movb   $0x0,-0x3a(%rbp)
   14f8c:	c6 45 c7 00          	movb   $0x0,-0x39(%rbp)
   14f90:	c6 45 c8 00          	movb   $0x0,-0x38(%rbp)
   14f94:	c6 45 c9 00          	movb   $0x0,-0x37(%rbp)
   14f98:	c6 45 ca 00          	movb   $0x0,-0x36(%rbp)
   14f9c:	c6 45 cb 00          	movb   $0x0,-0x35(%rbp)
   14fa0:	48 8d 45 90          	lea    -0x70(%rbp),%rax
   14fa4:	48 89 04 24          	mov    %rax,(%rsp)
   14fa8:	48 8d 75 cc          	lea    -0x34(%rbp),%rsi
   14fac:	48 8b 55 a8          	mov    -0x58(%rbp),%rdx
   14fb0:	48 8b 4d a0          	mov    -0x60(%rbp),%rcx
   14fb4:	4d 89 f0             	mov    %r14,%r8
   14fb7:	49 89 d9             	mov    %rbx,%r9
   14fba:	e8 f1 22 00 00       	call   172b0 <<chacha20poly1305::ChaChaPoly1305<C,N> as aead::AeadInPlace>::decrypt_in_place_detached>
   14fbf:	84 c0                	test   %al,%al
   14fc1:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   14fc5:	74 63                	je     1502a <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x21a>
   14fc7:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   14fcb:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   14fcf:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   14fd3:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   14fd7:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   14fdb:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   14fdf:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   14fe3:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   14fe7:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   14feb:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   14fef:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   14ff3:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   14ff7:	b8 01 00 00 00       	mov    $0x1,%eax
   14ffc:	48 89 47 08          	mov    %rax,0x8(%rdi)
   15000:	48 c7 07 01 00 00 00 	movq   $0x1,(%rdi)
   15007:	48 85 db             	test   %rbx,%rbx
   1500a:	0f 84 26 fe ff ff    	je     14e36 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x26>
   15010:	ba 01 00 00 00       	mov    $0x1,%edx
   15015:	4c 89 f7             	mov    %r14,%rdi
   15018:	48 89 de             	mov    %rbx,%rsi
   1501b:	ff 15 bf 38 07 00    	call   *0x738bf(%rip)        # 888e0 <_GLOBAL_OFFSET_TABLE_+0x3b0>
   15021:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   15025:	e9 0c fe ff ff       	jmp    14e36 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x26>
   1502a:	48 8b 45 b0          	mov    -0x50(%rbp),%rax
   1502e:	48 8b 48 40          	mov    0x40(%rax),%rcx
   15032:	48 83 f9 ff          	cmp    $0xffffffffffffffff,%rcx
   15036:	74 57                	je     1508f <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x27f>
   15038:	48 ff c1             	inc    %rcx
   1503b:	48 c7 40 40 00 00 00 	movq   $0x0,0x40(%rax)
   15042:	00 
   15043:	48 89 48 40          	mov    %rcx,0x40(%rax)
   15047:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   1504b:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   1504f:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   15053:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   15057:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   1505b:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   1505f:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   15063:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   15067:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   1506b:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   1506f:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   15073:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   15077:	48 89 5f 08          	mov    %rbx,0x8(%rdi)
   1507b:	4c 89 77 10          	mov    %r14,0x10(%rdi)
   1507f:	48 89 5f 18          	mov    %rbx,0x18(%rdi)
   15083:	48 c7 07 00 00 00 00 	movq   $0x0,(%rdi)
   1508a:	e9 a7 fd ff ff       	jmp    14e36 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x26>
   1508f:	c6 40 5e 01          	movb   $0x1,0x5e(%rax)
   15093:	eb b2                	jmp    15047 <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x237>
   15095:	49 89 c7             	mov    %rax,%r15
   15098:	c6 45 cc 00          	movb   $0x0,-0x34(%rbp)
   1509c:	c6 45 cd 00          	movb   $0x0,-0x33(%rbp)
   150a0:	c6 45 ce 00          	movb   $0x0,-0x32(%rbp)
   150a4:	c6 45 cf 00          	movb   $0x0,-0x31(%rbp)
   150a8:	c6 45 d0 00          	movb   $0x0,-0x30(%rbp)
   150ac:	c6 45 d1 00          	movb   $0x0,-0x2f(%rbp)
   150b0:	c6 45 d2 00          	movb   $0x0,-0x2e(%rbp)
   150b4:	c6 45 d3 00          	movb   $0x0,-0x2d(%rbp)
   150b8:	c6 45 d4 00          	movb   $0x0,-0x2c(%rbp)
   150bc:	c6 45 d5 00          	movb   $0x0,-0x2b(%rbp)
   150c0:	c6 45 d6 00          	movb   $0x0,-0x2a(%rbp)
   150c4:	c6 45 d7 00          	movb   $0x0,-0x29(%rbp)
   150c8:	48 85 db             	test   %rbx,%rbx
   150cb:	74 11                	je     150de <hpke::aead::AeadCtxR<A,Kdf,Kem>::open+0x2ce>
   150cd:	ba 01 00 00 00       	mov    $0x1,%edx
   150d2:	4c 89 f7             	mov    %r14,%rdi
   150d5:	48 89 de             	mov    %rbx,%rsi
   150d8:	ff 15 02 38 07 00    	call   *0x73802(%rip)        # 888e0 <_GLOBAL_OFFSET_TABLE_+0x3b0>
   150de:	4c 89 ff             	mov    %r15,%rdi
   150e1:	e8 6a 2f ff ff       	call   8050 <_Unwind_Resume@plt>
   150e6:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   150ed:	00 00 00 

00000000000150f0 <<hpke::aead::AeadKey<A> as core::ops::drop::Drop>::drop>:
   150f0:	55                   	push   %rbp
   150f1:	48 89 e5             	mov    %rsp,%rbp
   150f4:	c6 07 00             	movb   $0x0,(%rdi)
   150f7:	c6 47 01 00          	movb   $0x0,0x1(%rdi)
   150fb:	c6 47 02 00          	movb   $0x0,0x2(%rdi)
   150ff:	c6 47 03 00          	movb   $0x0,0x3(%rdi)
   15103:	c6 47 04 00          	movb   $0x0,0x4(%rdi)
   15107:	c6 47 05 00          	movb   $0x0,0x5(%rdi)
   1510b:	c6 47 06 00          	movb   $0x0,0x6(%rdi)
   1510f:	c6 47 07 00          	movb   $0x0,0x7(%rdi)
   15113:	c6 47 08 00          	movb   $0x0,0x8(%rdi)
   15117:	c6 47 09 00          	movb   $0x0,0x9(%rdi)
   1511b:	c6 47 0a 00          	movb   $0x0,0xa(%rdi)
   1511f:	c6 47 0b 00          	movb   $0x0,0xb(%rdi)
   15123:	c6 47 0c 00          	movb   $0x0,0xc(%rdi)
   15127:	c6 47 0d 00          	movb   $0x0,0xd(%rdi)
   1512b:	c6 47 0e 00          	movb   $0x0,0xe(%rdi)
   1512f:	c6 47 0f 00          	movb   $0x0,0xf(%rdi)
   15133:	c6 47 10 00          	movb   $0x0,0x10(%rdi)
   15137:	c6 47 11 00          	movb   $0x0,0x11(%rdi)
   1513b:	c6 47 12 00          	movb   $0x0,0x12(%rdi)
   1513f:	c6 47 13 00          	movb   $0x0,0x13(%rdi)
   15143:	c6 47 14 00          	movb   $0x0,0x14(%rdi)
   15147:	c6 47 15 00          	movb   $0x0,0x15(%rdi)
   1514b:	c6 47 16 00          	movb   $0x0,0x16(%rdi)
   1514f:	c6 47 17 00          	movb   $0x0,0x17(%rdi)
   15153:	c6 47 18 00          	movb   $0x0,0x18(%rdi)
   15157:	c6 47 19 00          	movb   $0x0,0x19(%rdi)
   1515b:	c6 47 1a 00          	movb   $0x0,0x1a(%rdi)
   1515f:	c6 47 1b 00          	movb   $0x0,0x1b(%rdi)
   15163:	c6 47 1c 00          	movb   $0x0,0x1c(%rdi)
   15167:	c6 47 1d 00          	movb   $0x0,0x1d(%rdi)
   1516b:	c6 47 1e 00          	movb   $0x0,0x1e(%rdi)
   1516f:	c6 47 1f 00          	movb   $0x0,0x1f(%rdi)
   15173:	5d                   	pop    %rbp
   15174:	c3                   	ret
   15175:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   1517c:	00 00 00 
   1517f:	90                   	nop

0000000000015180 <<chacha20::ChaChaCore<R> as core::ops::drop::Drop>::drop>:
   15180:	55                   	push   %rbp
   15181:	48 89 e5             	mov    %rsp,%rbp
   15184:	c7 07 00 00 00 00    	movl   $0x0,(%rdi)
   1518a:	c7 47 04 00 00 00 00 	movl   $0x0,0x4(%rdi)
   15191:	c7 47 08 00 00 00 00 	movl   $0x0,0x8(%rdi)
   15198:	c7 47 0c 00 00 00 00 	movl   $0x0,0xc(%rdi)
   1519f:	c7 47 10 00 00 00 00 	movl   $0x0,0x10(%rdi)
   151a6:	c7 47 14 00 00 00 00 	movl   $0x0,0x14(%rdi)
   151ad:	c7 47 18 00 00 00 00 	movl   $0x0,0x18(%rdi)
   151b4:	c7 47 1c 00 00 00 00 	movl   $0x0,0x1c(%rdi)
   151bb:	c7 47 20 00 00 00 00 	movl   $0x0,0x20(%rdi)
   151c2:	c7                   	.byte 0xc7
   151c3:	47                   	rex.RXB
   151c4:	24                   	.byte 0x24
=== A=0x4012cd3 F=0x12cd3 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000012c73 <star_core::serv::enclave::Enclave::issue_token_plain+0x43>:
   12c73:	0f 29 4d 80          	movaps %xmm1,-0x80(%rbp)
   12c77:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
   12c7e:	41 0f 10 07          	movups (%r15),%xmm0
   12c82:	41 0f 10 4f 10       	movups 0x10(%r15),%xmm1
   12c87:	0f 29 85 50 ff ff ff 	movaps %xmm0,-0xb0(%rbp)
   12c8e:	0f 29 8d 60 ff ff ff 	movaps %xmm1,-0xa0(%rbp)
   12c95:	49 8b 47 40          	mov    0x40(%r15),%rax
   12c99:	49 8b 77 48          	mov    0x48(%r15),%rsi
   12c9d:	4d 8b 45 60          	mov    0x60(%r13),%r8
   12ca1:	41 0f 10 45 40       	movups 0x40(%r13),%xmm0
   12ca6:	41 0f 10 4d 50       	movups 0x50(%r13),%xmm1
   12cab:	49 89 c5             	mov    %rax,%r13
   12cae:	0f 29 4d c0          	movaps %xmm1,-0x40(%rbp)
   12cb2:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
   12cb6:	48 8d bd 50 ff ff ff 	lea    -0xb0(%rbp),%rdi
   12cbd:	4c 8d 4d b0          	lea    -0x50(%rbp),%r9
   12cc1:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
   12cc5:	48 89 c2             	mov    %rax,%rdx
   12cc8:	4c 89 e1             	mov    %r12,%rcx
   12ccb:	ff 15 a7 5e 07 00    	call   *0x75ea7(%rip)        # 88b78 <_GLOBAL_OFFSET_TABLE_+0x648>
   12cd1:	84 c0                	test   %al,%al
   12cd3:	0f 84 9d 00 00 00    	je     12d76 <star_core::serv::enclave::Enclave::issue_token_plain+0x146>
   12cd9:	41 0f 10 07          	movups (%r15),%xmm0
   12cdd:	41 0f 10 4f 10       	movups 0x10(%r15),%xmm1
   12ce2:	0f 29 4d c0          	movaps %xmm1,-0x40(%rbp)
   12ce6:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
   12cea:	48 8d 7d 90          	lea    -0x70(%rbp),%rdi
   12cee:	48 8d 75 b0          	lea    -0x50(%rbp),%rsi
   12cf2:	48 8b 55 d0          	mov    -0x30(%rbp),%rdx
   12cf6:	4c 89 e9             	mov    %r13,%rcx
   12cf9:	ff 15 c1 5c 07 00    	call   *0x75cc1(%rip)        # 889c0 <_GLOBAL_OFFSET_TABLE_+0x490>
   12cff:	48 c7 45 b0 00 00 00 	movq   $0x0,-0x50(%rbp)
   12d06:	00 
   12d07:	48 c7 45 b8 01 00 00 	movq   $0x1,-0x48(%rbp)
   12d0e:	00 
   12d0f:	48 c7 45 c0 00 00 00 	movq   $0x0,-0x40(%rbp)
   12d16:	00 
   12d17:	48 8d 7d b0          	lea    -0x50(%rbp),%rdi
   12d1b:	ba 20 00 00 00       	mov    $0x20,%edx
   12d20:	b9 01 00 00 00       	mov    $0x1,%ecx
   12d25:	41 b8 01 00 00 00    	mov    $0x1,%r8d
   12d2b:	31 f6                	xor    %esi,%esi
   12d2d:	e8 3e 58 ff ff       	call   8570 <alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle>
   12d32:	4c 8b 65 b8          	mov    -0x48(%rbp),%r12
   12d36:	4c 8b 6d c0          	mov    -0x40(%rbp),%r13
   12d3a:	0f 10 45 90          	movups -0x70(%rbp),%xmm0
   12d3e:	0f 10 4d a0          	movups -0x60(%rbp),%xmm1
   12d42:	43 0f 11 4c 2c 10    	movups %xmm1,0x10(%r12,%r13,1)
   12d48:	43 0f 11 04 2c       	movups %xmm0,(%r12,%r13,1)
   12d4d:	48 8b 45 b0          	mov    -0x50(%rbp),%rax
   12d51:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   12d55:	4c 89 ff             	mov    %r15,%rdi
   12d58:	4c 89 f6             	mov    %r14,%rsi
   12d5b:	ff 15 97 5b 07 00    	call   *0x75b97(%rip)        # 888f8 <_GLOBAL_OFFSET_TABLE_+0x3c8>
   12d61:	49 83 c5 20          	add    $0x20,%r13
   12d65:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   12d69:	48 89 03             	mov    %rax,(%rbx)
   12d6c:	4c 89 63 08          	mov    %r12,0x8(%rbx)
   12d70:	4c 89 6b 10          	mov    %r13,0x10(%rbx)
   12d74:	eb 31                	jmp    12da7 <star_core::serv::enclave::Enclave::issue_token_plain+0x177>
   12d76:	ff 15 8c 59 07 00    	call   *0x7598c(%rip)        # 88708 <_GLOBAL_OFFSET_TABLE_+0x1d8>
   12d7c:	bf 01 00 00 00       	mov    $0x1,%edi
   12d81:	be 01 00 00 00       	mov    $0x1,%esi
   12d86:	ff 15 ec 5c 07 00    	call   *0x75cec(%rip)        # 88a78 <_GLOBAL_OFFSET_TABLE_+0x548>
   12d8c:	48 85 c0             	test   %rax,%rax
   12d8f:	74 2b                	je     12dbc <star_core::serv::enclave::Enclave::issue_token_plain+0x18c>
   12d91:	c6 00 00             	movb   $0x0,(%rax)
   12d94:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   12d9b:	48 89 43 08          	mov    %rax,0x8(%rbx)
   12d9f:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   12da6:	00 
   12da7:	48 89 d8             	mov    %rbx,%rax
   12daa:	48 81 c4 88 00 00 00 	add    $0x88,%rsp
   12db1:	5b                   	pop    %rbx
   12db2:	41 5c                	pop    %r12
   12db4:	41 5d                	pop    %r13
   12db6:	41 5e                	pop    %r14
   12db8:	41 5f                	pop    %r15
   12dba:	5d                   	pop    %rbp
   12dbb:	c3                   	ret
   12dbc:	bf 01 00 00 00       	mov    $0x1,%edi
   12dc1:	be 01 00 00 00       	mov    $0x1,%esi
   12dc6:	ff 15 14 5e 07 00    	call   *0x75e14(%rip)        # 88be0 <_GLOBAL_OFFSET_TABLE_+0x6b0>
   12dcc:	48 89 c3             	mov    %rax,%rbx
   12dcf:	48 83 7d d0 00       	cmpq   $0x0,-0x30(%rbp)
   12dd4:	74 29                	je     12dff <star_core::serv::enclave::Enclave::issue_token_plain+0x1cf>
   12dd6:	ba 01 00 00 00       	mov    $0x1,%edx
   12ddb:	4c 89 e7             	mov    %r12,%rdi
   12dde:	48 8b 75 d0          	mov    -0x30(%rbp),%rsi
   12de2:	eb 15                	jmp    12df9 <star_core::serv::enclave::Enclave::issue_token_plain+0x1c9>
   12de4:	48 89 c3             	mov    %rax,%rbx
   12de7:	48 8b 75 b0          	mov    -0x50(%rbp),%rsi
   12deb:	48 85 f6             	test   %rsi,%rsi
   12dee:	74 0f                	je     12dff <star_core::serv::enclave::Enclave::issue_token_plain+0x1cf>
   12df0:	48 8b 7d b8          	mov    -0x48(%rbp),%rdi
   12df4:	ba 01 00 00 00       	mov    $0x1,%edx
   12df9:	ff 15 e1 5a 07 00    	call   *0x75ae1(%rip)        # 888e0 <_GLOBAL_OFFSET_TABLE_+0x3b0>
   12dff:	48 89 df             	mov    %rbx,%rdi
   12e02:	e8 49 52 ff ff       	call   8050 <_Unwind_Resume@plt>
   12e07:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   12e0e:	00 00 

0000000000012e10 <star_core::serv::enclave::Enclave::new>:
   12e10:	55                   	push   %rbp
   12e11:	48 89 e5             	mov    %rsp,%rbp
   12e14:	41 57                	push   %r15
   12e16:	41 56                	push   %r14
   12e18:	53                   	push   %rbx
   12e19:	48 81 ec 08 01 00 00 	sub    $0x108,%rsp
   12e20:	49 89 d6             	mov    %rdx,%r14
   12e23:	41 89 f7             	mov    %esi,%r15d
   12e26:	48 89 fb             	mov    %rdi,%rbx
   12e29:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   12e30:	ba 20 00 00 00       	mov    $0x20,%edx
   12e35:	48 89 ce             	mov    %rcx,%rsi
   12e38:	ff 15 6a 57 07 00    	call   *0x7576a(%rip)        # 885a8 <_GLOBAL_OFFSET_TABLE_+0x78>
   12e3e:	80 bd 60 ff ff ff 00 	cmpb   $0x0,-0xa0(%rbp)
   12e45:	74 2b                	je     12e72 <star_core::serv::enclave::Enclave::new+0x62>
   12e47:	48 8b 85 78 ff ff ff 	mov    -0x88(%rbp),%rax
   12e4e:	0f 10 85 68 ff ff ff 	movups -0x98(%rbp),%xmm0
   12e55:	0f 11 85 27 ff ff ff 	movups %xmm0,-0xd9(%rbp)
   12e5c:	48 89 85 37 ff ff ff 	mov    %rax,-0xc9(%rbp)
   12e63:	48 89 43 18          	mov    %rax,0x18(%rbx)
   12e67:	0f 11 43 08          	movups %xmm0,0x8(%rbx)
   12e6b:	b0 01                	mov    $0x1,%al
   12e6d:	e9 a4 00 00 00       	jmp    12f16 <star_core::serv::enclave::Enclave::new+0x106>
   12e72:	0f 10 85 61 ff ff ff 	movups -0x9f(%rbp),%xmm0
   12e79:	0f 10 8d 71 ff ff ff 	movups -0x8f(%rbp),%xmm1
   12e80:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
   12e84:	0f 29 4d d0          	movaps %xmm1,-0x30(%rbp)
   12e88:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   12e8f:	48 8d 75 c0          	lea    -0x40(%rbp),%rsi
   12e93:	ff 15 ff 58 07 00    	call   *0x758ff(%rip)        # 88798 <_GLOBAL_OFFSET_TABLE_+0x268>
   12e99:	0f 28 45 c0          	movaps -0x40(%rbp),%xmm0
   12e9d:	0f 28 4d d0          	movaps -0x30(%rbp),%xmm1
   12ea1:	0f 29 8d 30 ff ff ff 	movaps %xmm1,-0xd0(%rbp)
   12ea8:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   12eaf:	0f 10 95 00 ff ff ff 	movups -0x100(%rbp),%xmm2
   12eb6:	0f 10 9d 10 ff ff ff 	movups -0xf0(%rbp),%xmm3
   12ebd:	0f 29 95 40 ff ff ff 	movaps %xmm2,-0xc0(%rbp)
   12ec4:	0f 29 9d 50 ff ff ff 	movaps %xmm3,-0xb0(%rbp)
   12ecb:	0f 29 5d 90          	movaps %xmm3,-0x70(%rbp)
   12ecf:	0f 29 55 80          	movaps %xmm2,-0x80(%rbp)
=== A=0x40121b5 F=0x121b5 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000012155 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x4b5>:
   12155:	7f 65                	jg     121bc <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x51c>
   12157:	07                   	(bad)
   12158:	00 0f                	add    %cl,(%rdi)
   1215a:	b6 4c                	mov    $0x4c,%dh
   1215c:	24 10                	and    $0x10,%al
   1215e:	22 4c 24 0f          	and    0xf(%rsp),%cl
   12162:	0f b6 54 24 0e       	movzbl 0xe(%rsp),%edx
   12167:	22 54 24 0d          	and    0xd(%rsp),%dl
   1216b:	20 ca                	and    %cl,%dl
   1216d:	44 22 74 24 0c       	and    0xc(%rsp),%r14b
   12172:	44 22 74 24 0b       	and    0xb(%rsp),%r14b
   12177:	41 20 d6             	and    %dl,%r14b
   1217a:	44 22 7c 24 0a       	and    0xa(%rsp),%r15b
   1217f:	44 22 7c 24 09       	and    0x9(%rsp),%r15b
   12184:	44 22 7c 24 08       	and    0x8(%rsp),%r15b
   12189:	45 20 f7             	and    %r14b,%r15b
   1218c:	44 22 6c 24 07       	and    0x7(%rsp),%r13b
   12191:	44 22 6c 24 06       	and    0x6(%rsp),%r13b
   12196:	44 22 6c 24 05       	and    0x5(%rsp),%r13b
   1219b:	41 20 c5             	and    %al,%r13b
   1219e:	45 20 fd             	and    %r15b,%r13b
   121a1:	41 80 e5 01          	and    $0x1,%r13b
   121a5:	41 0f b6 fd          	movzbl %r13b,%edi
   121a9:	ff 15 29 65 07 00    	call   *0x76529(%rip)        # 886d8 <_GLOBAL_OFFSET_TABLE_+0x1a8>
   121af:	84 c0                	test   %al,%al
   121b1:	41 0f 94 c6          	sete   %r14b
   121b5:	74 16                	je     121cd <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x52d>
   121b7:	48 8d bb 40 02 00 00 	lea    0x240(%rbx),%rdi
   121be:	48 8b 74 24 30       	mov    0x30(%rsp),%rsi
   121c3:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   121c8:	e8 13 04 00 00       	call   125e0 <cipher::stream::StreamCipher::apply_keystream>
   121cd:	4c 8d bb 40 02 00 00 	lea    0x240(%rbx),%r15
   121d4:	4c 89 ff             	mov    %r15,%rdi
   121d7:	e8 84 1f 00 00       	call   14160 <<cipher::stream_wrapper::StreamCipherCoreWrapper<T> as core::ops::drop::Drop>::drop>
   121dc:	41 c7 07 00 00 00 00 	movl   $0x0,(%r15)
   121e3:	c7 83 44 02 00 00 00 	movl   $0x0,0x244(%rbx)
   121ea:	00 00 00 
   121ed:	c7 83 48 02 00 00 00 	movl   $0x0,0x248(%rbx)
   121f4:	00 00 00 
   121f7:	c7 83 4c 02 00 00 00 	movl   $0x0,0x24c(%rbx)
   121fe:	00 00 00 
   12201:	c7 83 50 02 00 00 00 	movl   $0x0,0x250(%rbx)
   12208:	00 00 00 
   1220b:	c7 83 54 02 00 00 00 	movl   $0x0,0x254(%rbx)
   12212:	00 00 00 
   12215:	c7 83 58 02 00 00 00 	movl   $0x0,0x258(%rbx)
   1221c:	00 00 00 
   1221f:	c7 83 5c 02 00 00 00 	movl   $0x0,0x25c(%rbx)
   12226:	00 00 00 
   12229:	c7 83 60 02 00 00 00 	movl   $0x0,0x260(%rbx)
   12230:	00 00 00 
   12233:	c7 83 64 02 00 00 00 	movl   $0x0,0x264(%rbx)
   1223a:	00 00 00 
   1223d:	c7 83 68 02 00 00 00 	movl   $0x0,0x268(%rbx)
   12244:	00 00 00 
   12247:	c7 83 6c 02 00 00 00 	movl   $0x0,0x26c(%rbx)
   1224e:	00 00 00 
   12251:	c7 83 70 02 00 00 00 	movl   $0x0,0x270(%rbx)
   12258:	00 00 00 
   1225b:	c7 83 74 02 00 00 00 	movl   $0x0,0x274(%rbx)
   12262:	00 00 00 
   12265:	c7 83 78 02 00 00 00 	movl   $0x0,0x278(%rbx)
   1226c:	00 00 00 
   1226f:	c7 83 7c 02 00 00 00 	movl   $0x0,0x27c(%rbx)
   12276:	00 00 00 
   12279:	44 89 f0             	mov    %r14d,%eax
   1227c:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   12280:	5b                   	pop    %rbx
   12281:	41 5c                	pop    %r12
   12283:	41 5d                	pop    %r13
   12285:	41 5e                	pop    %r14
   12287:	41 5f                	pop    %r15
   12289:	5d                   	pop    %rbp
   1228a:	c3                   	ret
   1228b:	eb 3e                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   1228d:	eb 3c                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   1228f:	eb 3a                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   12291:	eb 38                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   12293:	eb 36                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   12295:	eb 34                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   12297:	49 89 c6             	mov    %rax,%r14
   1229a:	4c 89 ff             	mov    %r15,%rdi
   1229d:	e8 de 2e 00 00       	call   15180 <<chacha20::ChaChaCore<R> as core::ops::drop::Drop>::drop>
   122a2:	eb 32                	jmp    122d6 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x636>
   122a4:	ff 15 4e 6c 07 00    	call   *0x76c4e(%rip)        # 88ef8 <_GLOBAL_OFFSET_TABLE_+0x9c8>
   122aa:	49 89 c6             	mov    %rax,%r14
   122ad:	4c 89 ff             	mov    %r15,%rdi
   122b0:	e8 cb 2e 00 00       	call   15180 <<chacha20::ChaChaCore<R> as core::ops::drop::Drop>::drop>
   122b5:	eb 1f                	jmp    122d6 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x636>
   122b7:	ff 15 3b 6c 07 00    	call   *0x76c3b(%rip)        # 88ef8 <_GLOBAL_OFFSET_TABLE_+0x9c8>
   122bd:	eb 0c                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   122bf:	eb 0a                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   122c1:	eb 08                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   122c3:	eb 06                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   122c5:	eb 04                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   122c7:	eb 02                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   122c9:	eb 00                	jmp    122cb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x62b>
   122cb:	49 89 c6             	mov    %rax,%r14
   122ce:	48 89 df             	mov    %rbx,%rdi
   122d1:	e8 2a 02 00 00       	call   12500 <core::ptr::drop_in_place<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm,typenum::bit::B1>,typenum::bit::B0>,typenum::bit::B1>,typenum::bit::B0>>>>>>
   122d6:	4c 89 f7             	mov    %r14,%rdi
   122d9:	e8 72 5d ff ff       	call   8050 <_Unwind_Resume@plt>
   122de:	ff 15 14 6c 07 00    	call   *0x76c14(%rip)        # 88ef8 <_GLOBAL_OFFSET_TABLE_+0x9c8>
   122e4:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   122eb:	00 00 00 
   122ee:	66 90                	xchg   %ax,%ax

00000000000122f0 <chacha20poly1305::cipher::Cipher<C>::new>:
   122f0:	55                   	push   %rbp
   122f1:	48 89 e5             	mov    %rsp,%rbp
   122f4:	41 57                	push   %r15
   122f6:	41 56                	push   %r14
   122f8:	53                   	push   %rbx
   122f9:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   122fd:	48 81 ec 80 02 00 00 	sub    $0x280,%rsp
   12304:	49 89 f6             	mov    %rsi,%r14
   12307:	48 89 fb             	mov    %rdi,%rbx
   1230a:	0f 57 c0             	xorps  %xmm0,%xmm0
   1230d:	0f 29 44 24 10       	movaps %xmm0,0x10(%rsp)
   12312:	0f 29 04 24          	movaps %xmm0,(%rsp)
   12316:	48 89 e6             	mov    %rsp,%rsi
   12319:	ba 20 00 00 00       	mov    $0x20,%edx
   1231e:	4c 89 f7             	mov    %r14,%rdi
   12321:	e8 ba 02 00 00       	call   125e0 <cipher::stream::StreamCipher::apply_keystream>
   12326:	48 8d 7c 24 20       	lea    0x20(%rsp),%rdi
   1232b:	48 89 e6             	mov    %rsp,%rsi
   1232e:	ff 15 2c 67 07 00    	call   *0x7672c(%rip)        # 88a60 <_GLOBAL_OFFSET_TABLE_+0x530>
   12334:	c6 04 24 00          	movb   $0x0,(%rsp)
   12338:	c6 44 24 01 00       	movb   $0x0,0x1(%rsp)
   1233d:	c6 44 24 02 00       	movb   $0x0,0x2(%rsp)
   12342:	c6 44 24 03 00       	movb   $0x0,0x3(%rsp)
   12347:	c6 44 24 04 00       	movb   $0x0,0x4(%rsp)
   1234c:	c6 44 24 05 00       	movb   $0x0,0x5(%rsp)
   12351:	c6 44 24 06 00       	movb   $0x0,0x6(%rsp)
   12356:	c6 44 24 07 00       	movb   $0x0,0x7(%rsp)
   1235b:	c6 44 24 08 00       	movb   $0x0,0x8(%rsp)
   12360:	c6 44 24 09 00       	movb   $0x0,0x9(%rsp)
   12365:	c6 44 24 0a 00       	movb   $0x0,0xa(%rsp)
   1236a:	c6 44 24 0b 00       	movb   $0x0,0xb(%rsp)
   1236f:	c6 44 24 0c 00       	movb   $0x0,0xc(%rsp)
   12374:	c6 44 24 0d 00       	movb   $0x0,0xd(%rsp)
   12379:	c6 44 24 0e 00       	movb   $0x0,0xe(%rsp)
   1237e:	c6 44 24 0f 00       	movb   $0x0,0xf(%rsp)
   12383:	c6 44 24 10 00       	movb   $0x0,0x10(%rsp)
   12388:	c6 44 24 11 00       	movb   $0x0,0x11(%rsp)
   1238d:	c6 44 24 12 00       	movb   $0x0,0x12(%rsp)
   12392:	c6 44 24 13 00       	movb   $0x0,0x13(%rsp)
   12397:	c6 44 24 14 00       	movb   $0x0,0x14(%rsp)
   1239c:	c6 44 24 15 00       	movb   $0x0,0x15(%rsp)
   123a1:	c6 44 24 16 00       	movb   $0x0,0x16(%rsp)
   123a6:	c6 44 24 17 00       	movb   $0x0,0x17(%rsp)
   123ab:	c6 44 24 18 00       	movb   $0x0,0x18(%rsp)
   123b0:	c6 44 24 19 00       	movb   $0x0,0x19(%rsp)
=== A=0x4013af9 F=0x13af9 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000013a99 <star_core::serv::core::cred::Credential::is_valid+0x489>:
   13a99:	d0 66 0f             	shlb   $1,0xf(%rsi)
   13a9c:	ef                   	out    %eax,(%dx)
   13a9d:	db 66 0f             	(bad) 0xf(%rsi)
   13aa0:	68 d3 f2 0f 70       	push   $0x700ff2d3
   13aa5:	d2 1b                	rcrb   %cl,(%rbx)
   13aa7:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   13aac:	66 0f 60 c3          	punpcklbw %xmm3,%xmm0
   13ab0:	f2 0f 70 c0 1b       	pshuflw $0x1b,%xmm0,%xmm0
   13ab5:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   13aba:	66 0f 67 c2          	packuswb %xmm2,%xmm0
   13abe:	66 0f 6f d1          	movdqa %xmm1,%xmm2
   13ac2:	66 0f 68 d3          	punpckhbw %xmm3,%xmm2
   13ac6:	f2 0f 70 d2 1b       	pshuflw $0x1b,%xmm2,%xmm2
   13acb:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   13ad0:	66 0f 60 cb          	punpcklbw %xmm3,%xmm1
   13ad4:	f2 0f 70 c9 1b       	pshuflw $0x1b,%xmm1,%xmm1
   13ad9:	f3 0f 70 c9 1b       	pshufhw $0x1b,%xmm1,%xmm1
   13ade:	66 0f 67 ca          	packuswb %xmm2,%xmm1
   13ae2:	66 0f 7f 85 30 ff ff 	movdqa %xmm0,-0xd0(%rbp)
   13ae9:	ff 
   13aea:	66 0f 7f 8d 40 ff ff 	movdqa %xmm1,-0xc0(%rbp)
   13af1:	ff 
   13af2:	41 80 f6 01          	xor    $0x1,%r14b
   13af6:	45 08 fe             	or     %r15b,%r14b
   13af9:	75 30                	jne    13b2b <star_core::serv::core::cred::Credential::is_valid+0x51b>
   13afb:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   13aff:	f3 0f 6f 40 20       	movdqu 0x20(%rax),%xmm0
   13b04:	f3 0f 6f 48 30       	movdqu 0x30(%rax),%xmm1
   13b09:	66 0f 74 8d 40 ff ff 	pcmpeqb -0xc0(%rbp),%xmm1
   13b10:	ff 
   13b11:	66 0f 74 85 30 ff ff 	pcmpeqb -0xd0(%rbp),%xmm0
   13b18:	ff 
   13b19:	66 0f db c1          	pand   %xmm1,%xmm0
   13b1d:	66 0f d7 c0          	pmovmskb %xmm0,%eax
   13b21:	3d ff ff 00 00       	cmp    $0xffff,%eax
   13b26:	0f 94 c0             	sete   %al
   13b29:	eb 02                	jmp    13b2d <star_core::serv::core::cred::Credential::is_valid+0x51d>
   13b2b:	31 c0                	xor    %eax,%eax
   13b2d:	48 81 c4 e8 01 00 00 	add    $0x1e8,%rsp
   13b34:	5b                   	pop    %rbx
   13b35:	41 5c                	pop    %r12
   13b37:	41 5d                	pop    %r13
   13b39:	41 5e                	pop    %r14
   13b3b:	41 5f                	pop    %r15
   13b3d:	5d                   	pop    %rbp
   13b3e:	c3                   	ret
   13b3f:	90                   	nop

0000000000013b40 <core::ptr::drop_in_place<hpke::aead::AeadCtxR<hpke::aead::chacha20_poly1305::ChaCha20Poly1305,hpke::kdf::HkdfSha256,hpke::kem::dhkem::x25519_hkdfsha256::X25519HkdfSha256>>>:
   13b40:	55                   	push   %rbp
   13b41:	48 89 e5             	mov    %rsp,%rbp
   13b44:	c6 07 00             	movb   $0x0,(%rdi)
   13b47:	c6 47 01 00          	movb   $0x0,0x1(%rdi)
   13b4b:	c6 47 02 00          	movb   $0x0,0x2(%rdi)
   13b4f:	c6 47 03 00          	movb   $0x0,0x3(%rdi)
   13b53:	c6 47 04 00          	movb   $0x0,0x4(%rdi)
   13b57:	c6 47 05 00          	movb   $0x0,0x5(%rdi)
   13b5b:	c6 47 06 00          	movb   $0x0,0x6(%rdi)
   13b5f:	c6 47 07 00          	movb   $0x0,0x7(%rdi)
   13b63:	c6 47 08 00          	movb   $0x0,0x8(%rdi)
   13b67:	c6 47 09 00          	movb   $0x0,0x9(%rdi)
   13b6b:	c6 47 0a 00          	movb   $0x0,0xa(%rdi)
   13b6f:	c6 47 0b 00          	movb   $0x0,0xb(%rdi)
   13b73:	c6 47 0c 00          	movb   $0x0,0xc(%rdi)
   13b77:	c6 47 0d 00          	movb   $0x0,0xd(%rdi)
   13b7b:	c6 47 0e 00          	movb   $0x0,0xe(%rdi)
   13b7f:	c6 47 0f 00          	movb   $0x0,0xf(%rdi)
   13b83:	c6 47 10 00          	movb   $0x0,0x10(%rdi)
   13b87:	c6 47 11 00          	movb   $0x0,0x11(%rdi)
   13b8b:	c6 47 12 00          	movb   $0x0,0x12(%rdi)
   13b8f:	c6 47 13 00          	movb   $0x0,0x13(%rdi)
   13b93:	c6 47 14 00          	movb   $0x0,0x14(%rdi)
   13b97:	c6 47 15 00          	movb   $0x0,0x15(%rdi)
   13b9b:	c6 47 16 00          	movb   $0x0,0x16(%rdi)
   13b9f:	c6 47 17 00          	movb   $0x0,0x17(%rdi)
   13ba3:	c6 47 18 00          	movb   $0x0,0x18(%rdi)
   13ba7:	c6 47 19 00          	movb   $0x0,0x19(%rdi)
   13bab:	c6 47 1a 00          	movb   $0x0,0x1a(%rdi)
   13baf:	c6 47 1b 00          	movb   $0x0,0x1b(%rdi)
   13bb3:	c6 47 1c 00          	movb   $0x0,0x1c(%rdi)
   13bb7:	c6 47 1d 00          	movb   $0x0,0x1d(%rdi)
   13bbb:	c6 47 1e 00          	movb   $0x0,0x1e(%rdi)
   13bbf:	c6 47 1f 00          	movb   $0x0,0x1f(%rdi)
   13bc3:	c6 47 48 00          	movb   $0x0,0x48(%rdi)
   13bc7:	c6 47 49 00          	movb   $0x0,0x49(%rdi)
   13bcb:	c6 47 4a 00          	movb   $0x0,0x4a(%rdi)
   13bcf:	c6 47 4b 00          	movb   $0x0,0x4b(%rdi)
   13bd3:	c6 47 4c 00          	movb   $0x0,0x4c(%rdi)
   13bd7:	c6 47 4d 00          	movb   $0x0,0x4d(%rdi)
   13bdb:	c6 47 4e 00          	movb   $0x0,0x4e(%rdi)
   13bdf:	c6 47 4f 00          	movb   $0x0,0x4f(%rdi)
   13be3:	c6 47 50 00          	movb   $0x0,0x50(%rdi)
   13be7:	c6 47 51 00          	movb   $0x0,0x51(%rdi)
   13beb:	c6 47 52 00          	movb   $0x0,0x52(%rdi)
   13bef:	c6 47 53 00          	movb   $0x0,0x53(%rdi)
   13bf3:	c6 47 20 00          	movb   $0x0,0x20(%rdi)
   13bf7:	c6 47 21 00          	movb   $0x0,0x21(%rdi)
   13bfb:	c6 47 22 00          	movb   $0x0,0x22(%rdi)
   13bff:	c6 47 23 00          	movb   $0x0,0x23(%rdi)
   13c03:	c6 47 24 00          	movb   $0x0,0x24(%rdi)
   13c07:	c6 47 25 00          	movb   $0x0,0x25(%rdi)
   13c0b:	c6 47 26 00          	movb   $0x0,0x26(%rdi)
   13c0f:	c6 47 27 00          	movb   $0x0,0x27(%rdi)
   13c13:	c6 47 28 00          	movb   $0x0,0x28(%rdi)
   13c17:	c6 47 29 00          	movb   $0x0,0x29(%rdi)
   13c1b:	c6 47 2a 00          	movb   $0x0,0x2a(%rdi)
   13c1f:	c6 47 2b 00          	movb   $0x0,0x2b(%rdi)
   13c23:	c6 47 2c 00          	movb   $0x0,0x2c(%rdi)
   13c27:	c6 47 2d 00          	movb   $0x0,0x2d(%rdi)
   13c2b:	c6 47 2e 00          	movb   $0x0,0x2e(%rdi)
   13c2f:	c6 47 2f 00          	movb   $0x0,0x2f(%rdi)
   13c33:	c6 47 30 00          	movb   $0x0,0x30(%rdi)
   13c37:	c6 47 31 00          	movb   $0x0,0x31(%rdi)
   13c3b:	c6 47 32 00          	movb   $0x0,0x32(%rdi)
   13c3f:	c6 47 33 00          	movb   $0x0,0x33(%rdi)
   13c43:	c6 47 34 00          	movb   $0x0,0x34(%rdi)
   13c47:	c6 47 35 00          	movb   $0x0,0x35(%rdi)
   13c4b:	c6 47 36 00          	movb   $0x0,0x36(%rdi)
   13c4f:	c6 47 37 00          	movb   $0x0,0x37(%rdi)
   13c53:	c6 47 38 00          	movb   $0x0,0x38(%rdi)
   13c57:	c6 47 39 00          	movb   $0x0,0x39(%rdi)
   13c5b:	c6 47 3a 00          	movb   $0x0,0x3a(%rdi)
   13c5f:	c6 47 3b 00          	movb   $0x0,0x3b(%rdi)
   13c63:	c6 47 3c 00          	movb   $0x0,0x3c(%rdi)
   13c67:	c6 47 3d 00          	movb   $0x0,0x3d(%rdi)
   13c6b:	c6 47 3e 00          	movb   $0x0,0x3e(%rdi)
   13c6f:	c6 47 3f 00          	movb   $0x0,0x3f(%rdi)
   13c73:	48 c7 47 40 00 00 00 	movq   $0x0,0x40(%rdi)
   13c7a:	00 
   13c7b:	5d                   	pop    %rbp
   13c7c:	c3                   	ret
   13c7d:	0f 1f 00             	nopl   (%rax)

0000000000013c80 <core::ptr::drop_in_place<hpke::setup::ExporterSecret<hpke::kdf::HkdfSha256>>>:
   13c80:	55                   	push   %rbp
   13c81:	48 89 e5             	mov    %rsp,%rbp
   13c84:	c6 07 00             	movb   $0x0,(%rdi)
   13c87:	c6 47 01 00          	movb   $0x0,0x1(%rdi)
   13c8b:	c6 47 02 00          	movb   $0x0,0x2(%rdi)
   13c8f:	c6 47 03 00          	movb   $0x0,0x3(%rdi)
   13c93:	c6 47 04 00          	movb   $0x0,0x4(%rdi)
   13c97:	c6 47 05 00          	movb   $0x0,0x5(%rdi)
   13c9b:	c6 47 06 00          	movb   $0x0,0x6(%rdi)
   13c9f:	c6 47 07 00          	movb   $0x0,0x7(%rdi)
   13ca3:	c6 47 08 00          	movb   $0x0,0x8(%rdi)
   13ca7:	c6 47 09 00          	movb   $0x0,0x9(%rdi)
   13cab:	c6 47 0a 00          	movb   $0x0,0xa(%rdi)
   13caf:	c6 47 0b 00          	movb   $0x0,0xb(%rdi)
   13cb3:	c6 47 0c 00          	movb   $0x0,0xc(%rdi)
   13cb7:	c6 47 0d 00          	movb   $0x0,0xd(%rdi)
   13cbb:	c6 47 0e 00          	movb   $0x0,0xe(%rdi)
   13cbf:	c6 47 0f 00          	movb   $0x0,0xf(%rdi)
   13cc3:	c6 47 10 00          	movb   $0x0,0x10(%rdi)
   13cc7:	c6 47 11 00          	movb   $0x0,0x11(%rdi)
   13ccb:	c6 47 12 00          	movb   $0x0,0x12(%rdi)
   13ccf:	c6 47 13 00          	movb   $0x0,0x13(%rdi)
   13cd3:	c6 47 14 00          	movb   $0x0,0x14(%rdi)
   13cd7:	c6 47 15 00          	movb   $0x0,0x15(%rdi)
   13cdb:	c6 47 16 00          	movb   $0x0,0x16(%rdi)
   13cdf:	c6 47 17 00          	movb   $0x0,0x17(%rdi)
   13ce3:	c6 47 18 00          	movb   $0x0,0x18(%rdi)
   13ce7:	c6 47 19 00          	movb   $0x0,0x19(%rdi)
   13ceb:	c6 47 1a 00          	movb   $0x0,0x1a(%rdi)
   13cef:	c6 47 1b 00          	movb   $0x0,0x1b(%rdi)
   13cf3:	c6 47 1c 00          	movb   $0x0,0x1c(%rdi)
   13cf7:	c6                   	.byte 0xc6
   13cf8:	47                   	rex.RXB
