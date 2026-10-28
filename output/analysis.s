=== VALGRIND OUTPUT ===
==2556== Memcheck, a memory error detector
==2556== Copyright (C) 2002-2026, and GNU GPL'd, by Julian Seward et al.
==2556== Using Valgrind-3.27.1 and LibVEX; rerun with -h for copyright info
==2556== Command: ./target/release/analysis
==2556== 
==2556== Conditional jump or move depends on uninitialised value(s)
==2556==    at 0x4021370: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4034479: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400EDD8: analysis::main (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400D985: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400E2DB: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA16: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
==2556== Conditional jump or move depends on uninitialised value(s)
==2556==    at 0x401FC50: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035943: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400EDD8: analysis::main (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400D985: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400E2DB: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA16: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
==2556== Conditional jump or move depends on uninitialised value(s)
==2556==    at 0x4021370: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4034479: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035EF9: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400F40F: analysis::main (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400D985: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400E2DB: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA16: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
==2556== Conditional jump or move depends on uninitialised value(s)
==2556==    at 0x401FC50: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035943: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035EF9: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400F40F: analysis::main (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400D985: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400E2DB: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA16: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
==2556== Conditional jump or move depends on uninitialised value(s)
==2556==    at 0x4035FFB: <star_core::serv::enclave::Enclave>::issue_token_plain (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035F22: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400F40F: analysis::main (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400D985: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400E2DB: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA16: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
==2556== Conditional jump or move depends on uninitialised value(s)
==2556==    at 0x4021370: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4034479: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035EF9: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400F558: analysis::main (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400D985: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400E2DB: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA16: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
==2556== Conditional jump or move depends on uninitialised value(s)
==2556==    at 0x4021370: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4034479: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035EF9: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400F624: analysis::main (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400D985: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400E2DB: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA16: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
==2556== Conditional jump or move depends on uninitialised value(s)
==2556==    at 0x401FC50: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035943: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4035EF9: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400F624: analysis::main (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400D985: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2556==    by 0x400E2DB: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA16: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
done
==2556== 
==2556== HEAP SUMMARY:
==2556==     in use at exit: 544 bytes in 1 blocks
==2556==   total heap usage: 80 allocs, 79 frees, 6,868 bytes allocated
==2556== 
==2556== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==2556==    at 0x48C37D0: malloc (in /nix/store/mf7rzpknxwg0q1wf0nm8gfzikapg00h1-valgrind-3.27.1/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==2556==    by 0x4060D37: <alloc::collections::btree::map::entry::VacantEntry<usize, std::sys::pal::unix::stack_overflow::thread_info::ThreadInfo>>::insert_entry (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4051DD3: std::sys::pal::unix::stack_overflow::thread_info::set_current_info (in /workspace/star/core/target/release/analysis)
==2556==    by 0x40529FF: std::sys::pal::unix::stack_overflow::imp::init (in /workspace/star/core/target/release/analysis)
==2556==    by 0x40611FF: std::sys::pal::unix::init (in /workspace/star/core/target/release/analysis)
==2556==    by 0x404DA10: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2556==    by 0x4011483: main (in /workspace/star/core/target/release/analysis)
==2556== 
==2556== LEAK SUMMARY:
==2556==    definitely lost: 0 bytes in 0 blocks
==2556==    indirectly lost: 0 bytes in 0 blocks
==2556==      possibly lost: 0 bytes in 0 blocks
==2556==    still reachable: 544 bytes in 1 blocks
==2556==         suppressed: 0 bytes in 0 blocks
==2556== 
==2556== Use --track-origins=yes to see where uninitialised values come from
==2556== For lists of detected and suppressed errors, rerun with: -s
==2556== ERROR SUMMARY: 8 errors from 8 contexts (suppressed: 0 from 0)

=== A=0x4021370 F=0x21370 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000021170 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x140>:
   21170:	7d aa                	jge    2111c <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0xec>
   21172:	00 40 0f             	add    %al,0xf(%rax)
   21175:	94                   	xchg   %eax,%esp
   21176:	c7                   	(bad)
   21177:	ff 15 0b 8b 05 00    	call   *0x58b0b(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   2117d:	88 45 c8             	mov    %al,-0x38(%rbp)
   21180:	31 ff                	xor    %edi,%edi
   21182:	80 7d ab 00          	cmpb   $0x0,-0x55(%rbp)
   21186:	40 0f 94 c7          	sete   %dil
   2118a:	ff 15 f8 8a 05 00    	call   *0x58af8(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   21190:	88 45 c9             	mov    %al,-0x37(%rbp)
   21193:	31 ff                	xor    %edi,%edi
   21195:	80 7d ac 00          	cmpb   $0x0,-0x54(%rbp)
   21199:	40 0f 94 c7          	sete   %dil
   2119d:	ff 15 e5 8a 05 00    	call   *0x58ae5(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   211a3:	88 45 ca             	mov    %al,-0x36(%rbp)
   211a6:	31 ff                	xor    %edi,%edi
   211a8:	80 7d ad 00          	cmpb   $0x0,-0x53(%rbp)
   211ac:	40 0f 94 c7          	sete   %dil
   211b0:	ff 15 d2 8a 05 00    	call   *0x58ad2(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   211b6:	88 45 cb             	mov    %al,-0x35(%rbp)
   211b9:	31 ff                	xor    %edi,%edi
   211bb:	80 7d ae 00          	cmpb   $0x0,-0x52(%rbp)
   211bf:	40 0f 94 c7          	sete   %dil
   211c3:	ff 15 bf 8a 05 00    	call   *0x58abf(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   211c9:	41 89 c6             	mov    %eax,%r14d
   211cc:	31 ff                	xor    %edi,%edi
   211ce:	80 7d af 00          	cmpb   $0x0,-0x51(%rbp)
   211d2:	40 0f 94 c7          	sete   %dil
   211d6:	ff 15 ac 8a 05 00    	call   *0x58aac(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   211dc:	88 45 cc             	mov    %al,-0x34(%rbp)
   211df:	31 ff                	xor    %edi,%edi
   211e1:	80 7d b0 00          	cmpb   $0x0,-0x50(%rbp)
   211e5:	40 0f 94 c7          	sete   %dil
   211e9:	ff 15 99 8a 05 00    	call   *0x58a99(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   211ef:	88 45 cd             	mov    %al,-0x33(%rbp)
   211f2:	31 ff                	xor    %edi,%edi
   211f4:	80 7d b1 00          	cmpb   $0x0,-0x4f(%rbp)
   211f8:	40 0f 94 c7          	sete   %dil
   211fc:	ff 15 86 8a 05 00    	call   *0x58a86(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   21202:	88 45 ce             	mov    %al,-0x32(%rbp)
   21205:	31 ff                	xor    %edi,%edi
   21207:	80 7d b2 00          	cmpb   $0x0,-0x4e(%rbp)
   2120b:	40 0f 94 c7          	sete   %dil
   2120f:	ff 15 73 8a 05 00    	call   *0x58a73(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   21215:	88 45 cf             	mov    %al,-0x31(%rbp)
   21218:	31 ff                	xor    %edi,%edi
   2121a:	80 7d b3 00          	cmpb   $0x0,-0x4d(%rbp)
   2121e:	40 0f 94 c7          	sete   %dil
   21222:	ff 15 60 8a 05 00    	call   *0x58a60(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   21228:	88 45 d0             	mov    %al,-0x30(%rbp)
   2122b:	31 ff                	xor    %edi,%edi
   2122d:	80 7d b4 00          	cmpb   $0x0,-0x4c(%rbp)
   21231:	40 0f 94 c7          	sete   %dil
   21235:	ff 15 4d 8a 05 00    	call   *0x58a4d(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   2123b:	41 89 c4             	mov    %eax,%r12d
   2123e:	31 ff                	xor    %edi,%edi
   21240:	80 7d b5 00          	cmpb   $0x0,-0x4b(%rbp)
   21244:	40 0f 94 c7          	sete   %dil
   21248:	ff 15 3a 8a 05 00    	call   *0x58a3a(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   2124e:	88 45 d1             	mov    %al,-0x2f(%rbp)
   21251:	31 ff                	xor    %edi,%edi
   21253:	80 7d b6 00          	cmpb   $0x0,-0x4a(%rbp)
   21257:	40 0f 94 c7          	sete   %dil
   2125b:	ff 15 27 8a 05 00    	call   *0x58a27(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   21261:	88 45 d2             	mov    %al,-0x2e(%rbp)
   21264:	31 ff                	xor    %edi,%edi
   21266:	80 7d b7 00          	cmpb   $0x0,-0x49(%rbp)
   2126a:	40 0f 94 c7          	sete   %dil
   2126e:	ff 15 14 8a 05 00    	call   *0x58a14(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   21274:	88 45 d3             	mov    %al,-0x2d(%rbp)
   21277:	31 ff                	xor    %edi,%edi
   21279:	80 7d b8 00          	cmpb   $0x0,-0x48(%rbp)
   2127d:	40 0f 94 c7          	sete   %dil
   21281:	ff 15 01 8a 05 00    	call   *0x58a01(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   21287:	88 45 d4             	mov    %al,-0x2c(%rbp)
   2128a:	31 ff                	xor    %edi,%edi
   2128c:	80 7d b9 00          	cmpb   $0x0,-0x47(%rbp)
   21290:	40 0f 94 c7          	sete   %dil
   21294:	ff 15 ee 89 05 00    	call   *0x589ee(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   2129a:	88 45 d5             	mov    %al,-0x2b(%rbp)
   2129d:	31 ff                	xor    %edi,%edi
   2129f:	80 7d ba 00          	cmpb   $0x0,-0x46(%rbp)
   212a3:	40 0f 94 c7          	sete   %dil
   212a7:	ff 15 db 89 05 00    	call   *0x589db(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   212ad:	88 45 d6             	mov    %al,-0x2a(%rbp)
   212b0:	31 ff                	xor    %edi,%edi
   212b2:	80 7d bb 00          	cmpb   $0x0,-0x45(%rbp)
   212b6:	40 0f 94 c7          	sete   %dil
   212ba:	ff 15 c8 89 05 00    	call   *0x589c8(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   212c0:	41 89 c5             	mov    %eax,%r13d
   212c3:	31 ff                	xor    %edi,%edi
   212c5:	80 7d bc 00          	cmpb   $0x0,-0x44(%rbp)
   212c9:	40 0f 94 c7          	sete   %dil
   212cd:	ff 15 b5 89 05 00    	call   *0x589b5(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   212d3:	88 45 d7             	mov    %al,-0x29(%rbp)
   212d6:	31 ff                	xor    %edi,%edi
   212d8:	80 7d bd 00          	cmpb   $0x0,-0x43(%rbp)
   212dc:	40 0f 94 c7          	sete   %dil
   212e0:	ff 15 a2 89 05 00    	call   *0x589a2(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   212e6:	0f b6 4d be          	movzbl -0x42(%rbp),%ecx
   212ea:	22 4d bf             	and    -0x41(%rbp),%cl
   212ed:	0f b6 55 c0          	movzbl -0x40(%rbp),%edx
   212f1:	22 55 c1             	and    -0x3f(%rbp),%dl
   212f4:	20 ca                	and    %cl,%dl
   212f6:	0f b6 4d c2          	movzbl -0x3e(%rbp),%ecx
   212fa:	22 4d c3             	and    -0x3d(%rbp),%cl
   212fd:	22 4d c4             	and    -0x3c(%rbp),%cl
   21300:	20 d1                	and    %dl,%cl
   21302:	44 22 7d c5          	and    -0x3b(%rbp),%r15b
   21306:	44 22 7d c6          	and    -0x3a(%rbp),%r15b
   2130a:	44 22 7d c7          	and    -0x39(%rbp),%r15b
   2130e:	41 20 cf             	and    %cl,%r15b
   21311:	22 5d c8             	and    -0x38(%rbp),%bl
   21314:	22 5d c9             	and    -0x37(%rbp),%bl
   21317:	22 5d ca             	and    -0x36(%rbp),%bl
   2131a:	22 5d cb             	and    -0x35(%rbp),%bl
   2131d:	44 20 fb             	and    %r15b,%bl
   21320:	44 22 75 cc          	and    -0x34(%rbp),%r14b
   21324:	44 22 75 cd          	and    -0x33(%rbp),%r14b
   21328:	44 22 75 ce          	and    -0x32(%rbp),%r14b
   2132c:	44 22 75 cf          	and    -0x31(%rbp),%r14b
   21330:	44 22 75 d0          	and    -0x30(%rbp),%r14b
   21334:	41 20 de             	and    %bl,%r14b
   21337:	44 22 65 d1          	and    -0x2f(%rbp),%r12b
   2133b:	44 22 65 d2          	and    -0x2e(%rbp),%r12b
   2133f:	44 22 65 d3          	and    -0x2d(%rbp),%r12b
   21343:	44 22 65 d4          	and    -0x2c(%rbp),%r12b
   21347:	44 22 65 d5          	and    -0x2b(%rbp),%r12b
   2134b:	44 22 65 d6          	and    -0x2a(%rbp),%r12b
   2134f:	45 20 f4             	and    %r14b,%r12b
   21352:	44 22 6d d7          	and    -0x29(%rbp),%r13b
   21356:	41 20 c5             	and    %al,%r13b
   21359:	45 20 e5             	and    %r12b,%r13b
   2135c:	41 80 e5 01          	and    $0x1,%r13b
   21360:	41 0f b6 fd          	movzbl %r13b,%edi
   21364:	ff 15 1e 89 05 00    	call   *0x5891e(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   2136a:	84 c0                	test   %al,%al
   2136c:	48 8b 5d 90          	mov    -0x70(%rbp),%rbx
   21370:	74 0d                	je     2137f <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x34f>
   21372:	c6 43 08 03          	movb   $0x3,0x8(%rbx)
   21376:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   2137d:	eb 32                	jmp    213b1 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x381>
   2137f:	bf 20 00 00 00       	mov    $0x20,%edi
   21384:	ff 15 e6 88 05 00    	call   *0x588e6(%rip)        # 79c70 <malloc@GLIBC_2.2.5>
   2138a:	48 85 c0             	test   %rax,%rax
   2138d:	74 3e                	je     213cd <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x39d>
   2138f:	0f 10 45 9e          	movups -0x62(%rbp),%xmm0
   21393:	0f 10 4d ae          	movups -0x52(%rbp),%xmm1
   21397:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   2139b:	0f 11 00             	movups %xmm0,(%rax)
   2139e:	48 c7 03 20 00 00 00 	movq   $0x20,(%rbx)
   213a5:	48 89 43 08          	mov    %rax,0x8(%rbx)
   213a9:	48 c7 43 10 20 00 00 	movq   $0x20,0x10(%rbx)
   213b0:	00 
   213b1:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   213b5:	ff 15 75 88 05 00    	call   *0x58875(%rip)        # 79c30 <_GLOBAL_OFFSET_TABLE_+0x520>
   213bb:	48 89 d8             	mov    %rbx,%rax
   213be:	48 83 c4 68          	add    $0x68,%rsp
   213c2:	5b                   	pop    %rbx
   213c3:	41 5c                	pop    %r12
   213c5:	41 5d                	pop    %r13
   213c7:	41 5e                	pop    %r14
   213c9:	41 5f                	pop    %r15
   213cb:	5d                   	pop    %rbp
   213cc:	c3                   	ret
   213cd:	bf 01 00 00 00       	mov    $0x1,%edi
   213d2:	be 20 00 00 00       	mov    $0x20,%esi
   213d7:	ff 15 93 8a 05 00    	call   *0x58a93(%rip)        # 79e70 <_GLOBAL_OFFSET_TABLE_+0x760>
   213dd:	0f 0b                	ud2
   213df:	eb 00                	jmp    213e1 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x3b1>
   213e1:	48 89 c3             	mov    %rax,%rbx
   213e4:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   213e8:	ff 15 42 88 05 00    	call   *0x58842(%rip)        # 79c30 <_GLOBAL_OFFSET_TABLE_+0x520>
   213ee:	48 89 df             	mov    %rbx,%rdi
   213f1:	e8 4a 4c fe ff       	call   6040 <_Unwind_Resume@plt>
   213f6:	ff 15 cc 8b 05 00    	call   *0x58bcc(%rip)        # 79fc8 <_GLOBAL_OFFSET_TABLE_+0x8b8>
   213fc:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000021400 <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed>:
   21400:	55                   	push   %rbp
   21401:	48 89 e5             	mov    %rsp,%rbp
   21404:	41 57                	push   %r15
   21406:	41 56                	push   %r14
   21408:	41 55                	push   %r13
   2140a:	41 54                	push   %r12
   2140c:	53                   	push   %rbx
   2140d:	48 81 ec d8 00 00 00 	sub    $0xd8,%rsp
   21414:	48 89 fb             	mov    %rdi,%rbx
   21417:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   2141e:	ba 90 00 00 00       	mov    $0x90,%edx
   21423:	ff 15 77 87 05 00    	call   *0x58777(%rip)        # 79ba0 <memcpy@GLIBC_2.14>
   21429:	4c 8d b5 40 ff ff ff 	lea    -0xc0(%rbp),%r14
   21430:	44 0f b6 a5 3f ff ff 	movzbl -0xc1(%rbp),%r12d
   21437:	ff 
   21438:	4c 8b ad 60 ff ff ff 	mov    -0xa0(%rbp),%r13
   2143f:	4d 89 e7             	mov    %r12,%r15
   21442:	49 83 f7 3f          	xor    $0x3f,%r15
   21446:	42 c6 84 25 00 ff ff 	movb   $0x80,-0x100(%rbp,%r12,1)
   2144d:	ff 80 
   2144f:	49 8d 3c 2c          	lea    (%r12,%rbp,1),%rdi
   21453:	48 81 c7 01 ff ff ff 	add    $0xffffffffffffff01,%rdi
   2145a:	31 f6                	xor    %esi,%esi
   2145c:	4c 89 fa             	mov    %r15,%rdx
   2145f:	ff 15 8b 85 05 00    	call   *0x5858b(%rip)        # 799f0 <memset@GLIBC_2.2.5>
   21465:	49 83 ff 07          	cmp    $0x7,%r15
   21469:	77 63                	ja     214ce <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed+0xce>
   2146b:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   21472:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   21479:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   21480:	66 0f 6f 9d 30 ff ff 	movdqa -0xd0(%rbp),%xmm3
   21487:	ff 
   21488:	66 0f 7f 5d c0       	movdqa %xmm3,-0x40(%rbp)
   2148d:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   21491:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   21495:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   21499:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   2149d:	ba 01 00 00 00       	mov    $0x1,%edx
   214a2:	4c 89 f7             	mov    %r14,%rdi
   214a5:	ff 15 f5 87 05 00    	call   *0x587f5(%rip)        # 79ca0 <_GLOBAL_OFFSET_TABLE_+0x590>
   214ab:	0f 57 c0             	xorps  %xmm0,%xmm0
   214ae:	0f 29 85 00 ff ff ff 	movaps %xmm0,-0x100(%rbp)
   214b5:	0f 29 85 10 ff ff ff 	movaps %xmm0,-0xf0(%rbp)
   214bc:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   214c3:	48 c7 85 30 ff ff ff 	movq   $0x0,-0xd0(%rbp)
   214ca:	00 00 00 00 
   214ce:	49 c1 e5 09          	shl    $0x9,%r13
   214d2:	41 c1 e4 03          	shl    $0x3,%r12d
   214d6:	4d 09 ec             	or     %r13,%r12
   214d9:	49 0f cc             	bswap  %r12
   214dc:	4c 89 a5 38 ff ff ff 	mov    %r12,-0xc8(%rbp)
   214e3:	48 8b 85 30 ff ff ff 	mov    -0xd0(%rbp),%rax
   214ea:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   214ee:	48 8b 85 38 ff ff ff 	mov    -0xc8(%rbp),%rax
   214f5:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   214f9:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   21500:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   21507:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   2150e:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   21512:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   21516:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   2151a:	4c 8d 3d cf b7 00 00 	lea    0xb7cf(%rip),%r15        # 2ccf0 <sha2::sha256::compress256>
   21521:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   21525:	ba 01 00 00 00       	mov    $0x1,%edx
   2152a:	4c 89 f7             	mov    %r14,%rdi
   2152d:	41 ff d7             	call   *%r15
   21530:	48 8d bd 68 ff ff ff 	lea    -0x98(%rbp),%rdi
   21537:	66 0f 6f 85 40 ff ff 	movdqa -0xc0(%rbp),%xmm0
   2153e:	ff 
   2153f:	66 0f 6f 8d 50 ff ff 	movdqa -0xb0(%rbp),%xmm1
   21546:	ff 
   21547:	66 0f ef db          	pxor   %xmm3,%xmm3
   2154b:	66 0f 6f d0          	movdqa %xmm0,%xmm2
   2154f:	66 0f 68 d3          	punpckhbw %xmm3,%xmm2
   21553:	f2 0f 70 d2 1b       	pshuflw $0x1b,%xmm2,%xmm2
   21558:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   2155d:	66 0f 60 c3          	punpcklbw %xmm3,%xmm0
   21561:	f2 0f 70 c0 1b       	pshuflw $0x1b,%xmm0,%xmm0
   21566:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   2156b:	66 0f 67 c2          	packuswb %xmm2,%xmm0
   2156f:	66                   	data16

=== A=0x401FC50 F=0x1fc50 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000001fa50 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x520>:
   1fa50:	4c 89 f6             	mov    %r14,%rsi
   1fa53:	e8 38 68 00 00       	call   26290 <<poly1305::backend::autodetect::State as universal_hash::UniversalHash>::finalize>
   1fa58:	0f b6 44 24 50       	movzbl 0x50(%rsp),%eax
   1fa5d:	31 ff                	xor    %edi,%edi
   1fa5f:	41 3a 04 1f          	cmp    (%r15,%rbx,1),%al
   1fa63:	40 0f 94 c7          	sete   %dil
   1fa67:	ff 15 1b a2 05 00    	call   *0x5a21b(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fa6d:	88 44 24 40          	mov    %al,0x40(%rsp)
   1fa71:	0f b6 44 24 51       	movzbl 0x51(%rsp),%eax
   1fa76:	31 ff                	xor    %edi,%edi
   1fa78:	41 3a 44 1f 01       	cmp    0x1(%r15,%rbx,1),%al
   1fa7d:	40 0f 94 c7          	sete   %dil
   1fa81:	ff 15 01 a2 05 00    	call   *0x5a201(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fa87:	88 44 24 38          	mov    %al,0x38(%rsp)
   1fa8b:	0f b6 44 24 52       	movzbl 0x52(%rsp),%eax
   1fa90:	31 ff                	xor    %edi,%edi
   1fa92:	41 3a 44 1f 02       	cmp    0x2(%r15,%rbx,1),%al
   1fa97:	40 0f 94 c7          	sete   %dil
   1fa9b:	ff 15 e7 a1 05 00    	call   *0x5a1e7(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1faa1:	88 44 24 48          	mov    %al,0x48(%rsp)
   1faa5:	0f b6 44 24 53       	movzbl 0x53(%rsp),%eax
   1faaa:	31 ff                	xor    %edi,%edi
   1faac:	41 3a 44 1f 03       	cmp    0x3(%r15,%rbx,1),%al
   1fab1:	40 0f 94 c7          	sete   %dil
   1fab5:	ff 15 cd a1 05 00    	call   *0x5a1cd(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fabb:	88 44 24 27          	mov    %al,0x27(%rsp)
   1fabf:	0f b6 44 24 54       	movzbl 0x54(%rsp),%eax
   1fac4:	31 ff                	xor    %edi,%edi
   1fac6:	41 3a 44 1f 04       	cmp    0x4(%r15,%rbx,1),%al
   1facb:	40 0f 94 c7          	sete   %dil
   1facf:	ff 15 b3 a1 05 00    	call   *0x5a1b3(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fad5:	41 89 c5             	mov    %eax,%r13d
   1fad8:	0f b6 44 24 55       	movzbl 0x55(%rsp),%eax
   1fadd:	31 ff                	xor    %edi,%edi
   1fadf:	41 3a 44 1f 05       	cmp    0x5(%r15,%rbx,1),%al
   1fae4:	40 0f 94 c7          	sete   %dil
   1fae8:	ff 15 9a a1 05 00    	call   *0x5a19a(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1faee:	88 44 24 26          	mov    %al,0x26(%rsp)
   1faf2:	0f b6 44 24 56       	movzbl 0x56(%rsp),%eax
   1faf7:	31 ff                	xor    %edi,%edi
   1faf9:	41 3a 44 1f 06       	cmp    0x6(%r15,%rbx,1),%al
   1fafe:	40 0f 94 c7          	sete   %dil
   1fb02:	ff 15 80 a1 05 00    	call   *0x5a180(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fb08:	88 44 24 25          	mov    %al,0x25(%rsp)
   1fb0c:	0f b6 44 24 57       	movzbl 0x57(%rsp),%eax
   1fb11:	31 ff                	xor    %edi,%edi
   1fb13:	41 3a 44 1f 07       	cmp    0x7(%r15,%rbx,1),%al
   1fb18:	40 0f 94 c7          	sete   %dil
   1fb1c:	ff 15 66 a1 05 00    	call   *0x5a166(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fb22:	41 89 c6             	mov    %eax,%r14d
   1fb25:	0f b6 44 24 58       	movzbl 0x58(%rsp),%eax
   1fb2a:	31 ff                	xor    %edi,%edi
   1fb2c:	41 3a 44 1f 08       	cmp    0x8(%r15,%rbx,1),%al
   1fb31:	40 0f 94 c7          	sete   %dil
   1fb35:	ff 15 4d a1 05 00    	call   *0x5a14d(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fb3b:	88 44 24 24          	mov    %al,0x24(%rsp)
   1fb3f:	0f b6 44 24 59       	movzbl 0x59(%rsp),%eax
   1fb44:	31 ff                	xor    %edi,%edi
   1fb46:	41 3a 44 1f 09       	cmp    0x9(%r15,%rbx,1),%al
   1fb4b:	40 0f 94 c7          	sete   %dil
   1fb4f:	ff 15 33 a1 05 00    	call   *0x5a133(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fb55:	88 44 24 23          	mov    %al,0x23(%rsp)
   1fb59:	0f b6 44 24 5a       	movzbl 0x5a(%rsp),%eax
   1fb5e:	31 ff                	xor    %edi,%edi
   1fb60:	41 3a 44 1f 0a       	cmp    0xa(%r15,%rbx,1),%al
   1fb65:	40 0f 94 c7          	sete   %dil
   1fb69:	ff 15 19 a1 05 00    	call   *0x5a119(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fb6f:	88 44 24 22          	mov    %al,0x22(%rsp)
   1fb73:	0f b6 44 24 5b       	movzbl 0x5b(%rsp),%eax
   1fb78:	31 ff                	xor    %edi,%edi
   1fb7a:	41 3a 44 1f 0b       	cmp    0xb(%r15,%rbx,1),%al
   1fb7f:	40 0f 94 c7          	sete   %dil
   1fb83:	ff 15 ff a0 05 00    	call   *0x5a0ff(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fb89:	41 89 c4             	mov    %eax,%r12d
   1fb8c:	0f b6 44 24 5c       	movzbl 0x5c(%rsp),%eax
   1fb91:	31 ff                	xor    %edi,%edi
   1fb93:	41 3a 44 1f 0c       	cmp    0xc(%r15,%rbx,1),%al
   1fb98:	40 0f 94 c7          	sete   %dil
   1fb9c:	ff 15 e6 a0 05 00    	call   *0x5a0e6(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fba2:	88 44 24 21          	mov    %al,0x21(%rsp)
   1fba6:	0f b6 44 24 5d       	movzbl 0x5d(%rsp),%eax
   1fbab:	31 ff                	xor    %edi,%edi
   1fbad:	41 3a 44 1f 0d       	cmp    0xd(%r15,%rbx,1),%al
   1fbb2:	40 0f 94 c7          	sete   %dil
   1fbb6:	ff 15 cc a0 05 00    	call   *0x5a0cc(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fbbc:	88 44 24 20          	mov    %al,0x20(%rsp)
   1fbc0:	0f b6 44 24 5e       	movzbl 0x5e(%rsp),%eax
   1fbc5:	31 ff                	xor    %edi,%edi
   1fbc7:	41 3a 44 1f 0e       	cmp    0xe(%r15,%rbx,1),%al
   1fbcc:	40 0f 94 c7          	sete   %dil
   1fbd0:	ff 15 b2 a0 05 00    	call   *0x5a0b2(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fbd6:	88 44 24 1f          	mov    %al,0x1f(%rsp)
   1fbda:	0f b6 44 24 5f       	movzbl 0x5f(%rsp),%eax
   1fbdf:	31 ff                	xor    %edi,%edi
   1fbe1:	41 3a 44 1f 0f       	cmp    0xf(%r15,%rbx,1),%al
   1fbe6:	40 0f 94 c7          	sete   %dil
   1fbea:	ff 15 98 a0 05 00    	call   *0x5a098(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fbf0:	0f b6 4c 24 40       	movzbl 0x40(%rsp),%ecx
   1fbf5:	22 4c 24 38          	and    0x38(%rsp),%cl
   1fbf9:	0f b6 54 24 48       	movzbl 0x48(%rsp),%edx
   1fbfe:	22 54 24 27          	and    0x27(%rsp),%dl
   1fc02:	20 ca                	and    %cl,%dl
   1fc04:	44 22 6c 24 26       	and    0x26(%rsp),%r13b
   1fc09:	44 22 6c 24 25       	and    0x25(%rsp),%r13b
   1fc0e:	41 20 d5             	and    %dl,%r13b
   1fc11:	44 22 74 24 24       	and    0x24(%rsp),%r14b
   1fc16:	44 22 74 24 23       	and    0x23(%rsp),%r14b
   1fc1b:	44 22 74 24 22       	and    0x22(%rsp),%r14b
   1fc20:	45 20 ee             	and    %r13b,%r14b
   1fc23:	44 22 64 24 21       	and    0x21(%rsp),%r12b
   1fc28:	44 22 64 24 20       	and    0x20(%rsp),%r12b
   1fc2d:	44 22 64 24 1f       	and    0x1f(%rsp),%r12b
   1fc32:	41 20 c4             	and    %al,%r12b
   1fc35:	45 20 f4             	and    %r14b,%r12b
   1fc38:	41 80 e4 01          	and    $0x1,%r12b
   1fc3c:	41 0f b6 fc          	movzbl %r12b,%edi
   1fc40:	ff 15 42 a0 05 00    	call   *0x5a042(%rip)        # 79c88 <_GLOBAL_OFFSET_TABLE_+0x578>
   1fc46:	84 c0                	test   %al,%al
   1fc48:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   1fc4f:	00 
   1fc50:	74 42                	je     1fc94 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x764>
   1fc52:	4c 8d b4 24 20 05 00 	lea    0x520(%rsp),%r14
   1fc59:	00 
   1fc5a:	4c 89 f7             	mov    %r14,%rdi
   1fc5d:	4c 89 fe             	mov    %r15,%rsi
   1fc60:	48 8b 54 24 28       	mov    0x28(%rsp),%rdx
   1fc65:	e8 46 1a 00 00       	call   216b0 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as cipher::stream::StreamCipher>::apply_keystream>
   1fc6a:	4c 89 f7             	mov    %r14,%rdi
   1fc6d:	e8 ae e9 ff ff       	call   1e620 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   1fc72:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   1fc79:	00 
   1fc7a:	48 8b 45 18          	mov    0x18(%rbp),%rax
   1fc7e:	48 8b 4c 24 28       	mov    0x28(%rsp),%rcx
   1fc83:	48 39 c1             	cmp    %rax,%rcx
   1fc86:	48 0f 43 c8          	cmovae %rax,%rcx
   1fc8a:	4d 89 7d 08          	mov    %r15,0x8(%r13)
   1fc8e:	49 89 4d 10          	mov    %rcx,0x10(%r13)
   1fc92:	eb 22                	jmp    1fcb6 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x786>
   1fc94:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   1fc9b:	00 
   1fc9c:	e8 7f e9 ff ff       	call   1e620 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   1fca1:	4c 89 ff             	mov    %r15,%rdi
   1fca4:	ff 15 be 9a 05 00    	call   *0x59abe(%rip)        # 79768 <free@GLIBC_2.2.5>
   1fcaa:	41 c6 45 08 00       	movb   $0x0,0x8(%r13)
   1fcaf:	48 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%rax
   1fcb6:	49 89 45 00          	mov    %rax,0x0(%r13)
   1fcba:	4c 89 e8             	mov    %r13,%rax
   1fcbd:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   1fcc1:	5b                   	pop    %rbx
   1fcc2:	41 5c                	pop    %r12
   1fcc4:	41 5d                	pop    %r13
   1fcc6:	41 5e                	pop    %r14
   1fcc8:	41 5f                	pop    %r15
   1fcca:	5d                   	pop    %rbp
   1fccb:	c3                   	ret
   1fccc:	48 8d 15 3d 8a 04 00 	lea    0x48a3d(%rip),%rdx        # 68710 <core::unicode::unicode_data::cf::SHORT_OFFSET_RUNS+0x70>
   1fcd3:	4c 8d 0d 46 7d 05 00 	lea    0x57d46(%rip),%r9        # 77a20 <anon.d49aa5e532bb8905898215d271d902c3.14.llvm.14046659981651887190+0x70>
   1fcda:	48 8d b4 24 e0 02 00 	lea    0x2e0(%rsp),%rsi
   1fce1:	00 
   1fce2:	31 ff                	xor    %edi,%edi
   1fce4:	31 c9                	xor    %ecx,%ecx
   1fce6:	ff 15 ac 9f 05 00    	call   *0x59fac(%rip)        # 79c98 <_GLOBAL_OFFSET_TABLE_+0x588>
   1fcec:	ff 15 16 9b 05 00    	call   *0x59b16(%rip)        # 79808 <_GLOBAL_OFFSET_TABLE_+0xf8>
   1fcf2:	e9 00 f9 ff ff       	jmp    1f5f7 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0xc7>
   1fcf7:	bf 01 00 00 00       	mov    $0x1,%edi
   1fcfc:	48 89 de             	mov    %rbx,%rsi
   1fcff:	ff 15 6b a1 05 00    	call   *0x5a16b(%rip)        # 79e70 <_GLOBAL_OFFSET_TABLE_+0x760>
   1fd05:	49 89 c6             	mov    %rax,%r14
   1fd08:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   1fd0d:	ff 15 55 9a 05 00    	call   *0x59a55(%rip)        # 79768 <free@GLIBC_2.2.5>
   1fd13:	4c 89 f7             	mov    %r14,%rdi
   1fd16:	e8 25 63 fe ff       	call   6040 <_Unwind_Resume@plt>
   1fd1b:	eb 06                	jmp    1fd23 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   1fd1d:	eb 04                	jmp    1fd23 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   1fd1f:	eb 02                	jmp    1fd23 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   1fd21:	eb 00                	jmp    1fd23 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   1fd23:	49 89 c6             	mov    %rax,%r14
   1fd26:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   1fd2d:	00 
   1fd2e:	e8 ed e8 ff ff       	call   1e620 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   1fd33:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   1fd38:	ff 15 2a 9a 05 00    	call   *0x59a2a(%rip)        # 79768 <free@GLIBC_2.2.5>
   1fd3e:	4c 89 f7             	mov    %r14,%rdi
   1fd41:	e8 fa 62 fe ff       	call   6040 <_Unwind_Resume@plt>
   1fd46:	ff 15 7c a2 05 00    	call   *0x5a27c(%rip)        # 79fc8 <_GLOBAL_OFFSET_TABLE_+0x8b8>
   1fd4c:	0f 1f 40 00          	nopl   0x0(%rax)

000000000001fd50 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal>:
   1fd50:	55                   	push   %rbp
   1fd51:	48 89 e5             	mov    %rsp,%rbp
   1fd54:	41 57                	push   %r15
   1fd56:	41 56                	push   %r14
   1fd58:	41 55                	push   %r13
   1fd5a:	41 54                	push   %r12
   1fd5c:	53                   	push   %rbx
   1fd5d:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   1fd61:	48 81 ec c0 05 00 00 	sub    $0x5c0,%rsp
   1fd68:	4c 89 44 24 08       	mov    %r8,0x8(%rsp)
   1fd6d:	48 89 8c 24 c0 02 00 	mov    %rcx,0x2c0(%rsp)
   1fd74:	00 
   1fd75:	48 83 f9 0c          	cmp    $0xc,%rcx
   1fd79:	0f 85 74 05 00 00    	jne    202f3 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5a3>
   1fd7f:	48 89 fb             	mov    %rdi,%rbx
   1fd82:	48 8b 7d 18          	mov    0x18(%rbp),%rdi
   1fd86:	48 83 c7 10          	add    $0x10,%rdi
   1fd8a:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
   1fd8f:	0f 88 89 05 00 00    	js     2031e <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5ce>
   1fd95:	4d 89 cf             	mov    %r9,%r15
   1fd98:	49 89 d6             	mov    %rdx,%r14
   1fd9b:	49 89 f4             	mov    %rsi,%r12
   1fd9e:	ff 15 cc 9e 05 00    	call   *0x59ecc(%rip)        # 79c70 <malloc@GLIBC_2.2.5>
   1fda4:	48 85 c0             	test   %rax,%rax
   1fda7:	0f 84 7e 05 00 00    	je     2032b <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5db>
   1fdad:	49 89 c5             	mov    %rax,%r13
   1fdb0:	48 89 5c 24 28       	mov    %rbx,0x28(%rsp)
   1fdb5:	48 8b 5d 18          	mov    0x18(%rbp),%rbx
   1fdb9:	48 85 db             	test   %rbx,%rbx
   1fdbc:	74 10                	je     1fdce <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x7e>
   1fdbe:	48 8b 75 10          	mov    0x10(%rbp),%rsi
   1fdc2:	4c 89 ef             	mov    %r13,%rdi
   1fdc5:	48 89 da             	mov    %rbx,%rdx
   1fdc8:	ff 15 d2 9d 05 00    	call   *0x59dd2(%rip)        # 79ba0 <memcpy@GLIBC_2.14>
   1fdce:	0f 10 05 ab 5d 04 00 	movups 0x45dab(%rip),%xmm0        # 65b80 <anon.10746075dd51319079efedc96a688e81.1.llvm.4247047481296140766+0xad>
   1fdd5:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   1fdda:	41 0f 10 04 24       	movups (%r12),%xmm0
   1fddf:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   1fde5:	0f 29 44 24 50       	movaps %xmm0,0x50(%rsp)
   1fdea:	0f 29 4c 24 60       	movaps %xmm1,0x60(%rsp)
   1fdef:	49 8b 06             	mov    (%r14),%rax
   1fdf2:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
   1fdf7:	41 8b 46 08          	mov    0x8(%r14),%eax
   1fdfb:	89 44 24 38          	mov    %eax,0x38(%rsp)
   1fdff:	48 8d 05 0a a2 05 00 	lea    0x5a20a(%rip),%rax        # 7a010 <chacha20::avx2_cpuid::STORAGE>
   1fe06:	0f b6 00             	movzbl (%rax),%eax
   1fe09:	3c ff                	cmp    $0xff,%al
   1fe0b:	4c 89 2c 24          	mov    %r13,(%rsp)
   1fe0f:	49 89 de             	mov    %rbx,%r14
   1fe12:	0f 84 fb 04 00 00    	je     20313 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5c3>
   1fe18:	0f 57 c0             	xorps  %xmm0,%xmm0
   1fe1b:	0f 29 84 24 f0 00 00 	movaps %xmm0,0xf0(%rsp)
   1fe22:	00 
   1fe23:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
   1fe2a:	00 
   1fe2b:	0f 29 84 24 d0 00 00 	movaps %xmm0,0xd0(%rsp)
   1fe32:	00 
   1fe33:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   1fe3a:	00 
   1fe3b:	0f 28 44 24 40       	movaps 0x40(%rsp),%xmm0
   1fe40:	0f 28 4c 24 50       	movaps 0x50(%rsp),%xmm1
   1fe45:	0f 28 54 24 60       	movaps 0x60(%rsp),%xmm2
   1fe4a:	0f                   	.byte 0xf
   1fe4b:	29                   	.byte 0x29
   1fe4c:	84 24 80             	test   %ah,(%rax,%rax,4)
	...

=== A=0x4035FFB F=0x35ffb ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000035dfb <<star_core::secure_channel::server::SecureChannelServer>::new+0x2eb>:
   35dfb:	85 00                	test   %eax,(%rax)
   35dfd:	ff                   	(bad)
   35dfe:	ff                   	(bad)
   35dff:	ff 0f                	decl   (%rdi)
   35e01:	11 43 40             	adc    %eax,0x40(%rbx)
   35e04:	48 8b 85 f0 fe ff ff 	mov    -0x110(%rbp),%rax
   35e0b:	48 89 43 30          	mov    %rax,0x30(%rbx)
   35e0f:	48 8b 85 f8 fe ff ff 	mov    -0x108(%rbp),%rax
   35e16:	48 89 43 38          	mov    %rax,0x38(%rbx)
   35e1a:	48 8b 85 10 ff ff ff 	mov    -0xf0(%rbp),%rax
   35e21:	48 89 43 50          	mov    %rax,0x50(%rbx)
   35e25:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   35e29:	ff 15 f9 3f 04 00    	call   *0x43ff9(%rip)        # 79e28 <_GLOBAL_OFFSET_TABLE_+0x718>
   35e2f:	48 89 d8             	mov    %rbx,%rax
   35e32:	48 81 c4 28 01 00 00 	add    $0x128,%rsp
   35e39:	5b                   	pop    %rbx
   35e3a:	41 5e                	pop    %r14
   35e3c:	41 5f                	pop    %r15
   35e3e:	5d                   	pop    %rbp
   35e3f:	c3                   	ret
   35e40:	bf 01 00 00 00       	mov    $0x1,%edi
   35e45:	be 20 00 00 00       	mov    $0x20,%esi
   35e4a:	ff 15 20 40 04 00    	call   *0x44020(%rip)        # 79e70 <_GLOBAL_OFFSET_TABLE_+0x760>
   35e50:	0f 0b                	ud2
   35e52:	48 89 c3             	mov    %rax,%rbx
   35e55:	eb 0f                	jmp    35e66 <<star_core::secure_channel::server::SecureChannelServer>::new+0x356>
   35e57:	eb 21                	jmp    35e7a <<star_core::secure_channel::server::SecureChannelServer>::new+0x36a>
   35e59:	48 89 c3             	mov    %rax,%rbx
   35e5c:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   35e60:	ff 15 c2 3f 04 00    	call   *0x43fc2(%rip)        # 79e28 <_GLOBAL_OFFSET_TABLE_+0x718>
   35e66:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   35e6d:	e8 1e d1 ff ff       	call   32f90 <core::ptr::drop_glue::<hpke_ng::kem::dh::DhPrivateKey<hpke_ng::kem::dh::X25519>>>
   35e72:	eb 13                	jmp    35e87 <<star_core::secure_channel::server::SecureChannelServer>::new+0x377>
   35e74:	ff 15 4e 41 04 00    	call   *0x4414e(%rip)        # 79fc8 <_GLOBAL_OFFSET_TABLE_+0x8b8>
   35e7a:	48 89 c3             	mov    %rax,%rbx
   35e7d:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   35e81:	ff 15 a1 3f 04 00    	call   *0x43fa1(%rip)        # 79e28 <_GLOBAL_OFFSET_TABLE_+0x718>
   35e87:	48 89 df             	mov    %rbx,%rdi
   35e8a:	e8 b1 01 fd ff       	call   6040 <_Unwind_Resume@plt>
   35e8f:	ff 15 33 41 04 00    	call   *0x44133(%rip)        # 79fc8 <_GLOBAL_OFFSET_TABLE_+0x8b8>
   35e95:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   35e9c:	00 00 00 
   35e9f:	90                   	nop

0000000000035ea0 <<star_core::serv::enclave::Enclave>::issue_token>:
   35ea0:	55                   	push   %rbp
   35ea1:	48 89 e5             	mov    %rsp,%rbp
   35ea4:	41 57                	push   %r15
   35ea6:	41 56                	push   %r14
   35ea8:	41 55                	push   %r13
   35eaa:	41 54                	push   %r12
   35eac:	53                   	push   %rbx
   35ead:	48 83 ec 18          	sub    $0x18,%rsp
   35eb1:	48 89 fb             	mov    %rdi,%rbx
   35eb4:	48 83 f9 60          	cmp    $0x60,%rcx
   35eb8:	0f 82 84 00 00 00    	jb     35f42 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   35ebe:	4d 89 c6             	mov    %r8,%r14
   35ec1:	4c 8d 42 20          	lea    0x20(%rdx),%r8
   35ec5:	48 8d 42 40          	lea    0x40(%rdx),%rax
   35ec9:	4c 8d 52 60          	lea    0x60(%rdx),%r10
   35ecd:	48 83 c1 a0          	add    $0xffffffffffffffa0,%rcx
   35ed1:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   35ed5:	49 89 cb             	mov    %rcx,%r11
   35ed8:	b9 20 00 00 00       	mov    $0x20,%ecx
   35edd:	41 b9 20 00 00 00    	mov    $0x20,%r9d
   35ee3:	49 89 f4             	mov    %rsi,%r12
   35ee6:	49 89 d7             	mov    %rdx,%r15
   35ee9:	48 89 c2             	mov    %rax,%rdx
   35eec:	41 53                	push   %r11
   35eee:	41 52                	push   %r10
   35ef0:	6a 20                	push   $0x20
   35ef2:	41 57                	push   %r15
   35ef4:	ff 15 ce 3a 04 00    	call   *0x43ace(%rip)        # 799c8 <_GLOBAL_OFFSET_TABLE_+0x2b8>
   35efa:	48 83 c4 20          	add    $0x20,%rsp
   35efe:	4c 8b 6d c0          	mov    -0x40(%rbp),%r13
   35f02:	49 83 fd ff          	cmp    $0xffffffffffffffff,%r13
   35f06:	74 3a                	je     35f42 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   35f08:	4c 8b 7d c8          	mov    -0x38(%rbp),%r15
   35f0c:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
   35f10:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   35f14:	4c 89 e6             	mov    %r12,%rsi
   35f17:	4c 89 fa             	mov    %r15,%rdx
   35f1a:	4d 89 f0             	mov    %r14,%r8
   35f1d:	ff 15 5d 3c 04 00    	call   *0x43c5d(%rip)        # 79b80 <_GLOBAL_OFFSET_TABLE_+0x470>
   35f23:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   35f27:	48 89 43 10          	mov    %rax,0x10(%rbx)
   35f2b:	0f 10 45 c0          	movups -0x40(%rbp),%xmm0
   35f2f:	0f 11 03             	movups %xmm0,(%rbx)
   35f32:	4d 85 ed             	test   %r13,%r13
   35f35:	74 16                	je     35f4d <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   35f37:	4c 89 ff             	mov    %r15,%rdi
   35f3a:	ff 15 28 38 04 00    	call   *0x43828(%rip)        # 79768 <free@GLIBC_2.2.5>
   35f40:	eb 0b                	jmp    35f4d <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   35f42:	c6 43 08 01          	movb   $0x1,0x8(%rbx)
   35f46:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   35f4d:	48 89 d8             	mov    %rbx,%rax
   35f50:	48 83 c4 18          	add    $0x18,%rsp
   35f54:	5b                   	pop    %rbx
   35f55:	41 5c                	pop    %r12
   35f57:	41 5d                	pop    %r13
   35f59:	41 5e                	pop    %r14
   35f5b:	41 5f                	pop    %r15
   35f5d:	5d                   	pop    %rbp
   35f5e:	c3                   	ret
   35f5f:	48 89 c3             	mov    %rax,%rbx
   35f62:	4d 85 ed             	test   %r13,%r13
   35f65:	74 09                	je     35f70 <<star_core::serv::enclave::Enclave>::issue_token+0xd0>
   35f67:	4c 89 ff             	mov    %r15,%rdi
   35f6a:	ff 15 f8 37 04 00    	call   *0x437f8(%rip)        # 79768 <free@GLIBC_2.2.5>
   35f70:	48 89 df             	mov    %rbx,%rdi
   35f73:	e8 c8 00 fd ff       	call   6040 <_Unwind_Resume@plt>
   35f78:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   35f7f:	00 

0000000000035f80 <<star_core::serv::enclave::Enclave>::issue_token_plain>:
   35f80:	55                   	push   %rbp
   35f81:	48 89 e5             	mov    %rsp,%rbp
   35f84:	41 57                	push   %r15
   35f86:	41 56                	push   %r14
   35f88:	41 54                	push   %r12
   35f8a:	53                   	push   %rbx
   35f8b:	48 81 ec b0 00 00 00 	sub    $0xb0,%rsp
   35f92:	48 89 fb             	mov    %rdi,%rbx
   35f95:	48 83 f9 50          	cmp    $0x50,%rcx
   35f99:	72 62                	jb     35ffd <<star_core::serv::enclave::Enclave>::issue_token_plain+0x7d>
   35f9b:	49 89 f1             	mov    %rsi,%r9
   35f9e:	0f 10 42 20          	movups 0x20(%rdx),%xmm0
   35fa2:	0f 10 4a 30          	movups 0x30(%rdx),%xmm1
   35fa6:	0f 29 8d 60 ff ff ff 	movaps %xmm1,-0xa0(%rbp)
   35fad:	0f 29 85 50 ff ff ff 	movaps %xmm0,-0xb0(%rbp)
   35fb4:	f3 0f 6f 02          	movdqu (%rdx),%xmm0
   35fb8:	f3 0f 6f 4a 10       	movdqu 0x10(%rdx),%xmm1
   35fbd:	66 0f 7f 85 30 ff ff 	movdqa %xmm0,-0xd0(%rbp)
   35fc4:	ff 
   35fc5:	66 0f 7f 8d 40 ff ff 	movdqa %xmm1,-0xc0(%rbp)
   35fcc:	ff 
   35fcd:	4c 8b 72 40          	mov    0x40(%rdx),%r14
   35fd1:	49 89 d4             	mov    %rdx,%r12
   35fd4:	4c 8b 7a 48          	mov    0x48(%rdx),%r15
   35fd8:	48 8b 46 58          	mov    0x58(%rsi),%rax
   35fdc:	49 83 c1 60          	add    $0x60,%r9
   35fe0:	48 8d bd 30 ff ff ff 	lea    -0xd0(%rbp),%rdi
   35fe7:	4c 89 fe             	mov    %r15,%rsi
   35fea:	4c 89 f2             	mov    %r14,%rdx
   35fed:	4c 89 c1             	mov    %r8,%rcx
   35ff0:	49 89 c0             	mov    %rax,%r8
   35ff3:	ff 15 1f 3c 04 00    	call   *0x43c1f(%rip)        # 79c18 <_GLOBAL_OFFSET_TABLE_+0x508>
   35ff9:	84 c0                	test   %al,%al
   35ffb:	75 3d                	jne    3603a <<star_core::serv::enclave::Enclave>::issue_token_plain+0xba>
   35ffd:	bf 01 00 00 00       	mov    $0x1,%edi
   36002:	ff 15 68 3c 04 00    	call   *0x43c68(%rip)        # 79c70 <malloc@GLIBC_2.2.5>
   36008:	48 85 c0             	test   %rax,%rax
   3600b:	0f 84 1c 01 00 00    	je     3612d <<star_core::serv::enclave::Enclave>::issue_token_plain+0x1ad>
   36011:	c6 00 00             	movb   $0x0,(%rax)
   36014:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   3601b:	48 89 43 08          	mov    %rax,0x8(%rbx)
   3601f:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   36026:	00 
   36027:	48 89 d8             	mov    %rbx,%rax
   3602a:	48 81 c4 b0 00 00 00 	add    $0xb0,%rsp
   36031:	5b                   	pop    %rbx
   36032:	41 5c                	pop    %r12
   36034:	41 5e                	pop    %r14
   36036:	41 5f                	pop    %r15
   36038:	5d                   	pop    %rbp
   36039:	c3                   	ret
   3603a:	41 0f 10 04 24       	movups (%r12),%xmm0
   3603f:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   36045:	0f 11 4d b8          	movups %xmm1,-0x48(%rbp)
   36049:	0f 11 45 a8          	movups %xmm0,-0x58(%rbp)
   3604d:	0f 10 05 cc ff 02 00 	movups 0x2ffcc(%rip),%xmm0        # 66020 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x140>
   36054:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
   3605b:	f3 0f 6f 05 cd ff 02 	movdqu 0x2ffcd(%rip),%xmm0        # 66030 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x150>
   36062:	00 
   36063:	66 0f 7f 45 80       	movdqa %xmm0,-0x80(%rbp)
   36068:	48 c7 45 90 00 00 00 	movq   $0x0,-0x70(%rbp)
   3606f:	00 
   36070:	48 8d 75 98          	lea    -0x68(%rbp),%rsi
   36074:	4c 89 7d 98          	mov    %r15,-0x68(%rbp)
   36078:	4c 89 75 a0          	mov    %r14,-0x60(%rbp)
   3607c:	c6 45 d8 30          	movb   $0x30,-0x28(%rbp)
   36080:	c6 45 c8 80          	movb   $0x80,-0x38(%rbp)
   36084:	c7 45 c9 00 00 00 00 	movl   $0x0,-0x37(%rbp)
   3608b:	c7 45 cc 00 00 00 00 	movl   $0x0,-0x34(%rbp)
   36092:	48 b8 00 00 00 00 00 	movabs $0x8001000000000000,%rax
   36099:	00 01 80 
   3609c:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   360a0:	48 8d bd 70 ff ff ff 	lea    -0x90(%rbp),%rdi
   360a7:	ba 01 00 00 00       	mov    $0x1,%edx
   360ac:	ff 15 26 3c 04 00    	call   *0x43c26(%rip)        # 79cd8 <_GLOBAL_OFFSET_TABLE_+0x5c8>
   360b2:	66 0f ef c0          	pxor   %xmm0,%xmm0
   360b6:	66 0f 6f 8d 70 ff ff 	movdqa -0x90(%rbp),%xmm1
   360bd:	ff 
   360be:	66 0f 6f 55 80       	movdqa -0x80(%rbp),%xmm2
   360c3:	66 0f 6f d9          	movdqa %xmm1,%xmm3
   360c7:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   360cb:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   360d0:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   360d5:	66 0f 60 c8          	punpcklbw %xmm0,%xmm1
   360d9:	f2 0f 70 c9 1b       	pshuflw $0x1b,%xmm1,%xmm1
   360de:	f3 0f 70 c9 1b       	pshufhw $0x1b,%xmm1,%xmm1
   360e3:	66 0f 67 cb          	packuswb %xmm3,%xmm1
   360e7:	66 0f 6f da          	movdqa %xmm2,%xmm3
   360eb:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   360ef:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   360f4:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   360f9:	66 0f 60 d0          	punpcklbw %xmm0,%xmm2
   360fd:	f2 0f 70 c2 1b       	pshuflw $0x1b,%xmm2,%xmm0
   36102:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   36107:	66 0f 67 c3          	packuswb %xmm3,%xmm0
   3610b:	66 0f 7f 8d 70 ff ff 	movdqa %xmm1,-0x90(%rbp)
   36112:	ff 
   36113:	66 0f 7f 45 80       	movdqa %xmm0,-0x80(%rbp)
   36118:	48 8d b5 70 ff ff ff 	lea    -0x90(%rbp),%rsi
   3611f:	48 89 df             	mov    %rbx,%rdi
   36122:	ff 15 68 3d 04 00    	call   *0x43d68(%rip)        # 79e90 <_GLOBAL_OFFSET_TABLE_+0x780>
   36128:	e9 fa fe ff ff       	jmp    36027 <<star_core::serv::enclave::Enclave>::issue_token_plain+0xa7>
   3612d:	bf 01 00 00 00       	mov    $0x1,%edi
   36132:	be 01 00 00 00       	mov    $0x1,%esi
   36137:	ff 15 83 3c 04 00    	call   *0x43c83(%rip)        # 79dc0 <_GLOBAL_OFFSET_TABLE_+0x6b0>
   3613d:	0f 1f 00             	nopl   (%rax)

0000000000036140 <<star_core::serv::core::cred::Credential>::is_valid>:
   36140:	55                   	push   %rbp
   36141:	48 89 e5             	mov    %rsp,%rbp
   36144:	41 57                	push   %r15
   36146:	41 56                	push   %r14
   36148:	41 55                	push   %r13
   3614a:	41 54                	push   %r12
   3614c:	53                   	push   %rbx
   3614d:	48 81 ec e8 02 00 00 	sub    $0x2e8,%rsp
   36154:	4c 89 4d c8          	mov    %r9,-0x38(%rbp)
   36158:	4d 89 c5             	mov    %r8,%r13
   3615b:	49 89 d7             	mov    %rdx,%r15
   3615e:	48 89 7d c0          	mov    %rdi,-0x40(%rbp)
   36162:	48 89 d0             	mov    %rdx,%rax
   36165:	48 c1 e8 08          	shr    $0x8,%rax
   36169:	48 89 85 68 ff ff ff 	mov    %rax,-0x98(%rbp)
   36170:	48 89 d0             	mov    %rdx,%rax
   36173:	48 c1 e8 10          	shr    $0x10,%rax
   36177:	48 89 85 70 ff ff ff 	mov    %rax,-0x90(%rbp)
   3617e:	48 89 d0             	mov    %rdx,%rax
   36181:	48 c1 e8 18          	shr    $0x18,%rax
   36185:	48 89 85 78 ff ff ff 	mov    %rax,-0x88(%rbp)
   3618c:	48 89 d0             	mov    %rdx,%rax
   3618f:	48 c1 e8 20          	shr    $0x20,%rax
   36193:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
   36197:	48 89 d0             	mov    %rdx,%rax
   3619a:	48 c1 e8 28          	shr    $0x28,%rax
   3619e:	48 89 45 88          	mov    %rax,-0x78(%rbp)
   361a2:	48 89 d0             	mov    %rdx,%rax
   361a5:	48 c1 e8 30          	shr    $0x30,%rax
   361a9:	48 89 45 90          	mov    %rax,-0x70(%rbp)
   361ad:	48 89 d0             	mov    %rdx,%rax
   361b0:	48 c1 e8 38          	shr    $0x38,%rax
   361b4:	48 89 45 a0          	mov    %rax,-0x60(%rbp)
   361b8:	4c 89 c3             	mov    %r8,%rbx
   361bb:	48 c1 eb 08          	shr    $0x8,%rbx
   361bf:	48 89 9d b8 fe ff ff 	mov    %rbx,-0x148(%rbp)
   361c6:	4d 89 c4             	mov    %r8,%r12
   361c9:	49 c1 ec 10          	shr    $0x10,%r12
   361cd:	4d 89 c6             	mov    %r8,%r14
   361d0:	49 c1 ee 18          	shr    $0x18,%r14
   361d4:	4c 89 c0             	mov    %r8,%rax
   361d7:	48 c1 e8 20          	shr    $0x20,%rax
   361db:	48 89 45 80          	mov    %rax,-0x80(%rbp)
   361df:	4c 89 c0             	mov    %r8,%rax
   361e2:	48 c1 e8 28          	shr    $0x28,%rax
   361e6:	48 89 45 98          	mov    %rax,-0x68(%rbp)
   361ea:	4c 89 c0             	mov    %r8,%rax
   361ed:	48 c1 e8 30          	shr    $0x30,%rax
   361f1:	48 89 45 a8          	mov    %rax,-0x58(%rbp)
   361f5:	4c 89 c0             	mov    %r8,%rax
   361f8:	48                   	rex.W
   361f9:	c1                   	.byte 0xc1
   361fa:	e8                   	.byte 0xe8

=== A=0x48C37D0 F=0x8c37d0 ===

./target/release/analysis:     file format elf64-x86-64


