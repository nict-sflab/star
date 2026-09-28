=== VALGRIND OUTPUT ===
==2452== Memcheck, a memory error detector
==2452== Copyright (C) 2002-2026, and GNU GPL'd, by Julian Seward et al.
==2452== Using Valgrind-3.27.1 and LibVEX; rerun with -h for copyright info
==2452== Command: ./target/release/analysis
==2452== 
==2452== Conditional jump or move depends on uninitialised value(s)
==2452==    at 0x401B27E: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2452==    by 0x40171F1: <hpke_ng::kem::dh::DhKem<hpke_ng::kem::dh::X25519> as hpke_ng::kem::Kem>::decap (in /workspace/star/core/target/release/analysis)
==2452==    by 0x401339F: <hpke_ng::Hpke<hpke_ng::kem::dh::DhKem<hpke_ng::kem::dh::X25519>, hpke_ng::kdf::HkdfSha256, hpke_ng::aead::ChaCha20Poly1305>>::open_base (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4016AF4: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2452==    by 0x401195C: <star_core::serv::enclave::Enclave>::issue_cred::<rand_core::os::OsRng> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400D038: analysis::main (in /workspace/star/core/target/release/analysis)
==2452==    by 0x40125E5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400F83B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4034D13: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400DE43: main (in /workspace/star/core/target/release/analysis)
==2452== 
==2452== Conditional jump or move depends on uninitialised value(s)
==2452==    at 0x401CF45: <chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached (in /workspace/star/core/target/release/analysis)
==2452==    by 0x401B521: <chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x401AC7A: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4013D70: <hpke_ng::Hpke<hpke_ng::kem::dh::DhKem<hpke_ng::kem::dh::X25519>, hpke_ng::kdf::HkdfSha256, hpke_ng::aead::ChaCha20Poly1305>>::open_base (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4016AF4: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2452==    by 0x401195C: <star_core::serv::enclave::Enclave>::issue_cred::<rand_core::os::OsRng> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400D038: analysis::main (in /workspace/star/core/target/release/analysis)
==2452==    by 0x40125E5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400F83B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4034D13: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400DE43: main (in /workspace/star/core/target/release/analysis)
==2452== 
==2452== Conditional jump or move depends on uninitialised value(s)
==2452==    at 0x401B524: <chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x401AC7A: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4013D70: <hpke_ng::Hpke<hpke_ng::kem::dh::DhKem<hpke_ng::kem::dh::X25519>, hpke_ng::kdf::HkdfSha256, hpke_ng::aead::ChaCha20Poly1305>>::open_base (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4016AF4: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==2452==    by 0x401195C: <star_core::serv::enclave::Enclave>::issue_cred::<rand_core::os::OsRng> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400D038: analysis::main (in /workspace/star/core/target/release/analysis)
==2452==    by 0x40125E5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400F83B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4034D13: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400DE43: main (in /workspace/star/core/target/release/analysis)
==2452== 
==2452== Conditional jump or move depends on uninitialised value(s)
==2452==    at 0x40150AF: <star_core::serv::enclave::Enclave>::issue_token_plain (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4014FA2: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400D15F: analysis::main (in /workspace/star/core/target/release/analysis)
==2452==    by 0x40125E5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400F83B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4034D13: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400DE43: main (in /workspace/star/core/target/release/analysis)
==2452== 
==2452== Conditional jump or move depends on uninitialised value(s)
==2452==    at 0x401B27E: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400DBD0: <hpke_ng::kem::dh::DhKem<hpke_ng::kem::dh::X25519> as hpke_ng::kem::Kem>::decap (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4010328: <hpke_ng::Hpke<hpke_ng::kem::dh::DhKem<hpke_ng::kem::dh::X25519>, hpke_ng::kdf::HkdfSha384, hpke_ng::aead::ChaCha20Poly1305>>::setup_receiver_base (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400D345: analysis::main (in /workspace/star/core/target/release/analysis)
==2452==    by 0x40125E5: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400F83B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4034D13: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400DE43: main (in /workspace/star/core/target/release/analysis)
==2452== 
done
==2452== 
==2452== HEAP SUMMARY:
==2452==     in use at exit: 544 bytes in 1 blocks
==2452==   total heap usage: 95 allocs, 94 frees, 7,200 bytes allocated
==2452== 
==2452== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==2452==    at 0x48D87D0: malloc (in /nix/store/mf7rzpknxwg0q1wf0nm8gfzikapg00h1-valgrind-3.27.1/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==2452==    by 0x40484A2: <alloc::collections::btree::map::entry::VacantEntry<usize, std::sys::pal::unix::stack_overflow::thread_info::ThreadInfo>>::insert_entry (in /workspace/star/core/target/release/analysis)
==2452==    by 0x40390FC: std::sys::pal::unix::stack_overflow::thread_info::set_current_info (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4039DCF: std::sys::pal::unix::stack_overflow::imp::init (in /workspace/star/core/target/release/analysis)
==2452==    by 0x404893B: std::sys::pal::unix::init (in /workspace/star/core/target/release/analysis)
==2452==    by 0x4034D0D: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==2452==    by 0x400DE43: main (in /workspace/star/core/target/release/analysis)
==2452== 
==2452== LEAK SUMMARY:
==2452==    definitely lost: 0 bytes in 0 blocks
==2452==    indirectly lost: 0 bytes in 0 blocks
==2452==      possibly lost: 0 bytes in 0 blocks
==2452==    still reachable: 544 bytes in 1 blocks
==2452==         suppressed: 0 bytes in 0 blocks
==2452== 
==2452== Use --track-origins=yes to see where uninitialised values come from
==2452== For lists of detected and suppressed errors, rerun with: -s
==2452== ERROR SUMMARY: 11 errors from 5 contexts (suppressed: 0 from 0)

=== A=0x401B27E F=0x1b27e ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000001b07e <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x11e>:
   1b07e:	7d aa                	jge    1b02a <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0xca>
   1b080:	00 40 0f             	add    %al,0xf(%rax)
   1b083:	94                   	xchg   %eax,%esp
   1b084:	c7                   	(bad)
   1b085:	ff 15 0d 3f 07 00    	call   *0x73f0d(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b08b:	88 45 c8             	mov    %al,-0x38(%rbp)
   1b08e:	31 ff                	xor    %edi,%edi
   1b090:	80 7d ab 00          	cmpb   $0x0,-0x55(%rbp)
   1b094:	40 0f 94 c7          	sete   %dil
   1b098:	ff 15 fa 3e 07 00    	call   *0x73efa(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b09e:	88 45 c9             	mov    %al,-0x37(%rbp)
   1b0a1:	31 ff                	xor    %edi,%edi
   1b0a3:	80 7d ac 00          	cmpb   $0x0,-0x54(%rbp)
   1b0a7:	40 0f 94 c7          	sete   %dil
   1b0ab:	ff 15 e7 3e 07 00    	call   *0x73ee7(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b0b1:	88 45 ca             	mov    %al,-0x36(%rbp)
   1b0b4:	31 ff                	xor    %edi,%edi
   1b0b6:	80 7d ad 00          	cmpb   $0x0,-0x53(%rbp)
   1b0ba:	40 0f 94 c7          	sete   %dil
   1b0be:	ff 15 d4 3e 07 00    	call   *0x73ed4(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b0c4:	88 45 cb             	mov    %al,-0x35(%rbp)
   1b0c7:	31 ff                	xor    %edi,%edi
   1b0c9:	80 7d ae 00          	cmpb   $0x0,-0x52(%rbp)
   1b0cd:	40 0f 94 c7          	sete   %dil
   1b0d1:	ff 15 c1 3e 07 00    	call   *0x73ec1(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b0d7:	41 89 c6             	mov    %eax,%r14d
   1b0da:	31 ff                	xor    %edi,%edi
   1b0dc:	80 7d af 00          	cmpb   $0x0,-0x51(%rbp)
   1b0e0:	40 0f 94 c7          	sete   %dil
   1b0e4:	ff 15 ae 3e 07 00    	call   *0x73eae(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b0ea:	88 45 cc             	mov    %al,-0x34(%rbp)
   1b0ed:	31 ff                	xor    %edi,%edi
   1b0ef:	80 7d b0 00          	cmpb   $0x0,-0x50(%rbp)
   1b0f3:	40 0f 94 c7          	sete   %dil
   1b0f7:	ff 15 9b 3e 07 00    	call   *0x73e9b(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b0fd:	88 45 cd             	mov    %al,-0x33(%rbp)
   1b100:	31 ff                	xor    %edi,%edi
   1b102:	80 7d b1 00          	cmpb   $0x0,-0x4f(%rbp)
   1b106:	40 0f 94 c7          	sete   %dil
   1b10a:	ff 15 88 3e 07 00    	call   *0x73e88(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b110:	88 45 ce             	mov    %al,-0x32(%rbp)
   1b113:	31 ff                	xor    %edi,%edi
   1b115:	80 7d b2 00          	cmpb   $0x0,-0x4e(%rbp)
   1b119:	40 0f 94 c7          	sete   %dil
   1b11d:	ff 15 75 3e 07 00    	call   *0x73e75(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b123:	88 45 cf             	mov    %al,-0x31(%rbp)
   1b126:	31 ff                	xor    %edi,%edi
   1b128:	80 7d b3 00          	cmpb   $0x0,-0x4d(%rbp)
   1b12c:	40 0f 94 c7          	sete   %dil
   1b130:	ff 15 62 3e 07 00    	call   *0x73e62(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b136:	88 45 d0             	mov    %al,-0x30(%rbp)
   1b139:	31 ff                	xor    %edi,%edi
   1b13b:	80 7d b4 00          	cmpb   $0x0,-0x4c(%rbp)
   1b13f:	40 0f 94 c7          	sete   %dil
   1b143:	ff 15 4f 3e 07 00    	call   *0x73e4f(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b149:	41 89 c4             	mov    %eax,%r12d
   1b14c:	31 ff                	xor    %edi,%edi
   1b14e:	80 7d b5 00          	cmpb   $0x0,-0x4b(%rbp)
   1b152:	40 0f 94 c7          	sete   %dil
   1b156:	ff 15 3c 3e 07 00    	call   *0x73e3c(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b15c:	88 45 d1             	mov    %al,-0x2f(%rbp)
   1b15f:	31 ff                	xor    %edi,%edi
   1b161:	80 7d b6 00          	cmpb   $0x0,-0x4a(%rbp)
   1b165:	40 0f 94 c7          	sete   %dil
   1b169:	ff 15 29 3e 07 00    	call   *0x73e29(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b16f:	88 45 d2             	mov    %al,-0x2e(%rbp)
   1b172:	31 ff                	xor    %edi,%edi
   1b174:	80 7d b7 00          	cmpb   $0x0,-0x49(%rbp)
   1b178:	40 0f 94 c7          	sete   %dil
   1b17c:	ff 15 16 3e 07 00    	call   *0x73e16(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b182:	88 45 d3             	mov    %al,-0x2d(%rbp)
   1b185:	31 ff                	xor    %edi,%edi
   1b187:	80 7d b8 00          	cmpb   $0x0,-0x48(%rbp)
   1b18b:	40 0f 94 c7          	sete   %dil
   1b18f:	ff 15 03 3e 07 00    	call   *0x73e03(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b195:	88 45 d4             	mov    %al,-0x2c(%rbp)
   1b198:	31 ff                	xor    %edi,%edi
   1b19a:	80 7d b9 00          	cmpb   $0x0,-0x47(%rbp)
   1b19e:	40 0f 94 c7          	sete   %dil
   1b1a2:	ff 15 f0 3d 07 00    	call   *0x73df0(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b1a8:	88 45 d5             	mov    %al,-0x2b(%rbp)
   1b1ab:	31 ff                	xor    %edi,%edi
   1b1ad:	80 7d ba 00          	cmpb   $0x0,-0x46(%rbp)
   1b1b1:	40 0f 94 c7          	sete   %dil
   1b1b5:	ff 15 dd 3d 07 00    	call   *0x73ddd(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b1bb:	88 45 d6             	mov    %al,-0x2a(%rbp)
   1b1be:	31 ff                	xor    %edi,%edi
   1b1c0:	80 7d bb 00          	cmpb   $0x0,-0x45(%rbp)
   1b1c4:	40 0f 94 c7          	sete   %dil
   1b1c8:	ff 15 ca 3d 07 00    	call   *0x73dca(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b1ce:	41 89 c5             	mov    %eax,%r13d
   1b1d1:	31 ff                	xor    %edi,%edi
   1b1d3:	80 7d bc 00          	cmpb   $0x0,-0x44(%rbp)
   1b1d7:	40 0f 94 c7          	sete   %dil
   1b1db:	ff 15 b7 3d 07 00    	call   *0x73db7(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b1e1:	88 45 d7             	mov    %al,-0x29(%rbp)
   1b1e4:	31 ff                	xor    %edi,%edi
   1b1e6:	80 7d bd 00          	cmpb   $0x0,-0x43(%rbp)
   1b1ea:	40 0f 94 c7          	sete   %dil
   1b1ee:	ff 15 a4 3d 07 00    	call   *0x73da4(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b1f4:	0f b6 4d be          	movzbl -0x42(%rbp),%ecx
   1b1f8:	22 4d bf             	and    -0x41(%rbp),%cl
   1b1fb:	0f b6 55 c0          	movzbl -0x40(%rbp),%edx
   1b1ff:	22 55 c1             	and    -0x3f(%rbp),%dl
   1b202:	20 ca                	and    %cl,%dl
   1b204:	0f b6 4d c2          	movzbl -0x3e(%rbp),%ecx
   1b208:	22 4d c3             	and    -0x3d(%rbp),%cl
   1b20b:	22 4d c4             	and    -0x3c(%rbp),%cl
   1b20e:	20 d1                	and    %dl,%cl
   1b210:	44 22 7d c5          	and    -0x3b(%rbp),%r15b
   1b214:	44 22 7d c6          	and    -0x3a(%rbp),%r15b
   1b218:	44 22 7d c7          	and    -0x39(%rbp),%r15b
   1b21c:	41 20 cf             	and    %cl,%r15b
   1b21f:	22 5d c8             	and    -0x38(%rbp),%bl
   1b222:	22 5d c9             	and    -0x37(%rbp),%bl
   1b225:	22 5d ca             	and    -0x36(%rbp),%bl
   1b228:	22 5d cb             	and    -0x35(%rbp),%bl
   1b22b:	44 20 fb             	and    %r15b,%bl
   1b22e:	44 22 75 cc          	and    -0x34(%rbp),%r14b
   1b232:	44 22 75 cd          	and    -0x33(%rbp),%r14b
   1b236:	44 22 75 ce          	and    -0x32(%rbp),%r14b
   1b23a:	44 22 75 cf          	and    -0x31(%rbp),%r14b
   1b23e:	44 22 75 d0          	and    -0x30(%rbp),%r14b
   1b242:	41 20 de             	and    %bl,%r14b
   1b245:	44 22 65 d1          	and    -0x2f(%rbp),%r12b
   1b249:	44 22 65 d2          	and    -0x2e(%rbp),%r12b
   1b24d:	44 22 65 d3          	and    -0x2d(%rbp),%r12b
   1b251:	44 22 65 d4          	and    -0x2c(%rbp),%r12b
   1b255:	44 22 65 d5          	and    -0x2b(%rbp),%r12b
   1b259:	44 22 65 d6          	and    -0x2a(%rbp),%r12b
   1b25d:	45 20 f4             	and    %r14b,%r12b
   1b260:	44 22 6d d7          	and    -0x29(%rbp),%r13b
   1b264:	41 20 c5             	and    %al,%r13b
   1b267:	45 20 e5             	and    %r12b,%r13b
   1b26a:	41 80 e5 01          	and    $0x1,%r13b
   1b26e:	41 0f b6 fd          	movzbl %r13b,%edi
   1b272:	ff 15 20 3d 07 00    	call   *0x73d20(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1b278:	84 c0                	test   %al,%al
   1b27a:	48 8b 5d 90          	mov    -0x70(%rbp),%rbx
   1b27e:	74 0d                	je     1b28d <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x32d>
   1b280:	c6 43 08 03          	movb   $0x3,0x8(%rbx)
   1b284:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   1b28b:	eb 3d                	jmp    1b2ca <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x36a>
   1b28d:	ff 15 ad 37 07 00    	call   *0x737ad(%rip)        # 8ea40 <_GLOBAL_OFFSET_TABLE_+0x530>
   1b293:	bf 20 00 00 00       	mov    $0x20,%edi
   1b298:	be 01 00 00 00       	mov    $0x1,%esi
   1b29d:	ff 15 55 39 07 00    	call   *0x73955(%rip)        # 8ebf8 <_GLOBAL_OFFSET_TABLE_+0x6e8>
   1b2a3:	48 85 c0             	test   %rax,%rax
   1b2a6:	74 3e                	je     1b2e6 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x386>
   1b2a8:	0f 10 45 9e          	movups -0x62(%rbp),%xmm0
   1b2ac:	0f 10 4d ae          	movups -0x52(%rbp),%xmm1
   1b2b0:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   1b2b4:	0f 11 00             	movups %xmm0,(%rax)
   1b2b7:	48 c7 03 20 00 00 00 	movq   $0x20,(%rbx)
   1b2be:	48 89 43 08          	mov    %rax,0x8(%rbx)
   1b2c2:	48 c7 43 10 20 00 00 	movq   $0x20,0x10(%rbx)
   1b2c9:	00 
   1b2ca:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   1b2ce:	ff 15 fc 3a 07 00    	call   *0x73afc(%rip)        # 8edd0 <_GLOBAL_OFFSET_TABLE_+0x8c0>
   1b2d4:	48 89 d8             	mov    %rbx,%rax
   1b2d7:	48 83 c4 68          	add    $0x68,%rsp
   1b2db:	5b                   	pop    %rbx
   1b2dc:	41 5c                	pop    %r12
   1b2de:	41 5d                	pop    %r13
   1b2e0:	41 5e                	pop    %r14
   1b2e2:	41 5f                	pop    %r15
   1b2e4:	5d                   	pop    %rbp
   1b2e5:	c3                   	ret
   1b2e6:	bf 01 00 00 00       	mov    $0x1,%edi
   1b2eb:	be 20 00 00 00       	mov    $0x20,%esi
   1b2f0:	ff 15 12 34 07 00    	call   *0x73412(%rip)        # 8e708 <_GLOBAL_OFFSET_TABLE_+0x1f8>
   1b2f6:	0f 0b                	ud2
   1b2f8:	eb 00                	jmp    1b2fa <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x39a>
   1b2fa:	48 89 c3             	mov    %rax,%rbx
   1b2fd:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   1b301:	ff 15 c9 3a 07 00    	call   *0x73ac9(%rip)        # 8edd0 <_GLOBAL_OFFSET_TABLE_+0x8c0>
   1b307:	48 89 df             	mov    %rbx,%rdi
   1b30a:	e8 41 bd fe ff       	call   7050 <_Unwind_Resume@plt>
   1b30f:	ff 15 1b 35 07 00    	call   *0x7351b(%rip)        # 8e830 <_GLOBAL_OFFSET_TABLE_+0x320>
   1b315:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   1b31c:	00 00 00 
   1b31f:	90                   	nop

000000000001b320 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::sk_to_pk>:
   1b320:	55                   	push   %rbp
   1b321:	48 89 e5             	mov    %rsp,%rbp
   1b324:	53                   	push   %rbx
   1b325:	50                   	push   %rax
   1b326:	48 89 fb             	mov    %rdi,%rbx
   1b329:	ff 15 11 32 07 00    	call   *0x73211(%rip)        # 8e540 <_GLOBAL_OFFSET_TABLE_+0x30>
   1b32f:	48 89 d8             	mov    %rbx,%rax
   1b332:	48 83 c4 08          	add    $0x8,%rsp
   1b336:	5b                   	pop    %rbx
   1b337:	5d                   	pop    %rbp
   1b338:	c3                   	ret
   1b339:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

000000000001b340 <<hpke_ng::kem::dh::X25519PrivateKeyWrap as zeroize::Zeroize>::zeroize>:
   1b340:	55                   	push   %rbp
   1b341:	48 89 e5             	mov    %rsp,%rbp
   1b344:	48 83 ec 20          	sub    $0x20,%rsp
   1b348:	0f 10 07             	movups (%rdi),%xmm0
   1b34b:	0f 10 4f 10          	movups 0x10(%rdi),%xmm1
   1b34f:	0f 29 4d f0          	movaps %xmm1,-0x10(%rbp)
   1b353:	0f 29 45 e0          	movaps %xmm0,-0x20(%rbp)
   1b357:	0f 57 c0             	xorps  %xmm0,%xmm0
   1b35a:	0f 11 07             	movups %xmm0,(%rdi)
   1b35d:	0f 11 47 10          	movups %xmm0,0x10(%rdi)
   1b361:	48 8d 7d e0          	lea    -0x20(%rbp),%rdi
   1b365:	ff 15 dd 3a 07 00    	call   *0x73add(%rip)        # 8ee48 <_GLOBAL_OFFSET_TABLE_+0x938>
   1b36b:	48 83 c4 20          	add    $0x20,%rsp
   1b36f:	5d                   	pop    %rbp
   1b370:	c3                   	ret
   1b371:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   1b378:	00 00 00 
   1b37b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)

000000000001b380 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>>:
   1b380:	55                   	push   %rbp
   1b381:	48 89 e5             	mov    %rsp,%rbp
   1b384:	41 57                	push   %r15
   1b386:	41 56                	push   %r14
   1b388:	41 55                	push   %r13
   1b38a:	41 54                	push   %r12
   1b38c:	53                   	push   %rbx
   1b38d:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   1b391:	48 81 ec e0 03 00 00 	sub    $0x3e0,%rsp
   1b398:	4c 8b 71 08          	mov    0x8(%rcx),%r14
   1b39c:	4d 85 f6             	test   %r14,%r14
   1b39f:	79 0e                	jns    1b3af <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x2f>
   1b3a1:	31 db                	xor    %ebx,%ebx
   1b3a3:	48 89 df             	mov    %rbx,%rdi
   1b3a6:	4c 89 f6             	mov    %r14,%rsi
   1b3a9:	ff 15 59 33 07 00    	call   *0x73359(%rip)        # 8e708 <_GLOBAL_OFFSET_TABLE_+0x1f8>
   1b3af:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
   1b3b4:	0f 84 8b 01 00 00    	je     1b545 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x1c5>
   1b3ba:	49 89 d4             	mov    %rdx,%r12
   1b3bd:	49 89 f5             	mov    %rsi,%r13
   1b3c0:	48 8b 01             	mov    (%rcx),%rax
   1b3c3:	48 89 44 24 48       	mov    %rax,0x48(%rsp)
   1b3c8:	48 8b 41 10          	mov    0x10(%rcx),%rax
   1b3cc:	48 89 44 24 40       	mov    %rax,0x40(%rsp)
   1b3d1:	48 8b 41 18          	mov    0x18(%rcx),%rax
   1b3d5:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   1b3da:	ff 15 60 36 07 00    	call   *0x73660(%rip)        # 8ea40 <_GLOBAL_OFFSET_TABLE_+0x530>
   1b3e0:	bb 01 00 00 00       	mov    $0x1,%ebx
   1b3e5:	be 01 00 00 00       	mov    $0x1,%esi
   1b3ea:	4c 89 f7             	mov    %r14,%rdi
   1b3ed:	ff 15 05 38 07 00    	call   *0x73805(%rip)        # 8ebf8 <_GLOBAL_OFFSET_TABLE_+0x6e8>
   1b3f3:	48 85 c0             	test   %rax,%rax
   1b3f6:	74 ab                	je     1b3a3 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x23>
   1b3f8:	49 89 c7             	mov    %rax,%r15
   1b3fb:	4c 89 74 24 10       	mov    %r14,0x10(%rsp)
   1b400:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   1b405:	48 89 c7             	mov    %rax,%rdi
   1b408:	48 8b 74 24 48       	mov    0x48(%rsp),%rsi
   1b40d:	4c 89 f2             	mov    %r14,%rdx
   1b410:	ff 15 5a 3b 07 00    	call   *0x73b5a(%rip)        # 8ef70 <memcpy@GLIBC_2.14>
   1b416:	4c 89 74 24 20       	mov    %r14,0x20(%rsp)
   1b41b:	49 83 fe 10          	cmp    $0x10,%r14
   1b41f:	0f 82 01 01 00 00    	jb     1b526 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x1a6>
   1b425:	0f 10 05 fc bf 05 00 	movups 0x5bffc(%rip),%xmm0        # 77428 <anon.747060d88ef99e7a9a3b109ab8bfd015.9.llvm.6566165678146464307+0x38>
   1b42c:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
   1b433:	00 
   1b434:	41 0f 10 45 00       	movups 0x0(%r13),%xmm0
   1b439:	41 0f 10 4d 10       	movups 0x10(%r13),%xmm1
   1b43e:	0f 29 84 24 f0 00 00 	movaps %xmm0,0xf0(%rsp)
   1b445:	00 
   1b446:	0f 29 8c 24 00 01 00 	movaps %xmm1,0x100(%rsp)
   1b44d:	00 
   1b44e:	49 8b 04 24          	mov    (%r12),%rax
   1b452:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
   1b457:	41 8b 44 24 08       	mov    0x8(%r12),%eax
   1b45c:	89 44 24 30          	mov    %eax,0x30(%rsp)
   1b460:	48 8d 05 ad 3b 07 00 	lea    0x73bad(%rip),%rax        # 8f014 <chacha20::avx2_cpuid::STORAGE>
   1b467:	0f b6 00             	movzbl (%rax),%eax
   1b46a:	3c ff                	cmp    $0xff,%al
   1b46c:	0f 84 17 01 00 00    	je     1b589 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x209>
   1b472:	0f 57 c0             	xorps  %xmm0,%xmm0
   1b475:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   1b47c:	00 
   1b47d:	0f                   	.byte 0xf

=== A=0x401CF45 F=0x1cf45 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000001cd45 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x445>:
   1cd45:	f6 ff                	idiv   %bh
   1cd47:	15 9c 20 07 00       	adc    $0x7209c,%eax
   1cd4c:	0f b6 44 24 20       	movzbl 0x20(%rsp),%eax
   1cd51:	31 ff                	xor    %edi,%edi
   1cd53:	4c 8b 64 24 38       	mov    0x38(%rsp),%r12
   1cd58:	41 3a 04 24          	cmp    (%r12),%al
   1cd5c:	40 0f 94 c7          	sete   %dil
   1cd60:	ff 15 32 22 07 00    	call   *0x72232(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cd66:	88 44 24 10          	mov    %al,0x10(%rsp)
   1cd6a:	0f b6 44 24 21       	movzbl 0x21(%rsp),%eax
   1cd6f:	31 ff                	xor    %edi,%edi
   1cd71:	41 3a 44 24 01       	cmp    0x1(%r12),%al
   1cd76:	40 0f 94 c7          	sete   %dil
   1cd7a:	ff 15 18 22 07 00    	call   *0x72218(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cd80:	88 44 24 0f          	mov    %al,0xf(%rsp)
   1cd84:	0f b6 44 24 22       	movzbl 0x22(%rsp),%eax
   1cd89:	31 ff                	xor    %edi,%edi
   1cd8b:	41 3a 44 24 02       	cmp    0x2(%r12),%al
   1cd90:	40 0f 94 c7          	sete   %dil
   1cd94:	ff 15 fe 21 07 00    	call   *0x721fe(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cd9a:	88 44 24 0e          	mov    %al,0xe(%rsp)
   1cd9e:	0f b6 44 24 23       	movzbl 0x23(%rsp),%eax
   1cda3:	31 ff                	xor    %edi,%edi
   1cda5:	41 3a 44 24 03       	cmp    0x3(%r12),%al
   1cdaa:	40 0f 94 c7          	sete   %dil
   1cdae:	ff 15 e4 21 07 00    	call   *0x721e4(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cdb4:	88 44 24 0d          	mov    %al,0xd(%rsp)
   1cdb8:	0f b6 44 24 24       	movzbl 0x24(%rsp),%eax
   1cdbd:	31 ff                	xor    %edi,%edi
   1cdbf:	41 3a 44 24 04       	cmp    0x4(%r12),%al
   1cdc4:	40 0f 94 c7          	sete   %dil
   1cdc8:	ff 15 ca 21 07 00    	call   *0x721ca(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cdce:	41 89 c6             	mov    %eax,%r14d
   1cdd1:	0f b6 44 24 25       	movzbl 0x25(%rsp),%eax
   1cdd6:	31 ff                	xor    %edi,%edi
   1cdd8:	41 3a 44 24 05       	cmp    0x5(%r12),%al
   1cddd:	40 0f 94 c7          	sete   %dil
   1cde1:	ff 15 b1 21 07 00    	call   *0x721b1(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cde7:	88 44 24 0c          	mov    %al,0xc(%rsp)
   1cdeb:	0f b6 44 24 26       	movzbl 0x26(%rsp),%eax
   1cdf0:	31 ff                	xor    %edi,%edi
   1cdf2:	41 3a 44 24 06       	cmp    0x6(%r12),%al
   1cdf7:	40 0f 94 c7          	sete   %dil
   1cdfb:	ff 15 97 21 07 00    	call   *0x72197(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1ce01:	88 44 24 0b          	mov    %al,0xb(%rsp)
   1ce05:	0f b6 44 24 27       	movzbl 0x27(%rsp),%eax
   1ce0a:	31 ff                	xor    %edi,%edi
   1ce0c:	41 3a 44 24 07       	cmp    0x7(%r12),%al
   1ce11:	40 0f 94 c7          	sete   %dil
   1ce15:	ff 15 7d 21 07 00    	call   *0x7217d(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1ce1b:	41 89 c7             	mov    %eax,%r15d
   1ce1e:	0f b6 44 24 28       	movzbl 0x28(%rsp),%eax
   1ce23:	31 ff                	xor    %edi,%edi
   1ce25:	41 3a 44 24 08       	cmp    0x8(%r12),%al
   1ce2a:	40 0f 94 c7          	sete   %dil
   1ce2e:	ff 15 64 21 07 00    	call   *0x72164(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1ce34:	88 44 24 0a          	mov    %al,0xa(%rsp)
   1ce38:	0f b6 44 24 29       	movzbl 0x29(%rsp),%eax
   1ce3d:	31 ff                	xor    %edi,%edi
   1ce3f:	41 3a 44 24 09       	cmp    0x9(%r12),%al
   1ce44:	40 0f 94 c7          	sete   %dil
   1ce48:	ff 15 4a 21 07 00    	call   *0x7214a(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1ce4e:	88 44 24 09          	mov    %al,0x9(%rsp)
   1ce52:	0f b6 44 24 2a       	movzbl 0x2a(%rsp),%eax
   1ce57:	31 ff                	xor    %edi,%edi
   1ce59:	41 3a 44 24 0a       	cmp    0xa(%r12),%al
   1ce5e:	40 0f 94 c7          	sete   %dil
   1ce62:	ff 15 30 21 07 00    	call   *0x72130(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1ce68:	88 44 24 08          	mov    %al,0x8(%rsp)
   1ce6c:	0f b6 44 24 2b       	movzbl 0x2b(%rsp),%eax
   1ce71:	31 ff                	xor    %edi,%edi
   1ce73:	41 3a 44 24 0b       	cmp    0xb(%r12),%al
   1ce78:	40 0f 94 c7          	sete   %dil
   1ce7c:	ff 15 16 21 07 00    	call   *0x72116(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1ce82:	41 89 c5             	mov    %eax,%r13d
   1ce85:	0f b6 44 24 2c       	movzbl 0x2c(%rsp),%eax
   1ce8a:	31 ff                	xor    %edi,%edi
   1ce8c:	41 3a 44 24 0c       	cmp    0xc(%r12),%al
   1ce91:	40 0f 94 c7          	sete   %dil
   1ce95:	ff 15 fd 20 07 00    	call   *0x720fd(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1ce9b:	88 44 24 07          	mov    %al,0x7(%rsp)
   1ce9f:	0f b6 44 24 2d       	movzbl 0x2d(%rsp),%eax
   1cea4:	31 ff                	xor    %edi,%edi
   1cea6:	41 3a 44 24 0d       	cmp    0xd(%r12),%al
   1ceab:	40 0f 94 c7          	sete   %dil
   1ceaf:	ff 15 e3 20 07 00    	call   *0x720e3(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1ceb5:	88 44 24 06          	mov    %al,0x6(%rsp)
   1ceb9:	0f b6 44 24 2e       	movzbl 0x2e(%rsp),%eax
   1cebe:	31 ff                	xor    %edi,%edi
   1cec0:	41 3a 44 24 0e       	cmp    0xe(%r12),%al
   1cec5:	40 0f 94 c7          	sete   %dil
   1cec9:	ff 15 c9 20 07 00    	call   *0x720c9(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cecf:	88 44 24 05          	mov    %al,0x5(%rsp)
   1ced3:	0f b6 44 24 2f       	movzbl 0x2f(%rsp),%eax
   1ced8:	31 ff                	xor    %edi,%edi
   1ceda:	41 3a 44 24 0f       	cmp    0xf(%r12),%al
   1cedf:	40 0f 94 c7          	sete   %dil
   1cee3:	ff 15 af 20 07 00    	call   *0x720af(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cee9:	0f b6 4c 24 10       	movzbl 0x10(%rsp),%ecx
   1ceee:	22 4c 24 0f          	and    0xf(%rsp),%cl
   1cef2:	0f b6 54 24 0e       	movzbl 0xe(%rsp),%edx
   1cef7:	22 54 24 0d          	and    0xd(%rsp),%dl
   1cefb:	20 ca                	and    %cl,%dl
   1cefd:	44 22 74 24 0c       	and    0xc(%rsp),%r14b
   1cf02:	44 22 74 24 0b       	and    0xb(%rsp),%r14b
   1cf07:	41 20 d6             	and    %dl,%r14b
   1cf0a:	44 22 7c 24 0a       	and    0xa(%rsp),%r15b
   1cf0f:	44 22 7c 24 09       	and    0x9(%rsp),%r15b
   1cf14:	44 22 7c 24 08       	and    0x8(%rsp),%r15b
   1cf19:	45 20 f7             	and    %r14b,%r15b
   1cf1c:	44 22 6c 24 07       	and    0x7(%rsp),%r13b
   1cf21:	44 22 6c 24 06       	and    0x6(%rsp),%r13b
   1cf26:	44 22 6c 24 05       	and    0x5(%rsp),%r13b
   1cf2b:	41 20 c5             	and    %al,%r13b
   1cf2e:	45 20 fd             	and    %r15b,%r13b
   1cf31:	41 80 e5 01          	and    $0x1,%r13b
   1cf35:	41 0f b6 fd          	movzbl %r13b,%edi
   1cf39:	ff 15 59 20 07 00    	call   *0x72059(%rip)        # 8ef98 <_GLOBAL_OFFSET_TABLE_+0xa88>
   1cf3f:	84 c0                	test   %al,%al
   1cf41:	41 0f 94 c6          	sete   %r14b
   1cf45:	74 16                	je     1cf5d <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x65d>
   1cf47:	48 8d bb 40 02 00 00 	lea    0x240(%rbx),%rdi
   1cf4e:	48 8b 74 24 30       	mov    0x30(%rsp),%rsi
   1cf53:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   1cf58:	e8 53 15 00 00       	call   1e4b0 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as cipher::stream::StreamCipher>::apply_keystream>
   1cf5d:	4c 8d bb 40 02 00 00 	lea    0x240(%rbx),%r15
   1cf64:	4c 89 ff             	mov    %r15,%rdi
   1cf67:	e8 04 e9 ff ff       	call   1b870 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as core::ops::drop::Drop>::drop>
   1cf6c:	c7 83 40 02 00 00 00 	movl   $0x0,0x240(%rbx)
   1cf73:	00 00 00 
   1cf76:	48 8d 83 44 02 00 00 	lea    0x244(%rbx),%rax
   1cf7d:	c7 83 44 02 00 00 00 	movl   $0x0,0x244(%rbx)
   1cf84:	00 00 00 
   1cf87:	48 8d 83 48 02 00 00 	lea    0x248(%rbx),%rax
   1cf8e:	c7 83 48 02 00 00 00 	movl   $0x0,0x248(%rbx)
   1cf95:	00 00 00 
   1cf98:	48 8d 83 4c 02 00 00 	lea    0x24c(%rbx),%rax
   1cf9f:	c7 83 4c 02 00 00 00 	movl   $0x0,0x24c(%rbx)
   1cfa6:	00 00 00 
   1cfa9:	48 8d 83 50 02 00 00 	lea    0x250(%rbx),%rax
   1cfb0:	c7 83 50 02 00 00 00 	movl   $0x0,0x250(%rbx)
   1cfb7:	00 00 00 
   1cfba:	48 8d 83 54 02 00 00 	lea    0x254(%rbx),%rax
   1cfc1:	c7 83 54 02 00 00 00 	movl   $0x0,0x254(%rbx)
   1cfc8:	00 00 00 
   1cfcb:	48 8d 83 58 02 00 00 	lea    0x258(%rbx),%rax
   1cfd2:	c7 83 58 02 00 00 00 	movl   $0x0,0x258(%rbx)
   1cfd9:	00 00 00 
   1cfdc:	48 8d 83 5c 02 00 00 	lea    0x25c(%rbx),%rax
   1cfe3:	c7 83 5c 02 00 00 00 	movl   $0x0,0x25c(%rbx)
   1cfea:	00 00 00 
   1cfed:	48 8d 83 60 02 00 00 	lea    0x260(%rbx),%rax
   1cff4:	c7 83 60 02 00 00 00 	movl   $0x0,0x260(%rbx)
   1cffb:	00 00 00 
   1cffe:	48 8d 83 64 02 00 00 	lea    0x264(%rbx),%rax
   1d005:	c7 83 64 02 00 00 00 	movl   $0x0,0x264(%rbx)
   1d00c:	00 00 00 
   1d00f:	48 8d 83 68 02 00 00 	lea    0x268(%rbx),%rax
   1d016:	c7 83 68 02 00 00 00 	movl   $0x0,0x268(%rbx)
   1d01d:	00 00 00 
   1d020:	48 8d 83 6c 02 00 00 	lea    0x26c(%rbx),%rax
   1d027:	c7 83 6c 02 00 00 00 	movl   $0x0,0x26c(%rbx)
   1d02e:	00 00 00 
   1d031:	48 8d 83 70 02 00 00 	lea    0x270(%rbx),%rax
   1d038:	c7 83 70 02 00 00 00 	movl   $0x0,0x270(%rbx)
   1d03f:	00 00 00 
   1d042:	48 8d 83 74 02 00 00 	lea    0x274(%rbx),%rax
   1d049:	c7 83 74 02 00 00 00 	movl   $0x0,0x274(%rbx)
   1d050:	00 00 00 
   1d053:	48 8d 83 78 02 00 00 	lea    0x278(%rbx),%rax
   1d05a:	c7 83 78 02 00 00 00 	movl   $0x0,0x278(%rbx)
   1d061:	00 00 00 
   1d064:	c7 83 7c 02 00 00 00 	movl   $0x0,0x27c(%rbx)
   1d06b:	00 00 00 
   1d06e:	48 81 c3 7c 02 00 00 	add    $0x27c,%rbx
   1d075:	44 89 f0             	mov    %r14d,%eax
   1d078:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   1d07c:	5b                   	pop    %rbx
   1d07d:	41 5c                	pop    %r12
   1d07f:	41 5d                	pop    %r13
   1d081:	41 5e                	pop    %r14
   1d083:	41 5f                	pop    %r15
   1d085:	5d                   	pop    %rbp
   1d086:	c3                   	ret
   1d087:	eb 3e                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d089:	eb 3c                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d08b:	eb 3a                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d08d:	eb 38                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d08f:	eb 36                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d091:	eb 34                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d093:	49 89 c6             	mov    %rax,%r14
   1d096:	4c 89 ff             	mov    %r15,%rdi
   1d099:	e8 82 f5 ff ff       	call   1c620 <<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>> as core::ops::drop::Drop>::drop>
   1d09e:	eb 32                	jmp    1d0d2 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7d2>
   1d0a0:	ff 15 8a 17 07 00    	call   *0x7178a(%rip)        # 8e830 <_GLOBAL_OFFSET_TABLE_+0x320>
   1d0a6:	49 89 c6             	mov    %rax,%r14
   1d0a9:	4c 89 ff             	mov    %r15,%rdi
   1d0ac:	e8 6f f5 ff ff       	call   1c620 <<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>> as core::ops::drop::Drop>::drop>
   1d0b1:	eb 1f                	jmp    1d0d2 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7d2>
   1d0b3:	ff 15 77 17 07 00    	call   *0x71777(%rip)        # 8e830 <_GLOBAL_OFFSET_TABLE_+0x320>
   1d0b9:	eb 0c                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d0bb:	eb 0a                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d0bd:	eb 08                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d0bf:	eb 06                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d0c1:	eb 04                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d0c3:	eb 02                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d0c5:	eb 00                	jmp    1d0c7 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached+0x7c7>
   1d0c7:	49 89 c6             	mov    %rax,%r14
   1d0ca:	48 89 df             	mov    %rbx,%rdi
   1d0cd:	e8 0e f6 ff ff       	call   1c6e0 <core::ptr::drop_glue::<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>>
   1d0d2:	4c 89 f7             	mov    %r14,%rdi
   1d0d5:	e8 76 9f fe ff       	call   7050 <_Unwind_Resume@plt>
   1d0da:	ff 15 50 17 07 00    	call   *0x71750(%rip)        # 8e830 <_GLOBAL_OFFSET_TABLE_+0x320>

000000000001d0e0 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::encrypt_in_place_detached>:
   1d0e0:	55                   	push   %rbp
   1d0e1:	48 89 e5             	mov    %rsp,%rbp
   1d0e4:	41 57                	push   %r15
   1d0e6:	41 56                	push   %r14
   1d0e8:	41 55                	push   %r13
   1d0ea:	41 54                	push   %r12
   1d0ec:	53                   	push   %rbx
   1d0ed:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   1d0f1:	48 81 ec a0 02 00 00 	sub    $0x2a0,%rsp
   1d0f8:	4c 89 44 24 20       	mov    %r8,0x20(%rsp)
   1d0fd:	48 89 4c 24 18       	mov    %rcx,0x18(%rsp)
   1d102:	48 89 f3             	mov    %rsi,%rbx
   1d105:	49 89 ff             	mov    %rdi,%r15
   1d108:	48 b8 f0 ff ff ff 3f 	movabs $0x3ffffffff0,%rax
   1d10f:	00 00 00 
   1d112:	48 83 c0 cf          	add    $0xffffffffffffffcf,%rax
   1d116:	4c 89 4c 24 10       	mov    %r9,0x10(%rsp)
   1d11b:	49 39 c1             	cmp    %rax,%r9
   1d11e:	0f 86 21 01 00 00    	jbe    1d245 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::encrypt_in_place_detached+0x165>
   1d124:	41 c6 07 01          	movb   $0x1,(%r15)
   1d128:	4c 8d b3 40 02 00 00 	lea    0x240(%rbx),%r14
   1d12f:	4c 89 f7             	mov    %r14,%rdi
   1d132:	e8 39 e7 ff ff       	call   1b870 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as core::ops::drop::Drop>::drop>
   1d137:	c7 83 40 02 00 00 00 	movl   $0x0,0x240(%rbx)
   1d13e:	00 00 00 
   1d141:	48                   	rex.W
   1d142:	8d                   	.byte 0x8d
   1d143:	83                   	.byte 0x83
   1d144:	44                   	rex.R

=== A=0x401B524 F=0x1b524 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000001b324 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::sk_to_pk+0x4>:
   1b324:	53                   	push   %rbx
   1b325:	50                   	push   %rax
   1b326:	48 89 fb             	mov    %rdi,%rbx
   1b329:	ff 15 11 32 07 00    	call   *0x73211(%rip)        # 8e540 <_GLOBAL_OFFSET_TABLE_+0x30>
   1b32f:	48 89 d8             	mov    %rbx,%rax
   1b332:	48 83 c4 08          	add    $0x8,%rsp
   1b336:	5b                   	pop    %rbx
   1b337:	5d                   	pop    %rbp
   1b338:	c3                   	ret
   1b339:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

000000000001b340 <<hpke_ng::kem::dh::X25519PrivateKeyWrap as zeroize::Zeroize>::zeroize>:
   1b340:	55                   	push   %rbp
   1b341:	48 89 e5             	mov    %rsp,%rbp
   1b344:	48 83 ec 20          	sub    $0x20,%rsp
   1b348:	0f 10 07             	movups (%rdi),%xmm0
   1b34b:	0f 10 4f 10          	movups 0x10(%rdi),%xmm1
   1b34f:	0f 29 4d f0          	movaps %xmm1,-0x10(%rbp)
   1b353:	0f 29 45 e0          	movaps %xmm0,-0x20(%rbp)
   1b357:	0f 57 c0             	xorps  %xmm0,%xmm0
   1b35a:	0f 11 07             	movups %xmm0,(%rdi)
   1b35d:	0f 11 47 10          	movups %xmm0,0x10(%rdi)
   1b361:	48 8d 7d e0          	lea    -0x20(%rbp),%rdi
   1b365:	ff 15 dd 3a 07 00    	call   *0x73add(%rip)        # 8ee48 <_GLOBAL_OFFSET_TABLE_+0x938>
   1b36b:	48 83 c4 20          	add    $0x20,%rsp
   1b36f:	5d                   	pop    %rbp
   1b370:	c3                   	ret
   1b371:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   1b378:	00 00 00 
   1b37b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)

000000000001b380 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>>:
   1b380:	55                   	push   %rbp
   1b381:	48 89 e5             	mov    %rsp,%rbp
   1b384:	41 57                	push   %r15
   1b386:	41 56                	push   %r14
   1b388:	41 55                	push   %r13
   1b38a:	41 54                	push   %r12
   1b38c:	53                   	push   %rbx
   1b38d:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   1b391:	48 81 ec e0 03 00 00 	sub    $0x3e0,%rsp
   1b398:	4c 8b 71 08          	mov    0x8(%rcx),%r14
   1b39c:	4d 85 f6             	test   %r14,%r14
   1b39f:	79 0e                	jns    1b3af <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x2f>
   1b3a1:	31 db                	xor    %ebx,%ebx
   1b3a3:	48 89 df             	mov    %rbx,%rdi
   1b3a6:	4c 89 f6             	mov    %r14,%rsi
   1b3a9:	ff 15 59 33 07 00    	call   *0x73359(%rip)        # 8e708 <_GLOBAL_OFFSET_TABLE_+0x1f8>
   1b3af:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
   1b3b4:	0f 84 8b 01 00 00    	je     1b545 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x1c5>
   1b3ba:	49 89 d4             	mov    %rdx,%r12
   1b3bd:	49 89 f5             	mov    %rsi,%r13
   1b3c0:	48 8b 01             	mov    (%rcx),%rax
   1b3c3:	48 89 44 24 48       	mov    %rax,0x48(%rsp)
   1b3c8:	48 8b 41 10          	mov    0x10(%rcx),%rax
   1b3cc:	48 89 44 24 40       	mov    %rax,0x40(%rsp)
   1b3d1:	48 8b 41 18          	mov    0x18(%rcx),%rax
   1b3d5:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   1b3da:	ff 15 60 36 07 00    	call   *0x73660(%rip)        # 8ea40 <_GLOBAL_OFFSET_TABLE_+0x530>
   1b3e0:	bb 01 00 00 00       	mov    $0x1,%ebx
   1b3e5:	be 01 00 00 00       	mov    $0x1,%esi
   1b3ea:	4c 89 f7             	mov    %r14,%rdi
   1b3ed:	ff 15 05 38 07 00    	call   *0x73805(%rip)        # 8ebf8 <_GLOBAL_OFFSET_TABLE_+0x6e8>
   1b3f3:	48 85 c0             	test   %rax,%rax
   1b3f6:	74 ab                	je     1b3a3 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x23>
   1b3f8:	49 89 c7             	mov    %rax,%r15
   1b3fb:	4c 89 74 24 10       	mov    %r14,0x10(%rsp)
   1b400:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   1b405:	48 89 c7             	mov    %rax,%rdi
   1b408:	48 8b 74 24 48       	mov    0x48(%rsp),%rsi
   1b40d:	4c 89 f2             	mov    %r14,%rdx
   1b410:	ff 15 5a 3b 07 00    	call   *0x73b5a(%rip)        # 8ef70 <memcpy@GLIBC_2.14>
   1b416:	4c 89 74 24 20       	mov    %r14,0x20(%rsp)
   1b41b:	49 83 fe 10          	cmp    $0x10,%r14
   1b41f:	0f 82 01 01 00 00    	jb     1b526 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x1a6>
   1b425:	0f 10 05 fc bf 05 00 	movups 0x5bffc(%rip),%xmm0        # 77428 <anon.747060d88ef99e7a9a3b109ab8bfd015.9.llvm.6566165678146464307+0x38>
   1b42c:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
   1b433:	00 
   1b434:	41 0f 10 45 00       	movups 0x0(%r13),%xmm0
   1b439:	41 0f 10 4d 10       	movups 0x10(%r13),%xmm1
   1b43e:	0f 29 84 24 f0 00 00 	movaps %xmm0,0xf0(%rsp)
   1b445:	00 
   1b446:	0f 29 8c 24 00 01 00 	movaps %xmm1,0x100(%rsp)
   1b44d:	00 
   1b44e:	49 8b 04 24          	mov    (%r12),%rax
   1b452:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
   1b457:	41 8b 44 24 08       	mov    0x8(%r12),%eax
   1b45c:	89 44 24 30          	mov    %eax,0x30(%rsp)
   1b460:	48 8d 05 ad 3b 07 00 	lea    0x73bad(%rip),%rax        # 8f014 <chacha20::avx2_cpuid::STORAGE>
   1b467:	0f b6 00             	movzbl (%rax),%eax
   1b46a:	3c ff                	cmp    $0xff,%al
   1b46c:	0f 84 17 01 00 00    	je     1b589 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x209>
   1b472:	0f 57 c0             	xorps  %xmm0,%xmm0
   1b475:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   1b47c:	00 
   1b47d:	0f 29 84 24 b0 00 00 	movaps %xmm0,0xb0(%rsp)
   1b484:	00 
   1b485:	0f 29 84 24 a0 00 00 	movaps %xmm0,0xa0(%rsp)
   1b48c:	00 
   1b48d:	0f 29 84 24 90 00 00 	movaps %xmm0,0x90(%rsp)
   1b494:	00 
   1b495:	0f 28 84 24 e0 00 00 	movaps 0xe0(%rsp),%xmm0
   1b49c:	00 
   1b49d:	0f 28 8c 24 f0 00 00 	movaps 0xf0(%rsp),%xmm1
   1b4a4:	00 
   1b4a5:	0f 28 94 24 00 01 00 	movaps 0x100(%rsp),%xmm2
   1b4ac:	00 
   1b4ad:	0f 29 44 24 50       	movaps %xmm0,0x50(%rsp)
   1b4b2:	0f 29 4c 24 60       	movaps %xmm1,0x60(%rsp)
   1b4b7:	0f 29 54 24 70       	movaps %xmm2,0x70(%rsp)
   1b4bc:	c7 84 24 80 00 00 00 	movl   $0x0,0x80(%rsp)
   1b4c3:	00 00 00 00 
   1b4c7:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
   1b4cc:	48 89 84 24 84 00 00 	mov    %rax,0x84(%rsp)
   1b4d3:	00 
   1b4d4:	8b 44 24 30          	mov    0x30(%rsp),%eax
   1b4d8:	89 84 24 8c 00 00 00 	mov    %eax,0x8c(%rsp)
   1b4df:	c6 84 24 d0 00 00 00 	movb   $0x0,0xd0(%rsp)
   1b4e6:	00 
   1b4e7:	48 8d bc 24 e0 00 00 	lea    0xe0(%rsp),%rdi
   1b4ee:	00 
   1b4ef:	48 8d 74 24 50       	lea    0x50(%rsp),%rsi
   1b4f4:	e8 d7 21 00 00       	call   1d6d0 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::new>
   1b4f9:	49 8d 5e f0          	lea    -0x10(%r14),%rbx
   1b4fd:	4f 8d 0c 37          	lea    (%r15,%r14,1),%r9
   1b501:	49 83 c1 f0          	add    $0xfffffffffffffff0,%r9
   1b505:	48 8d bc 24 e0 00 00 	lea    0xe0(%rsp),%rdi
   1b50c:	00 
   1b50d:	48 8b 74 24 40       	mov    0x40(%rsp),%rsi
   1b512:	48 8b 54 24 38       	mov    0x38(%rsp),%rdx
   1b517:	4c 89 f9             	mov    %r15,%rcx
   1b51a:	49 89 d8             	mov    %rbx,%r8
   1b51d:	e8 de 13 00 00       	call   1c900 <<chacha20poly1305::cipher::Cipher<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>::decrypt_in_place_detached>
   1b522:	84 c0                	test   %al,%al
   1b524:	74 2d                	je     1b553 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x1d3>
   1b526:	48 8b 5c 24 08       	mov    0x8(%rsp),%rbx
   1b52b:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   1b532:	ba 01 00 00 00       	mov    $0x1,%edx
   1b537:	4c 89 ff             	mov    %r15,%rdi
   1b53a:	4c 89 f6             	mov    %r14,%rsi
   1b53d:	ff 15 4d 32 07 00    	call   *0x7324d(%rip)        # 8e790 <_GLOBAL_OFFSET_TABLE_+0x280>
   1b543:	eb 32                	jmp    1b577 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x1f7>
   1b545:	48 8b 5c 24 08       	mov    0x8(%rsp),%rbx
   1b54a:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   1b551:	eb 24                	jmp    1b577 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x1f7>
   1b553:	48 8d 7c 24 10       	lea    0x10(%rsp),%rdi
   1b558:	48 89 de             	mov    %rbx,%rsi
   1b55b:	ff 15 97 38 07 00    	call   *0x73897(%rip)        # 8edf8 <_GLOBAL_OFFSET_TABLE_+0x8e8>
   1b561:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   1b566:	48 8b 5c 24 08       	mov    0x8(%rsp),%rbx
   1b56b:	48 89 43 10          	mov    %rax,0x10(%rbx)
   1b56f:	0f 10 44 24 10       	movups 0x10(%rsp),%xmm0
   1b574:	0f 11 03             	movups %xmm0,(%rbx)
   1b577:	48 89 d8             	mov    %rbx,%rax
   1b57a:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   1b57e:	5b                   	pop    %rbx
   1b57f:	41 5c                	pop    %r12
   1b581:	41 5d                	pop    %r13
   1b583:	41 5e                	pop    %r14
   1b585:	41 5f                	pop    %r15
   1b587:	5d                   	pop    %rbp
   1b588:	c3                   	ret
   1b589:	ff 15 71 37 07 00    	call   *0x73771(%rip)        # 8ed00 <_GLOBAL_OFFSET_TABLE_+0x7f0>
   1b58f:	e9 de fe ff ff       	jmp    1b472 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0xf2>
   1b594:	48 89 c3             	mov    %rax,%rbx
   1b597:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   1b59c:	48 85 f6             	test   %rsi,%rsi
   1b59f:	74 10                	je     1b5b1 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::decrypt::<aead::Payload>+0x231>
   1b5a1:	48 8b 7c 24 18       	mov    0x18(%rsp),%rdi
   1b5a6:	ba 01 00 00 00       	mov    $0x1,%edx
   1b5ab:	ff 15 df 31 07 00    	call   *0x731df(%rip)        # 8e790 <_GLOBAL_OFFSET_TABLE_+0x280>
   1b5b1:	48 89 df             	mov    %rbx,%rdi
   1b5b4:	e8 97 ba fe ff       	call   7050 <_Unwind_Resume@plt>
   1b5b9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

000000000001b5c0 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::encrypt::<aead::Payload>>:
   1b5c0:	55                   	push   %rbp
   1b5c1:	48 89 e5             	mov    %rsp,%rbp
   1b5c4:	41 57                	push   %r15
   1b5c6:	41 56                	push   %r14
   1b5c8:	41 55                	push   %r13
   1b5ca:	41 54                	push   %r12
   1b5cc:	53                   	push   %rbx
   1b5cd:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   1b5d1:	48 81 ec 00 04 00 00 	sub    $0x400,%rsp
   1b5d8:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
   1b5dd:	48 8b 59 08          	mov    0x8(%rcx),%rbx
   1b5e1:	49 89 df             	mov    %rbx,%r15
   1b5e4:	49 83 c7 10          	add    $0x10,%r15
   1b5e8:	79 0f                	jns    1b5f9 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::encrypt::<aead::Payload>+0x39>
   1b5ea:	45 31 e4             	xor    %r12d,%r12d
   1b5ed:	4c 89 e7             	mov    %r12,%rdi
   1b5f0:	4c 89 fe             	mov    %r15,%rsi
   1b5f3:	ff 15 0f 31 07 00    	call   *0x7310f(%rip)        # 8e708 <_GLOBAL_OFFSET_TABLE_+0x1f8>
   1b5f9:	49 89 d5             	mov    %rdx,%r13
   1b5fc:	49 89 f6             	mov    %rsi,%r14
   1b5ff:	4c 8b 21             	mov    (%rcx),%r12
   1b602:	48 8b 41 10          	mov    0x10(%rcx),%rax
   1b606:	48 89 44 24 40       	mov    %rax,0x40(%rsp)
   1b60b:	48 8b 41 18          	mov    0x18(%rcx),%rax
   1b60f:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   1b614:	74 4b                	je     1b661 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::encrypt::<aead::Payload>+0xa1>
   1b616:	4c 89 64 24 18       	mov    %r12,0x18(%rsp)
   1b61b:	ff 15 1f 34 07 00    	call   *0x7341f(%rip)        # 8ea40 <_GLOBAL_OFFSET_TABLE_+0x530>
   1b621:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   1b627:	be 01 00 00 00       	mov    $0x1,%esi
   1b62c:	4c 89 ff             	mov    %r15,%rdi
   1b62f:	ff 15 c3 35 07 00    	call   *0x735c3(%rip)        # 8ebf8 <_GLOBAL_OFFSET_TABLE_+0x6e8>
   1b635:	48 85 c0             	test   %rax,%rax
   1b638:	74 b3                	je     1b5ed <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::encrypt::<aead::Payload>+0x2d>
   1b63a:	4c 89 3c 24          	mov    %r15,(%rsp)
   1b63e:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   1b643:	48 c7 44 24 10 00 00 	movq   $0x0,0x10(%rsp)
   1b64a:	00 00 
   1b64c:	48 83 fb f0          	cmp    $0xfffffffffffffff0,%rbx
   1b650:	4c 8b 64 24 18       	mov    0x18(%rsp),%r12
   1b655:	73 24                	jae    1b67b <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::encrypt::<aead::Payload>+0xbb>
   1b657:	45 31 ff             	xor    %r15d,%r15d
   1b65a:	48 85 db             	test   %rbx,%rbx
   1b65d:	75 3e                	jne    1b69d <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::encrypt::<aead::Payload>+0xdd>
   1b65f:	eb 52                	jmp    1b6b3 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::encrypt::<aead::Payload>+0xf3>
   1b661:	48 c7 04 24 00 00 00 	movq   $0x0,(%rsp)
   1b668:	00 
   1b669:	48 c7 44 24 08 01 00 	movq   $0x1,0x8(%rsp)
   1b670:	00 00 
   1b672:	48 c7 44 24 10 00 00 	movq   $0x0,0x10(%rsp)
   1b679:	00 00 
   1b67b:	48 89 e7             	mov    %rsp,%rdi
   1b67e:	b9 01 00 00 00       	mov    $0x1,%ecx
   1b683:	41 b8 01 00 00 00    	mov    $0x1,%r8d
   1b689:	31 f6                	xor    %esi,%esi
   1b68b:	48 89 da             	mov    %rbx,%rdx
   1b68e:	e8 7d bf fe ff       	call   7610 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   1b693:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   1b698:	4c 8b 7c 24 10       	mov    0x10(%rsp),%r15
   1b69d:	4a 8d 3c 38          	lea    (%rax,%r15,1),%rdi
   1b6a1:	4c 89 e6             	mov    %r12,%rsi
   1b6a4:	48 89 da             	mov    %rbx,%rdx
   1b6a7:	49 89 c4             	mov    %rax,%r12
   1b6aa:	ff 15 c0 38 07 00    	call   *0x738c0(%rip)        # 8ef70 <memcpy@GLIBC_2.14>
   1b6b0:	4c 89 e0             	mov    %r12,%rax
   1b6b3:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   1b6b8:	49 01 df             	add    %rbx,%r15
   1b6bb:	4c 89 7c 24 10       	mov    %r15,0x10(%rsp)
   1b6c0:	0f 10 05 61 bd 05 00 	movups 0x5bd61(%rip),%xmm0        # 77428 <anon.747060d88ef99e7a9a3b109ab8bfd015.9.llvm.6566165678146464307+0x38>
   1b6c7:	0f 29 84 24 00 01 00 	movaps %xmm0,0x100(%rsp)
   1b6ce:	00 
   1b6cf:	41 0f 10 06          	movups (%r14),%xmm0
   1b6d3:	41 0f 10 4e 10       	movups 0x10(%r14),%xmm1
   1b6d8:	0f 29 84 24 10 01 00 	movaps %xmm0,0x110(%rsp)
   1b6df:	00 
   1b6e0:	0f 29 8c 24 20 01 00 	movaps %xmm1,0x120(%rsp)
   1b6e7:	00 
   1b6e8:	49 8b 45 00          	mov    0x0(%r13),%rax
   1b6ec:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
   1b6f1:	41 8b 45 08          	mov    0x8(%r13),%eax
   1b6f5:	89 44 24 30          	mov    %eax,0x30(%rsp)
   1b6f9:	48 8d 05 14 39 07 00 	lea    0x73914(%rip),%rax        # 8f014 <chacha20::avx2_cpuid::STORAGE>
   1b700:	0f b6 00             	movzbl (%rax),%eax
   1b703:	3c ff                	cmp    $0xff,%al
   1b705:	0f 84 29 01 00 00    	je     1b834 <<chacha20poly1305::ChaChaPoly1305<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>> as aead::Aead>::encrypt::<aead::Payload>+0x274>
   1b70b:	0f 57 c0             	xorps  %xmm0,%xmm0
   1b70e:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
   1b715:	00 
   1b716:	0f 29 84 24 d0 00 00 	movaps %xmm0,0xd0(%rsp)
   1b71d:	00 
   1b71e:	0f                   	.byte 0xf
   1b71f:	29                   	.byte 0x29
   1b720:	84 24 c0             	test   %ah,(%rax,%rax,8)
	...

=== A=0x40150AF F=0x150af ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000014eaf <<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>> as cipher::stream_core::StreamCipherCore>::process_with_backend::<cipher::stream_core::ApplyBlocksCtx<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>>+0x19f>:
   14eaf:	0f 6f 5c 01 20       	movq   0x20(%rcx,%rax,1),%mm3
   14eb4:	f3 46 0f 6f 64 01 30 	movdqu 0x30(%rcx,%r8,1),%xmm12
   14ebb:	66 0f fe f0          	paddd  %xmm0,%xmm6
   14ebf:	66 41 0f ef f4       	pxor   %xmm12,%xmm6
   14ec4:	66 44 0f fe c3       	paddd  %xmm3,%xmm8
   14ec9:	66 45 0f ef c3       	pxor   %xmm11,%xmm8
   14ece:	66 0f fe ea          	paddd  %xmm2,%xmm5
   14ed2:	66 41 0f ef ea       	pxor   %xmm10,%xmm5
   14ed7:	66 0f fe f9          	paddd  %xmm1,%xmm7
   14edb:	66 41 0f ef f9       	pxor   %xmm9,%xmm7
   14ee0:	48 ff c6             	inc    %rsi
   14ee3:	66 0f fe c4          	paddd  %xmm4,%xmm0
   14ee7:	f3 42 0f 7f 3c 02    	movdqu %xmm7,(%rdx,%r8,1)
   14eed:	f3 42 0f 7f 6c 02 10 	movdqu %xmm5,0x10(%rdx,%r8,1)
   14ef4:	f3 46 0f 7f 44 02 20 	movdqu %xmm8,0x20(%rdx,%r8,1)
   14efb:	f3 42 0f 7f 74 02 30 	movdqu %xmm6,0x30(%rdx,%r8,1)
   14f02:	48 39 f0             	cmp    %rsi,%rax
   14f05:	0f 85 55 fe ff ff    	jne    14d60 <<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>> as cipher::stream_core::StreamCipherCore>::process_with_backend::<cipher::stream_core::ApplyBlocksCtx<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>>+0x50>
   14f0b:	66 0f 7e 47 30       	movd   %xmm0,0x30(%rdi)
   14f10:	5d                   	pop    %rbp
   14f11:	c3                   	ret
   14f12:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   14f19:	00 00 00 
   14f1c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000014f20 <<star_core::serv::enclave::Enclave>::issue_token>:
   14f20:	55                   	push   %rbp
   14f21:	48 89 e5             	mov    %rsp,%rbp
   14f24:	41 57                	push   %r15
   14f26:	41 56                	push   %r14
   14f28:	41 55                	push   %r13
   14f2a:	41 54                	push   %r12
   14f2c:	53                   	push   %rbx
   14f2d:	48 83 ec 18          	sub    $0x18,%rsp
   14f31:	48 89 fb             	mov    %rdi,%rbx
   14f34:	48 83 f9 60          	cmp    $0x60,%rcx
   14f38:	0f 82 8c 00 00 00    	jb     14fca <<star_core::serv::enclave::Enclave>::issue_token+0xaa>
   14f3e:	4d 89 c4             	mov    %r8,%r12
   14f41:	4c 8d 42 20          	lea    0x20(%rdx),%r8
   14f45:	48 8d 42 40          	lea    0x40(%rdx),%rax
   14f49:	48 83 c1 a0          	add    $0xffffffffffffffa0,%rcx
   14f4d:	4c 8d 52 60          	lea    0x60(%rdx),%r10
   14f51:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   14f55:	49 89 cb             	mov    %rcx,%r11
   14f58:	b9 20 00 00 00       	mov    $0x20,%ecx
   14f5d:	41 b9 20 00 00 00    	mov    $0x20,%r9d
   14f63:	49 89 f5             	mov    %rsi,%r13
   14f66:	49 89 d6             	mov    %rdx,%r14
   14f69:	48 89 c2             	mov    %rax,%rdx
   14f6c:	41 53                	push   %r11
   14f6e:	41 52                	push   %r10
   14f70:	6a 20                	push   $0x20
   14f72:	41 56                	push   %r14
   14f74:	ff 15 e6 9e 07 00    	call   *0x79ee6(%rip)        # 8ee60 <_GLOBAL_OFFSET_TABLE_+0x950>
   14f7a:	48 83 c4 20          	add    $0x20,%rsp
   14f7e:	4c 8b 75 c0          	mov    -0x40(%rbp),%r14
   14f82:	49 83 fe ff          	cmp    $0xffffffffffffffff,%r14
   14f86:	74 42                	je     14fca <<star_core::serv::enclave::Enclave>::issue_token+0xaa>
   14f88:	4c 8b 7d c8          	mov    -0x38(%rbp),%r15
   14f8c:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
   14f90:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   14f94:	4c 89 ee             	mov    %r13,%rsi
   14f97:	4c 89 fa             	mov    %r15,%rdx
   14f9a:	4d 89 e0             	mov    %r12,%r8
   14f9d:	ff 15 bd 97 07 00    	call   *0x797bd(%rip)        # 8e760 <_GLOBAL_OFFSET_TABLE_+0x250>
   14fa3:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   14fa7:	48 89 43 10          	mov    %rax,0x10(%rbx)
   14fab:	0f 10 45 c0          	movups -0x40(%rbp),%xmm0
   14faf:	0f 11 03             	movups %xmm0,(%rbx)
   14fb2:	4d 85 f6             	test   %r14,%r14
   14fb5:	74 1e                	je     14fd5 <<star_core::serv::enclave::Enclave>::issue_token+0xb5>
   14fb7:	ba 01 00 00 00       	mov    $0x1,%edx
   14fbc:	4c 89 ff             	mov    %r15,%rdi
   14fbf:	4c 89 f6             	mov    %r14,%rsi
   14fc2:	ff 15 c8 97 07 00    	call   *0x797c8(%rip)        # 8e790 <_GLOBAL_OFFSET_TABLE_+0x280>
   14fc8:	eb 0b                	jmp    14fd5 <<star_core::serv::enclave::Enclave>::issue_token+0xb5>
   14fca:	c6 43 08 01          	movb   $0x1,0x8(%rbx)
   14fce:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   14fd5:	48 89 d8             	mov    %rbx,%rax
   14fd8:	48 83 c4 18          	add    $0x18,%rsp
   14fdc:	5b                   	pop    %rbx
   14fdd:	41 5c                	pop    %r12
   14fdf:	41 5d                	pop    %r13
   14fe1:	41 5e                	pop    %r14
   14fe3:	41 5f                	pop    %r15
   14fe5:	5d                   	pop    %rbp
   14fe6:	c3                   	ret
   14fe7:	48 89 c3             	mov    %rax,%rbx
   14fea:	4d 85 f6             	test   %r14,%r14
   14fed:	74 11                	je     15000 <<star_core::serv::enclave::Enclave>::issue_token+0xe0>
   14fef:	ba 01 00 00 00       	mov    $0x1,%edx
   14ff4:	4c 89 ff             	mov    %r15,%rdi
   14ff7:	4c 89 f6             	mov    %r14,%rsi
   14ffa:	ff 15 90 97 07 00    	call   *0x79790(%rip)        # 8e790 <_GLOBAL_OFFSET_TABLE_+0x280>
   15000:	48 89 df             	mov    %rbx,%rdi
   15003:	e8 48 20 ff ff       	call   7050 <_Unwind_Resume@plt>
   15008:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   1500f:	00 

0000000000015010 <<star_core::serv::enclave::Enclave>::issue_token_plain>:
   15010:	55                   	push   %rbp
   15011:	48 89 e5             	mov    %rsp,%rbp
   15014:	41 57                	push   %r15
   15016:	41 56                	push   %r14
   15018:	41 55                	push   %r13
   1501a:	41 54                	push   %r12
   1501c:	53                   	push   %rbx
   1501d:	48 83 ec 78          	sub    $0x78,%rsp
   15021:	4d 89 c6             	mov    %r8,%r14
   15024:	49 89 cf             	mov    %rcx,%r15
   15027:	49 89 d4             	mov    %rdx,%r12
   1502a:	49 89 f5             	mov    %rsi,%r13
   1502d:	48 89 fb             	mov    %rdi,%rbx
   15030:	48 89 d7             	mov    %rdx,%rdi
   15033:	48 89 ce             	mov    %rcx,%rsi
   15036:	ff 15 94 95 07 00    	call   *0x79594(%rip)        # 8e5d0 <_GLOBAL_OFFSET_TABLE_+0xc0>
   1503c:	49 83 ff 50          	cmp    $0x50,%r15
   15040:	0f 82 e3 00 00 00    	jb     15129 <<star_core::serv::enclave::Enclave>::issue_token_plain+0x119>
   15046:	41 0f 10 44 24 20    	movups 0x20(%r12),%xmm0
   1504c:	41 0f 10 4c 24 30    	movups 0x30(%r12),%xmm1
   15052:	0f 29 4d 90          	movaps %xmm1,-0x70(%rbp)
   15056:	0f 29 45 80          	movaps %xmm0,-0x80(%rbp)
   1505a:	41 0f 10 04 24       	movups (%r12),%xmm0
   1505f:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   15065:	0f 29 85 60 ff ff ff 	movaps %xmm0,-0xa0(%rbp)
   1506c:	0f 29 8d 70 ff ff ff 	movaps %xmm1,-0x90(%rbp)
   15073:	4d 8b 7c 24 40       	mov    0x40(%r12),%r15
   15078:	4d 8b 64 24 48       	mov    0x48(%r12),%r12
   1507d:	4d 8b 45 58          	mov    0x58(%r13),%r8
   15081:	41 0f 10 45 60       	movups 0x60(%r13),%xmm0
   15086:	41 0f 10 4d 70       	movups 0x70(%r13),%xmm1
   1508b:	0f 29 4d b0          	movaps %xmm1,-0x50(%rbp)
   1508f:	0f 29 45 a0          	movaps %xmm0,-0x60(%rbp)
   15093:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   1509a:	4c 8d 4d a0          	lea    -0x60(%rbp),%r9
   1509e:	4c 89 e6             	mov    %r12,%rsi
   150a1:	4c 89 fa             	mov    %r15,%rdx
   150a4:	4c 89 f1             	mov    %r14,%rcx
   150a7:	ff 15 2b 9e 07 00    	call   *0x79e2b(%rip)        # 8eed8 <_GLOBAL_OFFSET_TABLE_+0x9c8>
   150ad:	84 c0                	test   %al,%al
   150af:	74 78                	je     15129 <<star_core::serv::enclave::Enclave>::issue_token_plain+0x119>
   150b1:	48 8d 7d a0          	lea    -0x60(%rbp),%rdi
   150b5:	48 8d b5 60 ff ff ff 	lea    -0xa0(%rbp),%rsi
   150bc:	4c 89 e2             	mov    %r12,%rdx
   150bf:	4c 89 f9             	mov    %r15,%rcx
   150c2:	ff 15 90 94 07 00    	call   *0x79490(%rip)        # 8e558 <_GLOBAL_OFFSET_TABLE_+0x48>
   150c8:	48 c7 45 c0 00 00 00 	movq   $0x0,-0x40(%rbp)
   150cf:	00 
   150d0:	48 c7 45 c8 01 00 00 	movq   $0x1,-0x38(%rbp)
   150d7:	00 
   150d8:	48 c7 45 d0 00 00 00 	movq   $0x0,-0x30(%rbp)
   150df:	00 
   150e0:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   150e4:	ba 20 00 00 00       	mov    $0x20,%edx
   150e9:	b9 01 00 00 00       	mov    $0x1,%ecx
   150ee:	41 b8 01 00 00 00    	mov    $0x1,%r8d
   150f4:	31 f6                	xor    %esi,%esi
   150f6:	e8 95 20 ff ff       	call   7190 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   150fb:	48 8b 45 c8          	mov    -0x38(%rbp),%rax
   150ff:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
   15103:	0f 10 45 a0          	movups -0x60(%rbp),%xmm0
   15107:	0f 10 4d b0          	movups -0x50(%rbp),%xmm1
   1510b:	0f 11 4c 08 10       	movups %xmm1,0x10(%rax,%rcx,1)
   15110:	0f 11 04 08          	movups %xmm0,(%rax,%rcx,1)
   15114:	48 83 c1 20          	add    $0x20,%rcx
   15118:	48 89 4d d0          	mov    %rcx,-0x30(%rbp)
   1511c:	48 89 4b 10          	mov    %rcx,0x10(%rbx)
   15120:	0f 10 45 c0          	movups -0x40(%rbp),%xmm0
   15124:	0f 11 03             	movups %xmm0,(%rbx)
   15127:	eb 31                	jmp    1515a <<star_core::serv::enclave::Enclave>::issue_token_plain+0x14a>
   15129:	ff 15 11 99 07 00    	call   *0x79911(%rip)        # 8ea40 <_GLOBAL_OFFSET_TABLE_+0x530>
   1512f:	bf 01 00 00 00       	mov    $0x1,%edi
   15134:	be 01 00 00 00       	mov    $0x1,%esi
   15139:	ff 15 b9 9a 07 00    	call   *0x79ab9(%rip)        # 8ebf8 <_GLOBAL_OFFSET_TABLE_+0x6e8>
   1513f:	48 85 c0             	test   %rax,%rax
   15142:	74 28                	je     1516c <<star_core::serv::enclave::Enclave>::issue_token_plain+0x15c>
   15144:	c6 00 00             	movb   $0x0,(%rax)
   15147:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   1514e:	48 89 43 08          	mov    %rax,0x8(%rbx)
   15152:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   15159:	00 
   1515a:	48 89 d8             	mov    %rbx,%rax
   1515d:	48 83 c4 78          	add    $0x78,%rsp
   15161:	5b                   	pop    %rbx
   15162:	41 5c                	pop    %r12
   15164:	41 5d                	pop    %r13
   15166:	41 5e                	pop    %r14
   15168:	41 5f                	pop    %r15
   1516a:	5d                   	pop    %rbp
   1516b:	c3                   	ret
   1516c:	bf 01 00 00 00       	mov    $0x1,%edi
   15171:	be 01 00 00 00       	mov    $0x1,%esi
   15176:	ff 15 74 9d 07 00    	call   *0x79d74(%rip)        # 8eef0 <_GLOBAL_OFFSET_TABLE_+0x9e0>
   1517c:	48 89 c3             	mov    %rax,%rbx
   1517f:	48 8b 75 c0          	mov    -0x40(%rbp),%rsi
   15183:	48 85 f6             	test   %rsi,%rsi
   15186:	74 0f                	je     15197 <<star_core::serv::enclave::Enclave>::issue_token_plain+0x187>
   15188:	48 8b 7d c8          	mov    -0x38(%rbp),%rdi
   1518c:	ba 01 00 00 00       	mov    $0x1,%edx
   15191:	ff 15 f9 95 07 00    	call   *0x795f9(%rip)        # 8e790 <_GLOBAL_OFFSET_TABLE_+0x280>
   15197:	48 89 df             	mov    %rbx,%rdi
   1519a:	e8 b1 1e ff ff       	call   7050 <_Unwind_Resume@plt>
   1519f:	90                   	nop

00000000000151a0 <<star_core::serv::enclave::Enclave>::new>:
   151a0:	55                   	push   %rbp
   151a1:	48 89 e5             	mov    %rsp,%rbp
   151a4:	41 57                	push   %r15
   151a6:	41 56                	push   %r14
   151a8:	53                   	push   %rbx
   151a9:	48 83 ec 58          	sub    $0x58,%rsp
   151ad:	49 89 d6             	mov    %rdx,%r14
   151b0:	41 89 f7             	mov    %esi,%r15d
   151b3:	48 89 fb             	mov    %rdi,%rbx
   151b6:	48 8d 7d 90          	lea    -0x70(%rbp),%rdi
   151ba:	48 89 ce             	mov    %rcx,%rsi
   151bd:	ff 15 2d 96 07 00    	call   *0x7962d(%rip)        # 8e7f0 <_GLOBAL_OFFSET_TABLE_+0x2e0>
   151c3:	48 8b 45 90          	mov    -0x70(%rbp),%rax
   151c7:	0f b6 4d 98          	movzbl -0x68(%rbp),%ecx
   151cb:	48 83 f8 ff          	cmp    $0xffffffffffffffff,%rax
   151cf:	74 43                	je     15214 <<star_core::serv::enclave::Enclave>::new+0x74>
   151d1:	0f 10 45 d8          	movups -0x28(%rbp),%xmm0
   151d5:	0f 11 43 48          	movups %xmm0,0x48(%rbx)
   151d9:	0f 10 45 99          	movups -0x67(%rbp),%xmm0
   151dd:	0f 10 4d a9          	movups -0x57(%rbp),%xmm1
   151e1:	0f 10 55 b9          	movups -0x47(%rbp),%xmm2
   151e5:	0f 10 5d c9          	movups -0x37(%rbp),%xmm3
   151e9:	0f 11 5b 39          	movups %xmm3,0x39(%rbx)
   151ed:	0f 11 53 29          	movups %xmm2,0x29(%rbx)
   151f1:	0f 11 4b 19          	movups %xmm1,0x19(%rbx)
   151f5:	0f 11 43 09          	movups %xmm0,0x9(%rbx)
   151f9:	44 89 fa             	mov    %r15d,%edx
   151fc:	48 0f ca             	bswap  %rdx
   151ff:	41 0f 10 06          	movups (%r14),%xmm0
   15203:	41 0f 10 4e 10       	movups 0x10(%r14),%xmm1
   15208:	0f 11 4b 70          	movups %xmm1,0x70(%rbx)
   1520c:	0f 11 43 60          	movups %xmm0,0x60(%rbx)
   15210:	48 89 53 58          	mov    %rdx,0x58(%rbx)
   15214:	88 4b 08             	mov    %cl,0x8(%rbx)
   15217:	48 89 03             	mov    %rax,(%rbx)
   1521a:	48 89 d8             	mov    %rbx,%rax
   1521d:	48 83 c4 58          	add    $0x58,%rsp
   15221:	5b                   	pop    %rbx
   15222:	41 5e                	pop    %r14
   15224:	41 5f                	pop    %r15
   15226:	5d                   	pop    %rbp
   15227:	c3                   	ret
   15228:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   1522f:	00 

0000000000015230 <<star_core::serv::core::token::Token>::random>:
   15230:	55                   	push   %rbp
   15231:	48 89 e5             	mov    %rsp,%rbp
   15234:	41 57                	push   %r15
   15236:	41 56                	push   %r14
   15238:	41 55                	push   %r13
   1523a:	41 54                	push   %r12
   1523c:	53                   	push   %rbx
   1523d:	48 81 ec e8 00 00 00 	sub    $0xe8,%rsp
   15244:	48 89 fb             	mov    %rdi,%rbx
   15247:	0f 57 db             	xorps  %xmm3,%xmm3
   1524a:	0f 29 9d 10 ff ff ff 	movaps %xmm3,-0xf0(%rbp)
   15251:	f3 0f 6f 06          	movdqu (%rsi),%xmm0
   15255:	0f 10 4e 10          	movups 0x10(%rsi),%xmm1
   15259:	66 0f 7f 85 f0 fe ff 	movdqa %xmm0,-0x110(%rbp)
   15260:	ff 
   15261:	0f 29 8d 00 ff ff ff 	movaps %xmm1,-0x100(%rbp)
   15268:	0f 10 15 81 de 05 00 	movups 0x5de81(%rip),%xmm2        # 730f0 <_fini+0x4d8>
   1526f:	0f 29 95 70 ff ff ff 	movaps %xmm2,-0x90(%rbp)
   15276:	0f 10 15 63 de 05 00 	movups 0x5de63(%rip),%xmm2        # 730e0 <_fini+0x4c8>
   1527d:	0f 29 95 60 ff ff ff 	movaps %xmm2,-0xa0(%rbp)
   15284:	48 c7 45 80 00 00 00 	movq   $0x0,-0x80(%rbp)
   1528b:	00 
   1528c:	4c 8d 75 88          	lea    -0x78(%rbp),%r14
   15290:	48 89 4d 88          	mov    %rcx,-0x78(%rbp)
   15294:	48 89 55 90          	mov    %rdx,-0x70(%rbp)
   15298:	f3 0f 7f 45 98       	movdqu %xmm0,-0x68(%rbp)
   1529d:	0f 11 4d a8          	movups %xmm1,-0x58(%rbp)
   152a1:	0f 11 5d b8          	movups %xmm3,-0x48(%rbp)
   152a5:	c6 45 c8 30          	movb   $0x30,-0x38(%rbp)
   152a9:	49                   	rex.WB
   152aa:	bc 00 00 00 00       	mov    $0x0,%esp

=== A=0x48D87D0 F=0x8d87d0 ===

./target/release/analysis:     file format elf64-x86-64


