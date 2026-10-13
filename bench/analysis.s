=== VALGRIND OUTPUT ===
==2371== Memcheck, a memory error detector
==2371== Copyright (C) 2002-2026, and GNU GPL'd, by Julian Seward et al.
==2371== Using Valgrind-3.27.1 and LibVEX; rerun with -h for copyright info
==2371== Command: ./target/release/analysis
==2371== 
==2371== Conditional jump or move depends on uninitialised value(s)
==2371==    at 0x40238A0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403B409: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400EBEB: analysis::main (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400D7F5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400E14B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A06: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
==2371== Conditional jump or move depends on uninitialised value(s)
==2371==    at 0x4021250: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403C8D3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400EBEB: analysis::main (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400D7F5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400E14B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A06: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
==2371== Conditional jump or move depends on uninitialised value(s)
==2371==    at 0x40238A0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403B409: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403CE89: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400F1B2: analysis::main (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400D7F5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400E14B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A06: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
==2371== Conditional jump or move depends on uninitialised value(s)
==2371==    at 0x4021250: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403C8D3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403CE89: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400F1B2: analysis::main (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400D7F5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400E14B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A06: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
==2371== Conditional jump or move depends on uninitialised value(s)
==2371==    at 0x403CFBE: <star_core::serv::enclave::Enclave>::issue_token_plain (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403CEB2: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400F1B2: analysis::main (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400D7F5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400E14B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A06: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
==2371== Conditional jump or move depends on uninitialised value(s)
==2371==    at 0x40238A0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400F4E3: analysis::main (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400D7F5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400E14B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A06: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
==2371== Conditional jump or move depends on uninitialised value(s)
==2371==    at 0x40238A0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403B409: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4010F64: analysis::main (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400D7F5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400E14B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A06: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
==2371== Conditional jump or move depends on uninitialised value(s)
==2371==    at 0x4021250: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2371==    by 0x403C8D3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4010F64: analysis::main (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400D7F5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2371==    by 0x400E14B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A06: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
done
==2371== 
==2371== HEAP SUMMARY:
==2371==     in use at exit: 544 bytes in 1 blocks
==2371==   total heap usage: 93 allocs, 92 frees, 7,136 bytes allocated
==2371== 
==2371== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==2371==    at 0x48CB7D0: malloc (in /nix/store/mf7rzpknxwg0q1wf0nm8gfzikapg00h1-valgrind-3.27.1/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==2371==    by 0x4067D27: <alloc::collections::btree::map::entry::VacantEntry<usize, std::sys::pal::unix::stack_overflow::thread_info::ThreadInfo>>::insert_entry (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4058DC3: std::sys::pal::unix::stack_overflow::thread_info::set_current_info (in /workspace/star/core/target/release/analysis)
==2371==    by 0x40599EF: std::sys::pal::unix::stack_overflow::imp::init (in /workspace/star/core/target/release/analysis)
==2371==    by 0x40681EF: std::sys::pal::unix::init (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4054A00: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2371==    by 0x4012253: main (in /workspace/star/core/target/release/analysis)
==2371== 
==2371== LEAK SUMMARY:
==2371==    definitely lost: 0 bytes in 0 blocks
==2371==    indirectly lost: 0 bytes in 0 blocks
==2371==      possibly lost: 0 bytes in 0 blocks
==2371==    still reachable: 544 bytes in 1 blocks
==2371==         suppressed: 0 bytes in 0 blocks
==2371== 
==2371== Use --track-origins=yes to see where uninitialised values come from
==2371== For lists of detected and suppressed errors, rerun with: -s
==2371== ERROR SUMMARY: 8 errors from 8 contexts (suppressed: 0 from 0)

=== A=0x40238A0 F=0x238a0 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

00000000000236a0 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x140>:
   236a0:	7d aa                	jge    2364c <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0xec>
   236a2:	00 40 0f             	add    %al,0xf(%rax)
   236a5:	94                   	xchg   %eax,%esp
   236a6:	c7                   	(bad)
   236a7:	ff 15 eb e5 05 00    	call   *0x5e5eb(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   236ad:	88 45 c8             	mov    %al,-0x38(%rbp)
   236b0:	31 ff                	xor    %edi,%edi
   236b2:	80 7d ab 00          	cmpb   $0x0,-0x55(%rbp)
   236b6:	40 0f 94 c7          	sete   %dil
   236ba:	ff 15 d8 e5 05 00    	call   *0x5e5d8(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   236c0:	88 45 c9             	mov    %al,-0x37(%rbp)
   236c3:	31 ff                	xor    %edi,%edi
   236c5:	80 7d ac 00          	cmpb   $0x0,-0x54(%rbp)
   236c9:	40 0f 94 c7          	sete   %dil
   236cd:	ff 15 c5 e5 05 00    	call   *0x5e5c5(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   236d3:	88 45 ca             	mov    %al,-0x36(%rbp)
   236d6:	31 ff                	xor    %edi,%edi
   236d8:	80 7d ad 00          	cmpb   $0x0,-0x53(%rbp)
   236dc:	40 0f 94 c7          	sete   %dil
   236e0:	ff 15 b2 e5 05 00    	call   *0x5e5b2(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   236e6:	88 45 cb             	mov    %al,-0x35(%rbp)
   236e9:	31 ff                	xor    %edi,%edi
   236eb:	80 7d ae 00          	cmpb   $0x0,-0x52(%rbp)
   236ef:	40 0f 94 c7          	sete   %dil
   236f3:	ff 15 9f e5 05 00    	call   *0x5e59f(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   236f9:	41 89 c6             	mov    %eax,%r14d
   236fc:	31 ff                	xor    %edi,%edi
   236fe:	80 7d af 00          	cmpb   $0x0,-0x51(%rbp)
   23702:	40 0f 94 c7          	sete   %dil
   23706:	ff 15 8c e5 05 00    	call   *0x5e58c(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2370c:	88 45 cc             	mov    %al,-0x34(%rbp)
   2370f:	31 ff                	xor    %edi,%edi
   23711:	80 7d b0 00          	cmpb   $0x0,-0x50(%rbp)
   23715:	40 0f 94 c7          	sete   %dil
   23719:	ff 15 79 e5 05 00    	call   *0x5e579(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2371f:	88 45 cd             	mov    %al,-0x33(%rbp)
   23722:	31 ff                	xor    %edi,%edi
   23724:	80 7d b1 00          	cmpb   $0x0,-0x4f(%rbp)
   23728:	40 0f 94 c7          	sete   %dil
   2372c:	ff 15 66 e5 05 00    	call   *0x5e566(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   23732:	88 45 ce             	mov    %al,-0x32(%rbp)
   23735:	31 ff                	xor    %edi,%edi
   23737:	80 7d b2 00          	cmpb   $0x0,-0x4e(%rbp)
   2373b:	40 0f 94 c7          	sete   %dil
   2373f:	ff 15 53 e5 05 00    	call   *0x5e553(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   23745:	88 45 cf             	mov    %al,-0x31(%rbp)
   23748:	31 ff                	xor    %edi,%edi
   2374a:	80 7d b3 00          	cmpb   $0x0,-0x4d(%rbp)
   2374e:	40 0f 94 c7          	sete   %dil
   23752:	ff 15 40 e5 05 00    	call   *0x5e540(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   23758:	88 45 d0             	mov    %al,-0x30(%rbp)
   2375b:	31 ff                	xor    %edi,%edi
   2375d:	80 7d b4 00          	cmpb   $0x0,-0x4c(%rbp)
   23761:	40 0f 94 c7          	sete   %dil
   23765:	ff 15 2d e5 05 00    	call   *0x5e52d(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2376b:	41 89 c4             	mov    %eax,%r12d
   2376e:	31 ff                	xor    %edi,%edi
   23770:	80 7d b5 00          	cmpb   $0x0,-0x4b(%rbp)
   23774:	40 0f 94 c7          	sete   %dil
   23778:	ff 15 1a e5 05 00    	call   *0x5e51a(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2377e:	88 45 d1             	mov    %al,-0x2f(%rbp)
   23781:	31 ff                	xor    %edi,%edi
   23783:	80 7d b6 00          	cmpb   $0x0,-0x4a(%rbp)
   23787:	40 0f 94 c7          	sete   %dil
   2378b:	ff 15 07 e5 05 00    	call   *0x5e507(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   23791:	88 45 d2             	mov    %al,-0x2e(%rbp)
   23794:	31 ff                	xor    %edi,%edi
   23796:	80 7d b7 00          	cmpb   $0x0,-0x49(%rbp)
   2379a:	40 0f 94 c7          	sete   %dil
   2379e:	ff 15 f4 e4 05 00    	call   *0x5e4f4(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   237a4:	88 45 d3             	mov    %al,-0x2d(%rbp)
   237a7:	31 ff                	xor    %edi,%edi
   237a9:	80 7d b8 00          	cmpb   $0x0,-0x48(%rbp)
   237ad:	40 0f 94 c7          	sete   %dil
   237b1:	ff 15 e1 e4 05 00    	call   *0x5e4e1(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   237b7:	88 45 d4             	mov    %al,-0x2c(%rbp)
   237ba:	31 ff                	xor    %edi,%edi
   237bc:	80 7d b9 00          	cmpb   $0x0,-0x47(%rbp)
   237c0:	40 0f 94 c7          	sete   %dil
   237c4:	ff 15 ce e4 05 00    	call   *0x5e4ce(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   237ca:	88 45 d5             	mov    %al,-0x2b(%rbp)
   237cd:	31 ff                	xor    %edi,%edi
   237cf:	80 7d ba 00          	cmpb   $0x0,-0x46(%rbp)
   237d3:	40 0f 94 c7          	sete   %dil
   237d7:	ff 15 bb e4 05 00    	call   *0x5e4bb(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   237dd:	88 45 d6             	mov    %al,-0x2a(%rbp)
   237e0:	31 ff                	xor    %edi,%edi
   237e2:	80 7d bb 00          	cmpb   $0x0,-0x45(%rbp)
   237e6:	40 0f 94 c7          	sete   %dil
   237ea:	ff 15 a8 e4 05 00    	call   *0x5e4a8(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   237f0:	41 89 c5             	mov    %eax,%r13d
   237f3:	31 ff                	xor    %edi,%edi
   237f5:	80 7d bc 00          	cmpb   $0x0,-0x44(%rbp)
   237f9:	40 0f 94 c7          	sete   %dil
   237fd:	ff 15 95 e4 05 00    	call   *0x5e495(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   23803:	88 45 d7             	mov    %al,-0x29(%rbp)
   23806:	31 ff                	xor    %edi,%edi
   23808:	80 7d bd 00          	cmpb   $0x0,-0x43(%rbp)
   2380c:	40 0f 94 c7          	sete   %dil
   23810:	ff 15 82 e4 05 00    	call   *0x5e482(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   23816:	0f b6 4d be          	movzbl -0x42(%rbp),%ecx
   2381a:	22 4d bf             	and    -0x41(%rbp),%cl
   2381d:	0f b6 55 c0          	movzbl -0x40(%rbp),%edx
   23821:	22 55 c1             	and    -0x3f(%rbp),%dl
   23824:	20 ca                	and    %cl,%dl
   23826:	0f b6 4d c2          	movzbl -0x3e(%rbp),%ecx
   2382a:	22 4d c3             	and    -0x3d(%rbp),%cl
   2382d:	22 4d c4             	and    -0x3c(%rbp),%cl
   23830:	20 d1                	and    %dl,%cl
   23832:	44 22 7d c5          	and    -0x3b(%rbp),%r15b
   23836:	44 22 7d c6          	and    -0x3a(%rbp),%r15b
   2383a:	44 22 7d c7          	and    -0x39(%rbp),%r15b
   2383e:	41 20 cf             	and    %cl,%r15b
   23841:	22 5d c8             	and    -0x38(%rbp),%bl
   23844:	22 5d c9             	and    -0x37(%rbp),%bl
   23847:	22 5d ca             	and    -0x36(%rbp),%bl
   2384a:	22 5d cb             	and    -0x35(%rbp),%bl
   2384d:	44 20 fb             	and    %r15b,%bl
   23850:	44 22 75 cc          	and    -0x34(%rbp),%r14b
   23854:	44 22 75 cd          	and    -0x33(%rbp),%r14b
   23858:	44 22 75 ce          	and    -0x32(%rbp),%r14b
   2385c:	44 22 75 cf          	and    -0x31(%rbp),%r14b
   23860:	44 22 75 d0          	and    -0x30(%rbp),%r14b
   23864:	41 20 de             	and    %bl,%r14b
   23867:	44 22 65 d1          	and    -0x2f(%rbp),%r12b
   2386b:	44 22 65 d2          	and    -0x2e(%rbp),%r12b
   2386f:	44 22 65 d3          	and    -0x2d(%rbp),%r12b
   23873:	44 22 65 d4          	and    -0x2c(%rbp),%r12b
   23877:	44 22 65 d5          	and    -0x2b(%rbp),%r12b
   2387b:	44 22 65 d6          	and    -0x2a(%rbp),%r12b
   2387f:	45 20 f4             	and    %r14b,%r12b
   23882:	44 22 6d d7          	and    -0x29(%rbp),%r13b
   23886:	41 20 c5             	and    %al,%r13b
   23889:	45 20 e5             	and    %r12b,%r13b
   2388c:	41 80 e5 01          	and    $0x1,%r13b
   23890:	41 0f b6 fd          	movzbl %r13b,%edi
   23894:	ff 15 fe e3 05 00    	call   *0x5e3fe(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2389a:	84 c0                	test   %al,%al
   2389c:	48 8b 5d 90          	mov    -0x70(%rbp),%rbx
   238a0:	74 0d                	je     238af <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x34f>
   238a2:	c6 43 08 03          	movb   $0x3,0x8(%rbx)
   238a6:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   238ad:	eb 32                	jmp    238e1 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x381>
   238af:	bf 20 00 00 00       	mov    $0x20,%edi
   238b4:	ff 15 be e3 05 00    	call   *0x5e3be(%rip)        # 81c78 <malloc@GLIBC_2.2.5>
   238ba:	48 85 c0             	test   %rax,%rax
   238bd:	74 3e                	je     238fd <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x39d>
   238bf:	0f 10 45 9e          	movups -0x62(%rbp),%xmm0
   238c3:	0f 10 4d ae          	movups -0x52(%rbp),%xmm1
   238c7:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   238cb:	0f 11 00             	movups %xmm0,(%rax)
   238ce:	48 c7 03 20 00 00 00 	movq   $0x20,(%rbx)
   238d5:	48 89 43 08          	mov    %rax,0x8(%rbx)
   238d9:	48 c7 43 10 20 00 00 	movq   $0x20,0x10(%rbx)
   238e0:	00 
   238e1:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   238e5:	ff 15 3d e3 05 00    	call   *0x5e33d(%rip)        # 81c28 <_GLOBAL_OFFSET_TABLE_+0x538>
   238eb:	48 89 d8             	mov    %rbx,%rax
   238ee:	48 83 c4 68          	add    $0x68,%rsp
   238f2:	5b                   	pop    %rbx
   238f3:	41 5c                	pop    %r12
   238f5:	41 5d                	pop    %r13
   238f7:	41 5e                	pop    %r14
   238f9:	41 5f                	pop    %r15
   238fb:	5d                   	pop    %rbp
   238fc:	c3                   	ret
   238fd:	bf 01 00 00 00       	mov    $0x1,%edi
   23902:	be 20 00 00 00       	mov    $0x20,%esi
   23907:	ff 15 73 e5 05 00    	call   *0x5e573(%rip)        # 81e80 <_GLOBAL_OFFSET_TABLE_+0x790>
   2390d:	0f 0b                	ud2
   2390f:	eb 00                	jmp    23911 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x3b1>
   23911:	48 89 c3             	mov    %rax,%rbx
   23914:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   23918:	ff 15 0a e3 05 00    	call   *0x5e30a(%rip)        # 81c28 <_GLOBAL_OFFSET_TABLE_+0x538>
   2391e:	48 89 df             	mov    %rbx,%rdi
   23921:	e8 1a 27 fe ff       	call   6040 <_Unwind_Resume@plt>
   23926:	ff 15 a4 e6 05 00    	call   *0x5e6a4(%rip)        # 81fd0 <_GLOBAL_OFFSET_TABLE_+0x8e0>
   2392c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000023930 <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed>:
   23930:	55                   	push   %rbp
   23931:	48 89 e5             	mov    %rsp,%rbp
   23934:	41 57                	push   %r15
   23936:	41 56                	push   %r14
   23938:	41 55                	push   %r13
   2393a:	41 54                	push   %r12
   2393c:	53                   	push   %rbx
   2393d:	48 81 ec d8 00 00 00 	sub    $0xd8,%rsp
   23944:	48 89 fb             	mov    %rdi,%rbx
   23947:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   2394e:	ba 90 00 00 00       	mov    $0x90,%edx
   23953:	ff 15 3f e2 05 00    	call   *0x5e23f(%rip)        # 81b98 <memcpy@GLIBC_2.14>
   23959:	4c 8d b5 40 ff ff ff 	lea    -0xc0(%rbp),%r14
   23960:	44 0f b6 a5 3f ff ff 	movzbl -0xc1(%rbp),%r12d
   23967:	ff 
   23968:	4c 8b ad 60 ff ff ff 	mov    -0xa0(%rbp),%r13
   2396f:	4d 89 e7             	mov    %r12,%r15
   23972:	49 83 f7 3f          	xor    $0x3f,%r15
   23976:	42 c6 84 25 00 ff ff 	movb   $0x80,-0x100(%rbp,%r12,1)
   2397d:	ff 80 
   2397f:	49 8d 3c 2c          	lea    (%r12,%rbp,1),%rdi
   23983:	48 81 c7 01 ff ff ff 	add    $0xffffffffffffff01,%rdi
   2398a:	31 f6                	xor    %esi,%esi
   2398c:	4c 89 fa             	mov    %r15,%rdx
   2398f:	ff 15 53 e0 05 00    	call   *0x5e053(%rip)        # 819e8 <memset@GLIBC_2.2.5>
   23995:	49 83 ff 07          	cmp    $0x7,%r15
   23999:	77 63                	ja     239fe <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed+0xce>
   2399b:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   239a2:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   239a9:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   239b0:	66 0f 6f 9d 30 ff ff 	movdqa -0xd0(%rbp),%xmm3
   239b7:	ff 
   239b8:	66 0f 7f 5d c0       	movdqa %xmm3,-0x40(%rbp)
   239bd:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   239c1:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   239c5:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   239c9:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   239cd:	ba 01 00 00 00       	mov    $0x1,%edx
   239d2:	4c 89 f7             	mov    %r14,%rdi
   239d5:	ff 15 d5 e2 05 00    	call   *0x5e2d5(%rip)        # 81cb0 <_GLOBAL_OFFSET_TABLE_+0x5c0>
   239db:	0f 57 c0             	xorps  %xmm0,%xmm0
   239de:	0f 29 85 00 ff ff ff 	movaps %xmm0,-0x100(%rbp)
   239e5:	0f 29 85 10 ff ff ff 	movaps %xmm0,-0xf0(%rbp)
   239ec:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   239f3:	48 c7 85 30 ff ff ff 	movq   $0x0,-0xd0(%rbp)
   239fa:	00 00 00 00 
   239fe:	49 c1 e5 09          	shl    $0x9,%r13
   23a02:	41 c1 e4 03          	shl    $0x3,%r12d
   23a06:	4d 09 ec             	or     %r13,%r12
   23a09:	49 0f cc             	bswap  %r12
   23a0c:	4c 89 a5 38 ff ff ff 	mov    %r12,-0xc8(%rbp)
   23a13:	48 8b 85 30 ff ff ff 	mov    -0xd0(%rbp),%rax
   23a1a:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   23a1e:	48 8b 85 38 ff ff ff 	mov    -0xc8(%rbp),%rax
   23a25:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   23a29:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   23a30:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   23a37:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   23a3e:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   23a42:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   23a46:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   23a4a:	4c 8d 3d 7f bb 00 00 	lea    0xbb7f(%rip),%r15        # 2f5d0 <sha2::sha256::compress256>
   23a51:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   23a55:	ba 01 00 00 00       	mov    $0x1,%edx
   23a5a:	4c 89 f7             	mov    %r14,%rdi
   23a5d:	41 ff d7             	call   *%r15
   23a60:	48 8d bd 68 ff ff ff 	lea    -0x98(%rbp),%rdi
   23a67:	66 0f 6f 85 40 ff ff 	movdqa -0xc0(%rbp),%xmm0
   23a6e:	ff 
   23a6f:	66 0f 6f 8d 50 ff ff 	movdqa -0xb0(%rbp),%xmm1
   23a76:	ff 
   23a77:	66 0f ef db          	pxor   %xmm3,%xmm3
   23a7b:	66 0f 6f d0          	movdqa %xmm0,%xmm2
   23a7f:	66 0f 68 d3          	punpckhbw %xmm3,%xmm2
   23a83:	f2 0f 70 d2 1b       	pshuflw $0x1b,%xmm2,%xmm2
   23a88:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   23a8d:	66 0f 60 c3          	punpcklbw %xmm3,%xmm0
   23a91:	f2 0f 70 c0 1b       	pshuflw $0x1b,%xmm0,%xmm0
   23a96:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   23a9b:	66 0f 67 c2          	packuswb %xmm2,%xmm0
   23a9f:	66                   	data16

=== A=0x4021250 F=0x21250 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000021050 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x520>:
   21050:	4c 89 f6             	mov    %r14,%rsi
   21053:	e8 f8 7a 00 00       	call   28b50 <<poly1305::backend::autodetect::State as universal_hash::UniversalHash>::finalize>
   21058:	0f b6 44 24 50       	movzbl 0x50(%rsp),%eax
   2105d:	31 ff                	xor    %edi,%edi
   2105f:	41 3a 04 1f          	cmp    (%r15,%rbx,1),%al
   21063:	40 0f 94 c7          	sete   %dil
   21067:	ff 15 2b 0c 06 00    	call   *0x60c2b(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2106d:	88 44 24 40          	mov    %al,0x40(%rsp)
   21071:	0f b6 44 24 51       	movzbl 0x51(%rsp),%eax
   21076:	31 ff                	xor    %edi,%edi
   21078:	41 3a 44 1f 01       	cmp    0x1(%r15,%rbx,1),%al
   2107d:	40 0f 94 c7          	sete   %dil
   21081:	ff 15 11 0c 06 00    	call   *0x60c11(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   21087:	88 44 24 38          	mov    %al,0x38(%rsp)
   2108b:	0f b6 44 24 52       	movzbl 0x52(%rsp),%eax
   21090:	31 ff                	xor    %edi,%edi
   21092:	41 3a 44 1f 02       	cmp    0x2(%r15,%rbx,1),%al
   21097:	40 0f 94 c7          	sete   %dil
   2109b:	ff 15 f7 0b 06 00    	call   *0x60bf7(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   210a1:	88 44 24 48          	mov    %al,0x48(%rsp)
   210a5:	0f b6 44 24 53       	movzbl 0x53(%rsp),%eax
   210aa:	31 ff                	xor    %edi,%edi
   210ac:	41 3a 44 1f 03       	cmp    0x3(%r15,%rbx,1),%al
   210b1:	40 0f 94 c7          	sete   %dil
   210b5:	ff 15 dd 0b 06 00    	call   *0x60bdd(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   210bb:	88 44 24 27          	mov    %al,0x27(%rsp)
   210bf:	0f b6 44 24 54       	movzbl 0x54(%rsp),%eax
   210c4:	31 ff                	xor    %edi,%edi
   210c6:	41 3a 44 1f 04       	cmp    0x4(%r15,%rbx,1),%al
   210cb:	40 0f 94 c7          	sete   %dil
   210cf:	ff 15 c3 0b 06 00    	call   *0x60bc3(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   210d5:	41 89 c5             	mov    %eax,%r13d
   210d8:	0f b6 44 24 55       	movzbl 0x55(%rsp),%eax
   210dd:	31 ff                	xor    %edi,%edi
   210df:	41 3a 44 1f 05       	cmp    0x5(%r15,%rbx,1),%al
   210e4:	40 0f 94 c7          	sete   %dil
   210e8:	ff 15 aa 0b 06 00    	call   *0x60baa(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   210ee:	88 44 24 26          	mov    %al,0x26(%rsp)
   210f2:	0f b6 44 24 56       	movzbl 0x56(%rsp),%eax
   210f7:	31 ff                	xor    %edi,%edi
   210f9:	41 3a 44 1f 06       	cmp    0x6(%r15,%rbx,1),%al
   210fe:	40 0f 94 c7          	sete   %dil
   21102:	ff 15 90 0b 06 00    	call   *0x60b90(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   21108:	88 44 24 25          	mov    %al,0x25(%rsp)
   2110c:	0f b6 44 24 57       	movzbl 0x57(%rsp),%eax
   21111:	31 ff                	xor    %edi,%edi
   21113:	41 3a 44 1f 07       	cmp    0x7(%r15,%rbx,1),%al
   21118:	40 0f 94 c7          	sete   %dil
   2111c:	ff 15 76 0b 06 00    	call   *0x60b76(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   21122:	41 89 c6             	mov    %eax,%r14d
   21125:	0f b6 44 24 58       	movzbl 0x58(%rsp),%eax
   2112a:	31 ff                	xor    %edi,%edi
   2112c:	41 3a 44 1f 08       	cmp    0x8(%r15,%rbx,1),%al
   21131:	40 0f 94 c7          	sete   %dil
   21135:	ff 15 5d 0b 06 00    	call   *0x60b5d(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2113b:	88 44 24 24          	mov    %al,0x24(%rsp)
   2113f:	0f b6 44 24 59       	movzbl 0x59(%rsp),%eax
   21144:	31 ff                	xor    %edi,%edi
   21146:	41 3a 44 1f 09       	cmp    0x9(%r15,%rbx,1),%al
   2114b:	40 0f 94 c7          	sete   %dil
   2114f:	ff 15 43 0b 06 00    	call   *0x60b43(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   21155:	88 44 24 23          	mov    %al,0x23(%rsp)
   21159:	0f b6 44 24 5a       	movzbl 0x5a(%rsp),%eax
   2115e:	31 ff                	xor    %edi,%edi
   21160:	41 3a 44 1f 0a       	cmp    0xa(%r15,%rbx,1),%al
   21165:	40 0f 94 c7          	sete   %dil
   21169:	ff 15 29 0b 06 00    	call   *0x60b29(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2116f:	88 44 24 22          	mov    %al,0x22(%rsp)
   21173:	0f b6 44 24 5b       	movzbl 0x5b(%rsp),%eax
   21178:	31 ff                	xor    %edi,%edi
   2117a:	41 3a 44 1f 0b       	cmp    0xb(%r15,%rbx,1),%al
   2117f:	40 0f 94 c7          	sete   %dil
   21183:	ff 15 0f 0b 06 00    	call   *0x60b0f(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   21189:	41 89 c4             	mov    %eax,%r12d
   2118c:	0f b6 44 24 5c       	movzbl 0x5c(%rsp),%eax
   21191:	31 ff                	xor    %edi,%edi
   21193:	41 3a 44 1f 0c       	cmp    0xc(%r15,%rbx,1),%al
   21198:	40 0f 94 c7          	sete   %dil
   2119c:	ff 15 f6 0a 06 00    	call   *0x60af6(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   211a2:	88 44 24 21          	mov    %al,0x21(%rsp)
   211a6:	0f b6 44 24 5d       	movzbl 0x5d(%rsp),%eax
   211ab:	31 ff                	xor    %edi,%edi
   211ad:	41 3a 44 1f 0d       	cmp    0xd(%r15,%rbx,1),%al
   211b2:	40 0f 94 c7          	sete   %dil
   211b6:	ff 15 dc 0a 06 00    	call   *0x60adc(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   211bc:	88 44 24 20          	mov    %al,0x20(%rsp)
   211c0:	0f b6 44 24 5e       	movzbl 0x5e(%rsp),%eax
   211c5:	31 ff                	xor    %edi,%edi
   211c7:	41 3a 44 1f 0e       	cmp    0xe(%r15,%rbx,1),%al
   211cc:	40 0f 94 c7          	sete   %dil
   211d0:	ff 15 c2 0a 06 00    	call   *0x60ac2(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   211d6:	88 44 24 1f          	mov    %al,0x1f(%rsp)
   211da:	0f b6 44 24 5f       	movzbl 0x5f(%rsp),%eax
   211df:	31 ff                	xor    %edi,%edi
   211e1:	41 3a 44 1f 0f       	cmp    0xf(%r15,%rbx,1),%al
   211e6:	40 0f 94 c7          	sete   %dil
   211ea:	ff 15 a8 0a 06 00    	call   *0x60aa8(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   211f0:	0f b6 4c 24 40       	movzbl 0x40(%rsp),%ecx
   211f5:	22 4c 24 38          	and    0x38(%rsp),%cl
   211f9:	0f b6 54 24 48       	movzbl 0x48(%rsp),%edx
   211fe:	22 54 24 27          	and    0x27(%rsp),%dl
   21202:	20 ca                	and    %cl,%dl
   21204:	44 22 6c 24 26       	and    0x26(%rsp),%r13b
   21209:	44 22 6c 24 25       	and    0x25(%rsp),%r13b
   2120e:	41 20 d5             	and    %dl,%r13b
   21211:	44 22 74 24 24       	and    0x24(%rsp),%r14b
   21216:	44 22 74 24 23       	and    0x23(%rsp),%r14b
   2121b:	44 22 74 24 22       	and    0x22(%rsp),%r14b
   21220:	45 20 ee             	and    %r13b,%r14b
   21223:	44 22 64 24 21       	and    0x21(%rsp),%r12b
   21228:	44 22 64 24 20       	and    0x20(%rsp),%r12b
   2122d:	44 22 64 24 1f       	and    0x1f(%rsp),%r12b
   21232:	41 20 c4             	and    %al,%r12b
   21235:	45 20 f4             	and    %r14b,%r12b
   21238:	41 80 e4 01          	and    $0x1,%r12b
   2123c:	41 0f b6 fc          	movzbl %r12b,%edi
   21240:	ff 15 52 0a 06 00    	call   *0x60a52(%rip)        # 81c98 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   21246:	84 c0                	test   %al,%al
   21248:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   2124f:	00 
   21250:	74 42                	je     21294 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x764>
   21252:	4c 8d b4 24 20 05 00 	lea    0x520(%rsp),%r14
   21259:	00 
   2125a:	4c 89 f7             	mov    %r14,%rdi
   2125d:	4c 89 fe             	mov    %r15,%rsi
   21260:	48 8b 54 24 28       	mov    0x28(%rsp),%rdx
   21265:	e8 06 2d 00 00       	call   23f70 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as cipher::stream::StreamCipher>::apply_keystream>
   2126a:	4c 89 f7             	mov    %r14,%rdi
   2126d:	e8 fe e5 ff ff       	call   1f870 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   21272:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   21279:	00 
   2127a:	48 8b 45 18          	mov    0x18(%rbp),%rax
   2127e:	48 8b 4c 24 28       	mov    0x28(%rsp),%rcx
   21283:	48 39 c1             	cmp    %rax,%rcx
   21286:	48 0f 43 c8          	cmovae %rax,%rcx
   2128a:	4d 89 7d 08          	mov    %r15,0x8(%r13)
   2128e:	49 89 4d 10          	mov    %rcx,0x10(%r13)
   21292:	eb 22                	jmp    212b6 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x786>
   21294:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   2129b:	00 
   2129c:	e8 cf e5 ff ff       	call   1f870 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   212a1:	4c 89 ff             	mov    %r15,%rdi
   212a4:	ff 15 9e 04 06 00    	call   *0x6049e(%rip)        # 81748 <free@GLIBC_2.2.5>
   212aa:	41 c6 45 08 00       	movb   $0x0,0x8(%r13)
   212af:	48 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%rax
   212b6:	49 89 45 00          	mov    %rax,0x0(%r13)
   212ba:	4c 89 e8             	mov    %r13,%rax
   212bd:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   212c1:	5b                   	pop    %rbx
   212c2:	41 5c                	pop    %r12
   212c4:	41 5d                	pop    %r13
   212c6:	41 5e                	pop    %r14
   212c8:	41 5f                	pop    %r15
   212ca:	5d                   	pop    %rbp
   212cb:	c3                   	ret
   212cc:	48 8d 15 35 e6 04 00 	lea    0x4e635(%rip),%rdx        # 6f908 <core::unicode::unicode_data::cf::SHORT_OFFSET_RUNS+0x70>
   212d3:	4c 8d 0d 0e e7 05 00 	lea    0x5e70e(%rip),%r9        # 7f9e8 <anon.d49aa5e532bb8905898215d271d902c3.14.llvm.14046659981651887190+0x70>
   212da:	48 8d b4 24 e0 02 00 	lea    0x2e0(%rsp),%rsi
   212e1:	00 
   212e2:	31 ff                	xor    %edi,%edi
   212e4:	31 c9                	xor    %ecx,%ecx
   212e6:	ff 15 bc 09 06 00    	call   *0x609bc(%rip)        # 81ca8 <_GLOBAL_OFFSET_TABLE_+0x5b8>
   212ec:	ff 15 e6 04 06 00    	call   *0x604e6(%rip)        # 817d8 <_GLOBAL_OFFSET_TABLE_+0xe8>
   212f2:	e9 00 f9 ff ff       	jmp    20bf7 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0xc7>
   212f7:	bf 01 00 00 00       	mov    $0x1,%edi
   212fc:	48 89 de             	mov    %rbx,%rsi
   212ff:	ff 15 7b 0b 06 00    	call   *0x60b7b(%rip)        # 81e80 <_GLOBAL_OFFSET_TABLE_+0x790>
   21305:	49 89 c6             	mov    %rax,%r14
   21308:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   2130d:	ff 15 35 04 06 00    	call   *0x60435(%rip)        # 81748 <free@GLIBC_2.2.5>
   21313:	4c 89 f7             	mov    %r14,%rdi
   21316:	e8 25 4d fe ff       	call   6040 <_Unwind_Resume@plt>
   2131b:	eb 06                	jmp    21323 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   2131d:	eb 04                	jmp    21323 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   2131f:	eb 02                	jmp    21323 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   21321:	eb 00                	jmp    21323 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   21323:	49 89 c6             	mov    %rax,%r14
   21326:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   2132d:	00 
   2132e:	e8 3d e5 ff ff       	call   1f870 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   21333:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   21338:	ff 15 0a 04 06 00    	call   *0x6040a(%rip)        # 81748 <free@GLIBC_2.2.5>
   2133e:	4c 89 f7             	mov    %r14,%rdi
   21341:	e8 fa 4c fe ff       	call   6040 <_Unwind_Resume@plt>
   21346:	ff 15 84 0c 06 00    	call   *0x60c84(%rip)        # 81fd0 <_GLOBAL_OFFSET_TABLE_+0x8e0>
   2134c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000021350 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal>:
   21350:	55                   	push   %rbp
   21351:	48 89 e5             	mov    %rsp,%rbp
   21354:	41 57                	push   %r15
   21356:	41 56                	push   %r14
   21358:	41 55                	push   %r13
   2135a:	41 54                	push   %r12
   2135c:	53                   	push   %rbx
   2135d:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   21361:	48 81 ec c0 05 00 00 	sub    $0x5c0,%rsp
   21368:	4c 89 44 24 08       	mov    %r8,0x8(%rsp)
   2136d:	48 89 8c 24 c0 02 00 	mov    %rcx,0x2c0(%rsp)
   21374:	00 
   21375:	48 83 f9 0c          	cmp    $0xc,%rcx
   21379:	0f 85 74 05 00 00    	jne    218f3 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5a3>
   2137f:	48 89 fb             	mov    %rdi,%rbx
   21382:	48 8b 7d 18          	mov    0x18(%rbp),%rdi
   21386:	48 83 c7 10          	add    $0x10,%rdi
   2138a:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
   2138f:	0f 88 89 05 00 00    	js     2191e <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5ce>
   21395:	4d 89 cf             	mov    %r9,%r15
   21398:	49 89 d6             	mov    %rdx,%r14
   2139b:	49 89 f4             	mov    %rsi,%r12
   2139e:	ff 15 d4 08 06 00    	call   *0x608d4(%rip)        # 81c78 <malloc@GLIBC_2.2.5>
   213a4:	48 85 c0             	test   %rax,%rax
   213a7:	0f 84 7e 05 00 00    	je     2192b <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5db>
   213ad:	49 89 c5             	mov    %rax,%r13
   213b0:	48 89 5c 24 28       	mov    %rbx,0x28(%rsp)
   213b5:	48 8b 5d 18          	mov    0x18(%rbp),%rbx
   213b9:	48 85 db             	test   %rbx,%rbx
   213bc:	74 10                	je     213ce <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x7e>
   213be:	48 8b 75 10          	mov    0x10(%rbp),%rsi
   213c2:	4c 89 ef             	mov    %r13,%rdi
   213c5:	48 89 da             	mov    %rbx,%rdx
   213c8:	ff 15 ca 07 06 00    	call   *0x607ca(%rip)        # 81b98 <memcpy@GLIBC_2.14>
   213ce:	0f 10 05 4b b7 04 00 	movups 0x4b74b(%rip),%xmm0        # 6cb20 <anon.10746075dd51319079efedc96a688e81.1.llvm.4247047481296140766+0x95>
   213d5:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   213da:	41 0f 10 04 24       	movups (%r12),%xmm0
   213df:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   213e5:	0f 29 44 24 50       	movaps %xmm0,0x50(%rsp)
   213ea:	0f 29 4c 24 60       	movaps %xmm1,0x60(%rsp)
   213ef:	49 8b 06             	mov    (%r14),%rax
   213f2:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
   213f7:	41 8b 46 08          	mov    0x8(%r14),%eax
   213fb:	89 44 24 38          	mov    %eax,0x38(%rsp)
   213ff:	48 8d 05 0a 0c 06 00 	lea    0x60c0a(%rip),%rax        # 82010 <chacha20::avx2_cpuid::STORAGE>
   21406:	0f b6 00             	movzbl (%rax),%eax
   21409:	3c ff                	cmp    $0xff,%al
   2140b:	4c 89 2c 24          	mov    %r13,(%rsp)
   2140f:	49 89 de             	mov    %rbx,%r14
   21412:	0f 84 fb 04 00 00    	je     21913 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5c3>
   21418:	0f 57 c0             	xorps  %xmm0,%xmm0
   2141b:	0f 29 84 24 f0 00 00 	movaps %xmm0,0xf0(%rsp)
   21422:	00 
   21423:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
   2142a:	00 
   2142b:	0f 29 84 24 d0 00 00 	movaps %xmm0,0xd0(%rsp)
   21432:	00 
   21433:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   2143a:	00 
   2143b:	0f 28 44 24 40       	movaps 0x40(%rsp),%xmm0
   21440:	0f 28 4c 24 50       	movaps 0x50(%rsp),%xmm1
   21445:	0f 28 54 24 60       	movaps 0x60(%rsp),%xmm2
   2144a:	0f                   	.byte 0xf
   2144b:	29                   	.byte 0x29
   2144c:	84 24 80             	test   %ah,(%rax,%rax,4)
	...

=== A=0x403CFBE F=0x3cfbe ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000003cdbe <<star_core::secure_channel::server::SecureChannelServer>::new+0x31e>:
   3cdbe:	00 48 89             	add    %cl,-0x77(%rax)
   3cdc1:	d8 48 81             	fmuls  -0x7f(%rax)
   3cdc4:	c4                   	(bad)
   3cdc5:	28 01                	sub    %al,(%rcx)
   3cdc7:	00 00                	add    %al,(%rax)
   3cdc9:	5b                   	pop    %rbx
   3cdca:	41 5e                	pop    %r14
   3cdcc:	41 5f                	pop    %r15
   3cdce:	5d                   	pop    %rbp
   3cdcf:	c3                   	ret
   3cdd0:	bf 01 00 00 00       	mov    $0x1,%edi
   3cdd5:	be 20 00 00 00       	mov    $0x20,%esi
   3cdda:	ff 15 a0 50 04 00    	call   *0x450a0(%rip)        # 81e80 <_GLOBAL_OFFSET_TABLE_+0x790>
   3cde0:	0f 0b                	ud2
   3cde2:	48 89 c3             	mov    %rax,%rbx
   3cde5:	eb 0f                	jmp    3cdf6 <<star_core::secure_channel::server::SecureChannelServer>::new+0x356>
   3cde7:	eb 21                	jmp    3ce0a <<star_core::secure_channel::server::SecureChannelServer>::new+0x36a>
   3cde9:	48 89 c3             	mov    %rax,%rbx
   3cdec:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3cdf0:	ff 15 4a 50 04 00    	call   *0x4504a(%rip)        # 81e40 <_GLOBAL_OFFSET_TABLE_+0x750>
   3cdf6:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   3cdfd:	e8 1e d1 ff ff       	call   39f20 <core::ptr::drop_glue::<hpke_ng::kem::dh::DhPrivateKey<hpke_ng::kem::dh::X25519>>>
   3ce02:	eb 13                	jmp    3ce17 <<star_core::secure_channel::server::SecureChannelServer>::new+0x377>
   3ce04:	ff 15 c6 51 04 00    	call   *0x451c6(%rip)        # 81fd0 <_GLOBAL_OFFSET_TABLE_+0x8e0>
   3ce0a:	48 89 c3             	mov    %rax,%rbx
   3ce0d:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3ce11:	ff 15 29 50 04 00    	call   *0x45029(%rip)        # 81e40 <_GLOBAL_OFFSET_TABLE_+0x750>
   3ce17:	48 89 df             	mov    %rbx,%rdi
   3ce1a:	e8 21 92 fc ff       	call   6040 <_Unwind_Resume@plt>
   3ce1f:	ff 15 ab 51 04 00    	call   *0x451ab(%rip)        # 81fd0 <_GLOBAL_OFFSET_TABLE_+0x8e0>
   3ce25:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   3ce2c:	00 00 00 
   3ce2f:	90                   	nop

000000000003ce30 <<star_core::serv::enclave::Enclave>::issue_token>:
   3ce30:	55                   	push   %rbp
   3ce31:	48 89 e5             	mov    %rsp,%rbp
   3ce34:	41 57                	push   %r15
   3ce36:	41 56                	push   %r14
   3ce38:	41 55                	push   %r13
   3ce3a:	41 54                	push   %r12
   3ce3c:	53                   	push   %rbx
   3ce3d:	48 83 ec 18          	sub    $0x18,%rsp
   3ce41:	48 89 fb             	mov    %rdi,%rbx
   3ce44:	48 83 f9 60          	cmp    $0x60,%rcx
   3ce48:	0f 82 84 00 00 00    	jb     3ced2 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   3ce4e:	4d 89 c6             	mov    %r8,%r14
   3ce51:	4c 8d 42 20          	lea    0x20(%rdx),%r8
   3ce55:	48 8d 42 40          	lea    0x40(%rdx),%rax
   3ce59:	48 83 c1 a0          	add    $0xffffffffffffffa0,%rcx
   3ce5d:	4c 8d 52 60          	lea    0x60(%rdx),%r10
   3ce61:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   3ce65:	49 89 cb             	mov    %rcx,%r11
   3ce68:	b9 20 00 00 00       	mov    $0x20,%ecx
   3ce6d:	41 b9 20 00 00 00    	mov    $0x20,%r9d
   3ce73:	49 89 f4             	mov    %rsi,%r12
   3ce76:	49 89 d7             	mov    %rdx,%r15
   3ce79:	48 89 c2             	mov    %rax,%rdx
   3ce7c:	41 53                	push   %r11
   3ce7e:	41 52                	push   %r10
   3ce80:	6a 20                	push   $0x20
   3ce82:	41 57                	push   %r15
   3ce84:	ff 15 be 4c 04 00    	call   *0x44cbe(%rip)        # 81b48 <_GLOBAL_OFFSET_TABLE_+0x458>
   3ce8a:	48 83 c4 20          	add    $0x20,%rsp
   3ce8e:	4c 8b 6d c0          	mov    -0x40(%rbp),%r13
   3ce92:	49 83 fd ff          	cmp    $0xffffffffffffffff,%r13
   3ce96:	74 3a                	je     3ced2 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   3ce98:	4c 8b 7d c8          	mov    -0x38(%rbp),%r15
   3ce9c:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
   3cea0:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   3cea4:	4c 89 e6             	mov    %r12,%rsi
   3cea7:	4c 89 fa             	mov    %r15,%rdx
   3ceaa:	4d 89 f0             	mov    %r14,%r8
   3cead:	ff 15 a5 4e 04 00    	call   *0x44ea5(%rip)        # 81d58 <_GLOBAL_OFFSET_TABLE_+0x668>
   3ceb3:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   3ceb7:	48 89 43 10          	mov    %rax,0x10(%rbx)
   3cebb:	0f 10 45 c0          	movups -0x40(%rbp),%xmm0
   3cebf:	0f 11 03             	movups %xmm0,(%rbx)
   3cec2:	4d 85 ed             	test   %r13,%r13
   3cec5:	74 16                	je     3cedd <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   3cec7:	4c 89 ff             	mov    %r15,%rdi
   3ceca:	ff 15 78 48 04 00    	call   *0x44878(%rip)        # 81748 <free@GLIBC_2.2.5>
   3ced0:	eb 0b                	jmp    3cedd <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   3ced2:	c6 43 08 01          	movb   $0x1,0x8(%rbx)
   3ced6:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   3cedd:	48 89 d8             	mov    %rbx,%rax
   3cee0:	48 83 c4 18          	add    $0x18,%rsp
   3cee4:	5b                   	pop    %rbx
   3cee5:	41 5c                	pop    %r12
   3cee7:	41 5d                	pop    %r13
   3cee9:	41 5e                	pop    %r14
   3ceeb:	41 5f                	pop    %r15
   3ceed:	5d                   	pop    %rbp
   3ceee:	c3                   	ret
   3ceef:	48 89 c3             	mov    %rax,%rbx
   3cef2:	4d 85 ed             	test   %r13,%r13
   3cef5:	74 09                	je     3cf00 <<star_core::serv::enclave::Enclave>::issue_token+0xd0>
   3cef7:	4c 89 ff             	mov    %r15,%rdi
   3cefa:	ff 15 48 48 04 00    	call   *0x44848(%rip)        # 81748 <free@GLIBC_2.2.5>
   3cf00:	48 89 df             	mov    %rbx,%rdi
   3cf03:	e8 38 91 fc ff       	call   6040 <_Unwind_Resume@plt>
   3cf08:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   3cf0f:	00 

000000000003cf10 <<star_core::serv::enclave::Enclave>::issue_token_plain>:
   3cf10:	55                   	push   %rbp
   3cf11:	48 89 e5             	mov    %rsp,%rbp
   3cf14:	41 57                	push   %r15
   3cf16:	41 56                	push   %r14
   3cf18:	41 55                	push   %r13
   3cf1a:	41 54                	push   %r12
   3cf1c:	53                   	push   %rbx
   3cf1d:	48 81 ec c8 00 00 00 	sub    $0xc8,%rsp
   3cf24:	4d 89 c7             	mov    %r8,%r15
   3cf27:	49 89 cd             	mov    %rcx,%r13
   3cf2a:	49 89 d6             	mov    %rdx,%r14
   3cf2d:	49 89 f4             	mov    %rsi,%r12
   3cf30:	48 89 fb             	mov    %rdi,%rbx
   3cf33:	48 c7 04 24 00 00 00 	movq   $0x0,(%rsp)
   3cf3a:	00 
   3cf3b:	be 01 00 43 4d       	mov    $0x4d430001,%esi
   3cf40:	31 ff                	xor    %edi,%edi
   3cf42:	45 31 c0             	xor    %r8d,%r8d
   3cf45:	45 31 c9             	xor    %r9d,%r9d
   3cf48:	ff 15 aa 49 04 00    	call   *0x449aa(%rip)        # 818f8 <_GLOBAL_OFFSET_TABLE_+0x208>
   3cf4e:	48 83 f8 ff          	cmp    $0xffffffffffffffff,%rax
   3cf52:	0f 85 a5 01 00 00    	jne    3d0fd <<star_core::serv::enclave::Enclave>::issue_token_plain+0x1ed>
   3cf58:	49 83 fd 50          	cmp    $0x50,%r13
   3cf5c:	72 62                	jb     3cfc0 <<star_core::serv::enclave::Enclave>::issue_token_plain+0xb0>
   3cf5e:	41 0f 10 46 20       	movups 0x20(%r14),%xmm0
   3cf63:	41 0f 10 4e 30       	movups 0x30(%r14),%xmm1
   3cf68:	0f 29 8d 50 ff ff ff 	movaps %xmm1,-0xb0(%rbp)
   3cf6f:	0f 29 85 40 ff ff ff 	movaps %xmm0,-0xc0(%rbp)
   3cf76:	f3 41 0f 6f 06       	movdqu (%r14),%xmm0
   3cf7b:	f3 41 0f 6f 4e 10    	movdqu 0x10(%r14),%xmm1
   3cf81:	66 0f 7f 85 20 ff ff 	movdqa %xmm0,-0xe0(%rbp)
   3cf88:	ff 
   3cf89:	66 0f 7f 8d 30 ff ff 	movdqa %xmm1,-0xd0(%rbp)
   3cf90:	ff 
   3cf91:	4d 8b 6e 40          	mov    0x40(%r14),%r13
   3cf95:	49 8b 76 48          	mov    0x48(%r14),%rsi
   3cf99:	4d 8b 44 24 58       	mov    0x58(%r12),%r8
   3cf9e:	49 83 c4 60          	add    $0x60,%r12
   3cfa2:	48 8d bd 20 ff ff ff 	lea    -0xe0(%rbp),%rdi
   3cfa9:	48 89 75 d0          	mov    %rsi,-0x30(%rbp)
   3cfad:	4c 89 ea             	mov    %r13,%rdx
   3cfb0:	4c 89 f9             	mov    %r15,%rcx
   3cfb3:	4d 89 e1             	mov    %r12,%r9
   3cfb6:	ff 15 fc 49 04 00    	call   *0x449fc(%rip)        # 819b8 <_GLOBAL_OFFSET_TABLE_+0x2c8>
   3cfbc:	84 c0                	test   %al,%al
   3cfbe:	75 3f                	jne    3cfff <<star_core::serv::enclave::Enclave>::issue_token_plain+0xef>
   3cfc0:	bf 01 00 00 00       	mov    $0x1,%edi
   3cfc5:	ff 15 ad 4c 04 00    	call   *0x44cad(%rip)        # 81c78 <malloc@GLIBC_2.2.5>
   3cfcb:	48 85 c0             	test   %rax,%rax
   3cfce:	0f 84 42 01 00 00    	je     3d116 <<star_core::serv::enclave::Enclave>::issue_token_plain+0x206>
   3cfd4:	c6 00 00             	movb   $0x0,(%rax)
   3cfd7:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   3cfde:	48 89 43 08          	mov    %rax,0x8(%rbx)
   3cfe2:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   3cfe9:	00 
   3cfea:	48 89 d8             	mov    %rbx,%rax
   3cfed:	48 81 c4 c8 00 00 00 	add    $0xc8,%rsp
   3cff4:	5b                   	pop    %rbx
   3cff5:	41 5c                	pop    %r12
   3cff7:	41 5d                	pop    %r13
   3cff9:	41 5e                	pop    %r14
   3cffb:	41 5f                	pop    %r15
   3cffd:	5d                   	pop    %rbp
   3cffe:	c3                   	ret
   3cfff:	41 0f 10 06          	movups (%r14),%xmm0
   3d003:	41 0f 10 4e 10       	movups 0x10(%r14),%xmm1
   3d008:	0f 11 4d a8          	movups %xmm1,-0x58(%rbp)
   3d00c:	0f 11 45 98          	movups %xmm0,-0x68(%rbp)
   3d010:	0f 10 05 49 00 03 00 	movups 0x30049(%rip),%xmm0        # 6d060 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x140>
   3d017:	0f 29 85 60 ff ff ff 	movaps %xmm0,-0xa0(%rbp)
   3d01e:	f3 0f 6f 05 4a 00 03 	movdqu 0x3004a(%rip),%xmm0        # 6d070 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x150>
   3d025:	00 
   3d026:	66 0f 7f 85 70 ff ff 	movdqa %xmm0,-0x90(%rbp)
   3d02d:	ff 
   3d02e:	48 c7 45 80 00 00 00 	movq   $0x0,-0x80(%rbp)
   3d035:	00 
   3d036:	48 8d 75 88          	lea    -0x78(%rbp),%rsi
   3d03a:	4c 89 6d 88          	mov    %r13,-0x78(%rbp)
   3d03e:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   3d042:	48 89 45 90          	mov    %rax,-0x70(%rbp)
   3d046:	c6 45 c8 30          	movb   $0x30,-0x38(%rbp)
   3d04a:	c6 45 b8 80          	movb   $0x80,-0x48(%rbp)
   3d04e:	c7 45 b9 00 00 00 00 	movl   $0x0,-0x47(%rbp)
   3d055:	c7 45 bc 00 00 00 00 	movl   $0x0,-0x44(%rbp)
   3d05c:	48 b8 00 00 00 00 00 	movabs $0x8001000000000000,%rax
   3d063:	00 01 80 
   3d066:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   3d06a:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   3d071:	ba 01 00 00 00       	mov    $0x1,%edx
   3d076:	ff 15 6c 4c 04 00    	call   *0x44c6c(%rip)        # 81ce8 <_GLOBAL_OFFSET_TABLE_+0x5f8>
   3d07c:	66 0f ef c0          	pxor   %xmm0,%xmm0
   3d080:	66 0f 6f 8d 60 ff ff 	movdqa -0xa0(%rbp),%xmm1
   3d087:	ff 
   3d088:	66 0f 6f 95 70 ff ff 	movdqa -0x90(%rbp),%xmm2
   3d08f:	ff 
   3d090:	66 0f 6f d9          	movdqa %xmm1,%xmm3
   3d094:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   3d098:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   3d09d:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   3d0a2:	66 0f 60 c8          	punpcklbw %xmm0,%xmm1
   3d0a6:	f2 0f 70 c9 1b       	pshuflw $0x1b,%xmm1,%xmm1
   3d0ab:	f3 0f 70 c9 1b       	pshufhw $0x1b,%xmm1,%xmm1
   3d0b0:	66 0f 67 cb          	packuswb %xmm3,%xmm1
   3d0b4:	66 0f 6f da          	movdqa %xmm2,%xmm3
   3d0b8:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   3d0bc:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   3d0c1:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   3d0c6:	66 0f 60 d0          	punpcklbw %xmm0,%xmm2
   3d0ca:	f2 0f 70 c2 1b       	pshuflw $0x1b,%xmm2,%xmm0
   3d0cf:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   3d0d4:	66 0f 67 c3          	packuswb %xmm3,%xmm0
   3d0d8:	66 0f 7f 8d 60 ff ff 	movdqa %xmm1,-0xa0(%rbp)
   3d0df:	ff 
   3d0e0:	66 0f 7f 85 70 ff ff 	movdqa %xmm0,-0x90(%rbp)
   3d0e7:	ff 
   3d0e8:	48 8d b5 60 ff ff ff 	lea    -0xa0(%rbp),%rsi
   3d0ef:	48 89 df             	mov    %rbx,%rdi
   3d0f2:	ff 15 68 48 04 00    	call   *0x44868(%rip)        # 81960 <_GLOBAL_OFFSET_TABLE_+0x270>
   3d0f8:	e9 ed fe ff ff       	jmp    3cfea <<star_core::serv::enclave::Enclave>::issue_token_plain+0xda>
   3d0fd:	48 8d 3d d5 63 03 00 	lea    0x363d5(%rip),%rdi        # 734d9 <anon.10746075dd51319079efedc96a688e81.0.llvm.4247047481296140766>
   3d104:	48 8d 15 a5 43 04 00 	lea    0x443a5(%rip),%rdx        # 814b0 <anon.10746075dd51319079efedc96a688e81.2.llvm.4247047481296140766>
   3d10b:	be 37 00 00 00       	mov    $0x37,%esi
   3d110:	ff 15 0a 46 04 00    	call   *0x4460a(%rip)        # 81720 <_GLOBAL_OFFSET_TABLE_+0x30>
   3d116:	bf 01 00 00 00       	mov    $0x1,%edi
   3d11b:	be 01 00 00 00       	mov    $0x1,%esi
   3d120:	ff 15 b2 4c 04 00    	call   *0x44cb2(%rip)        # 81dd8 <_GLOBAL_OFFSET_TABLE_+0x6e8>
   3d126:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   3d12d:	00 00 00 

000000000003d130 <<star_core::serv::core::cred::Credential>::is_valid>:
   3d130:	55                   	push   %rbp
   3d131:	48 89 e5             	mov    %rsp,%rbp
   3d134:	41 57                	push   %r15
   3d136:	41 56                	push   %r14
   3d138:	41 55                	push   %r13
   3d13a:	41 54                	push   %r12
   3d13c:	53                   	push   %rbx
   3d13d:	48 81 ec e8 02 00 00 	sub    $0x2e8,%rsp
   3d144:	4c 89 4d c8          	mov    %r9,-0x38(%rbp)
   3d148:	4d 89 c5             	mov    %r8,%r13
   3d14b:	49 89 d7             	mov    %rdx,%r15
   3d14e:	48 89 7d c0          	mov    %rdi,-0x40(%rbp)
   3d152:	48 89 d0             	mov    %rdx,%rax
   3d155:	48 c1 e8 08          	shr    $0x8,%rax
   3d159:	48 89 85 68 ff ff ff 	mov    %rax,-0x98(%rbp)
   3d160:	48 89 d0             	mov    %rdx,%rax
   3d163:	48 c1 e8 10          	shr    $0x10,%rax
   3d167:	48 89 85 70 ff ff ff 	mov    %rax,-0x90(%rbp)
   3d16e:	48 89 d0             	mov    %rdx,%rax
   3d171:	48 c1 e8 18          	shr    $0x18,%rax
   3d175:	48 89 85 78 ff ff ff 	mov    %rax,-0x88(%rbp)
   3d17c:	48 89 d0             	mov    %rdx,%rax
   3d17f:	48 c1 e8 20          	shr    $0x20,%rax
   3d183:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
   3d187:	48 89 d0             	mov    %rdx,%rax
   3d18a:	48 c1 e8 28          	shr    $0x28,%rax
   3d18e:	48 89 45 88          	mov    %rax,-0x78(%rbp)
   3d192:	48 89 d0             	mov    %rdx,%rax
   3d195:	48 c1 e8 30          	shr    $0x30,%rax
   3d199:	48 89 45 90          	mov    %rax,-0x70(%rbp)
   3d19d:	48 89 d0             	mov    %rdx,%rax
   3d1a0:	48 c1 e8 38          	shr    $0x38,%rax
   3d1a4:	48 89 45 a0          	mov    %rax,-0x60(%rbp)
   3d1a8:	4c 89 c3             	mov    %r8,%rbx
   3d1ab:	48 c1 eb 08          	shr    $0x8,%rbx
   3d1af:	48 89 9d b8 fe ff ff 	mov    %rbx,-0x148(%rbp)
   3d1b6:	4d 89 c4             	mov    %r8,%r12
   3d1b9:	49 c1 ec 10          	shr    $0x10,%r12
   3d1bd:	4d                   	rex.WRB

=== A=0x48CB7D0 F=0x8cb7d0 ===

./target/release/analysis:     file format elf64-x86-64


