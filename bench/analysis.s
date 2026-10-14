=== VALGRIND OUTPUT ===
==2555== Memcheck, a memory error detector
==2555== Copyright (C) 2002-2026, and GNU GPL'd, by Julian Seward et al.
==2555== Using Valgrind-3.27.1 and LibVEX; rerun with -h for copyright info
==2555== Command: ./target/release/analysis
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CB0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400C601: <star_core::secure_channel::client::SecureChannelClient>::send_hpke::<analysis::rng::PoisonedRng> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E6CD: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CB0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403C7F9: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400EDB2: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4022660: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403DCC3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400EDB2: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x403C5E4: <star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403BDCE: <star_core::client::User>::receive_credential (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F252: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CB0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400C601: <star_core::secure_channel::client::SecureChannelClient>::send_hpke::<analysis::rng::PoisonedRng> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F2A7: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CB0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403C7F9: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403E279: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F3B9: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4022660: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403DCC3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403E279: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F3B9: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x403E37B: <star_core::serv::enclave::Enclave>::issue_token_plain (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403E2A2: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F3B9: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CB0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400F7E8: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4024CB0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403C7F9: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4011F34: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
==2555== Conditional jump or move depends on uninitialised value(s)
==2555==    at 0x4022660: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2555==    by 0x403DCC3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4011F34: analysis::main (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400DA25: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2555==    by 0x400E37B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D96: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
==2555== 
done
==2555== 
==2555== HEAP SUMMARY:
==2555==     in use at exit: 544 bytes in 1 blocks
==2555==   total heap usage: 93 allocs, 92 frees, 7,105 bytes allocated
==2555== 
==2555== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==2555==    at 0x48CD7D0: malloc (in /nix/store/mf7rzpknxwg0q1wf0nm8gfzikapg00h1-valgrind-3.27.1/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==2555==    by 0x40690B7: <alloc::collections::btree::map::entry::VacantEntry<usize, std::sys::pal::unix::stack_overflow::thread_info::ThreadInfo>>::insert_entry (in /workspace/star/core/target/release/analysis)
==2555==    by 0x405A153: std::sys::pal::unix::stack_overflow::thread_info::set_current_info (in /workspace/star/core/target/release/analysis)
==2555==    by 0x405AD7F: std::sys::pal::unix::stack_overflow::imp::init (in /workspace/star/core/target/release/analysis)
==2555==    by 0x406957F: std::sys::pal::unix::init (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4055D90: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2555==    by 0x4013AE3: main (in /workspace/star/core/target/release/analysis)
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

=== A=0x4024CB0 F=0x24cb0 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000024ab0 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x140>:
   24ab0:	7d aa                	jge    24a5c <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0xec>
   24ab2:	00 40 0f             	add    %al,0xf(%rax)
   24ab5:	94                   	xchg   %eax,%esp
   24ab6:	c7                   	(bad)
   24ab7:	ff 15 cb f1 05 00    	call   *0x5f1cb(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24abd:	88 45 c8             	mov    %al,-0x38(%rbp)
   24ac0:	31 ff                	xor    %edi,%edi
   24ac2:	80 7d ab 00          	cmpb   $0x0,-0x55(%rbp)
   24ac6:	40 0f 94 c7          	sete   %dil
   24aca:	ff 15 b8 f1 05 00    	call   *0x5f1b8(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24ad0:	88 45 c9             	mov    %al,-0x37(%rbp)
   24ad3:	31 ff                	xor    %edi,%edi
   24ad5:	80 7d ac 00          	cmpb   $0x0,-0x54(%rbp)
   24ad9:	40 0f 94 c7          	sete   %dil
   24add:	ff 15 a5 f1 05 00    	call   *0x5f1a5(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24ae3:	88 45 ca             	mov    %al,-0x36(%rbp)
   24ae6:	31 ff                	xor    %edi,%edi
   24ae8:	80 7d ad 00          	cmpb   $0x0,-0x53(%rbp)
   24aec:	40 0f 94 c7          	sete   %dil
   24af0:	ff 15 92 f1 05 00    	call   *0x5f192(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24af6:	88 45 cb             	mov    %al,-0x35(%rbp)
   24af9:	31 ff                	xor    %edi,%edi
   24afb:	80 7d ae 00          	cmpb   $0x0,-0x52(%rbp)
   24aff:	40 0f 94 c7          	sete   %dil
   24b03:	ff 15 7f f1 05 00    	call   *0x5f17f(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b09:	41 89 c6             	mov    %eax,%r14d
   24b0c:	31 ff                	xor    %edi,%edi
   24b0e:	80 7d af 00          	cmpb   $0x0,-0x51(%rbp)
   24b12:	40 0f 94 c7          	sete   %dil
   24b16:	ff 15 6c f1 05 00    	call   *0x5f16c(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b1c:	88 45 cc             	mov    %al,-0x34(%rbp)
   24b1f:	31 ff                	xor    %edi,%edi
   24b21:	80 7d b0 00          	cmpb   $0x0,-0x50(%rbp)
   24b25:	40 0f 94 c7          	sete   %dil
   24b29:	ff 15 59 f1 05 00    	call   *0x5f159(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b2f:	88 45 cd             	mov    %al,-0x33(%rbp)
   24b32:	31 ff                	xor    %edi,%edi
   24b34:	80 7d b1 00          	cmpb   $0x0,-0x4f(%rbp)
   24b38:	40 0f 94 c7          	sete   %dil
   24b3c:	ff 15 46 f1 05 00    	call   *0x5f146(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b42:	88 45 ce             	mov    %al,-0x32(%rbp)
   24b45:	31 ff                	xor    %edi,%edi
   24b47:	80 7d b2 00          	cmpb   $0x0,-0x4e(%rbp)
   24b4b:	40 0f 94 c7          	sete   %dil
   24b4f:	ff 15 33 f1 05 00    	call   *0x5f133(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b55:	88 45 cf             	mov    %al,-0x31(%rbp)
   24b58:	31 ff                	xor    %edi,%edi
   24b5a:	80 7d b3 00          	cmpb   $0x0,-0x4d(%rbp)
   24b5e:	40 0f 94 c7          	sete   %dil
   24b62:	ff 15 20 f1 05 00    	call   *0x5f120(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b68:	88 45 d0             	mov    %al,-0x30(%rbp)
   24b6b:	31 ff                	xor    %edi,%edi
   24b6d:	80 7d b4 00          	cmpb   $0x0,-0x4c(%rbp)
   24b71:	40 0f 94 c7          	sete   %dil
   24b75:	ff 15 0d f1 05 00    	call   *0x5f10d(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b7b:	41 89 c4             	mov    %eax,%r12d
   24b7e:	31 ff                	xor    %edi,%edi
   24b80:	80 7d b5 00          	cmpb   $0x0,-0x4b(%rbp)
   24b84:	40 0f 94 c7          	sete   %dil
   24b88:	ff 15 fa f0 05 00    	call   *0x5f0fa(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24b8e:	88 45 d1             	mov    %al,-0x2f(%rbp)
   24b91:	31 ff                	xor    %edi,%edi
   24b93:	80 7d b6 00          	cmpb   $0x0,-0x4a(%rbp)
   24b97:	40 0f 94 c7          	sete   %dil
   24b9b:	ff 15 e7 f0 05 00    	call   *0x5f0e7(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24ba1:	88 45 d2             	mov    %al,-0x2e(%rbp)
   24ba4:	31 ff                	xor    %edi,%edi
   24ba6:	80 7d b7 00          	cmpb   $0x0,-0x49(%rbp)
   24baa:	40 0f 94 c7          	sete   %dil
   24bae:	ff 15 d4 f0 05 00    	call   *0x5f0d4(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bb4:	88 45 d3             	mov    %al,-0x2d(%rbp)
   24bb7:	31 ff                	xor    %edi,%edi
   24bb9:	80 7d b8 00          	cmpb   $0x0,-0x48(%rbp)
   24bbd:	40 0f 94 c7          	sete   %dil
   24bc1:	ff 15 c1 f0 05 00    	call   *0x5f0c1(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bc7:	88 45 d4             	mov    %al,-0x2c(%rbp)
   24bca:	31 ff                	xor    %edi,%edi
   24bcc:	80 7d b9 00          	cmpb   $0x0,-0x47(%rbp)
   24bd0:	40 0f 94 c7          	sete   %dil
   24bd4:	ff 15 ae f0 05 00    	call   *0x5f0ae(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bda:	88 45 d5             	mov    %al,-0x2b(%rbp)
   24bdd:	31 ff                	xor    %edi,%edi
   24bdf:	80 7d ba 00          	cmpb   $0x0,-0x46(%rbp)
   24be3:	40 0f 94 c7          	sete   %dil
   24be7:	ff 15 9b f0 05 00    	call   *0x5f09b(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24bed:	88 45 d6             	mov    %al,-0x2a(%rbp)
   24bf0:	31 ff                	xor    %edi,%edi
   24bf2:	80 7d bb 00          	cmpb   $0x0,-0x45(%rbp)
   24bf6:	40 0f 94 c7          	sete   %dil
   24bfa:	ff 15 88 f0 05 00    	call   *0x5f088(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24c00:	41 89 c5             	mov    %eax,%r13d
   24c03:	31 ff                	xor    %edi,%edi
   24c05:	80 7d bc 00          	cmpb   $0x0,-0x44(%rbp)
   24c09:	40 0f 94 c7          	sete   %dil
   24c0d:	ff 15 75 f0 05 00    	call   *0x5f075(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24c13:	88 45 d7             	mov    %al,-0x29(%rbp)
   24c16:	31 ff                	xor    %edi,%edi
   24c18:	80 7d bd 00          	cmpb   $0x0,-0x43(%rbp)
   24c1c:	40 0f 94 c7          	sete   %dil
   24c20:	ff 15 62 f0 05 00    	call   *0x5f062(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24c26:	0f b6 4d be          	movzbl -0x42(%rbp),%ecx
   24c2a:	22 4d bf             	and    -0x41(%rbp),%cl
   24c2d:	0f b6 55 c0          	movzbl -0x40(%rbp),%edx
   24c31:	22 55 c1             	and    -0x3f(%rbp),%dl
   24c34:	20 ca                	and    %cl,%dl
   24c36:	0f b6 4d c2          	movzbl -0x3e(%rbp),%ecx
   24c3a:	22 4d c3             	and    -0x3d(%rbp),%cl
   24c3d:	22 4d c4             	and    -0x3c(%rbp),%cl
   24c40:	20 d1                	and    %dl,%cl
   24c42:	44 22 7d c5          	and    -0x3b(%rbp),%r15b
   24c46:	44 22 7d c6          	and    -0x3a(%rbp),%r15b
   24c4a:	44 22 7d c7          	and    -0x39(%rbp),%r15b
   24c4e:	41 20 cf             	and    %cl,%r15b
   24c51:	22 5d c8             	and    -0x38(%rbp),%bl
   24c54:	22 5d c9             	and    -0x37(%rbp),%bl
   24c57:	22 5d ca             	and    -0x36(%rbp),%bl
   24c5a:	22 5d cb             	and    -0x35(%rbp),%bl
   24c5d:	44 20 fb             	and    %r15b,%bl
   24c60:	44 22 75 cc          	and    -0x34(%rbp),%r14b
   24c64:	44 22 75 cd          	and    -0x33(%rbp),%r14b
   24c68:	44 22 75 ce          	and    -0x32(%rbp),%r14b
   24c6c:	44 22 75 cf          	and    -0x31(%rbp),%r14b
   24c70:	44 22 75 d0          	and    -0x30(%rbp),%r14b
   24c74:	41 20 de             	and    %bl,%r14b
   24c77:	44 22 65 d1          	and    -0x2f(%rbp),%r12b
   24c7b:	44 22 65 d2          	and    -0x2e(%rbp),%r12b
   24c7f:	44 22 65 d3          	and    -0x2d(%rbp),%r12b
   24c83:	44 22 65 d4          	and    -0x2c(%rbp),%r12b
   24c87:	44 22 65 d5          	and    -0x2b(%rbp),%r12b
   24c8b:	44 22 65 d6          	and    -0x2a(%rbp),%r12b
   24c8f:	45 20 f4             	and    %r14b,%r12b
   24c92:	44 22 6d d7          	and    -0x29(%rbp),%r13b
   24c96:	41 20 c5             	and    %al,%r13b
   24c99:	45 20 e5             	and    %r12b,%r13b
   24c9c:	41 80 e5 01          	and    $0x1,%r13b
   24ca0:	41 0f b6 fd          	movzbl %r13b,%edi
   24ca4:	ff 15 de ef 05 00    	call   *0x5efde(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24caa:	84 c0                	test   %al,%al
   24cac:	48 8b 5d 90          	mov    -0x70(%rbp),%rbx
   24cb0:	74 0d                	je     24cbf <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x34f>
   24cb2:	c6 43 08 03          	movb   $0x3,0x8(%rbx)
   24cb6:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   24cbd:	eb 32                	jmp    24cf1 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x381>
   24cbf:	bf 20 00 00 00       	mov    $0x20,%edi
   24cc4:	ff 15 a6 ef 05 00    	call   *0x5efa6(%rip)        # 83c70 <malloc@GLIBC_2.2.5>
   24cca:	48 85 c0             	test   %rax,%rax
   24ccd:	74 3e                	je     24d0d <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x39d>
   24ccf:	0f 10 45 9e          	movups -0x62(%rbp),%xmm0
   24cd3:	0f 10 4d ae          	movups -0x52(%rbp),%xmm1
   24cd7:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   24cdb:	0f 11 00             	movups %xmm0,(%rax)
   24cde:	48 c7 03 20 00 00 00 	movq   $0x20,(%rbx)
   24ce5:	48 89 43 08          	mov    %rax,0x8(%rbx)
   24ce9:	48 c7 43 10 20 00 00 	movq   $0x20,0x10(%rbx)
   24cf0:	00 
   24cf1:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   24cf5:	ff 15 2d ef 05 00    	call   *0x5ef2d(%rip)        # 83c28 <_GLOBAL_OFFSET_TABLE_+0x530>
   24cfb:	48 89 d8             	mov    %rbx,%rax
   24cfe:	48 83 c4 68          	add    $0x68,%rsp
   24d02:	5b                   	pop    %rbx
   24d03:	41 5c                	pop    %r12
   24d05:	41 5d                	pop    %r13
   24d07:	41 5e                	pop    %r14
   24d09:	41 5f                	pop    %r15
   24d0b:	5d                   	pop    %rbp
   24d0c:	c3                   	ret
   24d0d:	bf 01 00 00 00       	mov    $0x1,%edi
   24d12:	be 20 00 00 00       	mov    $0x20,%esi
   24d17:	ff 15 53 f1 05 00    	call   *0x5f153(%rip)        # 83e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   24d1d:	0f 0b                	ud2
   24d1f:	eb 00                	jmp    24d21 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x3b1>
   24d21:	48 89 c3             	mov    %rax,%rbx
   24d24:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   24d28:	ff 15 fa ee 05 00    	call   *0x5eefa(%rip)        # 83c28 <_GLOBAL_OFFSET_TABLE_+0x530>
   24d2e:	48 89 df             	mov    %rbx,%rdi
   24d31:	e8 0a 13 fe ff       	call   6040 <_Unwind_Resume@plt>
   24d36:	ff 15 8c f2 05 00    	call   *0x5f28c(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   24d3c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000024d40 <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed>:
   24d40:	55                   	push   %rbp
   24d41:	48 89 e5             	mov    %rsp,%rbp
   24d44:	41 57                	push   %r15
   24d46:	41 56                	push   %r14
   24d48:	41 55                	push   %r13
   24d4a:	41 54                	push   %r12
   24d4c:	53                   	push   %rbx
   24d4d:	48 81 ec d8 00 00 00 	sub    $0xd8,%rsp
   24d54:	48 89 fb             	mov    %rdi,%rbx
   24d57:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   24d5e:	ba 90 00 00 00       	mov    $0x90,%edx
   24d63:	ff 15 2f ee 05 00    	call   *0x5ee2f(%rip)        # 83b98 <memcpy@GLIBC_2.14>
   24d69:	4c 8d b5 40 ff ff ff 	lea    -0xc0(%rbp),%r14
   24d70:	44 0f b6 a5 3f ff ff 	movzbl -0xc1(%rbp),%r12d
   24d77:	ff 
   24d78:	4c 8b ad 60 ff ff ff 	mov    -0xa0(%rbp),%r13
   24d7f:	4d 89 e7             	mov    %r12,%r15
   24d82:	49 83 f7 3f          	xor    $0x3f,%r15
   24d86:	42 c6 84 25 00 ff ff 	movb   $0x80,-0x100(%rbp,%r12,1)
   24d8d:	ff 80 
   24d8f:	49 8d 3c 2c          	lea    (%r12,%rbp,1),%rdi
   24d93:	48 81 c7 01 ff ff ff 	add    $0xffffffffffffff01,%rdi
   24d9a:	31 f6                	xor    %esi,%esi
   24d9c:	4c 89 fa             	mov    %r15,%rdx
   24d9f:	ff 15 3b ec 05 00    	call   *0x5ec3b(%rip)        # 839e0 <memset@GLIBC_2.2.5>
   24da5:	49 83 ff 07          	cmp    $0x7,%r15
   24da9:	77 63                	ja     24e0e <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed+0xce>
   24dab:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   24db2:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   24db9:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   24dc0:	66 0f 6f 9d 30 ff ff 	movdqa -0xd0(%rbp),%xmm3
   24dc7:	ff 
   24dc8:	66 0f 7f 5d c0       	movdqa %xmm3,-0x40(%rbp)
   24dcd:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   24dd1:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   24dd5:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   24dd9:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   24ddd:	ba 01 00 00 00       	mov    $0x1,%edx
   24de2:	4c 89 f7             	mov    %r14,%rdi
   24de5:	ff 15 b5 ee 05 00    	call   *0x5eeb5(%rip)        # 83ca0 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   24deb:	0f 57 c0             	xorps  %xmm0,%xmm0
   24dee:	0f 29 85 00 ff ff ff 	movaps %xmm0,-0x100(%rbp)
   24df5:	0f 29 85 10 ff ff ff 	movaps %xmm0,-0xf0(%rbp)
   24dfc:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   24e03:	48 c7 85 30 ff ff ff 	movq   $0x0,-0xd0(%rbp)
   24e0a:	00 00 00 00 
   24e0e:	49 c1 e5 09          	shl    $0x9,%r13
   24e12:	41 c1 e4 03          	shl    $0x3,%r12d
   24e16:	4d 09 ec             	or     %r13,%r12
   24e19:	49 0f cc             	bswap  %r12
   24e1c:	4c 89 a5 38 ff ff ff 	mov    %r12,-0xc8(%rbp)
   24e23:	48 8b 85 30 ff ff ff 	mov    -0xd0(%rbp),%rax
   24e2a:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   24e2e:	48 8b 85 38 ff ff ff 	mov    -0xc8(%rbp),%rax
   24e35:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   24e39:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   24e40:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   24e47:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   24e4e:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   24e52:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   24e56:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   24e5a:	4c 8d 3d 5f bb 00 00 	lea    0xbb5f(%rip),%r15        # 309c0 <sha2::sha256::compress256>
   24e61:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   24e65:	ba 01 00 00 00       	mov    $0x1,%edx
   24e6a:	4c 89 f7             	mov    %r14,%rdi
   24e6d:	41 ff d7             	call   *%r15
   24e70:	48 8d bd 68 ff ff ff 	lea    -0x98(%rbp),%rdi
   24e77:	66 0f 6f 85 40 ff ff 	movdqa -0xc0(%rbp),%xmm0
   24e7e:	ff 
   24e7f:	66 0f 6f 8d 50 ff ff 	movdqa -0xb0(%rbp),%xmm1
   24e86:	ff 
   24e87:	66 0f ef db          	pxor   %xmm3,%xmm3
   24e8b:	66 0f 6f d0          	movdqa %xmm0,%xmm2
   24e8f:	66 0f 68 d3          	punpckhbw %xmm3,%xmm2
   24e93:	f2 0f 70 d2 1b       	pshuflw $0x1b,%xmm2,%xmm2
   24e98:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   24e9d:	66 0f 60 c3          	punpcklbw %xmm3,%xmm0
   24ea1:	f2 0f 70 c0 1b       	pshuflw $0x1b,%xmm0,%xmm0
   24ea6:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   24eab:	66 0f 67 c2          	packuswb %xmm2,%xmm0
   24eaf:	66                   	data16

=== A=0x4022660 F=0x22660 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000022460 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x520>:
   22460:	4c 89 f6             	mov    %r14,%rsi
   22463:	e8 f8 7a 00 00       	call   29f60 <<poly1305::backend::autodetect::State as universal_hash::UniversalHash>::finalize>
   22468:	0f b6 44 24 50       	movzbl 0x50(%rsp),%eax
   2246d:	31 ff                	xor    %edi,%edi
   2246f:	41 3a 04 1f          	cmp    (%r15,%rbx,1),%al
   22473:	40 0f 94 c7          	sete   %dil
   22477:	ff 15 0b 18 06 00    	call   *0x6180b(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2247d:	88 44 24 40          	mov    %al,0x40(%rsp)
   22481:	0f b6 44 24 51       	movzbl 0x51(%rsp),%eax
   22486:	31 ff                	xor    %edi,%edi
   22488:	41 3a 44 1f 01       	cmp    0x1(%r15,%rbx,1),%al
   2248d:	40 0f 94 c7          	sete   %dil
   22491:	ff 15 f1 17 06 00    	call   *0x617f1(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22497:	88 44 24 38          	mov    %al,0x38(%rsp)
   2249b:	0f b6 44 24 52       	movzbl 0x52(%rsp),%eax
   224a0:	31 ff                	xor    %edi,%edi
   224a2:	41 3a 44 1f 02       	cmp    0x2(%r15,%rbx,1),%al
   224a7:	40 0f 94 c7          	sete   %dil
   224ab:	ff 15 d7 17 06 00    	call   *0x617d7(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   224b1:	88 44 24 48          	mov    %al,0x48(%rsp)
   224b5:	0f b6 44 24 53       	movzbl 0x53(%rsp),%eax
   224ba:	31 ff                	xor    %edi,%edi
   224bc:	41 3a 44 1f 03       	cmp    0x3(%r15,%rbx,1),%al
   224c1:	40 0f 94 c7          	sete   %dil
   224c5:	ff 15 bd 17 06 00    	call   *0x617bd(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   224cb:	88 44 24 27          	mov    %al,0x27(%rsp)
   224cf:	0f b6 44 24 54       	movzbl 0x54(%rsp),%eax
   224d4:	31 ff                	xor    %edi,%edi
   224d6:	41 3a 44 1f 04       	cmp    0x4(%r15,%rbx,1),%al
   224db:	40 0f 94 c7          	sete   %dil
   224df:	ff 15 a3 17 06 00    	call   *0x617a3(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   224e5:	41 89 c5             	mov    %eax,%r13d
   224e8:	0f b6 44 24 55       	movzbl 0x55(%rsp),%eax
   224ed:	31 ff                	xor    %edi,%edi
   224ef:	41 3a 44 1f 05       	cmp    0x5(%r15,%rbx,1),%al
   224f4:	40 0f 94 c7          	sete   %dil
   224f8:	ff 15 8a 17 06 00    	call   *0x6178a(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   224fe:	88 44 24 26          	mov    %al,0x26(%rsp)
   22502:	0f b6 44 24 56       	movzbl 0x56(%rsp),%eax
   22507:	31 ff                	xor    %edi,%edi
   22509:	41 3a 44 1f 06       	cmp    0x6(%r15,%rbx,1),%al
   2250e:	40 0f 94 c7          	sete   %dil
   22512:	ff 15 70 17 06 00    	call   *0x61770(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22518:	88 44 24 25          	mov    %al,0x25(%rsp)
   2251c:	0f b6 44 24 57       	movzbl 0x57(%rsp),%eax
   22521:	31 ff                	xor    %edi,%edi
   22523:	41 3a 44 1f 07       	cmp    0x7(%r15,%rbx,1),%al
   22528:	40 0f 94 c7          	sete   %dil
   2252c:	ff 15 56 17 06 00    	call   *0x61756(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22532:	41 89 c6             	mov    %eax,%r14d
   22535:	0f b6 44 24 58       	movzbl 0x58(%rsp),%eax
   2253a:	31 ff                	xor    %edi,%edi
   2253c:	41 3a 44 1f 08       	cmp    0x8(%r15,%rbx,1),%al
   22541:	40 0f 94 c7          	sete   %dil
   22545:	ff 15 3d 17 06 00    	call   *0x6173d(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2254b:	88 44 24 24          	mov    %al,0x24(%rsp)
   2254f:	0f b6 44 24 59       	movzbl 0x59(%rsp),%eax
   22554:	31 ff                	xor    %edi,%edi
   22556:	41 3a 44 1f 09       	cmp    0x9(%r15,%rbx,1),%al
   2255b:	40 0f 94 c7          	sete   %dil
   2255f:	ff 15 23 17 06 00    	call   *0x61723(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22565:	88 44 24 23          	mov    %al,0x23(%rsp)
   22569:	0f b6 44 24 5a       	movzbl 0x5a(%rsp),%eax
   2256e:	31 ff                	xor    %edi,%edi
   22570:	41 3a 44 1f 0a       	cmp    0xa(%r15,%rbx,1),%al
   22575:	40 0f 94 c7          	sete   %dil
   22579:	ff 15 09 17 06 00    	call   *0x61709(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2257f:	88 44 24 22          	mov    %al,0x22(%rsp)
   22583:	0f b6 44 24 5b       	movzbl 0x5b(%rsp),%eax
   22588:	31 ff                	xor    %edi,%edi
   2258a:	41 3a 44 1f 0b       	cmp    0xb(%r15,%rbx,1),%al
   2258f:	40 0f 94 c7          	sete   %dil
   22593:	ff 15 ef 16 06 00    	call   *0x616ef(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22599:	41 89 c4             	mov    %eax,%r12d
   2259c:	0f b6 44 24 5c       	movzbl 0x5c(%rsp),%eax
   225a1:	31 ff                	xor    %edi,%edi
   225a3:	41 3a 44 1f 0c       	cmp    0xc(%r15,%rbx,1),%al
   225a8:	40 0f 94 c7          	sete   %dil
   225ac:	ff 15 d6 16 06 00    	call   *0x616d6(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   225b2:	88 44 24 21          	mov    %al,0x21(%rsp)
   225b6:	0f b6 44 24 5d       	movzbl 0x5d(%rsp),%eax
   225bb:	31 ff                	xor    %edi,%edi
   225bd:	41 3a 44 1f 0d       	cmp    0xd(%r15,%rbx,1),%al
   225c2:	40 0f 94 c7          	sete   %dil
   225c6:	ff 15 bc 16 06 00    	call   *0x616bc(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   225cc:	88 44 24 20          	mov    %al,0x20(%rsp)
   225d0:	0f b6 44 24 5e       	movzbl 0x5e(%rsp),%eax
   225d5:	31 ff                	xor    %edi,%edi
   225d7:	41 3a 44 1f 0e       	cmp    0xe(%r15,%rbx,1),%al
   225dc:	40 0f 94 c7          	sete   %dil
   225e0:	ff 15 a2 16 06 00    	call   *0x616a2(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   225e6:	88 44 24 1f          	mov    %al,0x1f(%rsp)
   225ea:	0f b6 44 24 5f       	movzbl 0x5f(%rsp),%eax
   225ef:	31 ff                	xor    %edi,%edi
   225f1:	41 3a 44 1f 0f       	cmp    0xf(%r15,%rbx,1),%al
   225f6:	40 0f 94 c7          	sete   %dil
   225fa:	ff 15 88 16 06 00    	call   *0x61688(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22600:	0f b6 4c 24 40       	movzbl 0x40(%rsp),%ecx
   22605:	22 4c 24 38          	and    0x38(%rsp),%cl
   22609:	0f b6 54 24 48       	movzbl 0x48(%rsp),%edx
   2260e:	22 54 24 27          	and    0x27(%rsp),%dl
   22612:	20 ca                	and    %cl,%dl
   22614:	44 22 6c 24 26       	and    0x26(%rsp),%r13b
   22619:	44 22 6c 24 25       	and    0x25(%rsp),%r13b
   2261e:	41 20 d5             	and    %dl,%r13b
   22621:	44 22 74 24 24       	and    0x24(%rsp),%r14b
   22626:	44 22 74 24 23       	and    0x23(%rsp),%r14b
   2262b:	44 22 74 24 22       	and    0x22(%rsp),%r14b
   22630:	45 20 ee             	and    %r13b,%r14b
   22633:	44 22 64 24 21       	and    0x21(%rsp),%r12b
   22638:	44 22 64 24 20       	and    0x20(%rsp),%r12b
   2263d:	44 22 64 24 1f       	and    0x1f(%rsp),%r12b
   22642:	41 20 c4             	and    %al,%r12b
   22645:	45 20 f4             	and    %r14b,%r12b
   22648:	41 80 e4 01          	and    $0x1,%r12b
   2264c:	41 0f b6 fc          	movzbl %r12b,%edi
   22650:	ff 15 32 16 06 00    	call   *0x61632(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   22656:	84 c0                	test   %al,%al
   22658:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   2265f:	00 
   22660:	74 42                	je     226a4 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x764>
   22662:	4c 8d b4 24 20 05 00 	lea    0x520(%rsp),%r14
   22669:	00 
   2266a:	4c 89 f7             	mov    %r14,%rdi
   2266d:	4c 89 fe             	mov    %r15,%rsi
   22670:	48 8b 54 24 28       	mov    0x28(%rsp),%rdx
   22675:	e8 06 2d 00 00       	call   25380 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as cipher::stream::StreamCipher>::apply_keystream>
   2267a:	4c 89 f7             	mov    %r14,%rdi
   2267d:	e8 fe e5 ff ff       	call   20c80 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   22682:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   22689:	00 
   2268a:	48 8b 45 18          	mov    0x18(%rbp),%rax
   2268e:	48 8b 4c 24 28       	mov    0x28(%rsp),%rcx
   22693:	48 39 c1             	cmp    %rax,%rcx
   22696:	48 0f 43 c8          	cmovae %rax,%rcx
   2269a:	4d 89 7d 08          	mov    %r15,0x8(%r13)
   2269e:	49 89 4d 10          	mov    %rcx,0x10(%r13)
   226a2:	eb 22                	jmp    226c6 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x786>
   226a4:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   226ab:	00 
   226ac:	e8 cf e5 ff ff       	call   20c80 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   226b1:	4c 89 ff             	mov    %r15,%rdi
   226b4:	ff 15 96 10 06 00    	call   *0x61096(%rip)        # 83750 <free@GLIBC_2.2.5>
   226ba:	41 c6 45 08 00       	movb   $0x0,0x8(%r13)
   226bf:	48 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%rax
   226c6:	49 89 45 00          	mov    %rax,0x0(%r13)
   226ca:	4c 89 e8             	mov    %r13,%rax
   226cd:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   226d1:	5b                   	pop    %rbx
   226d2:	41 5c                	pop    %r12
   226d4:	41 5d                	pop    %r13
   226d6:	41 5e                	pop    %r14
   226d8:	41 5f                	pop    %r15
   226da:	5d                   	pop    %rbp
   226db:	c3                   	ret
   226dc:	48 8d 15 45 f2 04 00 	lea    0x4f245(%rip),%rdx        # 71928 <core::unicode::unicode_data::cf::SHORT_OFFSET_RUNS+0x70>
   226e3:	4c 8d 0d 1e f3 05 00 	lea    0x5f31e(%rip),%r9        # 81a08 <anon.d49aa5e532bb8905898215d271d902c3.14.llvm.14046659981651887190+0x70>
   226ea:	48 8d b4 24 e0 02 00 	lea    0x2e0(%rsp),%rsi
   226f1:	00 
   226f2:	31 ff                	xor    %edi,%edi
   226f4:	31 c9                	xor    %ecx,%ecx
   226f6:	ff 15 9c 15 06 00    	call   *0x6159c(%rip)        # 83c98 <_GLOBAL_OFFSET_TABLE_+0x5a0>
   226fc:	ff 15 ee 10 06 00    	call   *0x610ee(%rip)        # 837f0 <_GLOBAL_OFFSET_TABLE_+0xf8>
   22702:	e9 00 f9 ff ff       	jmp    22007 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0xc7>
   22707:	bf 01 00 00 00       	mov    $0x1,%edi
   2270c:	48 89 de             	mov    %rbx,%rsi
   2270f:	ff 15 5b 17 06 00    	call   *0x6175b(%rip)        # 83e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   22715:	49 89 c6             	mov    %rax,%r14
   22718:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   2271d:	ff 15 2d 10 06 00    	call   *0x6102d(%rip)        # 83750 <free@GLIBC_2.2.5>
   22723:	4c 89 f7             	mov    %r14,%rdi
   22726:	e8 15 39 fe ff       	call   6040 <_Unwind_Resume@plt>
   2272b:	eb 06                	jmp    22733 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   2272d:	eb 04                	jmp    22733 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   2272f:	eb 02                	jmp    22733 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   22731:	eb 00                	jmp    22733 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   22733:	49 89 c6             	mov    %rax,%r14
   22736:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   2273d:	00 
   2273e:	e8 3d e5 ff ff       	call   20c80 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   22743:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   22748:	ff 15 02 10 06 00    	call   *0x61002(%rip)        # 83750 <free@GLIBC_2.2.5>
   2274e:	4c 89 f7             	mov    %r14,%rdi
   22751:	e8 ea 38 fe ff       	call   6040 <_Unwind_Resume@plt>
   22756:	ff 15 6c 18 06 00    	call   *0x6186c(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   2275c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000022760 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal>:
   22760:	55                   	push   %rbp
   22761:	48 89 e5             	mov    %rsp,%rbp
   22764:	41 57                	push   %r15
   22766:	41 56                	push   %r14
   22768:	41 55                	push   %r13
   2276a:	41 54                	push   %r12
   2276c:	53                   	push   %rbx
   2276d:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   22771:	48 81 ec c0 05 00 00 	sub    $0x5c0,%rsp
   22778:	4c 89 44 24 08       	mov    %r8,0x8(%rsp)
   2277d:	48 89 8c 24 c0 02 00 	mov    %rcx,0x2c0(%rsp)
   22784:	00 
   22785:	48 83 f9 0c          	cmp    $0xc,%rcx
   22789:	0f 85 74 05 00 00    	jne    22d03 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5a3>
   2278f:	48 89 fb             	mov    %rdi,%rbx
   22792:	48 8b 7d 18          	mov    0x18(%rbp),%rdi
   22796:	48 83 c7 10          	add    $0x10,%rdi
   2279a:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
   2279f:	0f 88 89 05 00 00    	js     22d2e <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5ce>
   227a5:	4d 89 cf             	mov    %r9,%r15
   227a8:	49 89 d6             	mov    %rdx,%r14
   227ab:	49 89 f4             	mov    %rsi,%r12
   227ae:	ff 15 bc 14 06 00    	call   *0x614bc(%rip)        # 83c70 <malloc@GLIBC_2.2.5>
   227b4:	48 85 c0             	test   %rax,%rax
   227b7:	0f 84 7e 05 00 00    	je     22d3b <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5db>
   227bd:	49 89 c5             	mov    %rax,%r13
   227c0:	48 89 5c 24 28       	mov    %rbx,0x28(%rsp)
   227c5:	48 8b 5d 18          	mov    0x18(%rbp),%rbx
   227c9:	48 85 db             	test   %rbx,%rbx
   227cc:	74 10                	je     227de <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x7e>
   227ce:	48 8b 75 10          	mov    0x10(%rbp),%rsi
   227d2:	4c 89 ef             	mov    %r13,%rdi
   227d5:	48 89 da             	mov    %rbx,%rdx
   227d8:	ff 15 ba 13 06 00    	call   *0x613ba(%rip)        # 83b98 <memcpy@GLIBC_2.14>
   227de:	0f 10 05 9b c3 04 00 	movups 0x4c39b(%rip),%xmm0        # 6eb80 <anon.10746075dd51319079efedc96a688e81.1.llvm.4247047481296140766+0xad>
   227e5:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   227ea:	41 0f 10 04 24       	movups (%r12),%xmm0
   227ef:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   227f5:	0f 29 44 24 50       	movaps %xmm0,0x50(%rsp)
   227fa:	0f 29 4c 24 60       	movaps %xmm1,0x60(%rsp)
   227ff:	49 8b 06             	mov    (%r14),%rax
   22802:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
   22807:	41 8b 46 08          	mov    0x8(%r14),%eax
   2280b:	89 44 24 38          	mov    %eax,0x38(%rsp)
   2280f:	48 8d 05 fa 17 06 00 	lea    0x617fa(%rip),%rax        # 84010 <chacha20::avx2_cpuid::STORAGE>
   22816:	0f b6 00             	movzbl (%rax),%eax
   22819:	3c ff                	cmp    $0xff,%al
   2281b:	4c 89 2c 24          	mov    %r13,(%rsp)
   2281f:	49 89 de             	mov    %rbx,%r14
   22822:	0f 84 fb 04 00 00    	je     22d23 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5c3>
   22828:	0f 57 c0             	xorps  %xmm0,%xmm0
   2282b:	0f 29 84 24 f0 00 00 	movaps %xmm0,0xf0(%rsp)
   22832:	00 
   22833:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
   2283a:	00 
   2283b:	0f 29 84 24 d0 00 00 	movaps %xmm0,0xd0(%rsp)
   22842:	00 
   22843:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   2284a:	00 
   2284b:	0f 28 44 24 40       	movaps 0x40(%rsp),%xmm0
   22850:	0f 28 4c 24 50       	movaps 0x50(%rsp),%xmm1
   22855:	0f 28 54 24 60       	movaps 0x60(%rsp),%xmm2
   2285a:	0f                   	.byte 0xf
   2285b:	29                   	.byte 0x29
   2285c:	84 24 80             	test   %ah,(%rax,%rax,4)
	...

=== A=0x403C5E4 F=0x3c5e4 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000003c3e4 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x4c4>:
   3c3e4:	e8 77 db fe ff       	call   29f60 <<poly1305::backend::autodetect::State as universal_hash::UniversalHash>::finalize>
   3c3e9:	0f b6 44 24 70       	movzbl 0x70(%rsp),%eax
   3c3ee:	31 ff                	xor    %edi,%edi
   3c3f0:	41 3a 04 1c          	cmp    (%r12,%rbx,1),%al
   3c3f4:	40 0f 94 c7          	sete   %dil
   3c3f8:	ff 15 8a 78 04 00    	call   *0x4788a(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c3fe:	88 44 24 60          	mov    %al,0x60(%rsp)
   3c402:	0f b6 44 24 71       	movzbl 0x71(%rsp),%eax
   3c407:	31 ff                	xor    %edi,%edi
   3c409:	41 3a 44 1c 01       	cmp    0x1(%r12,%rbx,1),%al
   3c40e:	40 0f 94 c7          	sete   %dil
   3c412:	ff 15 70 78 04 00    	call   *0x47870(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c418:	88 44 24 0f          	mov    %al,0xf(%rsp)
   3c41c:	0f b6 44 24 72       	movzbl 0x72(%rsp),%eax
   3c421:	31 ff                	xor    %edi,%edi
   3c423:	41 3a 44 1c 02       	cmp    0x2(%r12,%rbx,1),%al
   3c428:	40 0f 94 c7          	sete   %dil
   3c42c:	ff 15 56 78 04 00    	call   *0x47856(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c432:	88 44 24 0e          	mov    %al,0xe(%rsp)
   3c436:	0f b6 44 24 73       	movzbl 0x73(%rsp),%eax
   3c43b:	31 ff                	xor    %edi,%edi
   3c43d:	41 3a 44 1c 03       	cmp    0x3(%r12,%rbx,1),%al
   3c442:	40 0f 94 c7          	sete   %dil
   3c446:	ff 15 3c 78 04 00    	call   *0x4783c(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c44c:	88 44 24 0d          	mov    %al,0xd(%rsp)
   3c450:	0f b6 44 24 74       	movzbl 0x74(%rsp),%eax
   3c455:	31 ff                	xor    %edi,%edi
   3c457:	41 3a 44 1c 04       	cmp    0x4(%r12,%rbx,1),%al
   3c45c:	40 0f 94 c7          	sete   %dil
   3c460:	ff 15 22 78 04 00    	call   *0x47822(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c466:	88 44 24 0c          	mov    %al,0xc(%rsp)
   3c46a:	0f b6 44 24 75       	movzbl 0x75(%rsp),%eax
   3c46f:	31 ff                	xor    %edi,%edi
   3c471:	41 3a 44 1c 05       	cmp    0x5(%r12,%rbx,1),%al
   3c476:	40 0f 94 c7          	sete   %dil
   3c47a:	ff 15 08 78 04 00    	call   *0x47808(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c480:	88 44 24 0b          	mov    %al,0xb(%rsp)
   3c484:	0f b6 44 24 76       	movzbl 0x76(%rsp),%eax
   3c489:	31 ff                	xor    %edi,%edi
   3c48b:	41 3a 44 1c 06       	cmp    0x6(%r12,%rbx,1),%al
   3c490:	40 0f 94 c7          	sete   %dil
   3c494:	ff 15 ee 77 04 00    	call   *0x477ee(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c49a:	88 44 24 0a          	mov    %al,0xa(%rsp)
   3c49e:	0f b6 44 24 77       	movzbl 0x77(%rsp),%eax
   3c4a3:	31 ff                	xor    %edi,%edi
   3c4a5:	41 3a 44 1c 07       	cmp    0x7(%r12,%rbx,1),%al
   3c4aa:	40 0f 94 c7          	sete   %dil
   3c4ae:	ff 15 d4 77 04 00    	call   *0x477d4(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c4b4:	41 89 c5             	mov    %eax,%r13d
   3c4b7:	0f b6 44 24 78       	movzbl 0x78(%rsp),%eax
   3c4bc:	31 ff                	xor    %edi,%edi
   3c4be:	41 3a 44 1c 08       	cmp    0x8(%r12,%rbx,1),%al
   3c4c3:	40 0f 94 c7          	sete   %dil
   3c4c7:	ff 15 bb 77 04 00    	call   *0x477bb(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c4cd:	88 44 24 09          	mov    %al,0x9(%rsp)
   3c4d1:	0f b6 44 24 79       	movzbl 0x79(%rsp),%eax
   3c4d6:	31 ff                	xor    %edi,%edi
   3c4d8:	41 3a 44 1c 09       	cmp    0x9(%r12,%rbx,1),%al
   3c4dd:	40 0f 94 c7          	sete   %dil
   3c4e1:	ff 15 a1 77 04 00    	call   *0x477a1(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c4e7:	88 44 24 08          	mov    %al,0x8(%rsp)
   3c4eb:	0f b6 44 24 7a       	movzbl 0x7a(%rsp),%eax
   3c4f0:	31 ff                	xor    %edi,%edi
   3c4f2:	41 3a 44 1c 0a       	cmp    0xa(%r12,%rbx,1),%al
   3c4f7:	40 0f 94 c7          	sete   %dil
   3c4fb:	ff 15 87 77 04 00    	call   *0x47787(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c501:	88 44 24 07          	mov    %al,0x7(%rsp)
   3c505:	0f b6 44 24 7b       	movzbl 0x7b(%rsp),%eax
   3c50a:	31 ff                	xor    %edi,%edi
   3c50c:	41 3a 44 1c 0b       	cmp    0xb(%r12,%rbx,1),%al
   3c511:	40 0f 94 c7          	sete   %dil
   3c515:	ff 15 6d 77 04 00    	call   *0x4776d(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c51b:	41 89 c6             	mov    %eax,%r14d
   3c51e:	0f b6 44 24 7c       	movzbl 0x7c(%rsp),%eax
   3c523:	31 ff                	xor    %edi,%edi
   3c525:	41 3a 44 1c 0c       	cmp    0xc(%r12,%rbx,1),%al
   3c52a:	40 0f 94 c7          	sete   %dil
   3c52e:	ff 15 54 77 04 00    	call   *0x47754(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c534:	88 44 24 06          	mov    %al,0x6(%rsp)
   3c538:	0f b6 44 24 7d       	movzbl 0x7d(%rsp),%eax
   3c53d:	31 ff                	xor    %edi,%edi
   3c53f:	41 3a 44 1c 0d       	cmp    0xd(%r12,%rbx,1),%al
   3c544:	40 0f 94 c7          	sete   %dil
   3c548:	ff 15 3a 77 04 00    	call   *0x4773a(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c54e:	88 44 24 05          	mov    %al,0x5(%rsp)
   3c552:	0f b6 44 24 7e       	movzbl 0x7e(%rsp),%eax
   3c557:	31 ff                	xor    %edi,%edi
   3c559:	41 3a 44 1c 0e       	cmp    0xe(%r12,%rbx,1),%al
   3c55e:	40 0f 94 c7          	sete   %dil
   3c562:	ff 15 20 77 04 00    	call   *0x47720(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c568:	88 44 24 04          	mov    %al,0x4(%rsp)
   3c56c:	0f b6 44 24 7f       	movzbl 0x7f(%rsp),%eax
   3c571:	31 ff                	xor    %edi,%edi
   3c573:	41 3a 44 1c 0f       	cmp    0xf(%r12,%rbx,1),%al
   3c578:	40 0f 94 c7          	sete   %dil
   3c57c:	ff 15 06 77 04 00    	call   *0x47706(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c582:	0f b6 4c 24 60       	movzbl 0x60(%rsp),%ecx
   3c587:	22 4c 24 0f          	and    0xf(%rsp),%cl
   3c58b:	0f b6 54 24 0e       	movzbl 0xe(%rsp),%edx
   3c590:	22 54 24 0d          	and    0xd(%rsp),%dl
   3c594:	20 ca                	and    %cl,%dl
   3c596:	0f b6 4c 24 0c       	movzbl 0xc(%rsp),%ecx
   3c59b:	22 4c 24 0b          	and    0xb(%rsp),%cl
   3c59f:	22 4c 24 0a          	and    0xa(%rsp),%cl
   3c5a3:	20 d1                	and    %dl,%cl
   3c5a5:	44 22 6c 24 09       	and    0x9(%rsp),%r13b
   3c5aa:	44 22 6c 24 08       	and    0x8(%rsp),%r13b
   3c5af:	44 22 6c 24 07       	and    0x7(%rsp),%r13b
   3c5b4:	41 20 cd             	and    %cl,%r13b
   3c5b7:	44 22 74 24 06       	and    0x6(%rsp),%r14b
   3c5bc:	44 22 74 24 05       	and    0x5(%rsp),%r14b
   3c5c1:	44 22 74 24 04       	and    0x4(%rsp),%r14b
   3c5c6:	41 20 c6             	and    %al,%r14b
   3c5c9:	45 20 ee             	and    %r13b,%r14b
   3c5cc:	41 80 e6 01          	and    $0x1,%r14b
   3c5d0:	41 0f b6 fe          	movzbl %r14b,%edi
   3c5d4:	ff 15 ae 76 04 00    	call   *0x476ae(%rip)        # 83c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   3c5da:	84 c0                	test   %al,%al
   3c5dc:	4c 8d ac 24 80 05 00 	lea    0x580(%rsp),%r13
   3c5e3:	00 
   3c5e4:	74 3b                	je     3c621 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x701>
   3c5e6:	4c 89 ef             	mov    %r13,%rdi
   3c5e9:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
   3c5ee:	48 89 da             	mov    %rbx,%rdx
   3c5f1:	e8 8a 27 00 00       	call   3ed80 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as cipher::stream::StreamCipher>::apply_keystream>
   3c5f6:	48 8d bc 24 80 05 00 	lea    0x580(%rsp),%rdi
   3c5fd:	00 
   3c5fe:	e8 3d eb ff ff       	call   3b140 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   3c603:	48 8b 44 24 68       	mov    0x68(%rsp),%rax
   3c608:	48 39 c3             	cmp    %rax,%rbx
   3c60b:	48 0f 43 d8          	cmovae %rax,%rbx
   3c60f:	49 89 07             	mov    %rax,(%r15)
   3c612:	48 8b 44 24 58       	mov    0x58(%rsp),%rax
   3c617:	49 89 47 08          	mov    %rax,0x8(%r15)
   3c61b:	49 89 5f 10          	mov    %rbx,0x10(%r15)
   3c61f:	eb 1d                	jmp    3c63e <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x71e>
   3c621:	4c 89 ef             	mov    %r13,%rdi
   3c624:	e8 17 eb ff ff       	call   3b140 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   3c629:	4c 8b 6c 24 58       	mov    0x58(%rsp),%r13
   3c62e:	49 c7 07 ff ff ff ff 	movq   $0xffffffffffffffff,(%r15)
   3c635:	4c 89 ef             	mov    %r13,%rdi
   3c638:	ff 15 12 71 04 00    	call   *0x47112(%rip)        # 83750 <free@GLIBC_2.2.5>
   3c63e:	c6 44 24 30 00       	movb   $0x0,0x30(%rsp)
   3c643:	c6 44 24 31 00       	movb   $0x0,0x31(%rsp)
   3c648:	c6 44 24 32 00       	movb   $0x0,0x32(%rsp)
   3c64d:	c6 44 24 33 00       	movb   $0x0,0x33(%rsp)
   3c652:	c6 44 24 34 00       	movb   $0x0,0x34(%rsp)
   3c657:	c6 44 24 35 00       	movb   $0x0,0x35(%rsp)
   3c65c:	c6 44 24 36 00       	movb   $0x0,0x36(%rsp)
   3c661:	c6 44 24 37 00       	movb   $0x0,0x37(%rsp)
   3c666:	c6 44 24 38 00       	movb   $0x0,0x38(%rsp)
   3c66b:	c6 44 24 39 00       	movb   $0x0,0x39(%rsp)
   3c670:	c6 44 24 3a 00       	movb   $0x0,0x3a(%rsp)
   3c675:	c6 44 24 3b 00       	movb   $0x0,0x3b(%rsp)
   3c67a:	c6 44 24 3c 00       	movb   $0x0,0x3c(%rsp)
   3c67f:	c6 44 24 3d 00       	movb   $0x0,0x3d(%rsp)
   3c684:	c6 44 24 3e 00       	movb   $0x0,0x3e(%rsp)
   3c689:	c6 44 24 3f 00       	movb   $0x0,0x3f(%rsp)
   3c68e:	c6 44 24 40 00       	movb   $0x0,0x40(%rsp)
   3c693:	c6 44 24 41 00       	movb   $0x0,0x41(%rsp)
   3c698:	c6 44 24 42 00       	movb   $0x0,0x42(%rsp)
   3c69d:	c6 44 24 43 00       	movb   $0x0,0x43(%rsp)
   3c6a2:	c6 44 24 44 00       	movb   $0x0,0x44(%rsp)
   3c6a7:	c6 44 24 45 00       	movb   $0x0,0x45(%rsp)
   3c6ac:	c6 44 24 46 00       	movb   $0x0,0x46(%rsp)
   3c6b1:	c6 44 24 47 00       	movb   $0x0,0x47(%rsp)
   3c6b6:	c6 44 24 48 00       	movb   $0x0,0x48(%rsp)
   3c6bb:	c6 44 24 49 00       	movb   $0x0,0x49(%rsp)
   3c6c0:	c6 44 24 4a 00       	movb   $0x0,0x4a(%rsp)
   3c6c5:	c6 44 24 4b 00       	movb   $0x0,0x4b(%rsp)
   3c6ca:	c6 44 24 4c 00       	movb   $0x0,0x4c(%rsp)
   3c6cf:	c6 44 24 4d 00       	movb   $0x0,0x4d(%rsp)
   3c6d4:	c6 44 24 4e 00       	movb   $0x0,0x4e(%rsp)
   3c6d9:	c6 44 24 4f 00       	movb   $0x0,0x4f(%rsp)
   3c6de:	48 8d 44 24 30       	lea    0x30(%rsp),%rax
   3c6e3:	4c 89 f8             	mov    %r15,%rax
   3c6e6:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   3c6ea:	5b                   	pop    %rbx
   3c6eb:	41 5c                	pop    %r12
   3c6ed:	41 5d                	pop    %r13
   3c6ef:	41 5e                	pop    %r14
   3c6f1:	41 5f                	pop    %r15
   3c6f3:	5d                   	pop    %rbp
   3c6f4:	c3                   	ret
   3c6f5:	ff 15 f5 70 04 00    	call   *0x470f5(%rip)        # 837f0 <_GLOBAL_OFFSET_TABLE_+0xf8>
   3c6fb:	e9 08 f9 ff ff       	jmp    3c008 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0xe8>
   3c700:	bf 01 00 00 00       	mov    $0x1,%edi
   3c705:	4c 89 fe             	mov    %r15,%rsi
   3c708:	ff 15 62 77 04 00    	call   *0x47762(%rip)        # 83e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   3c70e:	0f 0b                	ud2
   3c710:	48 89 c3             	mov    %rax,%rbx
   3c713:	eb 2e                	jmp    3c743 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x823>
   3c715:	48 89 c3             	mov    %rax,%rbx
   3c718:	48 8d 7c 24 70       	lea    0x70(%rsp),%rdi
   3c71d:	e8 1e ea ff ff       	call   3b140 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   3c722:	eb 1f                	jmp    3c743 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x823>
   3c724:	ff 15 9e 78 04 00    	call   *0x4789e(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3c72a:	48 89 c3             	mov    %rax,%rbx
   3c72d:	eb 1f                	jmp    3c74e <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x82e>
   3c72f:	eb 02                	jmp    3c733 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x813>
   3c731:	eb 00                	jmp    3c733 <<star_core::secure_channel::client::SecureChannelClient>::decrypt_with_shared_key+0x813>
   3c733:	48 89 c3             	mov    %rax,%rbx
   3c736:	48 8d bc 24 80 05 00 	lea    0x580(%rsp),%rdi
   3c73d:	00 
   3c73e:	e8 fd e9 ff ff       	call   3b140 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   3c743:	48 8b 7c 24 58       	mov    0x58(%rsp),%rdi
   3c748:	ff 15 02 70 04 00    	call   *0x47002(%rip)        # 83750 <free@GLIBC_2.2.5>
   3c74e:	48 8d 7c 24 30       	lea    0x30(%rsp),%rdi
   3c753:	e8 b8 e7 ff ff       	call   3af10 <core::ptr::drop_glue::<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>>
   3c758:	48 89 df             	mov    %rbx,%rdi
   3c75b:	e8 e0 98 fc ff       	call   6040 <_Unwind_Resume@plt>
   3c760:	ff 15 62 78 04 00    	call   *0x47862(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3c766:	ff 15 5c 78 04 00    	call   *0x4785c(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3c76c:	0f 1f 40 00          	nopl   0x0(%rax)

000000000003c770 <<star_core::secure_channel::server::SecureChannelServer>::receive_hpke>:
   3c770:	55                   	push   %rbp
   3c771:	48 89 e5             	mov    %rsp,%rbp
   3c774:	41 57                	push   %r15
   3c776:	41 56                	push   %r14
   3c778:	41 55                	push   %r13
   3c77a:	41 54                	push   %r12
   3c77c:	53                   	push   %rbx
   3c77d:	48 81 ec c8 02 00 00 	sub    $0x2c8,%rsp
   3c784:	49 89 fd             	mov    %rdi,%r13
   3c787:	48 83 f9 20          	cmp    $0x20,%rcx
   3c78b:	0f 85 38 03 00 00    	jne    3cac9 <<star_core::secure_channel::server::SecureChannelServer>::receive_hpke+0x359>
   3c791:	48 89 d3             	mov    %rdx,%rbx
   3c794:	49 89 f6             	mov    %rsi,%r14
   3c797:	4c 89 45 c8          	mov    %r8,-0x38(%rbp)
   3c79b:	4c 89 4d c0          	mov    %r9,-0x40(%rbp)
   3c79f:	bf 20 00 00 00       	mov    $0x20,%edi
   3c7a4:	ff 15 c6 74 04 00    	call   *0x474c6(%rip)        # 83c70 <malloc@GLIBC_2.2.5>
   3c7aa:	48 85 c0             	test   %rax,%rax
   3c7ad:	0f 84 93 15 00 00    	je     3dd46 <<star_core::secure_channel::server::SecureChannelServer>::receive_hpke+0x15d6>
   3c7b3:	49 89 c7             	mov    %rax,%r15
   3c7b6:	0f 10 03             	movups (%rbx),%xmm0
   3c7b9:	0f 10 4b 10          	movups 0x10(%rbx),%xmm1
   3c7bd:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   3c7c1:	0f 11 00             	movups %xmm0,(%rax)
   3c7c4:	0f b6 00             	movzbl (%rax),%eax
   3c7c7:	0f 10 43 01          	movups 0x1(%rbx),%xmm0
   3c7cb:	0f 10 4b 0f          	movups 0xf(%rbx),%xmm1
   3c7cf:	0f 11 4d 9f          	movups %xmm1,-0x61(%rbp)
   3c7d3:	0f 11 45 91          	movups %xmm0,-0x6f(%rbp)
   3c7d7:	41 0f b6 4f 1f       	movzbl 0x1f(%r15),%ecx
   3c7dc:	80 e1 7f             	and    $0x7f,%cl
   3c7df:	88 4d af             	mov    %cl,-0x51(%rbp)
   3c7e2:	88                   	.byte 0x88
   3c7e3:	45                   	rex.RB

=== A=0x403E37B F=0x3e37b ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000003e17b <<star_core::secure_channel::server::SecureChannelServer>::new+0x2eb>:
   3e17b:	85 00                	test   %eax,(%rax)
   3e17d:	ff                   	(bad)
   3e17e:	ff                   	(bad)
   3e17f:	ff 0f                	decl   (%rdi)
   3e181:	11 43 40             	adc    %eax,0x40(%rbx)
   3e184:	48 8b 85 f0 fe ff ff 	mov    -0x110(%rbp),%rax
   3e18b:	48 89 43 30          	mov    %rax,0x30(%rbx)
   3e18f:	48 8b 85 f8 fe ff ff 	mov    -0x108(%rbp),%rax
   3e196:	48 89 43 38          	mov    %rax,0x38(%rbx)
   3e19a:	48 8b 85 10 ff ff ff 	mov    -0xf0(%rbp),%rax
   3e1a1:	48 89 43 50          	mov    %rax,0x50(%rbx)
   3e1a5:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3e1a9:	ff 15 79 5c 04 00    	call   *0x45c79(%rip)        # 83e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3e1af:	48 89 d8             	mov    %rbx,%rax
   3e1b2:	48 81 c4 28 01 00 00 	add    $0x128,%rsp
   3e1b9:	5b                   	pop    %rbx
   3e1ba:	41 5e                	pop    %r14
   3e1bc:	41 5f                	pop    %r15
   3e1be:	5d                   	pop    %rbp
   3e1bf:	c3                   	ret
   3e1c0:	bf 01 00 00 00       	mov    $0x1,%edi
   3e1c5:	be 20 00 00 00       	mov    $0x20,%esi
   3e1ca:	ff 15 a0 5c 04 00    	call   *0x45ca0(%rip)        # 83e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   3e1d0:	0f 0b                	ud2
   3e1d2:	48 89 c3             	mov    %rax,%rbx
   3e1d5:	eb 0f                	jmp    3e1e6 <<star_core::secure_channel::server::SecureChannelServer>::new+0x356>
   3e1d7:	eb 21                	jmp    3e1fa <<star_core::secure_channel::server::SecureChannelServer>::new+0x36a>
   3e1d9:	48 89 c3             	mov    %rax,%rbx
   3e1dc:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3e1e0:	ff 15 42 5c 04 00    	call   *0x45c42(%rip)        # 83e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3e1e6:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   3e1ed:	e8 1e d1 ff ff       	call   3b310 <core::ptr::drop_glue::<hpke_ng::kem::dh::DhPrivateKey<hpke_ng::kem::dh::X25519>>>
   3e1f2:	eb 13                	jmp    3e207 <<star_core::secure_channel::server::SecureChannelServer>::new+0x377>
   3e1f4:	ff 15 ce 5d 04 00    	call   *0x45dce(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3e1fa:	48 89 c3             	mov    %rax,%rbx
   3e1fd:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3e201:	ff 15 21 5c 04 00    	call   *0x45c21(%rip)        # 83e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3e207:	48 89 df             	mov    %rbx,%rdi
   3e20a:	e8 31 7e fc ff       	call   6040 <_Unwind_Resume@plt>
   3e20f:	ff 15 b3 5d 04 00    	call   *0x45db3(%rip)        # 83fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3e215:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   3e21c:	00 00 00 
   3e21f:	90                   	nop

000000000003e220 <<star_core::serv::enclave::Enclave>::issue_token>:
   3e220:	55                   	push   %rbp
   3e221:	48 89 e5             	mov    %rsp,%rbp
   3e224:	41 57                	push   %r15
   3e226:	41 56                	push   %r14
   3e228:	41 55                	push   %r13
   3e22a:	41 54                	push   %r12
   3e22c:	53                   	push   %rbx
   3e22d:	48 83 ec 18          	sub    $0x18,%rsp
   3e231:	48 89 fb             	mov    %rdi,%rbx
   3e234:	48 83 f9 60          	cmp    $0x60,%rcx
   3e238:	0f 82 84 00 00 00    	jb     3e2c2 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   3e23e:	4d 89 c6             	mov    %r8,%r14
   3e241:	4c 8d 42 20          	lea    0x20(%rdx),%r8
   3e245:	48 8d 42 40          	lea    0x40(%rdx),%rax
   3e249:	4c 8d 52 60          	lea    0x60(%rdx),%r10
   3e24d:	48 83 c1 a0          	add    $0xffffffffffffffa0,%rcx
   3e251:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   3e255:	49 89 cb             	mov    %rcx,%r11
   3e258:	b9 20 00 00 00       	mov    $0x20,%ecx
   3e25d:	41 b9 20 00 00 00    	mov    $0x20,%r9d
   3e263:	49 89 f4             	mov    %rsi,%r12
   3e266:	49 89 d7             	mov    %rdx,%r15
   3e269:	48 89 c2             	mov    %rax,%rdx
   3e26c:	41 53                	push   %r11
   3e26e:	41 52                	push   %r10
   3e270:	6a 20                	push   $0x20
   3e272:	41 57                	push   %r15
   3e274:	ff 15 3e 57 04 00    	call   *0x4573e(%rip)        # 839b8 <_GLOBAL_OFFSET_TABLE_+0x2c0>
   3e27a:	48 83 c4 20          	add    $0x20,%rsp
   3e27e:	4c 8b 6d c0          	mov    -0x40(%rbp),%r13
   3e282:	49 83 fd ff          	cmp    $0xffffffffffffffff,%r13
   3e286:	74 3a                	je     3e2c2 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   3e288:	4c 8b 7d c8          	mov    -0x38(%rbp),%r15
   3e28c:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
   3e290:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   3e294:	4c 89 e6             	mov    %r12,%rsi
   3e297:	4c 89 fa             	mov    %r15,%rdx
   3e29a:	4d 89 f0             	mov    %r14,%r8
   3e29d:	ff 15 d5 58 04 00    	call   *0x458d5(%rip)        # 83b78 <_GLOBAL_OFFSET_TABLE_+0x480>
   3e2a3:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   3e2a7:	48 89 43 10          	mov    %rax,0x10(%rbx)
   3e2ab:	0f 10 45 c0          	movups -0x40(%rbp),%xmm0
   3e2af:	0f 11 03             	movups %xmm0,(%rbx)
   3e2b2:	4d 85 ed             	test   %r13,%r13
   3e2b5:	74 16                	je     3e2cd <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   3e2b7:	4c 89 ff             	mov    %r15,%rdi
   3e2ba:	ff 15 90 54 04 00    	call   *0x45490(%rip)        # 83750 <free@GLIBC_2.2.5>
   3e2c0:	eb 0b                	jmp    3e2cd <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   3e2c2:	c6 43 08 01          	movb   $0x1,0x8(%rbx)
   3e2c6:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   3e2cd:	48 89 d8             	mov    %rbx,%rax
   3e2d0:	48 83 c4 18          	add    $0x18,%rsp
   3e2d4:	5b                   	pop    %rbx
   3e2d5:	41 5c                	pop    %r12
   3e2d7:	41 5d                	pop    %r13
   3e2d9:	41 5e                	pop    %r14
   3e2db:	41 5f                	pop    %r15
   3e2dd:	5d                   	pop    %rbp
   3e2de:	c3                   	ret
   3e2df:	48 89 c3             	mov    %rax,%rbx
   3e2e2:	4d 85 ed             	test   %r13,%r13
   3e2e5:	74 09                	je     3e2f0 <<star_core::serv::enclave::Enclave>::issue_token+0xd0>
   3e2e7:	4c 89 ff             	mov    %r15,%rdi
   3e2ea:	ff 15 60 54 04 00    	call   *0x45460(%rip)        # 83750 <free@GLIBC_2.2.5>
   3e2f0:	48 89 df             	mov    %rbx,%rdi
   3e2f3:	e8 48 7d fc ff       	call   6040 <_Unwind_Resume@plt>
   3e2f8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   3e2ff:	00 

000000000003e300 <<star_core::serv::enclave::Enclave>::issue_token_plain>:
   3e300:	55                   	push   %rbp
   3e301:	48 89 e5             	mov    %rsp,%rbp
   3e304:	41 57                	push   %r15
   3e306:	41 56                	push   %r14
   3e308:	41 54                	push   %r12
   3e30a:	53                   	push   %rbx
   3e30b:	48 81 ec b0 00 00 00 	sub    $0xb0,%rsp
   3e312:	48 89 fb             	mov    %rdi,%rbx
   3e315:	48 83 f9 50          	cmp    $0x50,%rcx
   3e319:	72 62                	jb     3e37d <<star_core::serv::enclave::Enclave>::issue_token_plain+0x7d>
   3e31b:	49 89 f1             	mov    %rsi,%r9
   3e31e:	0f 10 42 20          	movups 0x20(%rdx),%xmm0
   3e322:	0f 10 4a 30          	movups 0x30(%rdx),%xmm1
   3e326:	0f 29 8d 60 ff ff ff 	movaps %xmm1,-0xa0(%rbp)
   3e32d:	0f 29 85 50 ff ff ff 	movaps %xmm0,-0xb0(%rbp)
   3e334:	f3 0f 6f 02          	movdqu (%rdx),%xmm0
   3e338:	f3 0f 6f 4a 10       	movdqu 0x10(%rdx),%xmm1
   3e33d:	66 0f 7f 85 30 ff ff 	movdqa %xmm0,-0xd0(%rbp)
   3e344:	ff 
   3e345:	66 0f 7f 8d 40 ff ff 	movdqa %xmm1,-0xc0(%rbp)
   3e34c:	ff 
   3e34d:	4c 8b 72 40          	mov    0x40(%rdx),%r14
   3e351:	49 89 d4             	mov    %rdx,%r12
   3e354:	4c 8b 7a 48          	mov    0x48(%rdx),%r15
   3e358:	48 8b 46 58          	mov    0x58(%rsi),%rax
   3e35c:	49 83 c1 60          	add    $0x60,%r9
   3e360:	48 8d bd 30 ff ff ff 	lea    -0xd0(%rbp),%rdi
   3e367:	4c 89 fe             	mov    %r15,%rsi
   3e36a:	4c 89 f2             	mov    %r14,%rdx
   3e36d:	4c 89 c1             	mov    %r8,%rcx
   3e370:	49 89 c0             	mov    %rax,%r8
   3e373:	ff 15 97 58 04 00    	call   *0x45897(%rip)        # 83c10 <_GLOBAL_OFFSET_TABLE_+0x518>
   3e379:	84 c0                	test   %al,%al
   3e37b:	75 3d                	jne    3e3ba <<star_core::serv::enclave::Enclave>::issue_token_plain+0xba>
   3e37d:	bf 01 00 00 00       	mov    $0x1,%edi
   3e382:	ff 15 e8 58 04 00    	call   *0x458e8(%rip)        # 83c70 <malloc@GLIBC_2.2.5>
   3e388:	48 85 c0             	test   %rax,%rax
   3e38b:	0f 84 1c 01 00 00    	je     3e4ad <<star_core::serv::enclave::Enclave>::issue_token_plain+0x1ad>
   3e391:	c6 00 00             	movb   $0x0,(%rax)
   3e394:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   3e39b:	48 89 43 08          	mov    %rax,0x8(%rbx)
   3e39f:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   3e3a6:	00 
   3e3a7:	48 89 d8             	mov    %rbx,%rax
   3e3aa:	48 81 c4 b0 00 00 00 	add    $0xb0,%rsp
   3e3b1:	5b                   	pop    %rbx
   3e3b2:	41 5c                	pop    %r12
   3e3b4:	41 5e                	pop    %r14
   3e3b6:	41 5f                	pop    %r15
   3e3b8:	5d                   	pop    %rbp
   3e3b9:	c3                   	ret
   3e3ba:	41 0f 10 04 24       	movups (%r12),%xmm0
   3e3bf:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   3e3c5:	0f 11 4d b8          	movups %xmm1,-0x48(%rbp)
   3e3c9:	0f 11 45 a8          	movups %xmm0,-0x58(%rbp)
   3e3cd:	0f 10 05 cc 0c 03 00 	movups 0x30ccc(%rip),%xmm0        # 6f0a0 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x130>
   3e3d4:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
   3e3db:	f3 0f 6f 05 cd 0c 03 	movdqu 0x30ccd(%rip),%xmm0        # 6f0b0 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x140>
   3e3e2:	00 
   3e3e3:	66 0f 7f 45 80       	movdqa %xmm0,-0x80(%rbp)
   3e3e8:	48 c7 45 90 00 00 00 	movq   $0x0,-0x70(%rbp)
   3e3ef:	00 
   3e3f0:	48 8d 75 98          	lea    -0x68(%rbp),%rsi
   3e3f4:	4c 89 75 98          	mov    %r14,-0x68(%rbp)
   3e3f8:	4c 89 7d a0          	mov    %r15,-0x60(%rbp)
   3e3fc:	c6 45 d8 30          	movb   $0x30,-0x28(%rbp)
   3e400:	c6 45 c8 80          	movb   $0x80,-0x38(%rbp)
   3e404:	c7 45 c9 00 00 00 00 	movl   $0x0,-0x37(%rbp)
   3e40b:	c7 45 cc 00 00 00 00 	movl   $0x0,-0x34(%rbp)
   3e412:	48 b8 00 00 00 00 00 	movabs $0x8001000000000000,%rax
   3e419:	00 01 80 
   3e41c:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   3e420:	48 8d bd 70 ff ff ff 	lea    -0x90(%rbp),%rdi
   3e427:	ba 01 00 00 00       	mov    $0x1,%edx
   3e42c:	ff 15 a6 58 04 00    	call   *0x458a6(%rip)        # 83cd8 <_GLOBAL_OFFSET_TABLE_+0x5e0>
   3e432:	66 0f ef c0          	pxor   %xmm0,%xmm0
   3e436:	66 0f 6f 8d 70 ff ff 	movdqa -0x90(%rbp),%xmm1
   3e43d:	ff 
   3e43e:	66 0f 6f 55 80       	movdqa -0x80(%rbp),%xmm2
   3e443:	66 0f 6f d9          	movdqa %xmm1,%xmm3
   3e447:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   3e44b:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   3e450:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   3e455:	66 0f 60 c8          	punpcklbw %xmm0,%xmm1
   3e459:	f2 0f 70 c9 1b       	pshuflw $0x1b,%xmm1,%xmm1
   3e45e:	f3 0f 70 c9 1b       	pshufhw $0x1b,%xmm1,%xmm1
   3e463:	66 0f 67 cb          	packuswb %xmm3,%xmm1
   3e467:	66 0f 6f da          	movdqa %xmm2,%xmm3
   3e46b:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   3e46f:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   3e474:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   3e479:	66 0f 60 d0          	punpcklbw %xmm0,%xmm2
   3e47d:	f2 0f 70 c2 1b       	pshuflw $0x1b,%xmm2,%xmm0
   3e482:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   3e487:	66 0f 67 c3          	packuswb %xmm3,%xmm0
   3e48b:	66 0f 7f 8d 70 ff ff 	movdqa %xmm1,-0x90(%rbp)
   3e492:	ff 
   3e493:	66 0f 7f 45 80       	movdqa %xmm0,-0x80(%rbp)
   3e498:	48 8d b5 70 ff ff ff 	lea    -0x90(%rbp),%rsi
   3e49f:	48 89 df             	mov    %rbx,%rdi
   3e4a2:	ff 15 e8 59 04 00    	call   *0x459e8(%rip)        # 83e90 <_GLOBAL_OFFSET_TABLE_+0x798>
   3e4a8:	e9 fa fe ff ff       	jmp    3e3a7 <<star_core::serv::enclave::Enclave>::issue_token_plain+0xa7>
   3e4ad:	bf 01 00 00 00       	mov    $0x1,%edi
   3e4b2:	be 01 00 00 00       	mov    $0x1,%esi
   3e4b7:	ff 15 03 59 04 00    	call   *0x45903(%rip)        # 83dc0 <_GLOBAL_OFFSET_TABLE_+0x6c8>
   3e4bd:	0f 1f 00             	nopl   (%rax)

000000000003e4c0 <<star_core::serv::core::cred::Credential>::is_valid>:
   3e4c0:	55                   	push   %rbp
   3e4c1:	48 89 e5             	mov    %rsp,%rbp
   3e4c4:	41 57                	push   %r15
   3e4c6:	41 56                	push   %r14
   3e4c8:	41 55                	push   %r13
   3e4ca:	41 54                	push   %r12
   3e4cc:	53                   	push   %rbx
   3e4cd:	48 81 ec e8 02 00 00 	sub    $0x2e8,%rsp
   3e4d4:	4c 89 4d c8          	mov    %r9,-0x38(%rbp)
   3e4d8:	4d 89 c5             	mov    %r8,%r13
   3e4db:	49 89 d7             	mov    %rdx,%r15
   3e4de:	48 89 7d c0          	mov    %rdi,-0x40(%rbp)
   3e4e2:	48 89 d0             	mov    %rdx,%rax
   3e4e5:	48 c1 e8 08          	shr    $0x8,%rax
   3e4e9:	48 89 85 68 ff ff ff 	mov    %rax,-0x98(%rbp)
   3e4f0:	48 89 d0             	mov    %rdx,%rax
   3e4f3:	48 c1 e8 10          	shr    $0x10,%rax
   3e4f7:	48 89 85 70 ff ff ff 	mov    %rax,-0x90(%rbp)
   3e4fe:	48 89 d0             	mov    %rdx,%rax
   3e501:	48 c1 e8 18          	shr    $0x18,%rax
   3e505:	48 89 85 78 ff ff ff 	mov    %rax,-0x88(%rbp)
   3e50c:	48 89 d0             	mov    %rdx,%rax
   3e50f:	48 c1 e8 20          	shr    $0x20,%rax
   3e513:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
   3e517:	48 89 d0             	mov    %rdx,%rax
   3e51a:	48 c1 e8 28          	shr    $0x28,%rax
   3e51e:	48 89 45 88          	mov    %rax,-0x78(%rbp)
   3e522:	48 89 d0             	mov    %rdx,%rax
   3e525:	48 c1 e8 30          	shr    $0x30,%rax
   3e529:	48 89 45 90          	mov    %rax,-0x70(%rbp)
   3e52d:	48 89 d0             	mov    %rdx,%rax
   3e530:	48 c1 e8 38          	shr    $0x38,%rax
   3e534:	48 89 45 a0          	mov    %rax,-0x60(%rbp)
   3e538:	4c 89 c3             	mov    %r8,%rbx
   3e53b:	48 c1 eb 08          	shr    $0x8,%rbx
   3e53f:	48 89 9d b8 fe ff ff 	mov    %rbx,-0x148(%rbp)
   3e546:	4d 89 c4             	mov    %r8,%r12
   3e549:	49 c1 ec 10          	shr    $0x10,%r12
   3e54d:	4d 89 c6             	mov    %r8,%r14
   3e550:	49 c1 ee 18          	shr    $0x18,%r14
   3e554:	4c 89 c0             	mov    %r8,%rax
   3e557:	48 c1 e8 20          	shr    $0x20,%rax
   3e55b:	48 89 45 80          	mov    %rax,-0x80(%rbp)
   3e55f:	4c 89 c0             	mov    %r8,%rax
   3e562:	48 c1 e8 28          	shr    $0x28,%rax
   3e566:	48 89 45 98          	mov    %rax,-0x68(%rbp)
   3e56a:	4c 89 c0             	mov    %r8,%rax
   3e56d:	48 c1 e8 30          	shr    $0x30,%rax
   3e571:	48 89 45 a8          	mov    %rax,-0x58(%rbp)
   3e575:	4c 89 c0             	mov    %r8,%rax
   3e578:	48                   	rex.W
   3e579:	c1                   	.byte 0xc1
   3e57a:	e8                   	.byte 0xe8

=== A=0x48CD7D0 F=0x8cd7d0 ===

./target/release/analysis:     file format elf64-x86-64


