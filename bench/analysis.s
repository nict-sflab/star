=== A=0x4018785 F=0x18785 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000018585 <<hpke::dhkex::x25519::X25519 as hpke::dhkex::DhKeyExchange>::dh+0x105>:
   18585:	7d aa                	jge    18531 <<hpke::dhkex::x25519::X25519 as hpke::dhkex::DhKeyExchange>::dh+0xb1>
   18587:	00 40 0f             	add    %al,0xf(%rax)
   1858a:	94                   	xchg   %eax,%esp
   1858b:	c7                   	(bad)
   1858c:	ff 15 36 31 07 00    	call   *0x73136(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   18592:	88 45 c8             	mov    %al,-0x38(%rbp)
   18595:	31 ff                	xor    %edi,%edi
   18597:	80 7d ab 00          	cmpb   $0x0,-0x55(%rbp)
   1859b:	40 0f 94 c7          	sete   %dil
   1859f:	ff 15 23 31 07 00    	call   *0x73123(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   185a5:	88 45 c9             	mov    %al,-0x37(%rbp)
   185a8:	31 ff                	xor    %edi,%edi
   185aa:	80 7d ac 00          	cmpb   $0x0,-0x54(%rbp)
   185ae:	40 0f 94 c7          	sete   %dil
   185b2:	ff 15 10 31 07 00    	call   *0x73110(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   185b8:	88 45 ca             	mov    %al,-0x36(%rbp)
   185bb:	31 ff                	xor    %edi,%edi
   185bd:	80 7d ad 00          	cmpb   $0x0,-0x53(%rbp)
   185c1:	40 0f 94 c7          	sete   %dil
   185c5:	ff 15 fd 30 07 00    	call   *0x730fd(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   185cb:	88 45 cb             	mov    %al,-0x35(%rbp)
   185ce:	31 ff                	xor    %edi,%edi
   185d0:	80 7d ae 00          	cmpb   $0x0,-0x52(%rbp)
   185d4:	40 0f 94 c7          	sete   %dil
   185d8:	ff 15 ea 30 07 00    	call   *0x730ea(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   185de:	41 89 c6             	mov    %eax,%r14d
   185e1:	31 ff                	xor    %edi,%edi
   185e3:	80 7d af 00          	cmpb   $0x0,-0x51(%rbp)
   185e7:	40 0f 94 c7          	sete   %dil
   185eb:	ff 15 d7 30 07 00    	call   *0x730d7(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   185f1:	88 45 cc             	mov    %al,-0x34(%rbp)
   185f4:	31 ff                	xor    %edi,%edi
   185f6:	80 7d b0 00          	cmpb   $0x0,-0x50(%rbp)
   185fa:	40 0f 94 c7          	sete   %dil
   185fe:	ff 15 c4 30 07 00    	call   *0x730c4(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   18604:	88 45 cd             	mov    %al,-0x33(%rbp)
   18607:	31 ff                	xor    %edi,%edi
   18609:	80 7d b1 00          	cmpb   $0x0,-0x4f(%rbp)
   1860d:	40 0f 94 c7          	sete   %dil
   18611:	ff 15 b1 30 07 00    	call   *0x730b1(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   18617:	88 45 ce             	mov    %al,-0x32(%rbp)
   1861a:	31 ff                	xor    %edi,%edi
   1861c:	80 7d b2 00          	cmpb   $0x0,-0x4e(%rbp)
   18620:	40 0f 94 c7          	sete   %dil
   18624:	ff 15 9e 30 07 00    	call   *0x7309e(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   1862a:	88 45 cf             	mov    %al,-0x31(%rbp)
   1862d:	31 ff                	xor    %edi,%edi
   1862f:	80 7d b3 00          	cmpb   $0x0,-0x4d(%rbp)
   18633:	40 0f 94 c7          	sete   %dil
   18637:	ff 15 8b 30 07 00    	call   *0x7308b(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   1863d:	88 45 d0             	mov    %al,-0x30(%rbp)
   18640:	31 ff                	xor    %edi,%edi
   18642:	80 7d b4 00          	cmpb   $0x0,-0x4c(%rbp)
   18646:	40 0f 94 c7          	sete   %dil
   1864a:	ff 15 78 30 07 00    	call   *0x73078(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   18650:	41 89 c4             	mov    %eax,%r12d
   18653:	31 ff                	xor    %edi,%edi
   18655:	80 7d b5 00          	cmpb   $0x0,-0x4b(%rbp)
   18659:	40 0f 94 c7          	sete   %dil
   1865d:	ff 15 65 30 07 00    	call   *0x73065(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   18663:	88 45 d1             	mov    %al,-0x2f(%rbp)
   18666:	31 ff                	xor    %edi,%edi
   18668:	80 7d b6 00          	cmpb   $0x0,-0x4a(%rbp)
   1866c:	40 0f 94 c7          	sete   %dil
   18670:	ff 15 52 30 07 00    	call   *0x73052(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   18676:	88 45 d2             	mov    %al,-0x2e(%rbp)
   18679:	31 ff                	xor    %edi,%edi
   1867b:	80 7d b7 00          	cmpb   $0x0,-0x49(%rbp)
   1867f:	40 0f 94 c7          	sete   %dil
   18683:	ff 15 3f 30 07 00    	call   *0x7303f(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   18689:	88 45 d3             	mov    %al,-0x2d(%rbp)
   1868c:	31 ff                	xor    %edi,%edi
   1868e:	80 7d b8 00          	cmpb   $0x0,-0x48(%rbp)
   18692:	40 0f 94 c7          	sete   %dil
   18696:	ff 15 2c 30 07 00    	call   *0x7302c(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   1869c:	88 45 d4             	mov    %al,-0x2c(%rbp)
   1869f:	31 ff                	xor    %edi,%edi
   186a1:	80 7d b9 00          	cmpb   $0x0,-0x47(%rbp)
   186a5:	40 0f 94 c7          	sete   %dil
   186a9:	ff 15 19 30 07 00    	call   *0x73019(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   186af:	88 45 d5             	mov    %al,-0x2b(%rbp)
   186b2:	31 ff                	xor    %edi,%edi
   186b4:	80 7d ba 00          	cmpb   $0x0,-0x46(%rbp)
   186b8:	40 0f 94 c7          	sete   %dil
   186bc:	ff 15 06 30 07 00    	call   *0x73006(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   186c2:	88 45 d6             	mov    %al,-0x2a(%rbp)
   186c5:	31 ff                	xor    %edi,%edi
   186c7:	80 7d bb 00          	cmpb   $0x0,-0x45(%rbp)
   186cb:	40 0f 94 c7          	sete   %dil
   186cf:	ff 15 f3 2f 07 00    	call   *0x72ff3(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   186d5:	41 89 c5             	mov    %eax,%r13d
   186d8:	31 ff                	xor    %edi,%edi
   186da:	80 7d bc 00          	cmpb   $0x0,-0x44(%rbp)
   186de:	40 0f 94 c7          	sete   %dil
   186e2:	ff 15 e0 2f 07 00    	call   *0x72fe0(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   186e8:	88 45 d7             	mov    %al,-0x29(%rbp)
   186eb:	31 ff                	xor    %edi,%edi
   186ed:	80 7d bd 00          	cmpb   $0x0,-0x43(%rbp)
   186f1:	40 0f 94 c7          	sete   %dil
   186f5:	ff 15 cd 2f 07 00    	call   *0x72fcd(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   186fb:	0f b6 4d be          	movzbl -0x42(%rbp),%ecx
   186ff:	22 4d bf             	and    -0x41(%rbp),%cl
   18702:	0f b6 55 c0          	movzbl -0x40(%rbp),%edx
   18706:	22 55 c1             	and    -0x3f(%rbp),%dl
   18709:	20 ca                	and    %cl,%dl
   1870b:	0f b6 4d c2          	movzbl -0x3e(%rbp),%ecx
   1870f:	22 4d c3             	and    -0x3d(%rbp),%cl
   18712:	22 4d c4             	and    -0x3c(%rbp),%cl
   18715:	20 d1                	and    %dl,%cl
   18717:	44 22 7d c5          	and    -0x3b(%rbp),%r15b
   1871b:	44 22 7d c6          	and    -0x3a(%rbp),%r15b
   1871f:	44 22 7d c7          	and    -0x39(%rbp),%r15b
   18723:	41 20 cf             	and    %cl,%r15b
   18726:	22 5d c8             	and    -0x38(%rbp),%bl
   18729:	22 5d c9             	and    -0x37(%rbp),%bl
   1872c:	22 5d ca             	and    -0x36(%rbp),%bl
   1872f:	22 5d cb             	and    -0x35(%rbp),%bl
   18732:	44 20 fb             	and    %r15b,%bl
   18735:	44 22 75 cc          	and    -0x34(%rbp),%r14b
   18739:	44 22 75 cd          	and    -0x33(%rbp),%r14b
   1873d:	44 22 75 ce          	and    -0x32(%rbp),%r14b
   18741:	44 22 75 cf          	and    -0x31(%rbp),%r14b
   18745:	44 22 75 d0          	and    -0x30(%rbp),%r14b
   18749:	41 20 de             	and    %bl,%r14b
   1874c:	44 22 65 d1          	and    -0x2f(%rbp),%r12b
   18750:	44 22 65 d2          	and    -0x2e(%rbp),%r12b
   18754:	44 22 65 d3          	and    -0x2d(%rbp),%r12b
   18758:	44 22 65 d4          	and    -0x2c(%rbp),%r12b
   1875c:	44 22 65 d5          	and    -0x2b(%rbp),%r12b
   18760:	44 22 65 d6          	and    -0x2a(%rbp),%r12b
   18764:	45 20 f4             	and    %r14b,%r12b
   18767:	44 22 6d d7          	and    -0x29(%rbp),%r13b
   1876b:	41 20 c5             	and    %al,%r13b
   1876e:	45 20 e5             	and    %r12b,%r13b
   18771:	41 80 e5 01          	and    $0x1,%r13b
   18775:	41 0f b6 fd          	movzbl %r13b,%edi
   18779:	ff 15 49 2f 07 00    	call   *0x72f49(%rip)        # 8b6c8 <_GLOBAL_OFFSET_TABLE_+0x180>
   1877f:	84 c0                	test   %al,%al
   18781:	48 8b 5d 90          	mov    -0x70(%rbp),%rbx
   18785:	74 0f                	je     18796 <<hpke::dhkex::x25519::X25519 as hpke::dhkex::DhKeyExchange>::dh+0x316>
   18787:	c6 03 01             	movb   $0x1,(%rbx)
   1878a:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   1878e:	ff 15 1c 37 07 00    	call   *0x7371c(%rip)        # 8beb0 <_GLOBAL_OFFSET_TABLE_+0x968>
   18794:	eb 13                	jmp    187a9 <<hpke::dhkex::x25519::X25519 as hpke::dhkex::DhKeyExchange>::dh+0x329>
   18796:	0f 10 45 9e          	movups -0x62(%rbp),%xmm0
   1879a:	0f 10 4d ae          	movups -0x52(%rbp),%xmm1
   1879e:	0f 11 4b 11          	movups %xmm1,0x11(%rbx)
   187a2:	0f 11 43 01          	movups %xmm0,0x1(%rbx)
   187a6:	c6 03 00             	movb   $0x0,(%rbx)
   187a9:	48 89 d8             	mov    %rbx,%rax
   187ac:	48 83 c4 48          	add    $0x48,%rsp
   187b0:	5b                   	pop    %rbx
   187b1:	41 5c                	pop    %r12
   187b3:	41 5d                	pop    %r13
   187b5:	41 5e                	pop    %r14
   187b7:	41 5f                	pop    %r15
   187b9:	5d                   	pop    %rbp
   187ba:	c3                   	ret
   187bb:	eb 00                	jmp    187bd <<hpke::dhkex::x25519::X25519 as hpke::dhkex::DhKeyExchange>::dh+0x33d>
   187bd:	48 89 c3             	mov    %rax,%rbx
   187c0:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   187c4:	ff 15 e6 36 07 00    	call   *0x736e6(%rip)        # 8beb0 <_GLOBAL_OFFSET_TABLE_+0x968>
   187ca:	48 89 df             	mov    %rbx,%rdi
   187cd:	e8 7e d8 fe ff       	call   6050 <_Unwind_Resume@plt>
   187d2:	ff 15 38 32 07 00    	call   *0x73238(%rip)        # 8ba10 <_GLOBAL_OFFSET_TABLE_+0x4c8>
   187d8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   187df:	00 

00000000000187e0 <<hkdf::Hkdf<D,hmac::simple::SimpleHmac<D>> as hpke::kdf::LabeledExpand>::labeled_expand>:
   187e0:	55                   	push   %rbp
   187e1:	48 89 e5             	mov    %rsp,%rbp
   187e4:	41 57                	push   %r15
   187e6:	41 56                	push   %r14
   187e8:	41 55                	push   %r13
   187ea:	41 54                	push   %r12
   187ec:	53                   	push   %rbx
   187ed:	48 81 ec 48 02 00 00 	sub    $0x248,%rsp
   187f4:	48 89 7d a0          	mov    %rdi,-0x60(%rbp)
   187f8:	48 8b 7d 20          	mov    0x20(%rbp),%rdi
   187fc:	b0 01                	mov    $0x1,%al
   187fe:	48 81 ff ff ff 00 00 	cmp    $0xffff,%rdi
   18805:	77 3f                	ja     18846 <<hkdf::Hkdf<D,hmac::simple::SimpleHmac<D>> as hpke::kdf::LabeledExpand>::labeled_expand+0x66>
   18807:	48 89 55 b8          	mov    %rdx,-0x48(%rbp)
   1880b:	48 89 75 c0          	mov    %rsi,-0x40(%rbp)
   1880f:	48 89 8d b0 fe ff ff 	mov    %rcx,-0x150(%rbp)
   18816:	4c 89 45 b0          	mov    %r8,-0x50(%rbp)
   1881a:	4c 89 4d a8          	mov    %r9,-0x58(%rbp)
   1881e:	89 f8                	mov    %edi,%eax
   18820:	66 c1 c0 08          	rol    $0x8,%ax
   18824:	66 89 45 d2          	mov    %ax,-0x2e(%rbp)
   18828:	66 89 45 d4          	mov    %ax,-0x2c(%rbp)
   1882c:	48 8d 87 1f e0 ff ff 	lea    -0x1fe1(%rdi),%rax
   18833:	48 3d 20 e0 ff ff    	cmp    $0xffffffffffffe020,%rax
   18839:	73 1d                	jae    18858 <<hkdf::Hkdf<D,hmac::simple::SimpleHmac<D>> as hpke::kdf::LabeledExpand>::labeled_expand+0x78>
   1883b:	48 81 7d 20 e1 1f 00 	cmpq   $0x1fe1,0x20(%rbp)
   18842:	00 
   18843:	0f 93 c0             	setae  %al
   18846:	48 81 c4 48 02 00 00 	add    $0x248,%rsp
   1884d:	5b                   	pop    %rbx
   1884e:	41 5c                	pop    %r12
   18850:	41 5d                	pop    %r13
   18852:	41 5e                	pop    %r14
   18854:	41 5f                	pop    %r15
   18856:	5d                   	pop    %rbp
   18857:	c3                   	ret
   18858:	4c 8b 75 18          	mov    0x18(%rbp),%r14
   1885c:	48 b9 c0 ff ff ff ff 	movabs $0x7fffffffffffffc0,%rcx
   18863:	ff ff 7f 
   18866:	48 8b 45 a0          	mov    -0x60(%rbp),%rax
   1886a:	48 8d 50 40          	lea    0x40(%rax),%rdx
   1886e:	48 89 95 b8 fe ff ff 	mov    %rdx,-0x148(%rbp)
   18875:	48 8d 50 68          	lea    0x68(%rax),%rdx
   18879:	48 89 95 c0 fe ff ff 	mov    %rdx,-0x140(%rbp)
   18880:	0f b6 80 a8 00 00 00 	movzbl 0xa8(%rax),%eax
   18887:	88 45 d7             	mov    %al,-0x29(%rbp)
   1888a:	0f b6 d0             	movzbl %al,%edx
   1888d:	b8 40 00 00 00       	mov    $0x40,%eax
   18892:	48 29 d0             	sub    %rdx,%rax
   18895:	48 8d 72 e0          	lea    -0x20(%rdx),%rsi
   18899:	48 89 85 d0 fe ff ff 	mov    %rax,-0x130(%rbp)
   188a0:	48 01 e8             	add    %rbp,%rax
   188a3:	48 05 50 fe ff ff    	add    $0xfffffffffffffe50,%rax
   188a9:	48 89 b5 d8 fe ff ff 	mov    %rsi,-0x128(%rbp)
   188b0:	48 21 ce             	and    %rcx,%rsi
   188b3:	48 01 c6             	add    %rax,%rsi
   188b6:	48 89 b5 e0 fe ff ff 	mov    %rsi,-0x120(%rbp)
   188bd:	48 89 d0             	mov    %rdx,%rax
   188c0:	48 83 c8 20          	or     $0x20,%rax
   188c4:	48 89 85 e8 fe ff ff 	mov    %rax,-0x118(%rbp)
   188cb:	41 b7 01             	mov    $0x1,%r15b
   188ce:	48 89 95 c8 fe ff ff 	mov    %rdx,-0x138(%rbp)
   188d5:	48 8d 04 2a          	lea    (%rdx,%rbp,1),%rax
   188d9:	48 05 58 ff ff ff    	add    $0xffffffffffffff58,%rax
   188df:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   188e3:	31 c0                	xor    %eax,%eax
   188e5:	4c 8b 65 20          	mov    0x20(%rbp),%r12
   188e9:	e9 d2 00 00 00       	jmp    189c0 <<hkdf::Hkdf<D,hmac::simple::SimpleHmac<D>> as hpke::kdf::LabeledExpand>::labeled_expand+0x1e0>
   188ee:	66 90                	xchg   %ax,%ax
   188f0:	46 88 bc 2d 58 ff ff 	mov    %r15b,-0xa8(%rbp,%r13,1)
   188f7:	ff 
   188f8:	41 8d 45 01          	lea    0x1(%r13),%eax
   188fc:	48 8b 9d 98 fe ff ff 	mov    -0x168(%rbp),%rbx
   18903:	49 8d 0c 1e          	lea    (%r14,%rbx,1),%rcx
   18907:	48 89 8d a0 fe ff ff 	mov    %rcx,-0x160(%rbp)
   1890e:	49 29 dc             	sub    %rbx,%r12
   18911:	4c 89 a5 a8 fe ff ff 	mov    %r12,-0x158(%rbp)
   18918:	88 45 98             	mov    %al,-0x68(%rbp)
   1891b:	ba b0 00 00 00       	mov    $0xb0,%edx
   18920:	4c 8d a5 90 fd ff ff 	lea    -0x270(%rbp),%r12
   18927:	4c 89 e7             	mov    %r12,%rdi
   1892a:	48 8d b5 f0 fe ff ff 	lea    -0x110(%rbp),%rsi
   18931:	48 8b 05 20 36 07 00 	mov    0x73620(%rip),%rax        # 8bf58 <memcpy@GLIBC_2.14>
   18938:	4d 89 f5             	mov    %r14,%r13
   1893b:	49 89 c6             	mov    %rax,%r14
   1893e:	ff d0                	call   *%rax
   18940:	0f 57 c0             	xorps  %xmm0,%xmm0
   18943:	0f 29 85 80 fe ff ff 	movaps %xmm0,-0x180(%rbp)
   1894a:	0f 29 85 70 fe ff ff 	movaps %xmm0,-0x190(%rbp)
   18951:	4c 89 e7             	mov    %r12,%rdi
   18954:	48 8d b5 70 fe ff ff 	lea    -0x190(%rbp),%rsi
   1895b:	e8 80 f5 ff ff       	call   17ee0 <<hmac::simple::SimpleHmac<D> as digest::FixedOutput>::finalize_into>
   18960:	0f 28 85 70 fe ff ff 	movaps -0x190(%rbp),%xmm0
   18967:	0f 28 8d 80 fe ff ff 	movaps -0x180(%rbp),%xmm1
   1896e:	0f 29 85 90 fd ff ff 	movaps %xmm0,-0x270(%rbp)
   18975:	0f 29 8d a0 fd ff ff 	movaps %xmm1,-0x260(%rbp)
   1897c:	4c 89 ef             	mov    %r13,%rdi
   1897f:	4c 89 e6             	mov    %r12,%rsi
   18982:	4c                   	rex.WR
   18983:	8b                   	.byte 0x8b
   18984:	a5                   	movsl  (%rsi),(%rdi)
=== A=0x4016099 F=0x16099 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000015e99 <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x319>:
   15e99:	f7 48 89 de ff 15 b5 	testl  $0xb515ffde,-0x77(%rax)
   15ea0:	60                   	(bad)
   15ea1:	07                   	(bad)
   15ea2:	00 48 8d             	add    %cl,-0x73(%rax)
   15ea5:	7c 24                	jl     15ecb <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x34b>
   15ea7:	18 4c 89 f6          	sbb    %cl,-0xa(%rcx,%rcx,4)
   15eab:	ff 15 87 60 07 00    	call   *0x76087(%rip)        # 8bf38 <_GLOBAL_OFFSET_TABLE_+0x9f0>
   15eb1:	0f b6 44 24 18       	movzbl 0x18(%rsp),%eax
   15eb6:	31 ff                	xor    %edi,%edi
   15eb8:	41 3a 45 00          	cmp    0x0(%r13),%al
   15ebc:	40 0f 94 c7          	sete   %dil
   15ec0:	ff 15 32 60 07 00    	call   *0x76032(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15ec6:	88 44 24 10          	mov    %al,0x10(%rsp)
   15eca:	0f b6 44 24 19       	movzbl 0x19(%rsp),%eax
   15ecf:	31 ff                	xor    %edi,%edi
   15ed1:	41 3a 45 01          	cmp    0x1(%r13),%al
   15ed5:	40 0f 94 c7          	sete   %dil
   15ed9:	ff 15 19 60 07 00    	call   *0x76019(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15edf:	88 44 24 38          	mov    %al,0x38(%rsp)
   15ee3:	0f b6 44 24 1a       	movzbl 0x1a(%rsp),%eax
   15ee8:	31 ff                	xor    %edi,%edi
   15eea:	41 3a 45 02          	cmp    0x2(%r13),%al
   15eee:	40 0f 94 c7          	sete   %dil
   15ef2:	ff 15 00 60 07 00    	call   *0x76000(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15ef8:	88 44 24 0f          	mov    %al,0xf(%rsp)
   15efc:	0f b6 44 24 1b       	movzbl 0x1b(%rsp),%eax
   15f01:	31 ff                	xor    %edi,%edi
   15f03:	41 3a 45 03          	cmp    0x3(%r13),%al
   15f07:	40 0f 94 c7          	sete   %dil
   15f0b:	ff 15 e7 5f 07 00    	call   *0x75fe7(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15f11:	88 44 24 0e          	mov    %al,0xe(%rsp)
   15f15:	0f b6 44 24 1c       	movzbl 0x1c(%rsp),%eax
   15f1a:	31 ff                	xor    %edi,%edi
   15f1c:	41 3a 45 04          	cmp    0x4(%r13),%al
   15f20:	40 0f 94 c7          	sete   %dil
   15f24:	ff 15 ce 5f 07 00    	call   *0x75fce(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15f2a:	41 89 c7             	mov    %eax,%r15d
   15f2d:	0f b6 44 24 1d       	movzbl 0x1d(%rsp),%eax
   15f32:	31 ff                	xor    %edi,%edi
   15f34:	41 3a 45 05          	cmp    0x5(%r13),%al
   15f38:	40 0f 94 c7          	sete   %dil
   15f3c:	ff 15 b6 5f 07 00    	call   *0x75fb6(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15f42:	88 44 24 0d          	mov    %al,0xd(%rsp)
   15f46:	0f b6 44 24 1e       	movzbl 0x1e(%rsp),%eax
   15f4b:	31 ff                	xor    %edi,%edi
   15f4d:	41 3a 45 06          	cmp    0x6(%r13),%al
   15f51:	40 0f 94 c7          	sete   %dil
   15f55:	ff 15 9d 5f 07 00    	call   *0x75f9d(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15f5b:	88 44 24 0c          	mov    %al,0xc(%rsp)
   15f5f:	0f b6 44 24 1f       	movzbl 0x1f(%rsp),%eax
   15f64:	31 ff                	xor    %edi,%edi
   15f66:	41 3a 45 07          	cmp    0x7(%r13),%al
   15f6a:	40 0f 94 c7          	sete   %dil
   15f6e:	ff 15 84 5f 07 00    	call   *0x75f84(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15f74:	41 89 c6             	mov    %eax,%r14d
   15f77:	0f b6 44 24 20       	movzbl 0x20(%rsp),%eax
   15f7c:	31 ff                	xor    %edi,%edi
   15f7e:	41 3a 45 08          	cmp    0x8(%r13),%al
   15f82:	40 0f 94 c7          	sete   %dil
   15f86:	ff 15 6c 5f 07 00    	call   *0x75f6c(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15f8c:	88 44 24 0b          	mov    %al,0xb(%rsp)
   15f90:	0f b6 44 24 21       	movzbl 0x21(%rsp),%eax
   15f95:	31 ff                	xor    %edi,%edi
   15f97:	41 3a 45 09          	cmp    0x9(%r13),%al
   15f9b:	40 0f 94 c7          	sete   %dil
   15f9f:	ff 15 53 5f 07 00    	call   *0x75f53(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15fa5:	88 44 24 0a          	mov    %al,0xa(%rsp)
   15fa9:	0f b6 44 24 22       	movzbl 0x22(%rsp),%eax
   15fae:	31 ff                	xor    %edi,%edi
   15fb0:	41 3a 45 0a          	cmp    0xa(%r13),%al
   15fb4:	40 0f 94 c7          	sete   %dil
   15fb8:	ff 15 3a 5f 07 00    	call   *0x75f3a(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15fbe:	88 44 24 09          	mov    %al,0x9(%rsp)
   15fc2:	0f b6 44 24 23       	movzbl 0x23(%rsp),%eax
   15fc7:	31 ff                	xor    %edi,%edi
   15fc9:	41 3a 45 0b          	cmp    0xb(%r13),%al
   15fcd:	40 0f 94 c7          	sete   %dil
   15fd1:	ff 15 21 5f 07 00    	call   *0x75f21(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15fd7:	41 89 c4             	mov    %eax,%r12d
   15fda:	0f b6 44 24 24       	movzbl 0x24(%rsp),%eax
   15fdf:	31 ff                	xor    %edi,%edi
   15fe1:	41 3a 45 0c          	cmp    0xc(%r13),%al
   15fe5:	40 0f 94 c7          	sete   %dil
   15fe9:	ff 15 09 5f 07 00    	call   *0x75f09(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   15fef:	88 44 24 08          	mov    %al,0x8(%rsp)
   15ff3:	0f b6 44 24 25       	movzbl 0x25(%rsp),%eax
   15ff8:	31 ff                	xor    %edi,%edi
   15ffa:	41 3a 45 0d          	cmp    0xd(%r13),%al
   15ffe:	40 0f 94 c7          	sete   %dil
   16002:	ff 15 f0 5e 07 00    	call   *0x75ef0(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   16008:	88 44 24 07          	mov    %al,0x7(%rsp)
   1600c:	0f b6 44 24 26       	movzbl 0x26(%rsp),%eax
   16011:	31 ff                	xor    %edi,%edi
   16013:	41 3a 45 0e          	cmp    0xe(%r13),%al
   16017:	40 0f 94 c7          	sete   %dil
   1601b:	ff 15 d7 5e 07 00    	call   *0x75ed7(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   16021:	88 44 24 06          	mov    %al,0x6(%rsp)
   16025:	0f b6 44 24 27       	movzbl 0x27(%rsp),%eax
   1602a:	31 ff                	xor    %edi,%edi
   1602c:	41 3a 45 0f          	cmp    0xf(%r13),%al
   16030:	40 0f 94 c7          	sete   %dil
   16034:	ff 15 be 5e 07 00    	call   *0x75ebe(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   1603a:	0f b6 4c 24 10       	movzbl 0x10(%rsp),%ecx
   1603f:	22 4c 24 38          	and    0x38(%rsp),%cl
   16043:	0f b6 54 24 0f       	movzbl 0xf(%rsp),%edx
   16048:	22 54 24 0e          	and    0xe(%rsp),%dl
   1604c:	20 ca                	and    %cl,%dl
   1604e:	44 22 7c 24 0d       	and    0xd(%rsp),%r15b
   16053:	44 22 7c 24 0c       	and    0xc(%rsp),%r15b
   16058:	41 20 d7             	and    %dl,%r15b
   1605b:	44 22 74 24 0b       	and    0xb(%rsp),%r14b
   16060:	44 22 74 24 0a       	and    0xa(%rsp),%r14b
   16065:	44 22 74 24 09       	and    0x9(%rsp),%r14b
   1606a:	45 20 fe             	and    %r15b,%r14b
   1606d:	44 22 64 24 08       	and    0x8(%rsp),%r12b
   16072:	44 22 64 24 07       	and    0x7(%rsp),%r12b
   16077:	44 22 64 24 06       	and    0x6(%rsp),%r12b
   1607c:	41 20 c4             	and    %al,%r12b
   1607f:	45 20 f4             	and    %r14b,%r12b
   16082:	41 80 e4 01          	and    $0x1,%r12b
   16086:	41 0f b6 fc          	movzbl %r12b,%edi
   1608a:	ff 15 68 5e 07 00    	call   *0x75e68(%rip)        # 8bef8 <_GLOBAL_OFFSET_TABLE_+0x9b0>
   16090:	48 8b 54 24 28       	mov    0x28(%rsp),%rdx
   16095:	84 c0                	test   %al,%al
   16097:	b0 01                	mov    $0x1,%al
   16099:	74 13                	je     160ae <chacha20poly1305::cipher::Cipher<C>::decrypt_in_place_detached+0x52e>
   1609b:	48 8d bb 40 02 00 00 	lea    0x240(%rbx),%rdi
   160a2:	48 8b 74 24 30       	mov    0x30(%rsp),%rsi
   160a7:	e8 d4 07 00 00       	call   16880 <cipher::stream::StreamCipher::apply_keystream>
   160ac:	31 c0                	xor    %eax,%eax
   160ae:	c6 83 80 02 00 00 00 	movb   $0x0,0x280(%rbx)
   160b5:	c6 83 81 02 00 00 00 	movb   $0x0,0x281(%rbx)
   160bc:	c6 83 82 02 00 00 00 	movb   $0x0,0x282(%rbx)
   160c3:	c6 83 83 02 00 00 00 	movb   $0x0,0x283(%rbx)
   160ca:	c6 83 84 02 00 00 00 	movb   $0x0,0x284(%rbx)
   160d1:	c6 83 85 02 00 00 00 	movb   $0x0,0x285(%rbx)
   160d8:	c6 83 86 02 00 00 00 	movb   $0x0,0x286(%rbx)
   160df:	c6 83 87 02 00 00 00 	movb   $0x0,0x287(%rbx)
   160e6:	c6 83 88 02 00 00 00 	movb   $0x0,0x288(%rbx)
   160ed:	c6 83 89 02 00 00 00 	movb   $0x0,0x289(%rbx)
   160f4:	c6 83 8a 02 00 00 00 	movb   $0x0,0x28a(%rbx)
   160fb:	c6 83 8b 02 00 00 00 	movb   $0x0,0x28b(%rbx)
   16102:	c6 83 8c 02 00 00 00 	movb   $0x0,0x28c(%rbx)
   16109:	c6 83 8d 02 00 00 00 	movb   $0x0,0x28d(%rbx)
   16110:	c6 83 8e 02 00 00 00 	movb   $0x0,0x28e(%rbx)
   16117:	c6 83 8f 02 00 00 00 	movb   $0x0,0x28f(%rbx)
   1611e:	c6 83 90 02 00 00 00 	movb   $0x0,0x290(%rbx)
   16125:	c6 83 91 02 00 00 00 	movb   $0x0,0x291(%rbx)
   1612c:	c6 83 92 02 00 00 00 	movb   $0x0,0x292(%rbx)
   16133:	c6 83 93 02 00 00 00 	movb   $0x0,0x293(%rbx)
   1613a:	c6 83 94 02 00 00 00 	movb   $0x0,0x294(%rbx)
   16141:	c6 83 95 02 00 00 00 	movb   $0x0,0x295(%rbx)
   16148:	c6 83 96 02 00 00 00 	movb   $0x0,0x296(%rbx)
   1614f:	c6 83 97 02 00 00 00 	movb   $0x0,0x297(%rbx)
   16156:	c6 83 98 02 00 00 00 	movb   $0x0,0x298(%rbx)
   1615d:	c6 83 99 02 00 00 00 	movb   $0x0,0x299(%rbx)
   16164:	c6 83 9a 02 00 00 00 	movb   $0x0,0x29a(%rbx)
   1616b:	c6 83 9b 02 00 00 00 	movb   $0x0,0x29b(%rbx)
   16172:	c6 83 9c 02 00 00 00 	movb   $0x0,0x29c(%rbx)
   16179:	c6 83 9d 02 00 00 00 	movb   $0x0,0x29d(%rbx)
   16180:	c6 83 9e 02 00 00 00 	movb   $0x0,0x29e(%rbx)
   16187:	c6 83 9f 02 00 00 00 	movb   $0x0,0x29f(%rbx)
   1618e:	c6 83 a0 02 00 00 00 	movb   $0x0,0x2a0(%rbx)
   16195:	c6 83 a1 02 00 00 00 	movb   $0x0,0x2a1(%rbx)
   1619c:	c6 83 a2 02 00 00 00 	movb   $0x0,0x2a2(%rbx)
   161a3:	c6 83 a3 02 00 00 00 	movb   $0x0,0x2a3(%rbx)
   161aa:	c6 83 a4 02 00 00 00 	movb   $0x0,0x2a4(%rbx)
   161b1:	c6 83 a5 02 00 00 00 	movb   $0x0,0x2a5(%rbx)
   161b8:	c6 83 a6 02 00 00 00 	movb   $0x0,0x2a6(%rbx)
   161bf:	c6 83 a7 02 00 00 00 	movb   $0x0,0x2a7(%rbx)
   161c6:	c6 83 a8 02 00 00 00 	movb   $0x0,0x2a8(%rbx)
   161cd:	c6 83 a9 02 00 00 00 	movb   $0x0,0x2a9(%rbx)
   161d4:	c6 83 aa 02 00 00 00 	movb   $0x0,0x2aa(%rbx)
   161db:	c6 83 ab 02 00 00 00 	movb   $0x0,0x2ab(%rbx)
   161e2:	c6 83 ac 02 00 00 00 	movb   $0x0,0x2ac(%rbx)
   161e9:	c6 83 ad 02 00 00 00 	movb   $0x0,0x2ad(%rbx)
   161f0:	c6 83 ae 02 00 00 00 	movb   $0x0,0x2ae(%rbx)
   161f7:	c6 83 af 02 00 00 00 	movb   $0x0,0x2af(%rbx)
   161fe:	c6 83 b0 02 00 00 00 	movb   $0x0,0x2b0(%rbx)
   16205:	c6 83 b1 02 00 00 00 	movb   $0x0,0x2b1(%rbx)
   1620c:	c6 83 b2 02 00 00 00 	movb   $0x0,0x2b2(%rbx)
   16213:	c6 83 b3 02 00 00 00 	movb   $0x0,0x2b3(%rbx)
   1621a:	c6 83 b4 02 00 00 00 	movb   $0x0,0x2b4(%rbx)
   16221:	c6 83 b5 02 00 00 00 	movb   $0x0,0x2b5(%rbx)
   16228:	c6 83 b6 02 00 00 00 	movb   $0x0,0x2b6(%rbx)
   1622f:	c6 83 b7 02 00 00 00 	movb   $0x0,0x2b7(%rbx)
   16236:	c6 83 b8 02 00 00 00 	movb   $0x0,0x2b8(%rbx)
   1623d:	c6 83 b9 02 00 00 00 	movb   $0x0,0x2b9(%rbx)
   16244:	c6 83 ba 02 00 00 00 	movb   $0x0,0x2ba(%rbx)
   1624b:	c6 83 bb 02 00 00 00 	movb   $0x0,0x2bb(%rbx)
   16252:	c6 83 bc 02 00 00 00 	movb   $0x0,0x2bc(%rbx)
   16259:	c6 83 bd 02 00 00 00 	movb   $0x0,0x2bd(%rbx)
   16260:	c6 83 be 02 00 00 00 	movb   $0x0,0x2be(%rbx)
   16267:	c6 83 bf 02 00 00 00 	movb   $0x0,0x2bf(%rbx)
   1626e:	c6 83 c0 02 00 00 00 	movb   $0x0,0x2c0(%rbx)
   16275:	c7 83 40 02 00 00 00 	movl   $0x0,0x240(%rbx)
   1627c:	00 00 00 
   1627f:	c7 83 44 02 00 00 00 	movl   $0x0,0x244(%rbx)
   16286:	00 00 00 
   16289:	c7 83 48 02 00 00 00 	movl   $0x0,0x248(%rbx)
   16290:	00 00 00 
   16293:	c7                   	.byte 0xc7
   16294:	83 4c 02 00 00       	orl    $0x0,0x0(%rdx,%rax,1)
=== A=0x40140F1 F=0x140f1 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000013ef1 <star_core::secure_channel::client::SecureChannelClient::new+0x31>:
   13ef1:	48 89 43 18          	mov    %rax,0x18(%rbx)
   13ef5:	0f 11 43 08          	movups %xmm0,0x8(%rbx)
   13ef9:	b0 01                	mov    $0x1,%al
   13efb:	eb 1a                	jmp    13f17 <star_core::secure_channel::client::SecureChannelClient::new+0x57>
   13efd:	0f 10 45 d1          	movups -0x2f(%rbp),%xmm0
   13f01:	0f 10 4d e1          	movups -0x1f(%rbp),%xmm1
   13f05:	0f 29 4d c0          	movaps %xmm1,-0x40(%rbp)
   13f09:	0f 29 45 b0          	movaps %xmm0,-0x50(%rbp)
   13f0d:	0f 11 4b 11          	movups %xmm1,0x11(%rbx)
   13f11:	0f 11 43 01          	movups %xmm0,0x1(%rbx)
   13f15:	31 c0                	xor    %eax,%eax
   13f17:	88 03                	mov    %al,(%rbx)
   13f19:	48 89 d8             	mov    %rbx,%rax
   13f1c:	48 83 c4 48          	add    $0x48,%rsp
   13f20:	5b                   	pop    %rbx
   13f21:	5d                   	pop    %rbp
   13f22:	c3                   	ret
   13f23:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   13f2a:	00 00 00 
   13f2d:	0f 1f 00             	nopl   (%rax)

0000000000013f30 <star_core::serv::enclave::Enclave::public_key>:
   13f30:	55                   	push   %rbp
   13f31:	48 89 e5             	mov    %rsp,%rbp
   13f34:	53                   	push   %rbx
   13f35:	50                   	push   %rax
   13f36:	48 89 fb             	mov    %rdi,%rbx
   13f39:	48 83 c6 20          	add    $0x20,%rsi
   13f3d:	ff 15 8d 7c 07 00    	call   *0x77c8d(%rip)        # 8bbd0 <_GLOBAL_OFFSET_TABLE_+0x688>
   13f43:	48 89 d8             	mov    %rbx,%rax
   13f46:	48 83 c4 08          	add    $0x8,%rsp
   13f4a:	5b                   	pop    %rbx
   13f4b:	5d                   	pop    %rbp
   13f4c:	c3                   	ret
   13f4d:	0f 1f 00             	nopl   (%rax)

0000000000013f50 <star_core::serv::enclave::Enclave::issue_token>:
   13f50:	55                   	push   %rbp
   13f51:	48 89 e5             	mov    %rsp,%rbp
   13f54:	41 57                	push   %r15
   13f56:	41 56                	push   %r14
   13f58:	41 55                	push   %r13
   13f5a:	41 54                	push   %r12
   13f5c:	53                   	push   %rbx
   13f5d:	48 83 ec 38          	sub    $0x38,%rsp
   13f61:	48 89 fb             	mov    %rdi,%rbx
   13f64:	48 83 f9 60          	cmp    $0x60,%rcx
   13f68:	72 46                	jb     13fb0 <star_core::serv::enclave::Enclave::issue_token+0x60>
   13f6a:	4d 89 c4             	mov    %r8,%r12
   13f6d:	4c 8d 42 20          	lea    0x20(%rdx),%r8
   13f71:	48 8d 42 40          	lea    0x40(%rdx),%rax
   13f75:	48 83 c1 a0          	add    $0xffffffffffffffa0,%rcx
   13f79:	4c 8d 52 60          	lea    0x60(%rdx),%r10
   13f7d:	48 8d 7d b8          	lea    -0x48(%rbp),%rdi
   13f81:	49 89 cb             	mov    %rcx,%r11
   13f84:	b9 20 00 00 00       	mov    $0x20,%ecx
   13f89:	41 b9 20 00 00 00    	mov    $0x20,%r9d
   13f8f:	49 89 f5             	mov    %rsi,%r13
   13f92:	49 89 d6             	mov    %rdx,%r14
   13f95:	48 89 c2             	mov    %rax,%rdx
   13f98:	41 53                	push   %r11
   13f9a:	41 52                	push   %r10
   13f9c:	6a 20                	push   $0x20
   13f9e:	41 56                	push   %r14
   13fa0:	ff 15 aa 7a 07 00    	call   *0x77aaa(%rip)        # 8ba50 <_GLOBAL_OFFSET_TABLE_+0x508>
   13fa6:	48 83 c4 20          	add    $0x20,%rsp
   13faa:	80 7d b8 00          	cmpb   $0x0,-0x48(%rbp)
   13fae:	74 23                	je     13fd3 <star_core::serv::enclave::Enclave::issue_token+0x83>
   13fb0:	c6 43 08 01          	movb   $0x1,0x8(%rbx)
   13fb4:	48 b8 00 00 00 00 00 	movabs $0x8000000000000000,%rax
   13fbb:	00 00 80 
   13fbe:	48 89 03             	mov    %rax,(%rbx)
   13fc1:	48 89 d8             	mov    %rbx,%rax
   13fc4:	48 83 c4 38          	add    $0x38,%rsp
   13fc8:	5b                   	pop    %rbx
   13fc9:	41 5c                	pop    %r12
   13fcb:	41 5d                	pop    %r13
   13fcd:	41 5e                	pop    %r14
   13fcf:	41 5f                	pop    %r15
   13fd1:	5d                   	pop    %rbp
   13fd2:	c3                   	ret
   13fd3:	4c 8b 75 c0          	mov    -0x40(%rbp),%r14
   13fd7:	4c 8b 7d c8          	mov    -0x38(%rbp),%r15
   13fdb:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
   13fdf:	48 8d 7d a0          	lea    -0x60(%rbp),%rdi
   13fe3:	4c 89 ee             	mov    %r13,%rsi
   13fe6:	4c 89 fa             	mov    %r15,%rdx
   13fe9:	4d 89 e0             	mov    %r12,%r8
   13fec:	ff 15 be 78 07 00    	call   *0x778be(%rip)        # 8b8b0 <_GLOBAL_OFFSET_TABLE_+0x368>
   13ff2:	48 8b 45 b0          	mov    -0x50(%rbp),%rax
   13ff6:	48 89 43 10          	mov    %rax,0x10(%rbx)
   13ffa:	0f 10 45 a0          	movups -0x60(%rbp),%xmm0
   13ffe:	0f 11 03             	movups %xmm0,(%rbx)
   14001:	4d 85 f6             	test   %r14,%r14
   14004:	74 bb                	je     13fc1 <star_core::serv::enclave::Enclave::issue_token+0x71>
   14006:	ba 01 00 00 00       	mov    $0x1,%edx
   1400b:	4c 89 ff             	mov    %r15,%rdi
   1400e:	4c 89 f6             	mov    %r14,%rsi
   14011:	ff 15 39 7b 07 00    	call   *0x77b39(%rip)        # 8bb50 <_GLOBAL_OFFSET_TABLE_+0x608>
   14017:	eb a8                	jmp    13fc1 <star_core::serv::enclave::Enclave::issue_token+0x71>
   14019:	48 89 c3             	mov    %rax,%rbx
   1401c:	4d 85 f6             	test   %r14,%r14
   1401f:	74 11                	je     14032 <star_core::serv::enclave::Enclave::issue_token+0xe2>
   14021:	ba 01 00 00 00       	mov    $0x1,%edx
   14026:	4c 89 ff             	mov    %r15,%rdi
   14029:	4c 89 f6             	mov    %r14,%rsi
   1402c:	ff 15 1e 7b 07 00    	call   *0x77b1e(%rip)        # 8bb50 <_GLOBAL_OFFSET_TABLE_+0x608>
   14032:	48 89 df             	mov    %rbx,%rdi
   14035:	e8 16 20 ff ff       	call   6050 <_Unwind_Resume@plt>
   1403a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)

0000000000014040 <star_core::serv::enclave::Enclave::issue_token_plain>:
   14040:	55                   	push   %rbp
   14041:	48 89 e5             	mov    %rsp,%rbp
   14044:	41 57                	push   %r15
   14046:	41 56                	push   %r14
   14048:	41 55                	push   %r13
   1404a:	41 54                	push   %r12
   1404c:	53                   	push   %rbx
   1404d:	48 81 ec d8 00 00 00 	sub    $0xd8,%rsp
   14054:	4d 89 c7             	mov    %r8,%r15
   14057:	49 89 cd             	mov    %rcx,%r13
   1405a:	49 89 d6             	mov    %rdx,%r14
   1405d:	49 89 f4             	mov    %rsi,%r12
   14060:	48 89 fb             	mov    %rdi,%rbx
   14063:	48 89 d7             	mov    %rdx,%rdi
   14066:	48 89 ce             	mov    %rcx,%rsi
   14069:	ff 15 29 7d 07 00    	call   *0x77d29(%rip)        # 8bd98 <_GLOBAL_OFFSET_TABLE_+0x850>
   1406f:	49 83 fd 50          	cmp    $0x50,%r13
   14073:	0f 82 58 01 00 00    	jb     141d1 <star_core::serv::enclave::Enclave::issue_token_plain+0x191>
   14079:	41 0f 10 46 20       	movups 0x20(%r14),%xmm0
   1407e:	41 0f 10 4e 30       	movups 0x30(%r14),%xmm1
   14083:	0f 29 8d 30 ff ff ff 	movaps %xmm1,-0xd0(%rbp)
   1408a:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   14091:	41 0f 10 06          	movups (%r14),%xmm0
   14095:	41 0f 10 4e 10       	movups 0x10(%r14),%xmm1
   1409a:	0f 29 85 00 ff ff ff 	movaps %xmm0,-0x100(%rbp)
   140a1:	0f 29 8d 10 ff ff ff 	movaps %xmm1,-0xf0(%rbp)
   140a8:	49 8b 46 40          	mov    0x40(%r14),%rax
   140ac:	4d 8b 6e 48          	mov    0x48(%r14),%r13
   140b0:	4d 8b 44 24 60       	mov    0x60(%r12),%r8
   140b5:	41 0f 10 44 24 40    	movups 0x40(%r12),%xmm0
   140bb:	41 0f 10 4c 24 50    	movups 0x50(%r12),%xmm1
   140c1:	49 89 c4             	mov    %rax,%r12
   140c4:	0f 29 8d 70 ff ff ff 	movaps %xmm1,-0x90(%rbp)
   140cb:	0f 29 85 60 ff ff ff 	movaps %xmm0,-0xa0(%rbp)
   140d2:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   140d9:	4c 8d 8d 60 ff ff ff 	lea    -0xa0(%rbp),%r9
   140e0:	4c 89 ee             	mov    %r13,%rsi
   140e3:	48 89 c2             	mov    %rax,%rdx
   140e6:	4c 89 f9             	mov    %r15,%rcx
   140e9:	ff 15 81 7b 07 00    	call   *0x77b81(%rip)        # 8bc70 <_GLOBAL_OFFSET_TABLE_+0x728>
   140ef:	84 c0                	test   %al,%al
   140f1:	0f 84 da 00 00 00    	je     141d1 <star_core::serv::enclave::Enclave::issue_token_plain+0x191>
   140f7:	41 0f 10 06          	movups (%r14),%xmm0
   140fb:	41 0f 10 4e 10       	movups 0x10(%r14),%xmm1
   14100:	0f 11 4d a8          	movups %xmm1,-0x58(%rbp)
   14104:	0f 11 45 98          	movups %xmm0,-0x68(%rbp)
   14108:	0f 57 c0             	xorps  %xmm0,%xmm0
   1410b:	0f 11 45 b8          	movups %xmm0,-0x48(%rbp)
   1410f:	0f 10 05 da 9b 05 00 	movups 0x59bda(%rip),%xmm0        # 6dcf0 <anon.00704b412267c1267c12de2e23d17fea.27.llvm.1279112248716796645>
   14116:	0f 29 85 60 ff ff ff 	movaps %xmm0,-0xa0(%rbp)
   1411d:	0f 10 05 dc 9b 05 00 	movups 0x59bdc(%rip),%xmm0        # 6dd00 <anon.00704b412267c1267c12de2e23d17fea.27.llvm.1279112248716796645+0x10>
   14124:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
   1412b:	48 c7 45 80 00 00 00 	movq   $0x0,-0x80(%rbp)
   14132:	00 
   14133:	4c 89 65 88          	mov    %r12,-0x78(%rbp)
   14137:	4c 89 6d 90          	mov    %r13,-0x70(%rbp)
   1413b:	c6 45 c8 30          	movb   $0x30,-0x38(%rbp)
   1413f:	48 8d bd 40 ff ff ff 	lea    -0xc0(%rbp),%rdi
   14146:	48 8d b5 60 ff ff ff 	lea    -0xa0(%rbp),%rsi
   1414d:	e8 ce 0e 00 00       	call   15020 <digest::FixedOutput::finalize_fixed>
   14152:	48 c7 85 60 ff ff ff 	movq   $0x0,-0xa0(%rbp)
   14159:	00 00 00 00 
   1415d:	48 c7 85 68 ff ff ff 	movq   $0x1,-0x98(%rbp)
   14164:	01 00 00 00 
   14168:	48 c7 85 70 ff ff ff 	movq   $0x0,-0x90(%rbp)
   1416f:	00 00 00 00 
   14173:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   1417a:	ba 20 00 00 00       	mov    $0x20,%edx
   1417f:	b9 01 00 00 00       	mov    $0x1,%ecx
   14184:	41 b8 01 00 00 00    	mov    $0x1,%r8d
   1418a:	31 f6                	xor    %esi,%esi
   1418c:	e8 0f 23 ff ff       	call   64a0 <alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle>
   14191:	48 8b 85 68 ff ff ff 	mov    -0x98(%rbp),%rax
   14198:	48 8b 8d 70 ff ff ff 	mov    -0x90(%rbp),%rcx
   1419f:	0f 10 85 40 ff ff ff 	movups -0xc0(%rbp),%xmm0
   141a6:	0f 10 8d 50 ff ff ff 	movups -0xb0(%rbp),%xmm1
   141ad:	0f 11 4c 08 10       	movups %xmm1,0x10(%rax,%rcx,1)
   141b2:	0f 11 04 08          	movups %xmm0,(%rax,%rcx,1)
   141b6:	48 83 c1 20          	add    $0x20,%rcx
   141ba:	48 89 8d 70 ff ff ff 	mov    %rcx,-0x90(%rbp)
   141c1:	48 89 4b 10          	mov    %rcx,0x10(%rbx)
   141c5:	0f 10 85 60 ff ff ff 	movups -0xa0(%rbp),%xmm0
   141cc:	0f 11 03             	movups %xmm0,(%rbx)
   141cf:	eb 31                	jmp    14202 <star_core::serv::enclave::Enclave::issue_token_plain+0x1c2>
   141d1:	ff 15 39 7b 07 00    	call   *0x77b39(%rip)        # 8bd10 <_GLOBAL_OFFSET_TABLE_+0x7c8>
   141d7:	bf 01 00 00 00       	mov    $0x1,%edi
   141dc:	be 01 00 00 00       	mov    $0x1,%esi
   141e1:	ff 15 e1 7b 07 00    	call   *0x77be1(%rip)        # 8bdc8 <_GLOBAL_OFFSET_TABLE_+0x880>
   141e7:	48 85 c0             	test   %rax,%rax
   141ea:	74 2b                	je     14217 <star_core::serv::enclave::Enclave::issue_token_plain+0x1d7>
   141ec:	c6 00 00             	movb   $0x0,(%rax)
   141ef:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   141f6:	48 89 43 08          	mov    %rax,0x8(%rbx)
   141fa:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   14201:	00 
   14202:	48 89 d8             	mov    %rbx,%rax
   14205:	48 81 c4 d8 00 00 00 	add    $0xd8,%rsp
   1420c:	5b                   	pop    %rbx
   1420d:	41 5c                	pop    %r12
   1420f:	41 5d                	pop    %r13
   14211:	41 5e                	pop    %r14
   14213:	41 5f                	pop    %r15
   14215:	5d                   	pop    %rbp
   14216:	c3                   	ret
   14217:	bf 01 00 00 00       	mov    $0x1,%edi
   1421c:	be 01 00 00 00       	mov    $0x1,%esi
   14221:	ff 15 39 76 07 00    	call   *0x77639(%rip)        # 8b860 <_GLOBAL_OFFSET_TABLE_+0x318>
   14227:	48 89 c3             	mov    %rax,%rbx
   1422a:	48 8b b5 60 ff ff ff 	mov    -0xa0(%rbp),%rsi
   14231:	48 85 f6             	test   %rsi,%rsi
   14234:	74 12                	je     14248 <star_core::serv::enclave::Enclave::issue_token_plain+0x208>
   14236:	48 8b bd 68 ff ff ff 	mov    -0x98(%rbp),%rdi
   1423d:	ba 01 00 00 00       	mov    $0x1,%edx
   14242:	ff 15 08 79 07 00    	call   *0x77908(%rip)        # 8bb50 <_GLOBAL_OFFSET_TABLE_+0x608>
   14248:	48 89 df             	mov    %rbx,%rdi
   1424b:	e8 00 1e ff ff       	call   6050 <_Unwind_Resume@plt>

0000000000014250 <star_core::serv::enclave::Enclave::new>:
   14250:	55                   	push   %rbp
   14251:	48 89 e5             	mov    %rsp,%rbp
   14254:	41 57                	push   %r15
   14256:	41 56                	push   %r14
   14258:	53                   	push   %rbx
   14259:	48 81 ec 08 01 00 00 	sub    $0x108,%rsp
   14260:	49 89 d6             	mov    %rdx,%r14
   14263:	41 89 f7             	mov    %esi,%r15d
   14266:	48 89 fb             	mov    %rdi,%rbx
   14269:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   14270:	ba 20 00 00 00       	mov    $0x20,%edx
   14275:	48 89 ce             	mov    %rcx,%rsi
   14278:	ff 15 c2 7a 07 00    	call   *0x77ac2(%rip)        # 8bd40 <_GLOBAL_OFFSET_TABLE_+0x7f8>
   1427e:	80 bd 60 ff ff ff 00 	cmpb   $0x0,-0xa0(%rbp)
   14285:	74 2b                	je     142b2 <star_core::serv::enclave::Enclave::new+0x62>
   14287:	48 8b 85 78 ff ff ff 	mov    -0x88(%rbp),%rax
   1428e:	0f 10 85 68 ff ff ff 	movups -0x98(%rbp),%xmm0
   14295:	0f 11 85 27 ff ff ff 	movups %xmm0,-0xd9(%rbp)
   1429c:	48 89 85 37 ff ff ff 	mov    %rax,-0xc9(%rbp)
   142a3:	48 89 43 18          	mov    %rax,0x18(%rbx)
   142a7:	0f 11 43 08          	movups %xmm0,0x8(%rbx)
   142ab:	b0 01                	mov    $0x1,%al
   142ad:	e9 a4 00 00 00       	jmp    14356 <star_core::serv::enclave::Enclave::new+0x106>
   142b2:	0f 10 85 61 ff ff ff 	movups -0x9f(%rbp),%xmm0
   142b9:	0f 10 8d 71 ff ff ff 	movups -0x8f(%rbp),%xmm1
   142c0:	0f 29 45 c0          	movaps %xmm0,-0x40(%rbp)
   142c4:	0f 29 4d d0          	movaps %xmm1,-0x30(%rbp)
   142c8:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   142cf:	48 8d 75 c0          	lea    -0x40(%rbp),%rsi
   142d3:	ff 15 1f 79 07 00    	call   *0x7791f(%rip)        # 8bbf8 <_GLOBAL_OFFSET_TABLE_+0x6b0>
   142d9:	0f 28 45 c0          	movaps -0x40(%rbp),%xmm0
   142dd:	0f 28 4d d0          	movaps -0x30(%rbp),%xmm1
   142e1:	0f 29 8d 30 ff ff ff 	movaps %xmm1,-0xd0(%rbp)
   142e8:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   142ef:	0f                   	.byte 0xf
   142f0:	10                   	.byte 0x10
=== A=0x48D5810 F=0x8d5810 ===

./target/release/analysis:     file format elf64-x86-64

