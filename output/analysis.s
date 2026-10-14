=== VALGRIND OUTPUT ===
==2555== Memcheck, a memory error detector
==2555== Copyright (C) 2002-2026, and GNU GPL'd, by Julian Seward et al.
==2555== Using Valgrind-3.27.1 and LibVEX; rerun with -h for copyright info
==2555== Command: ./target/release/analysis
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CC0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400C601: <star_core::secure_channel::client::SecureChannelClient>::send_hpke::<analysis::rng::PoisonedRng> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E6D3: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CC0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403C809: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400EDB8: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4022670: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403DCD3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400EDB8: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x403C5F4: <star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403BDDE: <star_core::client::User>::receive_credential (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F258: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CC0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400C601: <star_core::secure_channel::client::SecureChannelClient>::send_hpke::<analysis::rng::PoisonedRng> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F2AD: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CC0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403C809: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403E289: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F3BF: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4022670: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403DCD3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403E289: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F3BF: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x403E38B: <star_core::serv::enclave::Enclave>::issue_token_plain (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403E2B2: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F3BF: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CC0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F7EE: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CC0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403C809: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4011F44: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4022670: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403DCD3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4011F44: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
done
==2555== 
==2555== HEAP SUMMARY:
==2555==     in use at exit: 544 bytes in 1 blocks
==2555==   total heap usage: 93 allocs, 92 frees, 7,136 bytes allocated
==2555== 
==2555== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==2555==    at 0x48CD7D0: malloc (in /nix/store/mf7rzpknxwg0q1wf0nm8gfzikapg00h1-valgrind-3.27.1/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==2555==    by 0x40690C7: <alloc::collections::btree::map::entry::VacantEntry<usize, std::sys::pal::unix::stack_overflow::thread_info::ThreadInfo>>::insert_entry (in /workspace/star/core/target/release/analysis)
==2555==    by 0x405A163: std::sys::pal::unix::stack_overflow::thread_info::set_current_info (in /workspace/star/core/target/release/analysis)
==2555==    by 0x405AD8F: std::sys::pal::unix::stack_overflow::imp::init (in /workspace/star/core/target/release/analysis)
==2555==    by 0x406958F: std::sys::pal::unix::init (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055DA0: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AF3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== LEAK SUMMARY:
==2555==    definitely lost: 0 bytes in 0 blocks
==2555==    indirectly lost: 0 bytes in 0 blocks
==2555==      possibly lost: 0 bytes in 0 blocks
==2555==    still reachable: 544 bytes in 1 blocks
==2555==         suppressed: 0 bytes in 0 blocks
==2555== 
==2555== Use --track-origins=yes to see where uninitialised values come from
==2555== For lists of detected and suppressed errors, rerun with: -s
==2555== ERROR SUMMARY: 11 errors from 11 contexts (suppressed: 0 from 0)

=== A=0x4024CC0 F=0x24cc0 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000024ac0 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x140>:
   24ac0:	7d aa                	jge    24a6c <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0xec>
   24ac2:	00 40 0f             	add    %al,0xf(%rax)
   24ac5:	94                   	xchg   %eax,%esp
   24ac6:	c7                   	(bad)
   24ac7:	ff 15 bb f1 05 00    	call   *0x5f1bb(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24acd:	88 45 c8             	mov    %al,-0x38(%rbp)
   24ad0:	31 ff                	xor    %edi,%edi
   24ad2:	80 7d ab 00          	cmpb   $0x0,-0x55(%rbp)
   24ad6:	40 0f 94 c7          	sete   %dil
   24ada:	ff 15 a8 f1 05 00    	call   *0x5f1a8(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24ae0:	88 45 c9             	mov    %al,-0x37(%rbp)
   24ae3:	31 ff                	xor    %edi,%edi
   24ae5:	80 7d ac 00          	cmpb   $0x0,-0x54(%rbp)
   24ae9:	40 0f 94 c7          	sete   %dil
   24aed:	ff 15 95 f1 05 00    	call   *0x5f195(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24af3:	88 45 ca             	mov    %al,-0x36(%rbp)
   24af6:	31 ff                	xor    %edi,%edi
   24af8:	80 7d ad 00          	cmpb   $0x0,-0x53(%rbp)
   24afc:	40 0f 94 c7          	sete   %dil
   24b00:	ff 15 82 f1 05 00    	call   *0x5f182(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b06:	88 45 cb             	mov    %al,-0x35(%rbp)
   24b09:	31 ff                	xor    %edi,%edi
   24b0b:	80 7d ae 00          	cmpb   $0x0,-0x52(%rbp)
   24b0f:	40 0f 94 c7          	sete   %dil
   24b13:	ff 15 6f f1 05 00    	call   *0x5f16f(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b19:	41 89 c6             	mov    %eax,%r14d
   24b1c:	31 ff                	xor    %edi,%edi
   24b1e:	80 7d af 00          	cmpb   $0x0,-0x51(%rbp)
   24b22:	40 0f 94 c7          	sete   %dil
   24b26:	ff 15 5c f1 05 00    	call   *0x5f15c(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b2c:	88 45 cc             	mov    %al,-0x34(%rbp)
   24b2f:	31 ff                	xor    %edi,%edi
   24b31:	80 7d b0 00          	cmpb   $0x0,-0x50(%rbp)
   24b35:	40 0f 94 c7          	sete   %dil
   24b39:	ff 15 49 f1 05 00    	call   *0x5f149(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b3f:	88 45 cd             	mov    %al,-0x33(%rbp)
   24b42:	31 ff                	xor    %edi,%edi
   24b44:	80 7d b1 00          	cmpb   $0x0,-0x4f(%rbp)
   24b48:	40 0f 94 c7          	sete   %dil
   24b4c:	ff 15 36 f1 05 00    	call   *0x5f136(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b52:	88 45 ce             	mov    %al,-0x32(%rbp)
   24b55:	31 ff                	xor    %edi,%edi
   24b57:	80 7d b2 00          	cmpb   $0x0,-0x4e(%rbp)
   24b5b:	40 0f 94 c7          	sete   %dil
   24b5f:	ff 15 23 f1 05 00    	call   *0x5f123(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b65:	88 45 cf             	mov    %al,-0x31(%rbp)
   24b68:	31 ff                	xor    %edi,%edi
   24b6a:	80 7d b3 00          	cmpb   $0x0,-0x4d(%rbp)
   24b6e:	40 0f 94 c7          	sete   %dil
   24b72:	ff 15 10 f1 05 00    	call   *0x5f110(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b78:	88 45 d0             	mov    %al,-0x30(%rbp)
   24b7b:	31 ff                	xor    %edi,%edi
   24b7d:	80 7d b4 00          	cmpb   $0x0,-0x4c(%rbp)
   24b81:	40 0f 94 c7          	sete   %dil
   24b85:	ff 15 fd f0 05 00    	call   *0x5f0fd(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b8b:	41 89 c4             	mov    %eax,%r12d
   24b8e:	31 ff                	xor    %edi,%edi
   24b90:	80 7d b5 00          	cmpb   $0x0,-0x4b(%rbp)
   24b94:	40 0f 94 c7          	sete   %dil
   24b98:	ff 15 ea f0 05 00    	call   *0x5f0ea(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b9e:	88 45 d1             	mov    %al,-0x2f(%rbp)
   24ba1:	31 ff                	xor    %edi,%edi
   24ba3:	80 7d b6 00          	cmpb   $0x0,-0x4a(%rbp)
   24ba7:	40 0f 94 c7          	sete   %dil
   24bab:	ff 15 d7 f0 05 00    	call   *0x5f0d7(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bb1:	88 45 d2             	mov    %al,-0x2e(%rbp)
   24bb4:	31 ff                	xor    %edi,%edi
   24bb6:	80 7d b7 00          	cmpb   $0x0,-0x49(%rbp)
   24bba:	40 0f 94 c7          	sete   %dil
   24bbe:	ff 15 c4 f0 05 00    	call   *0x5f0c4(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bc4:	88 45 d3             	mov    %al,-0x2d(%rbp)
   24bc7:	31 ff                	xor    %edi,%edi
   24bc9:	80 7d b8 00          	cmpb   $0x0,-0x48(%rbp)
   24bcd:	40 0f 94 c7          	sete   %dil
   24bd1:	ff 15 b1 f0 05 00    	call   *0x5f0b1(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bd7:	88 45 d4             	mov    %al,-0x2c(%rbp)
   24bda:	31 ff                	xor    %edi,%edi
   24bdc:	80 7d b9 00          	cmpb   $0x0,-0x47(%rbp)
   24be0:	40 0f 94 c7          	sete   %dil
   24be4:	ff 15 9e f0 05 00    	call   *0x5f09e(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bea:	88 45 d5             	mov    %al,-0x2b(%rbp)
   24bed:	31 ff                	xor    %edi,%edi
   24bef:	80 7d ba 00          	cmpb   $0x0,-0x46(%rbp)
   24bf3:	40 0f 94 c7          	sete   %dil
   24bf7:	ff 15 8b f0 05 00    	call   *0x5f08b(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bfd:	88 45 d6             	mov    %al,-0x2a(%rbp)
   24c00:	31 ff                	xor    %edi,%edi
   24c02:	80 7d bb 00          	cmpb   $0x0,-0x45(%rbp)
   24c06:	40 0f 94 c7          	sete   %dil
   24c0a:	ff 15 78 f0 05 00    	call   *0x5f078(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24c10:	41 89 c5             	mov    %eax,%r13d
   24c13:	31 ff                	xor    %edi,%edi
   24c15:	80 7d bc 00          	cmpb   $0x0,-0x44(%rbp)
   24c19:	40 0f 94 c7          	sete   %dil
   24c1d:	ff 15 65 f0 05 00    	call   *0x5f065(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24c23:	88 45 d7             	mov    %al,-0x29(%rbp)
   24c26:	31 ff                	xor    %edi,%edi
   24c28:	80 7d bd 00          	cmpb   $0x0,-0x43(%rbp)
   24c2c:	40 0f 94 c7          	sete   %dil
   24c30:	ff 15 52 f0 05 00    	call   *0x5f052(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24c36:	0f b6 4d be          	movzbl -0x42(%rbp),%ecx
   24c3a:	22 4d bf             	and    -0x41(%rbp),%cl
   24c3d:	0f b6 55 c0          	movzbl -0x40(%rbp),%edx
   24c41:	22 55 c1             	and    -0x3f(%rbp),%dl
   24c44:	20 ca                	and    %cl,%dl
   24c46:	0f b6 4d c2          	movzbl -0x3e(%rbp),%ecx
   24c4a:	22 4d c3             	and    -0x3d(%rbp),%cl
   24c4d:	22 4d c4             	and    -0x3c(%rbp),%cl
   24c50:	20 d1                	and    %dl,%cl
   24c52:	44 22 7d c5          	and    -0x3b(%rbp),%r15b
   24c56:	44 22 7d c6          	and    -0x3a(%rbp),%r15b
   24c5a:	44 22 7d c7          	and    -0x39(%rbp),%r15b
   24c5e:	41 20 cf             	and    %cl,%r15b
   24c61:	22 5d c8             	and    -0x38(%rbp),%bl
   24c64:	22 5d c9             	and    -0x37(%rbp),%bl
   24c67:	22 5d ca             	and    -0x36(%rbp),%bl
   24c6a:	22 5d cb             	and    -0x35(%rbp),%bl
   24c6d:	44 20 fb             	and    %r15b,%bl
   24c70:	44 22 75 cc          	and    -0x34(%rbp),%r14b
   24c74:	44 22 75 cd          	and    -0x33(%rbp),%r14b
   24c78:	44 22 75 ce          	and    -0x32(%rbp),%r14b
   24c7c:	44 22 75 cf          	and    -0x31(%rbp),%r14b
   24c80:	44 22 75 d0          	and    -0x30(%rbp),%r14b
   24c84:	41 20 de             	and    %bl,%r14b
   24c87:	44 22 65 d1          	and    -0x2f(%rbp),%r12b
   24c8b:	44 22 65 d2          	and    -0x2e(%rbp),%r12b
   24c8f:	44 22 65 d3          	and    -0x2d(%rbp),%r12b
   24c93:	44 22 65 d4          	and    -0x2c(%rbp),%r12b
   24c97:	44 22 65 d5          	and    -0x2b(%rbp),%r12b
   24c9b:	44 22 65 d6          	and    -0x2a(%rbp),%r12b
   24c9f:	45 20 f4             	and    %r14b,%r12b
   24ca2:	44 22 6d d7          	and    -0x29(%rbp),%r13b
   24ca6:	41 20 c5             	and    %al,%r13b
   24ca9:	45 20 e5             	and    %r12b,%r13b
   24cac:	41 80 e5 01          	and    $0x1,%r13b
   24cb0:	41 0f b6 fd          	movzbl %r13b,%edi
   24cb4:	ff 15 ce ef 05 00    	call   *0x5efce(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24cba:	84 c0                	test   %al,%al
   24cbc:	48 8b 5d 90          	mov    -0x70(%rbp),%rbx
   24cc0:	74 0d                	je     24ccf <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x34f>
   24cc2:	c6 43 08 03          	movb   $0x3,0x8(%rbx)
   24cc6:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   24ccd:	eb 32                	jmp    24d01 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x381>
   24ccf:	bf 20 00 00 00       	mov    $0x20,%edi
   24cd4:	ff 15 96 ef 05 00    	call   *0x5ef96(%rip)        # 83c70 <malloc@GLIBC_2.2.5>
   24cda:	48 85 c0             	test   %rax,%rax
   24cdd:	74 3e                	je     24d1d <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x39d>
   24cdf:	0f 10 45 9e          	movups -0x62(%rbp),%xmm0
   24ce3:	0f 10 4d ae          	movups -0x52(%rbp),%xmm1
   24ce7:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   24ceb:	0f 11 00             	movups %xmm0,(%rax)
   24cee:	48 c7 03 20 00 00 00 	movq   $0x20,(%rbx)
   24cf5:	48 89 43 08          	mov    %rax,0x8(%rbx)
   24cf9:	48 c7 43 10 20 00 00 	movq   $0x20,0x10(%rbx)
   24d00:	00 
   24d01:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   24d05:	ff 15 1d ef 05 00    	call   *0x5ef1d(%rip)        # 83c28 <_GLOBAL_OFFSET_TABLE_+0x530>
   24d0b:	48 89 d8             	mov    %rbx,%rax
   24d0e:	48 83 c4 68          	add    $0x68,%rsp
   24d12:	5b                   	pop    %rbx
   24d13:	41 5c                	pop    %r12
   24d15:	41 5d                	pop    %r13
   24d17:	41 5e                	pop    %r14
   24d19:	41 5f                	pop    %r15
   24d1b:	5d                   	pop    %rbp
   24d1c:	c3                   	ret
   24d1d:	bf 01 00 00 00       	mov    $0x1,%edi
   24d22:	be 20 00 00 00       	mov    $0x20,%esi
   24d27:	ff 15 43 f1 05 00    	call   *0x5f143(%rip)        # 83e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   24d2d:	0f 0b                	ud2
   24d2f:	eb 00                	jmp    24d31 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x3b1>
   24d31:	48 89 c3             	mov    %rax,%rbx
   24d34:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   24d38:	ff 15 ea ee 05 00    	call   *0x5eeea(%rip)        # 83c28 <_GLOBAL_OFFSET_TABLE_+0x530>
   24d3e:	48 89 df             	mov    %rbx,%rdi
   24d41:	e8 fa 12 fe ff       	call   6040 <_Unwind_Resume@plt>
   24d46:	ff 15 7c f2 05 00    	call   *0x5f27c(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   24d4c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000024d50 <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed>:
   24d50:	55                   	push   %rbp
   24d51:	48 89 e5             	mov    %rsp,%rbp
   24d54:	41 57                	push   %r15
   24d56:	41 56                	push   %r14
   24d58:	41 55                	push   %r13
   24d5a:	41 54                	push   %r12
   24d5c:	53                   	push   %rbx
   24d5d:	48 81 ec d8 00 00 00 	sub    $0xd8,%rsp
   24d64:	48 89 fb             	mov    %rdi,%rbx
   24d67:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   24d6e:	ba 90 00 00 00       	mov    $0x90,%edx
   24d73:	ff 15 1f ee 05 00    	call   *0x5ee1f(%rip)        # 83b98 <memcpy@GLIBC_2.14>
   24d79:	4c 8d b5 40 ff ff ff 	lea    -0xc0(%rbp),%r14
   24d80:	44 0f b6 a5 3f ff ff 	movzbl -0xc1(%rbp),%r12d
   24d87:	ff 
   24d88:	4c 8b ad 60 ff ff ff 	mov    -0xa0(%rbp),%r13
   24d8f:	4d 89 e7             	mov    %r12,%r15
   24d92:	49 83 f7 3f          	xor    $0x3f,%r15
   24d96:	42 c6 84 25 00 ff ff 	movb   $0x80,-0x100(%rbp,%r12,1)
   24d9d:	ff 80 
   24d9f:	49 8d 3c 2c          	lea    (%r12,%rbp,1),%rdi
   24da3:	48 81 c7 01 ff ff ff 	add    $0xffffffffffffff01,%rdi
   24daa:	31 f6                	xor    %esi,%esi
   24dac:	4c 89 fa             	mov    %r15,%rdx
   24daf:	ff 15 2b ec 05 00    	call   *0x5ec2b(%rip)        # 839e0 <memset@GLIBC_2.2.5>
   24db5:	49 83 ff 07          	cmp    $0x7,%r15
   24db9:	77 63                	ja     24e1e <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed+0xce>
   24dbb:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   24dc2:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   24dc9:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   24dd0:	66 0f 6f 9d 30 ff ff 	movdqa -0xd0(%rbp),%xmm3
   24dd7:	ff 
   24dd8:	66 0f 7f 5d c0       	movdqa %xmm3,-0x40(%rbp)
   24ddd:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   24de1:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   24de5:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   24de9:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   24ded:	ba 01 00 00 00       	mov    $0x1,%edx
   24df2:	4c 89 f7             	mov    %r14,%rdi
   24df5:	ff 15 a5 ee 05 00    	call   *0x5eea5(%rip)        # 83ca0 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   24dfb:	0f 57 c0             	xorps  %xmm0,%xmm0
   24dfe:	0f 29 85 00 ff ff ff 	movaps %xmm0,-0x100(%rbp)
   24e05:	0f 29 85 10 ff ff ff 	movaps %xmm0,-0xf0(%rbp)
   24e0c:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   24e13:	48 c7 85 30 ff ff ff 	movq   $0x0,-0xd0(%rbp)
   24e1a:	00 00 00 00 
   24e1e:	49 c1 e5 09          	shl    $0x9,%r13
   24e22:	41 c1 e4 03          	shl    $0x3,%r12d
   24e26:	4d 09 ec             	or     %r13,%r12
   24e29:	49 0f cc             	bswap  %r12
   24e2c:	4c 89 a5 38 ff ff ff 	mov    %r12,-0xc8(%rbp)
   24e33:	48 8b 85 30 ff ff ff 	mov    -0xd0(%rbp),%rax
   24e3a:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   24e3e:	48 8b 85 38 ff ff ff 	mov    -0xc8(%rbp),%rax
   24e45:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   24e49:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   24e50:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   24e57:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   24e5e:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   24e62:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   24e66:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   24e6a:	4c 8d 3d 5f bb 00 00 	lea    0xbb5f(%rip),%r15        # 309d0 <sha2::sha256::compress256>
   24e71:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   24e75:	ba 01 00 00 00       	mov    $0x1,%edx
   24e7a:	4c 89 f7             	mov    %r14,%rdi
   24e7d:	41 ff d7             	call   *%r15
   24e80:	48 8d bd 68 ff ff ff 	lea    -0x98(%rbp),%rdi
   24e87:	66 0f 6f 85 40 ff ff 	movdqa -0xc0(%rbp),%xmm0
   24e8e:	ff 
   24e8f:	66 0f 6f 8d 50 ff ff 	movdqa -0xb0(%rbp),%xmm1
   24e96:	ff 
   24e97:	66 0f ef db          	pxor   %xmm3,%xmm3
   24e9b:	66 0f 6f d0          	movdqa %xmm0,%xmm2
   24e9f:	66 0f 68 d3          	punpckhbw %xmm3,%xmm2
   24ea3:	f2 0f 70 d2 1b       	pshuflw $0x1b,%xmm2,%xmm2
   24ea8:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   24ead:	66 0f 60 c3          	punpcklbw %xmm3,%xmm0
   24eb1:	f2 0f 70 c0 1b       	pshuflw $0x1b,%xmm0,%xmm0
   24eb6:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   24ebb:	66 0f 67 c2          	packuswb %xmm2,%xmm0
   24ebf:	66                   	data16

=== A=0x4022670 F=0x22670 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000022470 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x520>:
   22470:	4c 89 f6             	mov    %r14,%rsi
   22473:	e8 f8 7a 00 00       	call   29f70 <<poly1305::backend::autodetect::State as universal_hash::UniversalHash>::finalize>
   22478:	0f b6 44 24 50       	movzbl 0x50(%rsp),%eax
   2247d:	31 ff                	xor    %edi,%edi
   2247f:	41 3a 04 1f          	cmp    (%r15,%rbx,1),%al
   22483:	40 0f 94 c7          	sete   %dil
   22487:	ff 15 fb 17 06 00    	call   *0x617fb(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2248d:	88 44 24 40          	mov    %al,0x40(%rsp)
   22491:	0f b6 44 24 51       	movzbl 0x51(%rsp),%eax
   22496:	31 ff                	xor    %edi,%edi
   22498:	41 3a 44 1f 01       	cmp    0x1(%r15,%rbx,1),%al
   2249d:	40 0f 94 c7          	sete   %dil
   224a1:	ff 15 e1 17 06 00    	call   *0x617e1(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   224a7:	88 44 24 38          	mov    %al,0x38(%rsp)
   224ab:	0f b6 44 24 52       	movzbl 0x52(%rsp),%eax
   224b0:	31 ff                	xor    %edi,%edi
   224b2:	41 3a 44 1f 02       	cmp    0x2(%r15,%rbx,1),%al
   224b7:	40 0f 94 c7          	sete   %dil
   224bb:	ff 15 c7 17 06 00    	call   *0x617c7(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   224c1:	88 44 24 48          	mov    %al,0x48(%rsp)
   224c5:	0f b6 44 24 53       	movzbl 0x53(%rsp),%eax
   224ca:	31 ff                	xor    %edi,%edi
   224cc:	41 3a 44 1f 03       	cmp    0x3(%r15,%rbx,1),%al
   224d1:	40 0f 94 c7          	sete   %dil
   224d5:	ff 15 ad 17 06 00    	call   *0x617ad(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   224db:	88 44 24 27          	mov    %al,0x27(%rsp)
   224df:	0f b6 44 24 54       	movzbl 0x54(%rsp),%eax
   224e4:	31 ff                	xor    %edi,%edi
   224e6:	41 3a 44 1f 04       	cmp    0x4(%r15,%rbx,1),%al
   224eb:	40 0f 94 c7          	sete   %dil
   224ef:	ff 15 93 17 06 00    	call   *0x61793(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   224f5:	41 89 c5             	mov    %eax,%r13d
   224f8:	0f b6 44 24 55       	movzbl 0x55(%rsp),%eax
   224fd:	31 ff                	xor    %edi,%edi
   224ff:	41 3a 44 1f 05       	cmp    0x5(%r15,%rbx,1),%al
   22504:	40 0f 94 c7          	sete   %dil
   22508:	ff 15 7a 17 06 00    	call   *0x6177a(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2250e:	88 44 24 26          	mov    %al,0x26(%rsp)
   22512:	0f b6 44 24 56       	movzbl 0x56(%rsp),%eax
   22517:	31 ff                	xor    %edi,%edi
   22519:	41 3a 44 1f 06       	cmp    0x6(%r15,%rbx,1),%al
   2251e:	40 0f 94 c7          	sete   %dil
   22522:	ff 15 60 17 06 00    	call   *0x61760(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22528:	88 44 24 25          	mov    %al,0x25(%rsp)
   2252c:	0f b6 44 24 57       	movzbl 0x57(%rsp),%eax
   22531:	31 ff                	xor    %edi,%edi
   22533:	41 3a 44 1f 07       	cmp    0x7(%r15,%rbx,1),%al
   22538:	40 0f 94 c7          	sete   %dil
   2253c:	ff 15 46 17 06 00    	call   *0x61746(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22542:	41 89 c6             	mov    %eax,%r14d
   22545:	0f b6 44 24 58       	movzbl 0x58(%rsp),%eax
   2254a:	31 ff                	xor    %edi,%edi
   2254c:	41 3a 44 1f 08       	cmp    0x8(%r15,%rbx,1),%al
   22551:	40 0f 94 c7          	sete   %dil
   22555:	ff 15 2d 17 06 00    	call   *0x6172d(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2255b:	88 44 24 24          	mov    %al,0x24(%rsp)
   2255f:	0f b6 44 24 59       	movzbl 0x59(%rsp),%eax
   22564:	31 ff                	xor    %edi,%edi
   22566:	41 3a 44 1f 09       	cmp    0x9(%r15,%rbx,1),%al
   2256b:	40 0f 94 c7          	sete   %dil
   2256f:	ff 15 13 17 06 00    	call   *0x61713(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22575:	88 44 24 23          	mov    %al,0x23(%rsp)
   22579:	0f b6 44 24 5a       	movzbl 0x5a(%rsp),%eax
   2257e:	31 ff                	xor    %edi,%edi
   22580:	41 3a 44 1f 0a       	cmp    0xa(%r15,%rbx,1),%al
   22585:	40 0f 94 c7          	sete   %dil
   22589:	ff 15 f9 16 06 00    	call   *0x616f9(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2258f:	88 44 24 22          	mov    %al,0x22(%rsp)
   22593:	0f b6 44 24 5b       	movzbl 0x5b(%rsp),%eax
   22598:	31 ff                	xor    %edi,%edi
   2259a:	41 3a 44 1f 0b       	cmp    0xb(%r15,%rbx,1),%al
   2259f:	40 0f 94 c7          	sete   %dil
   225a3:	ff 15 df 16 06 00    	call   *0x616df(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   225a9:	41 89 c4             	mov    %eax,%r12d
   225ac:	0f b6 44 24 5c       	movzbl 0x5c(%rsp),%eax
   225b1:	31 ff                	xor    %edi,%edi
   225b3:	41 3a 44 1f 0c       	cmp    0xc(%r15,%rbx,1),%al
   225b8:	40 0f 94 c7          	sete   %dil
   225bc:	ff 15 c6 16 06 00    	call   *0x616c6(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   225c2:	88 44 24 21          	mov    %al,0x21(%rsp)
   225c6:	0f b6 44 24 5d       	movzbl 0x5d(%rsp),%eax
   225cb:	31 ff                	xor    %edi,%edi
   225cd:	41 3a 44 1f 0d       	cmp    0xd(%r15,%rbx,1),%al
   225d2:	40 0f 94 c7          	sete   %dil
   225d6:	ff 15 ac 16 06 00    	call   *0x616ac(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   225dc:	88 44 24 20          	mov    %al,0x20(%rsp)
   225e0:	0f b6 44 24 5e       	movzbl 0x5e(%rsp),%eax
   225e5:	31 ff                	xor    %edi,%edi
   225e7:	41 3a 44 1f 0e       	cmp    0xe(%r15,%rbx,1),%al
   225ec:	40 0f 94 c7          	sete   %dil
   225f0:	ff 15 92 16 06 00    	call   *0x61692(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   225f6:	88 44 24 1f          	mov    %al,0x1f(%rsp)
   225fa:	0f b6 44 24 5f       	movzbl 0x5f(%rsp),%eax
   225ff:	31 ff                	xor    %edi,%edi
   22601:	41 3a 44 1f 0f       	cmp    0xf(%r15,%rbx,1),%al
   22606:	40 0f 94 c7          	sete   %dil
   2260a:	ff 15 78 16 06 00    	call   *0x61678(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22610:	0f b6 4c 24 40       	movzbl 0x40(%rsp),%ecx
   22615:	22 4c 24 38          	and    0x38(%rsp),%cl
   22619:	0f b6 54 24 48       	movzbl 0x48(%rsp),%edx
   2261e:	22 54 24 27          	and    0x27(%rsp),%dl
   22622:	20 ca                	and    %cl,%dl
   22624:	44 22 6c 24 26       	and    0x26(%rsp),%r13b
   22629:	44 22 6c 24 25       	and    0x25(%rsp),%r13b
   2262e:	41 20 d5             	and    %dl,%r13b
   22631:	44 22 74 24 24       	and    0x24(%rsp),%r14b
   22636:	44 22 74 24 23       	and    0x23(%rsp),%r14b
   2263b:	44 22 74 24 22       	and    0x22(%rsp),%r14b
   22640:	45 20 ee             	and    %r13b,%r14b
   22643:	44 22 64 24 21       	and    0x21(%rsp),%r12b
   22648:	44 22 64 24 20       	and    0x20(%rsp),%r12b
   2264d:	44 22 64 24 1f       	and    0x1f(%rsp),%r12b
   22652:	41 20 c4             	and    %al,%r12b
   22655:	45 20 f4             	and    %r14b,%r12b
   22658:	41 80 e4 01          	and    $0x1,%r12b
   2265c:	41 0f b6 fc          	movzbl %r12b,%edi
   22660:	ff 15 22 16 06 00    	call   *0x61622(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22666:	84 c0                	test   %al,%al
   22668:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   2266f:	00 
   22670:	74 42                	je     226b4 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x764>
   22672:	4c 8d b4 24 20 05 00 	lea    0x520(%rsp),%r14
   22679:	00 
   2267a:	4c 89 f7             	mov    %r14,%rdi
   2267d:	4c 89 fe             	mov    %r15,%rsi
   22680:	48 8b 54 24 28       	mov    0x28(%rsp),%rdx
   22685:	e8 06 2d 00 00       	call   25390 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as cipher::stream::StreamCipher>::apply_keystream>
   2268a:	4c 89 f7             	mov    %r14,%rdi
   2268d:	e8 fe e5 ff ff       	call   20c90 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   22692:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   22699:	00 
   2269a:	48 8b 45 18          	mov    0x18(%rbp),%rax
   2269e:	48 8b 4c 24 28       	mov    0x28(%rsp),%rcx
   226a3:	48 39 c1             	cmp    %rax,%rcx
   226a6:	48 0f 43 c8          	cmovae %rax,%rcx
   226aa:	4d 89 7d 08          	mov    %r15,0x8(%r13)
   226ae:	49 89 4d 10          	mov    %rcx,0x10(%r13)
   226b2:	eb 22                	jmp    226d6 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x786>
   226b4:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   226bb:	00 
   226bc:	e8 cf e5 ff ff       	call   20c90 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   226c1:	4c 89 ff             	mov    %r15,%rdi
   226c4:	ff 15 86 10 06 00    	call   *0x61086(%rip)        # 83750 <free@GLIBC_2.2.5>
   226ca:	41 c6 45 08 00       	movb   $0x0,0x8(%r13)
   226cf:	48 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%rax
   226d6:	49 89 45 00          	mov    %rax,0x0(%r13)
   226da:	4c 89 e8             	mov    %r13,%rax
   226dd:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   226e1:	5b                   	pop    %rbx
   226e2:	41 5c                	pop    %r12
   226e4:	41 5d                	pop    %r13
   226e6:	41 5e                	pop    %r14
   226e8:	41 5f                	pop    %r15
   226ea:	5d                   	pop    %rbp
   226eb:	c3                   	ret
   226ec:	48 8d 15 35 f2 04 00 	lea    0x4f235(%rip),%rdx        # 71928 <core::unicode::unicode_data::cf::SHORT_OFFSET_RUNS+0x70>
   226f3:	4c 8d 0d 0e f3 05 00 	lea    0x5f30e(%rip),%r9        # 81a08 <anon.d49aa5e532bb8905898215d271d902c3.14.llvm.14046659981651887190+0x70>
   226fa:	48 8d b4 24 e0 02 00 	lea    0x2e0(%rsp),%rsi
   22701:	00 
   22702:	31 ff                	xor    %edi,%edi
   22704:	31 c9                	xor    %ecx,%ecx
   22706:	ff 15 8c 15 06 00    	call   *0x6158c(%rip)        # 83c98 <_GLOBAL_OFFSET_TABLE_+0x5a0>
   2270c:	ff 15 de 10 06 00    	call   *0x610de(%rip)        # 837f0 <_GLOBAL_OFFSET_TABLE_+0xf8>
   22712:	e9 00 f9 ff ff       	jmp    22017 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0xc7>
   22717:	bf 01 00 00 00       	mov    $0x1,%edi
   2271c:	48 89 de             	mov    %rbx,%rsi
   2271f:	ff 15 4b 17 06 00    	call   *0x6174b(%rip)        # 83e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   22725:	49 89 c6             	mov    %rax,%r14
   22728:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   2272d:	ff 15 1d 10 06 00    	call   *0x6101d(%rip)        # 83750 <free@GLIBC_2.2.5>
   22733:	4c 89 f7             	mov    %r14,%rdi
   22736:	e8 05 39 fe ff       	call   6040 <_Unwind_Resume@plt>
   2273b:	eb 06                	jmp    22743 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   2273d:	eb 04                	jmp    22743 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   2273f:	eb 02                	jmp    22743 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   22741:	eb 00                	jmp    22743 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   22743:	49 89 c6             	mov    %rax,%r14
   22746:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   2274d:	00 
   2274e:	e8 3d e5 ff ff       	call   20c90 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   22753:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   22758:	ff 15 f2 0f 06 00    	call   *0x60ff2(%rip)        # 83750 <free@GLIBC_2.2.5>
   2275e:	4c 89 f7             	mov    %r14,%rdi
   22761:	e8 da 38 fe ff       	call   6040 <_Unwind_Resume@plt>
   22766:	ff 15 5c 18 06 00    	call   *0x6185c(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   2276c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000022770 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal>:
   22770:	55                   	push   %rbp
   22771:	48 89 e5             	mov    %rsp,%rbp
   22774:	41 57                	push   %r15
   22776:	41 56                	push   %r14
   22778:	41 55                	push   %r13
   2277a:	41 54                	push   %r12
   2277c:	53                   	push   %rbx
   2277d:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   22781:	48 81 ec c0 05 00 00 	sub    $0x5c0,%rsp
   22788:	4c 89 44 24 08       	mov    %r8,0x8(%rsp)
   2278d:	48 89 8c 24 c0 02 00 	mov    %rcx,0x2c0(%rsp)
   22794:	00 
   22795:	48 83 f9 0c          	cmp    $0xc,%rcx
   22799:	0f 85 74 05 00 00    	jne    22d13 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5a3>
   2279f:	48 89 fb             	mov    %rdi,%rbx
   227a2:	48 8b 7d 18          	mov    0x18(%rbp),%rdi
   227a6:	48 83 c7 10          	add    $0x10,%rdi
   227aa:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
   227af:	0f 88 89 05 00 00    	js     22d3e <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5ce>
   227b5:	4d 89 cf             	mov    %r9,%r15
   227b8:	49 89 d6             	mov    %rdx,%r14
   227bb:	49 89 f4             	mov    %rsi,%r12
   227be:	ff 15 ac 14 06 00    	call   *0x614ac(%rip)        # 83c70 <malloc@GLIBC_2.2.5>
   227c4:	48 85 c0             	test   %rax,%rax
   227c7:	0f 84 7e 05 00 00    	je     22d4b <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5db>
   227cd:	49 89 c5             	mov    %rax,%r13
   227d0:	48 89 5c 24 28       	mov    %rbx,0x28(%rsp)
   227d5:	48 8b 5d 18          	mov    0x18(%rbp),%rbx
   227d9:	48 85 db             	test   %rbx,%rbx
   227dc:	74 10                	je     227ee <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x7e>
   227de:	48 8b 75 10          	mov    0x10(%rbp),%rsi
   227e2:	4c 89 ef             	mov    %r13,%rdi
   227e5:	48 89 da             	mov    %rbx,%rdx
   227e8:	ff 15 aa 13 06 00    	call   *0x613aa(%rip)        # 83b98 <memcpy@GLIBC_2.14>
   227ee:	0f 10 05 8b c3 04 00 	movups 0x4c38b(%rip),%xmm0        # 6eb80 <anon.10746075dd51319079efedc96a688e81.1.llvm.4247047481296140766+0xad>
   227f5:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   227fa:	41 0f 10 04 24       	movups (%r12),%xmm0
   227ff:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   22805:	0f 29 44 24 50       	movaps %xmm0,0x50(%rsp)
   2280a:	0f 29 4c 24 60       	movaps %xmm1,0x60(%rsp)
   2280f:	49 8b 06             	mov    (%r14),%rax
   22812:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
   22817:	41 8b 46 08          	mov    0x8(%r14),%eax
   2281b:	89 44 24 38          	mov    %eax,0x38(%rsp)
   2281f:	48 8d 05 ea 17 06 00 	lea    0x617ea(%rip),%rax        # 84010 <chacha20::avx2_cpuid::STORAGE>
   22826:	0f b6 00             	movzbl (%rax),%eax
   22829:	3c ff                	cmp    $0xff,%al
   2282b:	4c 89 2c 24          	mov    %r13,(%rsp)
   2282f:	49 89 de             	mov    %rbx,%r14
   22832:	0f 84 fb 04 00 00    	je     22d33 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5c3>
   22838:	0f 57 c0             	xorps  %xmm0,%xmm0
   2283b:	0f 29 84 24 f0 00 00 	movaps %xmm0,0xf0(%rsp)
   22842:	00 
   22843:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
   2284a:	00 
   2284b:	0f 29 84 24 d0 00 00 	movaps %xmm0,0xd0(%rsp)
   22852:	00 
   22853:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   2285a:	00 
   2285b:	0f 28 44 24 40       	movaps 0x40(%rsp),%xmm0
   22860:	0f 28 4c 24 50       	movaps 0x50(%rsp),%xmm1
   22865:	0f 28 54 24 60       	movaps 0x60(%rsp),%xmm2
   2286a:	0f                   	.byte 0xf
   2286b:	29                   	.byte 0x29
   2286c:	84 24 80             	test   %ah,(%rax,%rax,4)
	...

=== A=0x403C5F4 F=0x3c5f4 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000003c3f4 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x4c4>:
   3c3f4:	e8 77 db fe ff       	call   29f70 <<poly1305::backend::autodetect::State as universal_hash::UniversalHash>::finalize>
   3c3f9:	0f b6 44 24 70       	movzbl 0x70(%rsp),%eax
   3c3fe:	31 ff                	xor    %edi,%edi
   3c400:	41 3a 04 1c          	cmp    (%r12,%rbx,1),%al
   3c404:	40 0f 94 c7          	sete   %dil
   3c408:	ff 15 7a 78 04 00    	call   *0x4787a(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c40e:	88 44 24 60          	mov    %al,0x60(%rsp)
   3c412:	0f b6 44 24 71       	movzbl 0x71(%rsp),%eax
   3c417:	31 ff                	xor    %edi,%edi
   3c419:	41 3a 44 1c 01       	cmp    0x1(%r12,%rbx,1),%al
   3c41e:	40 0f 94 c7          	sete   %dil
   3c422:	ff 15 60 78 04 00    	call   *0x47860(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c428:	88 44 24 0f          	mov    %al,0xf(%rsp)
   3c42c:	0f b6 44 24 72       	movzbl 0x72(%rsp),%eax
   3c431:	31 ff                	xor    %edi,%edi
   3c433:	41 3a 44 1c 02       	cmp    0x2(%r12,%rbx,1),%al
   3c438:	40 0f 94 c7          	sete   %dil
   3c43c:	ff 15 46 78 04 00    	call   *0x47846(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c442:	88 44 24 0e          	mov    %al,0xe(%rsp)
   3c446:	0f b6 44 24 73       	movzbl 0x73(%rsp),%eax
   3c44b:	31 ff                	xor    %edi,%edi
   3c44d:	41 3a 44 1c 03       	cmp    0x3(%r12,%rbx,1),%al
   3c452:	40 0f 94 c7          	sete   %dil
   3c456:	ff 15 2c 78 04 00    	call   *0x4782c(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c45c:	88 44 24 0d          	mov    %al,0xd(%rsp)
   3c460:	0f b6 44 24 74       	movzbl 0x74(%rsp),%eax
   3c465:	31 ff                	xor    %edi,%edi
   3c467:	41 3a 44 1c 04       	cmp    0x4(%r12,%rbx,1),%al
   3c46c:	40 0f 94 c7          	sete   %dil
   3c470:	ff 15 12 78 04 00    	call   *0x47812(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c476:	88 44 24 0c          	mov    %al,0xc(%rsp)
   3c47a:	0f b6 44 24 75       	movzbl 0x75(%rsp),%eax
   3c47f:	31 ff                	xor    %edi,%edi
   3c481:	41 3a 44 1c 05       	cmp    0x5(%r12,%rbx,1),%al
   3c486:	40 0f 94 c7          	sete   %dil
   3c48a:	ff 15 f8 77 04 00    	call   *0x477f8(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c490:	88 44 24 0b          	mov    %al,0xb(%rsp)
   3c494:	0f b6 44 24 76       	movzbl 0x76(%rsp),%eax
   3c499:	31 ff                	xor    %edi,%edi
   3c49b:	41 3a 44 1c 06       	cmp    0x6(%r12,%rbx,1),%al
   3c4a0:	40 0f 94 c7          	sete   %dil
   3c4a4:	ff 15 de 77 04 00    	call   *0x477de(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c4aa:	88 44 24 0a          	mov    %al,0xa(%rsp)
   3c4ae:	0f b6 44 24 77       	movzbl 0x77(%rsp),%eax
   3c4b3:	31 ff                	xor    %edi,%edi
   3c4b5:	41 3a 44 1c 07       	cmp    0x7(%r12,%rbx,1),%al
   3c4ba:	40 0f 94 c7          	sete   %dil
   3c4be:	ff 15 c4 77 04 00    	call   *0x477c4(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c4c4:	41 89 c5             	mov    %eax,%r13d
   3c4c7:	0f b6 44 24 78       	movzbl 0x78(%rsp),%eax
   3c4cc:	31 ff                	xor    %edi,%edi
   3c4ce:	41 3a 44 1c 08       	cmp    0x8(%r12,%rbx,1),%al
   3c4d3:	40 0f 94 c7          	sete   %dil
   3c4d7:	ff 15 ab 77 04 00    	call   *0x477ab(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c4dd:	88 44 24 09          	mov    %al,0x9(%rsp)
   3c4e1:	0f b6 44 24 79       	movzbl 0x79(%rsp),%eax
   3c4e6:	31 ff                	xor    %edi,%edi
   3c4e8:	41 3a 44 1c 09       	cmp    0x9(%r12,%rbx,1),%al
   3c4ed:	40 0f 94 c7          	sete   %dil
   3c4f1:	ff 15 91 77 04 00    	call   *0x47791(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c4f7:	88 44 24 08          	mov    %al,0x8(%rsp)
   3c4fb:	0f b6 44 24 7a       	movzbl 0x7a(%rsp),%eax
   3c500:	31 ff                	xor    %edi,%edi
   3c502:	41 3a 44 1c 0a       	cmp    0xa(%r12,%rbx,1),%al
   3c507:	40 0f 94 c7          	sete   %dil
   3c50b:	ff 15 77 77 04 00    	call   *0x47777(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c511:	88 44 24 07          	mov    %al,0x7(%rsp)
   3c515:	0f b6 44 24 7b       	movzbl 0x7b(%rsp),%eax
   3c51a:	31 ff                	xor    %edi,%edi
   3c51c:	41 3a 44 1c 0b       	cmp    0xb(%r12,%rbx,1),%al
   3c521:	40 0f 94 c7          	sete   %dil
   3c525:	ff 15 5d 77 04 00    	call   *0x4775d(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c52b:	41 89 c6             	mov    %eax,%r14d
   3c52e:	0f b6 44 24 7c       	movzbl 0x7c(%rsp),%eax
   3c533:	31 ff                	xor    %edi,%edi
   3c535:	41 3a 44 1c 0c       	cmp    0xc(%r12,%rbx,1),%al
   3c53a:	40 0f 94 c7          	sete   %dil
   3c53e:	ff 15 44 77 04 00    	call   *0x47744(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c544:	88 44 24 06          	mov    %al,0x6(%rsp)
   3c548:	0f b6 44 24 7d       	movzbl 0x7d(%rsp),%eax
   3c54d:	31 ff                	xor    %edi,%edi
   3c54f:	41 3a 44 1c 0d       	cmp    0xd(%r12,%rbx,1),%al
   3c554:	40 0f 94 c7          	sete   %dil
   3c558:	ff 15 2a 77 04 00    	call   *0x4772a(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c55e:	88 44 24 05          	mov    %al,0x5(%rsp)
   3c562:	0f b6 44 24 7e       	movzbl 0x7e(%rsp),%eax
   3c567:	31 ff                	xor    %edi,%edi
   3c569:	41 3a 44 1c 0e       	cmp    0xe(%r12,%rbx,1),%al
   3c56e:	40 0f 94 c7          	sete   %dil
   3c572:	ff 15 10 77 04 00    	call   *0x47710(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c578:	88 44 24 04          	mov    %al,0x4(%rsp)
   3c57c:	0f b6 44 24 7f       	movzbl 0x7f(%rsp),%eax
   3c581:	31 ff                	xor    %edi,%edi
   3c583:	41 3a 44 1c 0f       	cmp    0xf(%r12,%rbx,1),%al
   3c588:	40 0f 94 c7          	sete   %dil
   3c58c:	ff 15 f6 76 04 00    	call   *0x476f6(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c592:	0f b6 4c 24 60       	movzbl 0x60(%rsp),%ecx
   3c597:	22 4c 24 0f          	and    0xf(%rsp),%cl
   3c59b:	0f b6 54 24 0e       	movzbl 0xe(%rsp),%edx
   3c5a0:	22 54 24 0d          	and    0xd(%rsp),%dl
   3c5a4:	20 ca                	and    %cl,%dl
   3c5a6:	0f b6 4c 24 0c       	movzbl 0xc(%rsp),%ecx
   3c5ab:	22 4c 24 0b          	and    0xb(%rsp),%cl
   3c5af:	22 4c 24 0a          	and    0xa(%rsp),%cl
   3c5b3:	20 d1                	and    %dl,%cl
   3c5b5:	44 22 6c 24 09       	and    0x9(%rsp),%r13b
   3c5ba:	44 22 6c 24 08       	and    0x8(%rsp),%r13b
   3c5bf:	44 22 6c 24 07       	and    0x7(%rsp),%r13b
   3c5c4:	41 20 cd             	and    %cl,%r13b
   3c5c7:	44 22 74 24 06       	and    0x6(%rsp),%r14b
   3c5cc:	44 22 74 24 05       	and    0x5(%rsp),%r14b
   3c5d1:	44 22 74 24 04       	and    0x4(%rsp),%r14b
   3c5d6:	41 20 c6             	and    %al,%r14b
   3c5d9:	45 20 ee             	and    %r13b,%r14b
   3c5dc:	41 80 e6 01          	and    $0x1,%r14b
   3c5e0:	41 0f b6 fe          	movzbl %r14b,%edi
   3c5e4:	ff 15 9e 76 04 00    	call   *0x4769e(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c5ea:	84 c0                	test   %al,%al
   3c5ec:	4c 8d ac 24 80 05 00 	lea    0x580(%rsp),%r13
   3c5f3:	00 
   3c5f4:	74 3b                	je     3c631 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x701>
   3c5f6:	4c 89 ef             	mov    %r13,%rdi
   3c5f9:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
   3c5fe:	48 89 da             	mov    %rbx,%rdx
   3c601:	e8 8a 27 00 00       	call   3ed90 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as cipher::stream::StreamCipher>::apply_keystream>
   3c606:	48 8d bc 24 80 05 00 	lea    0x580(%rsp),%rdi
   3c60d:	00 
   3c60e:	e8 3d eb ff ff       	call   3b150 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   3c613:	48 8b 44 24 68       	mov    0x68(%rsp),%rax
   3c618:	48 39 c3             	cmp    %rax,%rbx
   3c61b:	48 0f 43 d8          	cmovae %rax,%rbx
   3c61f:	49 89 07             	mov    %rax,(%r15)
   3c622:	48 8b 44 24 58       	mov    0x58(%rsp),%rax
   3c627:	49 89 47 08          	mov    %rax,0x8(%r15)
   3c62b:	49 89 5f 10          	mov    %rbx,0x10(%r15)
   3c62f:	eb 1d                	jmp    3c64e <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x71e>
   3c631:	4c 89 ef             	mov    %r13,%rdi
   3c634:	e8 17 eb ff ff       	call   3b150 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   3c639:	4c 8b 6c 24 58       	mov    0x58(%rsp),%r13
   3c63e:	49 c7 07 ff ff ff ff 	movq   $0xffffffffffffffff,(%r15)
   3c645:	4c 89 ef             	mov    %r13,%rdi
   3c648:	ff 15 02 71 04 00    	call   *0x47102(%rip)        # 83750 <free@GLIBC_2.2.5>
   3c64e:	c6 44 24 30 00       	movb   $0x0,0x30(%rsp)
   3c653:	c6 44 24 31 00       	movb   $0x0,0x31(%rsp)
   3c658:	c6 44 24 32 00       	movb   $0x0,0x32(%rsp)
   3c65d:	c6 44 24 33 00       	movb   $0x0,0x33(%rsp)
   3c662:	c6 44 24 34 00       	movb   $0x0,0x34(%rsp)
   3c667:	c6 44 24 35 00       	movb   $0x0,0x35(%rsp)
   3c66c:	c6 44 24 36 00       	movb   $0x0,0x36(%rsp)
   3c671:	c6 44 24 37 00       	movb   $0x0,0x37(%rsp)
   3c676:	c6 44 24 38 00       	movb   $0x0,0x38(%rsp)
   3c67b:	c6 44 24 39 00       	movb   $0x0,0x39(%rsp)
   3c680:	c6 44 24 3a 00       	movb   $0x0,0x3a(%rsp)
   3c685:	c6 44 24 3b 00       	movb   $0x0,0x3b(%rsp)
   3c68a:	c6 44 24 3c 00       	movb   $0x0,0x3c(%rsp)
   3c68f:	c6 44 24 3d 00       	movb   $0x0,0x3d(%rsp)
   3c694:	c6 44 24 3e 00       	movb   $0x0,0x3e(%rsp)
   3c699:	c6 44 24 3f 00       	movb   $0x0,0x3f(%rsp)
   3c69e:	c6 44 24 40 00       	movb   $0x0,0x40(%rsp)
   3c6a3:	c6 44 24 41 00       	movb   $0x0,0x41(%rsp)
   3c6a8:	c6 44 24 42 00       	movb   $0x0,0x42(%rsp)
   3c6ad:	c6 44 24 43 00       	movb   $0x0,0x43(%rsp)
   3c6b2:	c6 44 24 44 00       	movb   $0x0,0x44(%rsp)
   3c6b7:	c6 44 24 45 00       	movb   $0x0,0x45(%rsp)
   3c6bc:	c6 44 24 46 00       	movb   $0x0,0x46(%rsp)
   3c6c1:	c6 44 24 47 00       	movb   $0x0,0x47(%rsp)
   3c6c6:	c6 44 24 48 00       	movb   $0x0,0x48(%rsp)
   3c6cb:	c6 44 24 49 00       	movb   $0x0,0x49(%rsp)
   3c6d0:	c6 44 24 4a 00       	movb   $0x0,0x4a(%rsp)
   3c6d5:	c6 44 24 4b 00       	movb   $0x0,0x4b(%rsp)
   3c6da:	c6 44 24 4c 00       	movb   $0x0,0x4c(%rsp)
   3c6df:	c6 44 24 4d 00       	movb   $0x0,0x4d(%rsp)
   3c6e4:	c6 44 24 4e 00       	movb   $0x0,0x4e(%rsp)
   3c6e9:	c6 44 24 4f 00       	movb   $0x0,0x4f(%rsp)
   3c6ee:	48 8d 44 24 30       	lea    0x30(%rsp),%rax
   3c6f3:	4c 89 f8             	mov    %r15,%rax
   3c6f6:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   3c6fa:	5b                   	pop    %rbx
   3c6fb:	41 5c                	pop    %r12
   3c6fd:	41 5d                	pop    %r13
   3c6ff:	41 5e                	pop    %r14
   3c701:	41 5f                	pop    %r15
   3c703:	5d                   	pop    %rbp
   3c704:	c3                   	ret
   3c705:	ff 15 e5 70 04 00    	call   *0x470e5(%rip)        # 837f0 <_GLOBAL_OFFSET_TABLE_+0xf8>
   3c70b:	e9 08 f9 ff ff       	jmp    3c018 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0xe8>
   3c710:	bf 01 00 00 00       	mov    $0x1,%edi
   3c715:	4c 89 fe             	mov    %r15,%rsi
   3c718:	ff 15 52 77 04 00    	call   *0x47752(%rip)        # 83e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   3c71e:	0f 0b                	ud2
   3c720:	48 89 c3             	mov    %rax,%rbx
   3c723:	eb 2e                	jmp    3c753 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x823>
   3c725:	48 89 c3             	mov    %rax,%rbx
   3c728:	48 8d 7c 24 70       	lea    0x70(%rsp),%rdi
   3c72d:	e8 1e ea ff ff       	call   3b150 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   3c732:	eb 1f                	jmp    3c753 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x823>
   3c734:	ff 15 8e 78 04 00    	call   *0x4788e(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3c73a:	48 89 c3             	mov    %rax,%rbx
   3c73d:	eb 1f                	jmp    3c75e <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x82e>
   3c73f:	eb 02                	jmp    3c743 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x813>
   3c741:	eb 00                	jmp    3c743 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x813>
   3c743:	48 89 c3             	mov    %rax,%rbx
   3c746:	48 8d bc 24 80 05 00 	lea    0x580(%rsp),%rdi
   3c74d:	00 
   3c74e:	e8 fd e9 ff ff       	call   3b150 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   3c753:	48 8b 7c 24 58       	mov    0x58(%rsp),%rdi
   3c758:	ff 15 f2 6f 04 00    	call   *0x46ff2(%rip)        # 83750 <free@GLIBC_2.2.5>
   3c75e:	48 8d 7c 24 30       	lea    0x30(%rsp),%rdi
   3c763:	e8 b8 e7 ff ff       	call   3af20 <core::ptr::drop_glue::<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>>
   3c768:	48 89 df             	mov    %rbx,%rdi
   3c76b:	e8 d0 98 fc ff       	call   6040 <_Unwind_Resume@plt>
   3c770:	ff 15 52 78 04 00    	call   *0x47852(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3c776:	ff 15 4c 78 04 00    	call   *0x4784c(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3c77c:	0f 1f 40 00          	nopl   0x0(%rax)

000000000003c780 <<star_core::secure_channel::server::SecureChannelServer>::receive_hpke>:
   3c780:	55                   	push   %rbp
   3c781:	48 89 e5             	mov    %rsp,%rbp
   3c784:	41 57                	push   %r15
   3c786:	41 56                	push   %r14
   3c788:	41 55                	push   %r13
   3c78a:	41 54                	push   %r12
   3c78c:	53                   	push   %rbx
   3c78d:	48 81 ec c8 02 00 00 	sub    $0x2c8,%rsp
   3c794:	49 89 fd             	mov    %rdi,%r13
   3c797:	48 83 f9 20          	cmp    $0x20,%rcx
   3c79b:	0f 85 38 03 00 00    	jne    3cad9 <<star_core::secure_channel::server::SecureChannelServer>::receive_hpke+0x359>
   3c7a1:	48 89 d3             	mov    %rdx,%rbx
   3c7a4:	49 89 f6             	mov    %rsi,%r14
   3c7a7:	4c 89 45 c8          	mov    %r8,-0x38(%rbp)
   3c7ab:	4c 89 4d c0          	mov    %r9,-0x40(%rbp)
   3c7af:	bf 20 00 00 00       	mov    $0x20,%edi
   3c7b4:	ff 15 b6 74 04 00    	call   *0x474b6(%rip)        # 83c70 <malloc@GLIBC_2.2.5>
   3c7ba:	48 85 c0             	test   %rax,%rax
   3c7bd:	0f 84 93 15 00 00    	je     3dd56 <<star_core::secure_channel::server::SecureChannelServer>::receive_hpke+0x15d6>
   3c7c3:	49 89 c7             	mov    %rax,%r15
   3c7c6:	0f 10 03             	movups (%rbx),%xmm0
   3c7c9:	0f 10 4b 10          	movups 0x10(%rbx),%xmm1
   3c7cd:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   3c7d1:	0f 11 00             	movups %xmm0,(%rax)
   3c7d4:	0f b6 00             	movzbl (%rax),%eax
   3c7d7:	0f 10 43 01          	movups 0x1(%rbx),%xmm0
   3c7db:	0f 10 4b 0f          	movups 0xf(%rbx),%xmm1
   3c7df:	0f 11 4d 9f          	movups %xmm1,-0x61(%rbp)
   3c7e3:	0f 11 45 91          	movups %xmm0,-0x6f(%rbp)
   3c7e7:	41 0f b6 4f 1f       	movzbl 0x1f(%r15),%ecx
   3c7ec:	80 e1 7f             	and    $0x7f,%cl
   3c7ef:	88 4d af             	mov    %cl,-0x51(%rbp)
   3c7f2:	88                   	.byte 0x88
   3c7f3:	45                   	rex.RB

=== A=0x403E38B F=0x3e38b ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000003e18b <<star_core::secure_channel::server::SecureChannelServer>::new+0x2eb>:
   3e18b:	85 00                	test   %eax,(%rax)
   3e18d:	ff                   	(bad)
   3e18e:	ff                   	(bad)
   3e18f:	ff 0f                	decl   (%rdi)
   3e191:	11 43 40             	adc    %eax,0x40(%rbx)
   3e194:	48 8b 85 f0 fe ff ff 	mov    -0x110(%rbp),%rax
   3e19b:	48 89 43 30          	mov    %rax,0x30(%rbx)
   3e19f:	48 8b 85 f8 fe ff ff 	mov    -0x108(%rbp),%rax
   3e1a6:	48 89 43 38          	mov    %rax,0x38(%rbx)
   3e1aa:	48 8b 85 10 ff ff ff 	mov    -0xf0(%rbp),%rax
   3e1b1:	48 89 43 50          	mov    %rax,0x50(%rbx)
   3e1b5:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3e1b9:	ff 15 69 5c 04 00    	call   *0x45c69(%rip)        # 83e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3e1bf:	48 89 d8             	mov    %rbx,%rax
   3e1c2:	48 81 c4 28 01 00 00 	add    $0x128,%rsp
   3e1c9:	5b                   	pop    %rbx
   3e1ca:	41 5e                	pop    %r14
   3e1cc:	41 5f                	pop    %r15
   3e1ce:	5d                   	pop    %rbp
   3e1cf:	c3                   	ret
   3e1d0:	bf 01 00 00 00       	mov    $0x1,%edi
   3e1d5:	be 20 00 00 00       	mov    $0x20,%esi
   3e1da:	ff 15 90 5c 04 00    	call   *0x45c90(%rip)        # 83e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   3e1e0:	0f 0b                	ud2
   3e1e2:	48 89 c3             	mov    %rax,%rbx
   3e1e5:	eb 0f                	jmp    3e1f6 <<star_core::secure_channel::server::SecureChannelServer>::new+0x356>
   3e1e7:	eb 21                	jmp    3e20a <<star_core::secure_channel::server::SecureChannelServer>::new+0x36a>
   3e1e9:	48 89 c3             	mov    %rax,%rbx
   3e1ec:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3e1f0:	ff 15 32 5c 04 00    	call   *0x45c32(%rip)        # 83e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3e1f6:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   3e1fd:	e8 1e d1 ff ff       	call   3b320 <core::ptr::drop_glue::<hpke_ng::kem::dh::DhPrivateKey<hpke_ng::kem::dh::X25519>>>
   3e202:	eb 13                	jmp    3e217 <<star_core::secure_channel::server::SecureChannelServer>::new+0x377>
   3e204:	ff 15 be 5d 04 00    	call   *0x45dbe(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3e20a:	48 89 c3             	mov    %rax,%rbx
   3e20d:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3e211:	ff 15 11 5c 04 00    	call   *0x45c11(%rip)        # 83e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3e217:	48 89 df             	mov    %rbx,%rdi
   3e21a:	e8 21 7e fc ff       	call   6040 <_Unwind_Resume@plt>
   3e21f:	ff 15 a3 5d 04 00    	call   *0x45da3(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3e225:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   3e22c:	00 00 00 
   3e22f:	90                   	nop

000000000003e230 <<star_core::serv::enclave::Enclave>::issue_token>:
   3e230:	55                   	push   %rbp
   3e231:	48 89 e5             	mov    %rsp,%rbp
   3e234:	41 57                	push   %r15
   3e236:	41 56                	push   %r14
   3e238:	41 55                	push   %r13
   3e23a:	41 54                	push   %r12
   3e23c:	53                   	push   %rbx
   3e23d:	48 83 ec 18          	sub    $0x18,%rsp
   3e241:	48 89 fb             	mov    %rdi,%rbx
   3e244:	48 83 f9 60          	cmp    $0x60,%rcx
   3e248:	0f 82 84 00 00 00    	jb     3e2d2 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   3e24e:	4d 89 c6             	mov    %r8,%r14
   3e251:	4c 8d 42 20          	lea    0x20(%rdx),%r8
   3e255:	48 8d 42 40          	lea    0x40(%rdx),%rax
   3e259:	4c 8d 52 60          	lea    0x60(%rdx),%r10
   3e25d:	48 83 c1 a0          	add    $0xffffffffffffffa0,%rcx
   3e261:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   3e265:	49 89 cb             	mov    %rcx,%r11
   3e268:	b9 20 00 00 00       	mov    $0x20,%ecx
   3e26d:	41 b9 20 00 00 00    	mov    $0x20,%r9d
   3e273:	49 89 f4             	mov    %rsi,%r12
   3e276:	49 89 d7             	mov    %rdx,%r15
   3e279:	48 89 c2             	mov    %rax,%rdx
   3e27c:	41 53                	push   %r11
   3e27e:	41 52                	push   %r10
   3e280:	6a 20                	push   $0x20
   3e282:	41 57                	push   %r15
   3e284:	ff 15 2e 57 04 00    	call   *0x4572e(%rip)        # 839b8 <_GLOBAL_OFFSET_TABLE_+0x2c0>
   3e28a:	48 83 c4 20          	add    $0x20,%rsp
   3e28e:	4c 8b 6d c0          	mov    -0x40(%rbp),%r13
   3e292:	49 83 fd ff          	cmp    $0xffffffffffffffff,%r13
   3e296:	74 3a                	je     3e2d2 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   3e298:	4c 8b 7d c8          	mov    -0x38(%rbp),%r15
   3e29c:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
   3e2a0:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   3e2a4:	4c 89 e6             	mov    %r12,%rsi
   3e2a7:	4c 89 fa             	mov    %r15,%rdx
   3e2aa:	4d 89 f0             	mov    %r14,%r8
   3e2ad:	ff 15 c5 58 04 00    	call   *0x458c5(%rip)        # 83b78 <_GLOBAL_OFFSET_TABLE_+0x480>
   3e2b3:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   3e2b7:	48 89 43 10          	mov    %rax,0x10(%rbx)
   3e2bb:	0f 10 45 c0          	movups -0x40(%rbp),%xmm0
   3e2bf:	0f 11 03             	movups %xmm0,(%rbx)
   3e2c2:	4d 85 ed             	test   %r13,%r13
   3e2c5:	74 16                	je     3e2dd <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   3e2c7:	4c 89 ff             	mov    %r15,%rdi
   3e2ca:	ff 15 80 54 04 00    	call   *0x45480(%rip)        # 83750 <free@GLIBC_2.2.5>
   3e2d0:	eb 0b                	jmp    3e2dd <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   3e2d2:	c6 43 08 01          	movb   $0x1,0x8(%rbx)
   3e2d6:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   3e2dd:	48 89 d8             	mov    %rbx,%rax
   3e2e0:	48 83 c4 18          	add    $0x18,%rsp
   3e2e4:	5b                   	pop    %rbx
   3e2e5:	41 5c                	pop    %r12
   3e2e7:	41 5d                	pop    %r13
   3e2e9:	41 5e                	pop    %r14
   3e2eb:	41 5f                	pop    %r15
   3e2ed:	5d                   	pop    %rbp
   3e2ee:	c3                   	ret
   3e2ef:	48 89 c3             	mov    %rax,%rbx
   3e2f2:	4d 85 ed             	test   %r13,%r13
   3e2f5:	74 09                	je     3e300 <<star_core::serv::enclave::Enclave>::issue_token+0xd0>
   3e2f7:	4c 89 ff             	mov    %r15,%rdi
   3e2fa:	ff 15 50 54 04 00    	call   *0x45450(%rip)        # 83750 <free@GLIBC_2.2.5>
   3e300:	48 89 df             	mov    %rbx,%rdi
   3e303:	e8 38 7d fc ff       	call   6040 <_Unwind_Resume@plt>
   3e308:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   3e30f:	00 

000000000003e310 <<star_core::serv::enclave::Enclave>::issue_token_plain>:
   3e310:	55                   	push   %rbp
   3e311:	48 89 e5             	mov    %rsp,%rbp
   3e314:	41 57                	push   %r15
   3e316:	41 56                	push   %r14
   3e318:	41 54                	push   %r12
   3e31a:	53                   	push   %rbx
   3e31b:	48 81 ec b0 00 00 00 	sub    $0xb0,%rsp
   3e322:	48 89 fb             	mov    %rdi,%rbx
   3e325:	48 83 f9 50          	cmp    $0x50,%rcx
   3e329:	72 62                	jb     3e38d <<star_core::serv::enclave::Enclave>::issue_token_plain+0x7d>
   3e32b:	49 89 f1             	mov    %rsi,%r9
   3e32e:	0f 10 42 20          	movups 0x20(%rdx),%xmm0
   3e332:	0f 10 4a 30          	movups 0x30(%rdx),%xmm1
   3e336:	0f 29 8d 60 ff ff ff 	movaps %xmm1,-0xa0(%rbp)
   3e33d:	0f 29 85 50 ff ff ff 	movaps %xmm0,-0xb0(%rbp)
   3e344:	f3 0f 6f 02          	movdqu (%rdx),%xmm0
   3e348:	f3 0f 6f 4a 10       	movdqu 0x10(%rdx),%xmm1
   3e34d:	66 0f 7f 85 30 ff ff 	movdqa %xmm0,-0xd0(%rbp)
   3e354:	ff 
   3e355:	66 0f 7f 8d 40 ff ff 	movdqa %xmm1,-0xc0(%rbp)
   3e35c:	ff 
   3e35d:	4c 8b 72 40          	mov    0x40(%rdx),%r14
   3e361:	49 89 d4             	mov    %rdx,%r12
   3e364:	4c 8b 7a 48          	mov    0x48(%rdx),%r15
   3e368:	48 8b 46 58          	mov    0x58(%rsi),%rax
   3e36c:	49 83 c1 60          	add    $0x60,%r9
   3e370:	48 8d bd 30 ff ff ff 	lea    -0xd0(%rbp),%rdi
   3e377:	4c 89 fe             	mov    %r15,%rsi
   3e37a:	4c 89 f2             	mov    %r14,%rdx
   3e37d:	4c 89 c1             	mov    %r8,%rcx
   3e380:	49 89 c0             	mov    %rax,%r8
   3e383:	ff 15 87 58 04 00    	call   *0x45887(%rip)        # 83c10 <_GLOBAL_OFFSET_TABLE_+0x518>
   3e389:	84 c0                	test   %al,%al
   3e38b:	75 3d                	jne    3e3ca <<star_core::serv::enclave::Enclave>::issue_token_plain+0xba>
   3e38d:	bf 01 00 00 00       	mov    $0x1,%edi
   3e392:	ff 15 d8 58 04 00    	call   *0x458d8(%rip)        # 83c70 <malloc@GLIBC_2.2.5>
   3e398:	48 85 c0             	test   %rax,%rax
   3e39b:	0f 84 1c 01 00 00    	je     3e4bd <<star_core::serv::enclave::Enclave>::issue_token_plain+0x1ad>
   3e3a1:	c6 00 00             	movb   $0x0,(%rax)
   3e3a4:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   3e3ab:	48 89 43 08          	mov    %rax,0x8(%rbx)
   3e3af:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   3e3b6:	00 
   3e3b7:	48 89 d8             	mov    %rbx,%rax
   3e3ba:	48 81 c4 b0 00 00 00 	add    $0xb0,%rsp
   3e3c1:	5b                   	pop    %rbx
   3e3c2:	41 5c                	pop    %r12
   3e3c4:	41 5e                	pop    %r14
   3e3c6:	41 5f                	pop    %r15
   3e3c8:	5d                   	pop    %rbp
   3e3c9:	c3                   	ret
   3e3ca:	41 0f 10 04 24       	movups (%r12),%xmm0
   3e3cf:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   3e3d5:	0f 11 4d b8          	movups %xmm1,-0x48(%rbp)
   3e3d9:	0f 11 45 a8          	movups %xmm0,-0x58(%rbp)
   3e3dd:	0f 10 05 bc 0c 03 00 	movups 0x30cbc(%rip),%xmm0        # 6f0a0 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x130>
   3e3e4:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
   3e3eb:	f3 0f 6f 05 bd 0c 03 	movdqu 0x30cbd(%rip),%xmm0        # 6f0b0 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x140>
   3e3f2:	00 
   3e3f3:	66 0f 7f 45 80       	movdqa %xmm0,-0x80(%rbp)
   3e3f8:	48 c7 45 90 00 00 00 	movq   $0x0,-0x70(%rbp)
   3e3ff:	00 
   3e400:	48 8d 75 98          	lea    -0x68(%rbp),%rsi
   3e404:	4c 89 7d 98          	mov    %r15,-0x68(%rbp)
   3e408:	4c 89 75 a0          	mov    %r14,-0x60(%rbp)
   3e40c:	c6 45 d8 30          	movb   $0x30,-0x28(%rbp)
   3e410:	c6 45 c8 80          	movb   $0x80,-0x38(%rbp)
   3e414:	c7 45 c9 00 00 00 00 	movl   $0x0,-0x37(%rbp)
   3e41b:	c7 45 cc 00 00 00 00 	movl   $0x0,-0x34(%rbp)
   3e422:	48 b8 00 00 00 00 00 	movabs $0x8001000000000000,%rax
   3e429:	00 01 80 
   3e42c:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   3e430:	48 8d bd 70 ff ff ff 	lea    -0x90(%rbp),%rdi
   3e437:	ba 01 00 00 00       	mov    $0x1,%edx
   3e43c:	ff 15 96 58 04 00    	call   *0x45896(%rip)        # 83cd8 <_GLOBAL_OFFSET_TABLE_+0x5e0>
   3e442:	66 0f ef c0          	pxor   %xmm0,%xmm0
   3e446:	66 0f 6f 8d 70 ff ff 	movdqa -0x90(%rbp),%xmm1
   3e44d:	ff 
   3e44e:	66 0f 6f 55 80       	movdqa -0x80(%rbp),%xmm2
   3e453:	66 0f 6f d9          	movdqa %xmm1,%xmm3
   3e457:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   3e45b:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   3e460:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   3e465:	66 0f 60 c8          	punpcklbw %xmm0,%xmm1
   3e469:	f2 0f 70 c9 1b       	pshuflw $0x1b,%xmm1,%xmm1
   3e46e:	f3 0f 70 c9 1b       	pshufhw $0x1b,%xmm1,%xmm1
   3e473:	66 0f 67 cb          	packuswb %xmm3,%xmm1
   3e477:	66 0f 6f da          	movdqa %xmm2,%xmm3
   3e47b:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   3e47f:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   3e484:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   3e489:	66 0f 60 d0          	punpcklbw %xmm0,%xmm2
   3e48d:	f2 0f 70 c2 1b       	pshuflw $0x1b,%xmm2,%xmm0
   3e492:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   3e497:	66 0f 67 c3          	packuswb %xmm3,%xmm0
   3e49b:	66 0f 7f 8d 70 ff ff 	movdqa %xmm1,-0x90(%rbp)
   3e4a2:	ff 
   3e4a3:	66 0f 7f 45 80       	movdqa %xmm0,-0x80(%rbp)
   3e4a8:	48 8d b5 70 ff ff ff 	lea    -0x90(%rbp),%rsi
   3e4af:	48 89 df             	mov    %rbx,%rdi
   3e4b2:	ff 15 d8 59 04 00    	call   *0x459d8(%rip)        # 83e90 <_GLOBAL_OFFSET_TABLE_+0x798>
   3e4b8:	e9 fa fe ff ff       	jmp    3e3b7 <<star_core::serv::enclave::Enclave>::issue_token_plain+0xa7>
   3e4bd:	bf 01 00 00 00       	mov    $0x1,%edi
   3e4c2:	be 01 00 00 00       	mov    $0x1,%esi
   3e4c7:	ff 15 f3 58 04 00    	call   *0x458f3(%rip)        # 83dc0 <_GLOBAL_OFFSET_TABLE_+0x6c8>
   3e4cd:	0f 1f 00             	nopl   (%rax)

000000000003e4d0 <<star_core::serv::core::cred::Credential>::is_valid>:
   3e4d0:	55                   	push   %rbp
   3e4d1:	48 89 e5             	mov    %rsp,%rbp
   3e4d4:	41 57                	push   %r15
   3e4d6:	41 56                	push   %r14
   3e4d8:	41 55                	push   %r13
   3e4da:	41 54                	push   %r12
   3e4dc:	53                   	push   %rbx
   3e4dd:	48 81 ec e8 02 00 00 	sub    $0x2e8,%rsp
   3e4e4:	4c 89 4d c8          	mov    %r9,-0x38(%rbp)
   3e4e8:	4d 89 c5             	mov    %r8,%r13
   3e4eb:	49 89 d7             	mov    %rdx,%r15
   3e4ee:	48 89 7d c0          	mov    %rdi,-0x40(%rbp)
   3e4f2:	48 89 d0             	mov    %rdx,%rax
   3e4f5:	48 c1 e8 08          	shr    $0x8,%rax
   3e4f9:	48 89 85 68 ff ff ff 	mov    %rax,-0x98(%rbp)
   3e500:	48 89 d0             	mov    %rdx,%rax
   3e503:	48 c1 e8 10          	shr    $0x10,%rax
   3e507:	48 89 85 70 ff ff ff 	mov    %rax,-0x90(%rbp)
   3e50e:	48 89 d0             	mov    %rdx,%rax
   3e511:	48 c1 e8 18          	shr    $0x18,%rax
   3e515:	48 89 85 78 ff ff ff 	mov    %rax,-0x88(%rbp)
   3e51c:	48 89 d0             	mov    %rdx,%rax
   3e51f:	48 c1 e8 20          	shr    $0x20,%rax
   3e523:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
   3e527:	48 89 d0             	mov    %rdx,%rax
   3e52a:	48 c1 e8 28          	shr    $0x28,%rax
   3e52e:	48 89 45 88          	mov    %rax,-0x78(%rbp)
   3e532:	48 89 d0             	mov    %rdx,%rax
   3e535:	48 c1 e8 30          	shr    $0x30,%rax
   3e539:	48 89 45 90          	mov    %rax,-0x70(%rbp)
   3e53d:	48 89 d0             	mov    %rdx,%rax
   3e540:	48 c1 e8 38          	shr    $0x38,%rax
   3e544:	48 89 45 a0          	mov    %rax,-0x60(%rbp)
   3e548:	4c 89 c3             	mov    %r8,%rbx
   3e54b:	48 c1 eb 08          	shr    $0x8,%rbx
   3e54f:	48 89 9d b8 fe ff ff 	mov    %rbx,-0x148(%rbp)
   3e556:	4d 89 c4             	mov    %r8,%r12
   3e559:	49 c1 ec 10          	shr    $0x10,%r12
   3e55d:	4d 89 c6             	mov    %r8,%r14
   3e560:	49 c1 ee 18          	shr    $0x18,%r14
   3e564:	4c 89 c0             	mov    %r8,%rax
   3e567:	48 c1 e8 20          	shr    $0x20,%rax
   3e56b:	48 89 45 80          	mov    %rax,-0x80(%rbp)
   3e56f:	4c 89 c0             	mov    %r8,%rax
   3e572:	48 c1 e8 28          	shr    $0x28,%rax
   3e576:	48 89 45 98          	mov    %rax,-0x68(%rbp)
   3e57a:	4c 89 c0             	mov    %r8,%rax
   3e57d:	48 c1 e8 30          	shr    $0x30,%rax
   3e581:	48 89 45 a8          	mov    %rax,-0x58(%rbp)
   3e585:	4c 89 c0             	mov    %r8,%rax
   3e588:	48                   	rex.W
   3e589:	c1                   	.byte 0xc1
   3e58a:	e8                   	.byte 0xe8

=== A=0x48CD7D0 F=0x8cd7d0 ===

./target/release/analysis:     file format elf64-x86-64


