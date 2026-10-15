=== VALGRIND OUTPUT ===
==107== Memcheck, a memory error detector
==107== Copyright (C) 2002-2026, and GNU GPL'd, by Julian Seward et al.
==107== Using Valgrind-3.27.1 and LibVEX; rerun with -h for copyright info
==107== Command: ./target/release/analysis
==107== 
==107== Conditional jump or move depends on uninitialised value(s)
==107==    at 0x40242E0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==107==    by 0x403BE29: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==107==    by 0x400F02E: analysis::main (in /workspace/star/core/target/release/analysis)
==107==    by 0x400DD05: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==107==    by 0x400E65B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
==107== Conditional jump or move depends on uninitialised value(s)
==107==    at 0x4021C90: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==107==    by 0x403D2F3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==107==    by 0x400F02E: analysis::main (in /workspace/star/core/target/release/analysis)
==107==    by 0x400DD05: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==107==    by 0x400E65B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
==107== Conditional jump or move depends on uninitialised value(s)
==107==    at 0x40242E0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==107==    by 0x403BE29: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==107==    by 0x403D8A9: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==107==    by 0x400F665: analysis::main (in /workspace/star/core/target/release/analysis)
==107==    by 0x400DD05: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==107==    by 0x400E65B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
==107== Conditional jump or move depends on uninitialised value(s)
==107==    at 0x4021C90: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==107==    by 0x403D2F3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==107==    by 0x403D8A9: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==107==    by 0x400F665: analysis::main (in /workspace/star/core/target/release/analysis)
==107==    by 0x400DD05: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==107==    by 0x400E65B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
==107== Conditional jump or move depends on uninitialised value(s)
==107==    at 0x403D9AB: <star_core::serv::enclave::Enclave>::issue_token_plain (in /workspace/star/core/target/release/analysis)
==107==    by 0x403D8D2: <star_core::serv::enclave::Enclave>::issue_token (in /workspace/star/core/target/release/analysis)
==107==    by 0x400F665: analysis::main (in /workspace/star/core/target/release/analysis)
==107==    by 0x400DD05: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==107==    by 0x400E65B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
==107== Conditional jump or move depends on uninitialised value(s)
==107==    at 0x40242E0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==107==    by 0x400FA9D: analysis::main (in /workspace/star/core/target/release/analysis)
==107==    by 0x400DD05: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==107==    by 0x400E65B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
==107== Conditional jump or move depends on uninitialised value(s)
==107==    at 0x40242E0: <hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh (in /workspace/star/core/target/release/analysis)
==107==    by 0x403BE29: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==107==    by 0x40117F7: analysis::main (in /workspace/star/core/target/release/analysis)
==107==    by 0x400DD05: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==107==    by 0x400E65B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
==107== Conditional jump or move depends on uninitialised value(s)
==107==    at 0x4021C90: <hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open (in /workspace/star/core/target/release/analysis)
==107==    by 0x403D2F3: <star_core::secure_channel::server::SecureChannelServer>::receive_hpke (in /workspace/star/core/target/release/analysis)
==107==    by 0x40117F7: analysis::main (in /workspace/star/core/target/release/analysis)
==107==    by 0x400DD05: std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()> (in /workspace/star/core/target/release/analysis)
==107==    by 0x400E65B: std::rt::lang_start::<()>::{closure#0} (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C6: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
done
==107== 
==107== HEAP SUMMARY:
==107==     in use at exit: 544 bytes in 1 blocks
==107==   total heap usage: 93 allocs, 92 frees, 7,136 bytes allocated
==107== 
==107== 544 bytes in 1 blocks are still reachable in loss record 1 of 1
==107==    at 0x48CC7D0: malloc (in /nix/store/mf7rzpknxwg0q1wf0nm8gfzikapg00h1-valgrind-3.27.1/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==107==    by 0x40686E7: <alloc::collections::btree::map::entry::VacantEntry<usize, std::sys::pal::unix::stack_overflow::thread_info::ThreadInfo>>::insert_entry (in /workspace/star/core/target/release/analysis)
==107==    by 0x4059783: std::sys::pal::unix::stack_overflow::thread_info::set_current_info (in /workspace/star/core/target/release/analysis)
==107==    by 0x405A3AF: std::sys::pal::unix::stack_overflow::imp::init (in /workspace/star/core/target/release/analysis)
==107==    by 0x4068BAF: std::sys::pal::unix::init (in /workspace/star/core/target/release/analysis)
==107==    by 0x40553C0: std::rt::lang_start_internal (in /workspace/star/core/target/release/analysis)
==107==    by 0x4013113: main (in /workspace/star/core/target/release/analysis)
==107== 
==107== LEAK SUMMARY:
==107==    definitely lost: 0 bytes in 0 blocks
==107==    indirectly lost: 0 bytes in 0 blocks
==107==      possibly lost: 0 bytes in 0 blocks
==107==    still reachable: 544 bytes in 1 blocks
==107==         suppressed: 0 bytes in 0 blocks
==107== 
==107== Use --track-origins=yes to see where uninitialised values come from
==107== For lists of detected and suppressed errors, rerun with: -s
==107== ERROR SUMMARY: 8 errors from 8 contexts (suppressed: 0 from 0)

=== A=0x40242E0 F=0x242e0 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

00000000000240e0 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x140>:
   240e0:	7d aa                	jge    2408c <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0xec>
   240e2:	00 40 0f             	add    %al,0xf(%rax)
   240e5:	94                   	xchg   %eax,%esp
   240e6:	c7                   	(bad)
   240e7:	ff 15 9b eb 05 00    	call   *0x5eb9b(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   240ed:	88 45 c8             	mov    %al,-0x38(%rbp)
   240f0:	31 ff                	xor    %edi,%edi
   240f2:	80 7d ab 00          	cmpb   $0x0,-0x55(%rbp)
   240f6:	40 0f 94 c7          	sete   %dil
   240fa:	ff 15 88 eb 05 00    	call   *0x5eb88(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24100:	88 45 c9             	mov    %al,-0x37(%rbp)
   24103:	31 ff                	xor    %edi,%edi
   24105:	80 7d ac 00          	cmpb   $0x0,-0x54(%rbp)
   24109:	40 0f 94 c7          	sete   %dil
   2410d:	ff 15 75 eb 05 00    	call   *0x5eb75(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24113:	88 45 ca             	mov    %al,-0x36(%rbp)
   24116:	31 ff                	xor    %edi,%edi
   24118:	80 7d ad 00          	cmpb   $0x0,-0x53(%rbp)
   2411c:	40 0f 94 c7          	sete   %dil
   24120:	ff 15 62 eb 05 00    	call   *0x5eb62(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24126:	88 45 cb             	mov    %al,-0x35(%rbp)
   24129:	31 ff                	xor    %edi,%edi
   2412b:	80 7d ae 00          	cmpb   $0x0,-0x52(%rbp)
   2412f:	40 0f 94 c7          	sete   %dil
   24133:	ff 15 4f eb 05 00    	call   *0x5eb4f(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24139:	41 89 c6             	mov    %eax,%r14d
   2413c:	31 ff                	xor    %edi,%edi
   2413e:	80 7d af 00          	cmpb   $0x0,-0x51(%rbp)
   24142:	40 0f 94 c7          	sete   %dil
   24146:	ff 15 3c eb 05 00    	call   *0x5eb3c(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2414c:	88 45 cc             	mov    %al,-0x34(%rbp)
   2414f:	31 ff                	xor    %edi,%edi
   24151:	80 7d b0 00          	cmpb   $0x0,-0x50(%rbp)
   24155:	40 0f 94 c7          	sete   %dil
   24159:	ff 15 29 eb 05 00    	call   *0x5eb29(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2415f:	88 45 cd             	mov    %al,-0x33(%rbp)
   24162:	31 ff                	xor    %edi,%edi
   24164:	80 7d b1 00          	cmpb   $0x0,-0x4f(%rbp)
   24168:	40 0f 94 c7          	sete   %dil
   2416c:	ff 15 16 eb 05 00    	call   *0x5eb16(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24172:	88 45 ce             	mov    %al,-0x32(%rbp)
   24175:	31 ff                	xor    %edi,%edi
   24177:	80 7d b2 00          	cmpb   $0x0,-0x4e(%rbp)
   2417b:	40 0f 94 c7          	sete   %dil
   2417f:	ff 15 03 eb 05 00    	call   *0x5eb03(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24185:	88 45 cf             	mov    %al,-0x31(%rbp)
   24188:	31 ff                	xor    %edi,%edi
   2418a:	80 7d b3 00          	cmpb   $0x0,-0x4d(%rbp)
   2418e:	40 0f 94 c7          	sete   %dil
   24192:	ff 15 f0 ea 05 00    	call   *0x5eaf0(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24198:	88 45 d0             	mov    %al,-0x30(%rbp)
   2419b:	31 ff                	xor    %edi,%edi
   2419d:	80 7d b4 00          	cmpb   $0x0,-0x4c(%rbp)
   241a1:	40 0f 94 c7          	sete   %dil
   241a5:	ff 15 dd ea 05 00    	call   *0x5eadd(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   241ab:	41 89 c4             	mov    %eax,%r12d
   241ae:	31 ff                	xor    %edi,%edi
   241b0:	80 7d b5 00          	cmpb   $0x0,-0x4b(%rbp)
   241b4:	40 0f 94 c7          	sete   %dil
   241b8:	ff 15 ca ea 05 00    	call   *0x5eaca(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   241be:	88 45 d1             	mov    %al,-0x2f(%rbp)
   241c1:	31 ff                	xor    %edi,%edi
   241c3:	80 7d b6 00          	cmpb   $0x0,-0x4a(%rbp)
   241c7:	40 0f 94 c7          	sete   %dil
   241cb:	ff 15 b7 ea 05 00    	call   *0x5eab7(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   241d1:	88 45 d2             	mov    %al,-0x2e(%rbp)
   241d4:	31 ff                	xor    %edi,%edi
   241d6:	80 7d b7 00          	cmpb   $0x0,-0x49(%rbp)
   241da:	40 0f 94 c7          	sete   %dil
   241de:	ff 15 a4 ea 05 00    	call   *0x5eaa4(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   241e4:	88 45 d3             	mov    %al,-0x2d(%rbp)
   241e7:	31 ff                	xor    %edi,%edi
   241e9:	80 7d b8 00          	cmpb   $0x0,-0x48(%rbp)
   241ed:	40 0f 94 c7          	sete   %dil
   241f1:	ff 15 91 ea 05 00    	call   *0x5ea91(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   241f7:	88 45 d4             	mov    %al,-0x2c(%rbp)
   241fa:	31 ff                	xor    %edi,%edi
   241fc:	80 7d b9 00          	cmpb   $0x0,-0x47(%rbp)
   24200:	40 0f 94 c7          	sete   %dil
   24204:	ff 15 7e ea 05 00    	call   *0x5ea7e(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2420a:	88 45 d5             	mov    %al,-0x2b(%rbp)
   2420d:	31 ff                	xor    %edi,%edi
   2420f:	80 7d ba 00          	cmpb   $0x0,-0x46(%rbp)
   24213:	40 0f 94 c7          	sete   %dil
   24217:	ff 15 6b ea 05 00    	call   *0x5ea6b(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   2421d:	88 45 d6             	mov    %al,-0x2a(%rbp)
   24220:	31 ff                	xor    %edi,%edi
   24222:	80 7d bb 00          	cmpb   $0x0,-0x45(%rbp)
   24226:	40 0f 94 c7          	sete   %dil
   2422a:	ff 15 58 ea 05 00    	call   *0x5ea58(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24230:	41 89 c5             	mov    %eax,%r13d
   24233:	31 ff                	xor    %edi,%edi
   24235:	80 7d bc 00          	cmpb   $0x0,-0x44(%rbp)
   24239:	40 0f 94 c7          	sete   %dil
   2423d:	ff 15 45 ea 05 00    	call   *0x5ea45(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24243:	88 45 d7             	mov    %al,-0x29(%rbp)
   24246:	31 ff                	xor    %edi,%edi
   24248:	80 7d bd 00          	cmpb   $0x0,-0x43(%rbp)
   2424c:	40 0f 94 c7          	sete   %dil
   24250:	ff 15 32 ea 05 00    	call   *0x5ea32(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   24256:	0f b6 4d be          	movzbl -0x42(%rbp),%ecx
   2425a:	22 4d bf             	and    -0x41(%rbp),%cl
   2425d:	0f b6 55 c0          	movzbl -0x40(%rbp),%edx
   24261:	22 55 c1             	and    -0x3f(%rbp),%dl
   24264:	20 ca                	and    %cl,%dl
   24266:	0f b6 4d c2          	movzbl -0x3e(%rbp),%ecx
   2426a:	22 4d c3             	and    -0x3d(%rbp),%cl
   2426d:	22 4d c4             	and    -0x3c(%rbp),%cl
   24270:	20 d1                	and    %dl,%cl
   24272:	44 22 7d c5          	and    -0x3b(%rbp),%r15b
   24276:	44 22 7d c6          	and    -0x3a(%rbp),%r15b
   2427a:	44 22 7d c7          	and    -0x39(%rbp),%r15b
   2427e:	41 20 cf             	and    %cl,%r15b
   24281:	22 5d c8             	and    -0x38(%rbp),%bl
   24284:	22 5d c9             	and    -0x37(%rbp),%bl
   24287:	22 5d ca             	and    -0x36(%rbp),%bl
   2428a:	22 5d cb             	and    -0x35(%rbp),%bl
   2428d:	44 20 fb             	and    %r15b,%bl
   24290:	44 22 75 cc          	and    -0x34(%rbp),%r14b
   24294:	44 22 75 cd          	and    -0x33(%rbp),%r14b
   24298:	44 22 75 ce          	and    -0x32(%rbp),%r14b
   2429c:	44 22 75 cf          	and    -0x31(%rbp),%r14b
   242a0:	44 22 75 d0          	and    -0x30(%rbp),%r14b
   242a4:	41 20 de             	and    %bl,%r14b
   242a7:	44 22 65 d1          	and    -0x2f(%rbp),%r12b
   242ab:	44 22 65 d2          	and    -0x2e(%rbp),%r12b
   242af:	44 22 65 d3          	and    -0x2d(%rbp),%r12b
   242b3:	44 22 65 d4          	and    -0x2c(%rbp),%r12b
   242b7:	44 22 65 d5          	and    -0x2b(%rbp),%r12b
   242bb:	44 22 65 d6          	and    -0x2a(%rbp),%r12b
   242bf:	45 20 f4             	and    %r14b,%r12b
   242c2:	44 22 6d d7          	and    -0x29(%rbp),%r13b
   242c6:	41 20 c5             	and    %al,%r13b
   242c9:	45 20 e5             	and    %r12b,%r13b
   242cc:	41 80 e5 01          	and    $0x1,%r13b
   242d0:	41 0f b6 fd          	movzbl %r13b,%edi
   242d4:	ff 15 ae e9 05 00    	call   *0x5e9ae(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   242da:	84 c0                	test   %al,%al
   242dc:	48 8b 5d 90          	mov    -0x70(%rbp),%rbx
   242e0:	74 0d                	je     242ef <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x34f>
   242e2:	c6 43 08 03          	movb   $0x3,0x8(%rbx)
   242e6:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   242ed:	eb 32                	jmp    24321 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x381>
   242ef:	bf 20 00 00 00       	mov    $0x20,%edi
   242f4:	ff 15 76 e9 05 00    	call   *0x5e976(%rip)        # 82c70 <malloc@GLIBC_2.2.5>
   242fa:	48 85 c0             	test   %rax,%rax
   242fd:	74 3e                	je     2433d <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x39d>
   242ff:	0f 10 45 9e          	movups -0x62(%rbp),%xmm0
   24303:	0f 10 4d ae          	movups -0x52(%rbp),%xmm1
   24307:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   2430b:	0f 11 00             	movups %xmm0,(%rax)
   2430e:	48 c7 03 20 00 00 00 	movq   $0x20,(%rbx)
   24315:	48 89 43 08          	mov    %rax,0x8(%rbx)
   24319:	48 c7 43 10 20 00 00 	movq   $0x20,0x10(%rbx)
   24320:	00 
   24321:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   24325:	ff 15 fd e8 05 00    	call   *0x5e8fd(%rip)        # 82c28 <_GLOBAL_OFFSET_TABLE_+0x530>
   2432b:	48 89 d8             	mov    %rbx,%rax
   2432e:	48 83 c4 68          	add    $0x68,%rsp
   24332:	5b                   	pop    %rbx
   24333:	41 5c                	pop    %r12
   24335:	41 5d                	pop    %r13
   24337:	41 5e                	pop    %r14
   24339:	41 5f                	pop    %r15
   2433b:	5d                   	pop    %rbp
   2433c:	c3                   	ret
   2433d:	bf 01 00 00 00       	mov    $0x1,%edi
   24342:	be 20 00 00 00       	mov    $0x20,%esi
   24347:	ff 15 23 eb 05 00    	call   *0x5eb23(%rip)        # 82e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   2434d:	0f 0b                	ud2
   2434f:	eb 00                	jmp    24351 <<hpke_ng::kem::dh::X25519 as hpke_ng::kem::dh::DiffieHellman>::dh+0x3b1>
   24351:	48 89 c3             	mov    %rax,%rbx
   24354:	48 8d 7d 9e          	lea    -0x62(%rbp),%rdi
   24358:	ff 15 ca e8 05 00    	call   *0x5e8ca(%rip)        # 82c28 <_GLOBAL_OFFSET_TABLE_+0x530>
   2435e:	48 89 df             	mov    %rbx,%rdi
   24361:	e8 da 1c fe ff       	call   6040 <_Unwind_Resume@plt>
   24366:	ff 15 5c ec 05 00    	call   *0x5ec5c(%rip)        # 82fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   2436c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000024370 <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed>:
   24370:	55                   	push   %rbp
   24371:	48 89 e5             	mov    %rsp,%rbp
   24374:	41 57                	push   %r15
   24376:	41 56                	push   %r14
   24378:	41 55                	push   %r13
   2437a:	41 54                	push   %r12
   2437c:	53                   	push   %rbx
   2437d:	48 81 ec d8 00 00 00 	sub    $0xd8,%rsp
   24384:	48 89 fb             	mov    %rdi,%rbx
   24387:	48 8d bd 00 ff ff ff 	lea    -0x100(%rbp),%rdi
   2438e:	ba 90 00 00 00       	mov    $0x90,%edx
   24393:	ff 15 ff e7 05 00    	call   *0x5e7ff(%rip)        # 82b98 <memcpy@GLIBC_2.14>
   24399:	4c 8d b5 40 ff ff ff 	lea    -0xc0(%rbp),%r14
   243a0:	44 0f b6 a5 3f ff ff 	movzbl -0xc1(%rbp),%r12d
   243a7:	ff 
   243a8:	4c 8b ad 60 ff ff ff 	mov    -0xa0(%rbp),%r13
   243af:	4d 89 e7             	mov    %r12,%r15
   243b2:	49 83 f7 3f          	xor    $0x3f,%r15
   243b6:	42 c6 84 25 00 ff ff 	movb   $0x80,-0x100(%rbp,%r12,1)
   243bd:	ff 80 
   243bf:	49 8d 3c 2c          	lea    (%r12,%rbp,1),%rdi
   243c3:	48 81 c7 01 ff ff ff 	add    $0xffffffffffffff01,%rdi
   243ca:	31 f6                	xor    %esi,%esi
   243cc:	4c 89 fa             	mov    %r15,%rdx
   243cf:	ff 15 0b e6 05 00    	call   *0x5e60b(%rip)        # 829e0 <memset@GLIBC_2.2.5>
   243d5:	49 83 ff 07          	cmp    $0x7,%r15
   243d9:	77 63                	ja     2443e <<hmac::Hmac<sha2::Sha256> as digest::FixedOutput>::finalize_fixed+0xce>
   243db:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   243e2:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   243e9:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   243f0:	66 0f 6f 9d 30 ff ff 	movdqa -0xd0(%rbp),%xmm3
   243f7:	ff 
   243f8:	66 0f 7f 5d c0       	movdqa %xmm3,-0x40(%rbp)
   243fd:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   24401:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   24405:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   24409:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   2440d:	ba 01 00 00 00       	mov    $0x1,%edx
   24412:	4c 89 f7             	mov    %r14,%rdi
   24415:	ff 15 85 e8 05 00    	call   *0x5e885(%rip)        # 82ca0 <_GLOBAL_OFFSET_TABLE_+0x5a8>
   2441b:	0f 57 c0             	xorps  %xmm0,%xmm0
   2441e:	0f 29 85 00 ff ff ff 	movaps %xmm0,-0x100(%rbp)
   24425:	0f 29 85 10 ff ff ff 	movaps %xmm0,-0xf0(%rbp)
   2442c:	0f 29 85 20 ff ff ff 	movaps %xmm0,-0xe0(%rbp)
   24433:	48 c7 85 30 ff ff ff 	movq   $0x0,-0xd0(%rbp)
   2443a:	00 00 00 00 
   2443e:	49 c1 e5 09          	shl    $0x9,%r13
   24442:	41 c1 e4 03          	shl    $0x3,%r12d
   24446:	4d 09 ec             	or     %r13,%r12
   24449:	49 0f cc             	bswap  %r12
   2444c:	4c 89 a5 38 ff ff ff 	mov    %r12,-0xc8(%rbp)
   24453:	48 8b 85 30 ff ff ff 	mov    -0xd0(%rbp),%rax
   2445a:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   2445e:	48 8b 85 38 ff ff ff 	mov    -0xc8(%rbp),%rax
   24465:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
   24469:	0f 28 85 00 ff ff ff 	movaps -0x100(%rbp),%xmm0
   24470:	0f 28 8d 10 ff ff ff 	movaps -0xf0(%rbp),%xmm1
   24477:	0f 28 95 20 ff ff ff 	movaps -0xe0(%rbp),%xmm2
   2447e:	0f 29 55 b0          	movaps %xmm2,-0x50(%rbp)
   24482:	0f 29 4d a0          	movaps %xmm1,-0x60(%rbp)
   24486:	0f 29 45 90          	movaps %xmm0,-0x70(%rbp)
   2448a:	4c 8d 3d 5f bb 00 00 	lea    0xbb5f(%rip),%r15        # 2fff0 <sha2::sha256::compress256>
   24491:	48 8d 75 90          	lea    -0x70(%rbp),%rsi
   24495:	ba 01 00 00 00       	mov    $0x1,%edx
   2449a:	4c 89 f7             	mov    %r14,%rdi
   2449d:	41 ff d7             	call   *%r15
   244a0:	48 8d bd 68 ff ff ff 	lea    -0x98(%rbp),%rdi
   244a7:	66 0f 6f 85 40 ff ff 	movdqa -0xc0(%rbp),%xmm0
   244ae:	ff 
   244af:	66 0f 6f 8d 50 ff ff 	movdqa -0xb0(%rbp),%xmm1
   244b6:	ff 
   244b7:	66 0f ef db          	pxor   %xmm3,%xmm3
   244bb:	66 0f 6f d0          	movdqa %xmm0,%xmm2
   244bf:	66 0f 68 d3          	punpckhbw %xmm3,%xmm2
   244c3:	f2 0f 70 d2 1b       	pshuflw $0x1b,%xmm2,%xmm2
   244c8:	f3 0f 70 d2 1b       	pshufhw $0x1b,%xmm2,%xmm2
   244cd:	66 0f 60 c3          	punpcklbw %xmm3,%xmm0
   244d1:	f2 0f 70 c0 1b       	pshuflw $0x1b,%xmm0,%xmm0
   244d6:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   244db:	66 0f 67 c2          	packuswb %xmm2,%xmm0
   244df:	66                   	data16

=== A=0x4021C90 F=0x21c90 ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

0000000000021a90 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x520>:
   21a90:	4c 89 f6             	mov    %r14,%rsi
   21a93:	e8 f8 7a 00 00       	call   29590 <<poly1305::backend::autodetect::State as universal_hash::UniversalHash>::finalize>
   21a98:	0f b6 44 24 50       	movzbl 0x50(%rsp),%eax
   21a9d:	31 ff                	xor    %edi,%edi
   21a9f:	41 3a 04 1f          	cmp    (%r15,%rbx,1),%al
   21aa3:	40 0f 94 c7          	sete   %dil
   21aa7:	ff 15 db 11 06 00    	call   *0x611db(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21aad:	88 44 24 40          	mov    %al,0x40(%rsp)
   21ab1:	0f b6 44 24 51       	movzbl 0x51(%rsp),%eax
   21ab6:	31 ff                	xor    %edi,%edi
   21ab8:	41 3a 44 1f 01       	cmp    0x1(%r15,%rbx,1),%al
   21abd:	40 0f 94 c7          	sete   %dil
   21ac1:	ff 15 c1 11 06 00    	call   *0x611c1(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21ac7:	88 44 24 38          	mov    %al,0x38(%rsp)
   21acb:	0f b6 44 24 52       	movzbl 0x52(%rsp),%eax
   21ad0:	31 ff                	xor    %edi,%edi
   21ad2:	41 3a 44 1f 02       	cmp    0x2(%r15,%rbx,1),%al
   21ad7:	40 0f 94 c7          	sete   %dil
   21adb:	ff 15 a7 11 06 00    	call   *0x611a7(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21ae1:	88 44 24 48          	mov    %al,0x48(%rsp)
   21ae5:	0f b6 44 24 53       	movzbl 0x53(%rsp),%eax
   21aea:	31 ff                	xor    %edi,%edi
   21aec:	41 3a 44 1f 03       	cmp    0x3(%r15,%rbx,1),%al
   21af1:	40 0f 94 c7          	sete   %dil
   21af5:	ff 15 8d 11 06 00    	call   *0x6118d(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21afb:	88 44 24 27          	mov    %al,0x27(%rsp)
   21aff:	0f b6 44 24 54       	movzbl 0x54(%rsp),%eax
   21b04:	31 ff                	xor    %edi,%edi
   21b06:	41 3a 44 1f 04       	cmp    0x4(%r15,%rbx,1),%al
   21b0b:	40 0f 94 c7          	sete   %dil
   21b0f:	ff 15 73 11 06 00    	call   *0x61173(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21b15:	41 89 c5             	mov    %eax,%r13d
   21b18:	0f b6 44 24 55       	movzbl 0x55(%rsp),%eax
   21b1d:	31 ff                	xor    %edi,%edi
   21b1f:	41 3a 44 1f 05       	cmp    0x5(%r15,%rbx,1),%al
   21b24:	40 0f 94 c7          	sete   %dil
   21b28:	ff 15 5a 11 06 00    	call   *0x6115a(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21b2e:	88 44 24 26          	mov    %al,0x26(%rsp)
   21b32:	0f b6 44 24 56       	movzbl 0x56(%rsp),%eax
   21b37:	31 ff                	xor    %edi,%edi
   21b39:	41 3a 44 1f 06       	cmp    0x6(%r15,%rbx,1),%al
   21b3e:	40 0f 94 c7          	sete   %dil
   21b42:	ff 15 40 11 06 00    	call   *0x61140(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21b48:	88 44 24 25          	mov    %al,0x25(%rsp)
   21b4c:	0f b6 44 24 57       	movzbl 0x57(%rsp),%eax
   21b51:	31 ff                	xor    %edi,%edi
   21b53:	41 3a 44 1f 07       	cmp    0x7(%r15,%rbx,1),%al
   21b58:	40 0f 94 c7          	sete   %dil
   21b5c:	ff 15 26 11 06 00    	call   *0x61126(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21b62:	41 89 c6             	mov    %eax,%r14d
   21b65:	0f b6 44 24 58       	movzbl 0x58(%rsp),%eax
   21b6a:	31 ff                	xor    %edi,%edi
   21b6c:	41 3a 44 1f 08       	cmp    0x8(%r15,%rbx,1),%al
   21b71:	40 0f 94 c7          	sete   %dil
   21b75:	ff 15 0d 11 06 00    	call   *0x6110d(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21b7b:	88 44 24 24          	mov    %al,0x24(%rsp)
   21b7f:	0f b6 44 24 59       	movzbl 0x59(%rsp),%eax
   21b84:	31 ff                	xor    %edi,%edi
   21b86:	41 3a 44 1f 09       	cmp    0x9(%r15,%rbx,1),%al
   21b8b:	40 0f 94 c7          	sete   %dil
   21b8f:	ff 15 f3 10 06 00    	call   *0x610f3(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21b95:	88 44 24 23          	mov    %al,0x23(%rsp)
   21b99:	0f b6 44 24 5a       	movzbl 0x5a(%rsp),%eax
   21b9e:	31 ff                	xor    %edi,%edi
   21ba0:	41 3a 44 1f 0a       	cmp    0xa(%r15,%rbx,1),%al
   21ba5:	40 0f 94 c7          	sete   %dil
   21ba9:	ff 15 d9 10 06 00    	call   *0x610d9(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21baf:	88 44 24 22          	mov    %al,0x22(%rsp)
   21bb3:	0f b6 44 24 5b       	movzbl 0x5b(%rsp),%eax
   21bb8:	31 ff                	xor    %edi,%edi
   21bba:	41 3a 44 1f 0b       	cmp    0xb(%r15,%rbx,1),%al
   21bbf:	40 0f 94 c7          	sete   %dil
   21bc3:	ff 15 bf 10 06 00    	call   *0x610bf(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21bc9:	41 89 c4             	mov    %eax,%r12d
   21bcc:	0f b6 44 24 5c       	movzbl 0x5c(%rsp),%eax
   21bd1:	31 ff                	xor    %edi,%edi
   21bd3:	41 3a 44 1f 0c       	cmp    0xc(%r15,%rbx,1),%al
   21bd8:	40 0f 94 c7          	sete   %dil
   21bdc:	ff 15 a6 10 06 00    	call   *0x610a6(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21be2:	88 44 24 21          	mov    %al,0x21(%rsp)
   21be6:	0f b6 44 24 5d       	movzbl 0x5d(%rsp),%eax
   21beb:	31 ff                	xor    %edi,%edi
   21bed:	41 3a 44 1f 0d       	cmp    0xd(%r15,%rbx,1),%al
   21bf2:	40 0f 94 c7          	sete   %dil
   21bf6:	ff 15 8c 10 06 00    	call   *0x6108c(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21bfc:	88 44 24 20          	mov    %al,0x20(%rsp)
   21c00:	0f b6 44 24 5e       	movzbl 0x5e(%rsp),%eax
   21c05:	31 ff                	xor    %edi,%edi
   21c07:	41 3a 44 1f 0e       	cmp    0xe(%r15,%rbx,1),%al
   21c0c:	40 0f 94 c7          	sete   %dil
   21c10:	ff 15 72 10 06 00    	call   *0x61072(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21c16:	88 44 24 1f          	mov    %al,0x1f(%rsp)
   21c1a:	0f b6 44 24 5f       	movzbl 0x5f(%rsp),%eax
   21c1f:	31 ff                	xor    %edi,%edi
   21c21:	41 3a 44 1f 0f       	cmp    0xf(%r15,%rbx,1),%al
   21c26:	40 0f 94 c7          	sete   %dil
   21c2a:	ff 15 58 10 06 00    	call   *0x61058(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21c30:	0f b6 4c 24 40       	movzbl 0x40(%rsp),%ecx
   21c35:	22 4c 24 38          	and    0x38(%rsp),%cl
   21c39:	0f b6 54 24 48       	movzbl 0x48(%rsp),%edx
   21c3e:	22 54 24 27          	and    0x27(%rsp),%dl
   21c42:	20 ca                	and    %cl,%dl
   21c44:	44 22 6c 24 26       	and    0x26(%rsp),%r13b
   21c49:	44 22 6c 24 25       	and    0x25(%rsp),%r13b
   21c4e:	41 20 d5             	and    %dl,%r13b
   21c51:	44 22 74 24 24       	and    0x24(%rsp),%r14b
   21c56:	44 22 74 24 23       	and    0x23(%rsp),%r14b
   21c5b:	44 22 74 24 22       	and    0x22(%rsp),%r14b
   21c60:	45 20 ee             	and    %r13b,%r14b
   21c63:	44 22 64 24 21       	and    0x21(%rsp),%r12b
   21c68:	44 22 64 24 20       	and    0x20(%rsp),%r12b
   21c6d:	44 22 64 24 1f       	and    0x1f(%rsp),%r12b
   21c72:	41 20 c4             	and    %al,%r12b
   21c75:	45 20 f4             	and    %r14b,%r12b
   21c78:	41 80 e4 01          	and    $0x1,%r12b
   21c7c:	41 0f b6 fc          	movzbl %r12b,%edi
   21c80:	ff 15 02 10 06 00    	call   *0x61002(%rip)        # 82c88 <_GLOBAL_OFFSET_TABLE_+0x590>
   21c86:	84 c0                	test   %al,%al
   21c88:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   21c8f:	00 
   21c90:	74 42                	je     21cd4 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x764>
   21c92:	4c 8d b4 24 20 05 00 	lea    0x520(%rsp),%r14
   21c99:	00 
   21c9a:	4c 89 f7             	mov    %r14,%rdi
   21c9d:	4c 89 fe             	mov    %r15,%rsi
   21ca0:	48 8b 54 24 28       	mov    0x28(%rsp),%rdx
   21ca5:	e8 06 2d 00 00       	call   249b0 <<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>> as cipher::stream::StreamCipher>::apply_keystream>
   21caa:	4c 89 f7             	mov    %r14,%rdi
   21cad:	e8 fe e5 ff ff       	call   202b0 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   21cb2:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   21cb9:	00 
   21cba:	48 8b 45 18          	mov    0x18(%rbp),%rax
   21cbe:	48 8b 4c 24 28       	mov    0x28(%rsp),%rcx
   21cc3:	48 39 c1             	cmp    %rax,%rcx
   21cc6:	48 0f 43 c8          	cmovae %rax,%rcx
   21cca:	4d 89 7d 08          	mov    %r15,0x8(%r13)
   21cce:	49 89 4d 10          	mov    %rcx,0x10(%r13)
   21cd2:	eb 22                	jmp    21cf6 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x786>
   21cd4:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   21cdb:	00 
   21cdc:	e8 cf e5 ff ff       	call   202b0 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   21ce1:	4c 89 ff             	mov    %r15,%rdi
   21ce4:	ff 15 66 0a 06 00    	call   *0x60a66(%rip)        # 82750 <free@GLIBC_2.2.5>
   21cea:	41 c6 45 08 00       	movb   $0x0,0x8(%r13)
   21cef:	48 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%rax
   21cf6:	49 89 45 00          	mov    %rax,0x0(%r13)
   21cfa:	4c 89 e8             	mov    %r13,%rax
   21cfd:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   21d01:	5b                   	pop    %rbx
   21d02:	41 5c                	pop    %r12
   21d04:	41 5d                	pop    %r13
   21d06:	41 5e                	pop    %r14
   21d08:	41 5f                	pop    %r15
   21d0a:	5d                   	pop    %rbp
   21d0b:	c3                   	ret
   21d0c:	48 8d 15 2d ec 04 00 	lea    0x4ec2d(%rip),%rdx        # 70940 <core::unicode::unicode_data::cf::SHORT_OFFSET_RUNS+0x70>
   21d13:	4c 8d 0d ee ec 05 00 	lea    0x5ecee(%rip),%r9        # 80a08 <anon.d49aa5e532bb8905898215d271d902c3.14.llvm.14046659981651887190+0x70>
   21d1a:	48 8d b4 24 e0 02 00 	lea    0x2e0(%rsp),%rsi
   21d21:	00 
   21d22:	31 ff                	xor    %edi,%edi
   21d24:	31 c9                	xor    %ecx,%ecx
   21d26:	ff 15 6c 0f 06 00    	call   *0x60f6c(%rip)        # 82c98 <_GLOBAL_OFFSET_TABLE_+0x5a0>
   21d2c:	ff 15 be 0a 06 00    	call   *0x60abe(%rip)        # 827f0 <_GLOBAL_OFFSET_TABLE_+0xf8>
   21d32:	e9 00 f9 ff ff       	jmp    21637 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0xc7>
   21d37:	bf 01 00 00 00       	mov    $0x1,%edi
   21d3c:	48 89 de             	mov    %rbx,%rsi
   21d3f:	ff 15 2b 11 06 00    	call   *0x6112b(%rip)        # 82e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   21d45:	49 89 c6             	mov    %rax,%r14
   21d48:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   21d4d:	ff 15 fd 09 06 00    	call   *0x609fd(%rip)        # 82750 <free@GLIBC_2.2.5>
   21d53:	4c 89 f7             	mov    %r14,%rdi
   21d56:	e8 e5 42 fe ff       	call   6040 <_Unwind_Resume@plt>
   21d5b:	eb 06                	jmp    21d63 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   21d5d:	eb 04                	jmp    21d63 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   21d5f:	eb 02                	jmp    21d63 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   21d61:	eb 00                	jmp    21d63 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::open+0x7f3>
   21d63:	49 89 c6             	mov    %rax,%r14
   21d66:	48 8d bc 24 20 05 00 	lea    0x520(%rsp),%rdi
   21d6d:	00 
   21d6e:	e8 3d e5 ff ff       	call   202b0 <core::ptr::drop_glue::<cipher::stream_wrapper::StreamCipherCoreWrapper<chacha20::ChaChaCore<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B1>, typenum::bit::B0>>>>>
   21d73:	48 8b 7c 24 30       	mov    0x30(%rsp),%rdi
   21d78:	ff 15 d2 09 06 00    	call   *0x609d2(%rip)        # 82750 <free@GLIBC_2.2.5>
   21d7e:	4c 89 f7             	mov    %r14,%rdi
   21d81:	e8 ba 42 fe ff       	call   6040 <_Unwind_Resume@plt>
   21d86:	ff 15 3c 12 06 00    	call   *0x6123c(%rip)        # 82fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   21d8c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000021d90 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal>:
   21d90:	55                   	push   %rbp
   21d91:	48 89 e5             	mov    %rsp,%rbp
   21d94:	41 57                	push   %r15
   21d96:	41 56                	push   %r14
   21d98:	41 55                	push   %r13
   21d9a:	41 54                	push   %r12
   21d9c:	53                   	push   %rbx
   21d9d:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   21da1:	48 81 ec c0 05 00 00 	sub    $0x5c0,%rsp
   21da8:	4c 89 44 24 08       	mov    %r8,0x8(%rsp)
   21dad:	48 89 8c 24 c0 02 00 	mov    %rcx,0x2c0(%rsp)
   21db4:	00 
   21db5:	48 83 f9 0c          	cmp    $0xc,%rcx
   21db9:	0f 85 74 05 00 00    	jne    22333 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5a3>
   21dbf:	48 89 fb             	mov    %rdi,%rbx
   21dc2:	48 8b 7d 18          	mov    0x18(%rbp),%rdi
   21dc6:	48 83 c7 10          	add    $0x10,%rdi
   21dca:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
   21dcf:	0f 88 89 05 00 00    	js     2235e <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5ce>
   21dd5:	4d 89 cf             	mov    %r9,%r15
   21dd8:	49 89 d6             	mov    %rdx,%r14
   21ddb:	49 89 f4             	mov    %rsi,%r12
   21dde:	ff 15 8c 0e 06 00    	call   *0x60e8c(%rip)        # 82c70 <malloc@GLIBC_2.2.5>
   21de4:	48 85 c0             	test   %rax,%rax
   21de7:	0f 84 7e 05 00 00    	je     2236b <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5db>
   21ded:	49 89 c5             	mov    %rax,%r13
   21df0:	48 89 5c 24 28       	mov    %rbx,0x28(%rsp)
   21df5:	48 8b 5d 18          	mov    0x18(%rbp),%rbx
   21df9:	48 85 db             	test   %rbx,%rbx
   21dfc:	74 10                	je     21e0e <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x7e>
   21dfe:	48 8b 75 10          	mov    0x10(%rbp),%rsi
   21e02:	4c 89 ef             	mov    %r13,%rdi
   21e05:	48 89 da             	mov    %rbx,%rdx
   21e08:	ff 15 8a 0d 06 00    	call   *0x60d8a(%rip)        # 82b98 <memcpy@GLIBC_2.14>
   21e0e:	0f 10 05 6b bd 04 00 	movups 0x4bd6b(%rip),%xmm0        # 6db80 <anon.10746075dd51319079efedc96a688e81.1.llvm.4247047481296140766+0xad>
   21e15:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   21e1a:	41 0f 10 04 24       	movups (%r12),%xmm0
   21e1f:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   21e25:	0f 29 44 24 50       	movaps %xmm0,0x50(%rsp)
   21e2a:	0f 29 4c 24 60       	movaps %xmm1,0x60(%rsp)
   21e2f:	49 8b 06             	mov    (%r14),%rax
   21e32:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
   21e37:	41 8b 46 08          	mov    0x8(%r14),%eax
   21e3b:	89 44 24 38          	mov    %eax,0x38(%rsp)
   21e3f:	48 8d 05 ca 11 06 00 	lea    0x611ca(%rip),%rax        # 83010 <chacha20::avx2_cpuid::STORAGE>
   21e46:	0f b6 00             	movzbl (%rax),%eax
   21e49:	3c ff                	cmp    $0xff,%al
   21e4b:	4c 89 2c 24          	mov    %r13,(%rsp)
   21e4f:	49 89 de             	mov    %rbx,%r14
   21e52:	0f 84 fb 04 00 00    	je     22353 <<hpke_ng::aead::ChaCha20Poly1305 as hpke_ng::aead::SealingAead>::seal+0x5c3>
   21e58:	0f 57 c0             	xorps  %xmm0,%xmm0
   21e5b:	0f 29 84 24 f0 00 00 	movaps %xmm0,0xf0(%rsp)
   21e62:	00 
   21e63:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
   21e6a:	00 
   21e6b:	0f 29 84 24 d0 00 00 	movaps %xmm0,0xd0(%rsp)
   21e72:	00 
   21e73:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   21e7a:	00 
   21e7b:	0f 28 44 24 40       	movaps 0x40(%rsp),%xmm0
   21e80:	0f 28 4c 24 50       	movaps 0x50(%rsp),%xmm1
   21e85:	0f 28 54 24 60       	movaps 0x60(%rsp),%xmm2
   21e8a:	0f                   	.byte 0xf
   21e8b:	29                   	.byte 0x29
   21e8c:	84 24 80             	test   %ah,(%rax,%rax,4)
	...

=== A=0x403D9AB F=0x3d9ab ===

./target/release/analysis:     file format elf64-x86-64


Disassembly of section .text:

000000000003d7ab <<star_core::secure_channel::server::SecureChannelServer>::new+0x2eb>:
   3d7ab:	85 00                	test   %eax,(%rax)
   3d7ad:	ff                   	(bad)
   3d7ae:	ff                   	(bad)
   3d7af:	ff 0f                	decl   (%rdi)
   3d7b1:	11 43 40             	adc    %eax,0x40(%rbx)
   3d7b4:	48 8b 85 f0 fe ff ff 	mov    -0x110(%rbp),%rax
   3d7bb:	48 89 43 30          	mov    %rax,0x30(%rbx)
   3d7bf:	48 8b 85 f8 fe ff ff 	mov    -0x108(%rbp),%rax
   3d7c6:	48 89 43 38          	mov    %rax,0x38(%rbx)
   3d7ca:	48 8b 85 10 ff ff ff 	mov    -0xf0(%rbp),%rax
   3d7d1:	48 89 43 50          	mov    %rax,0x50(%rbx)
   3d7d5:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3d7d9:	ff 15 49 56 04 00    	call   *0x45649(%rip)        # 82e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3d7df:	48 89 d8             	mov    %rbx,%rax
   3d7e2:	48 81 c4 28 01 00 00 	add    $0x128,%rsp
   3d7e9:	5b                   	pop    %rbx
   3d7ea:	41 5e                	pop    %r14
   3d7ec:	41 5f                	pop    %r15
   3d7ee:	5d                   	pop    %rbp
   3d7ef:	c3                   	ret
   3d7f0:	bf 01 00 00 00       	mov    $0x1,%edi
   3d7f5:	be 20 00 00 00       	mov    $0x20,%esi
   3d7fa:	ff 15 70 56 04 00    	call   *0x45670(%rip)        # 82e70 <_GLOBAL_OFFSET_TABLE_+0x778>
   3d800:	0f 0b                	ud2
   3d802:	48 89 c3             	mov    %rax,%rbx
   3d805:	eb 0f                	jmp    3d816 <<star_core::secure_channel::server::SecureChannelServer>::new+0x356>
   3d807:	eb 21                	jmp    3d82a <<star_core::secure_channel::server::SecureChannelServer>::new+0x36a>
   3d809:	48 89 c3             	mov    %rax,%rbx
   3d80c:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3d810:	ff 15 12 56 04 00    	call   *0x45612(%rip)        # 82e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3d816:	48 8d bd 60 ff ff ff 	lea    -0xa0(%rbp),%rdi
   3d81d:	e8 1e d1 ff ff       	call   3a940 <core::ptr::drop_glue::<hpke_ng::kem::dh::DhPrivateKey<hpke_ng::kem::dh::X25519>>>
   3d822:	eb 13                	jmp    3d837 <<star_core::secure_channel::server::SecureChannelServer>::new+0x377>
   3d824:	ff 15 9e 57 04 00    	call   *0x4579e(%rip)        # 82fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3d82a:	48 89 c3             	mov    %rax,%rbx
   3d82d:	48 8d 7d c8          	lea    -0x38(%rbp),%rdi
   3d831:	ff 15 f1 55 04 00    	call   *0x455f1(%rip)        # 82e28 <_GLOBAL_OFFSET_TABLE_+0x730>
   3d837:	48 89 df             	mov    %rbx,%rdi
   3d83a:	e8 01 88 fc ff       	call   6040 <_Unwind_Resume@plt>
   3d83f:	ff 15 83 57 04 00    	call   *0x45783(%rip)        # 82fc8 <_GLOBAL_OFFSET_TABLE_+0x8d0>
   3d845:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   3d84c:	00 00 00 
   3d84f:	90                   	nop

000000000003d850 <<star_core::serv::enclave::Enclave>::issue_token>:
   3d850:	55                   	push   %rbp
   3d851:	48 89 e5             	mov    %rsp,%rbp
   3d854:	41 57                	push   %r15
   3d856:	41 56                	push   %r14
   3d858:	41 55                	push   %r13
   3d85a:	41 54                	push   %r12
   3d85c:	53                   	push   %rbx
   3d85d:	48 83 ec 18          	sub    $0x18,%rsp
   3d861:	48 89 fb             	mov    %rdi,%rbx
   3d864:	48 83 f9 60          	cmp    $0x60,%rcx
   3d868:	0f 82 84 00 00 00    	jb     3d8f2 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   3d86e:	4d 89 c6             	mov    %r8,%r14
   3d871:	4c 8d 42 20          	lea    0x20(%rdx),%r8
   3d875:	48 8d 42 40          	lea    0x40(%rdx),%rax
   3d879:	4c 8d 52 60          	lea    0x60(%rdx),%r10
   3d87d:	48 83 c1 a0          	add    $0xffffffffffffffa0,%rcx
   3d881:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   3d885:	49 89 cb             	mov    %rcx,%r11
   3d888:	b9 20 00 00 00       	mov    $0x20,%ecx
   3d88d:	41 b9 20 00 00 00    	mov    $0x20,%r9d
   3d893:	49 89 f4             	mov    %rsi,%r12
   3d896:	49 89 d7             	mov    %rdx,%r15
   3d899:	48 89 c2             	mov    %rax,%rdx
   3d89c:	41 53                	push   %r11
   3d89e:	41 52                	push   %r10
   3d8a0:	6a 20                	push   $0x20
   3d8a2:	41 57                	push   %r15
   3d8a4:	ff 15 0e 51 04 00    	call   *0x4510e(%rip)        # 829b8 <_GLOBAL_OFFSET_TABLE_+0x2c0>
   3d8aa:	48 83 c4 20          	add    $0x20,%rsp
   3d8ae:	4c 8b 6d c0          	mov    -0x40(%rbp),%r13
   3d8b2:	49 83 fd ff          	cmp    $0xffffffffffffffff,%r13
   3d8b6:	74 3a                	je     3d8f2 <<star_core::serv::enclave::Enclave>::issue_token+0xa2>
   3d8b8:	4c 8b 7d c8          	mov    -0x38(%rbp),%r15
   3d8bc:	48 8b 4d d0          	mov    -0x30(%rbp),%rcx
   3d8c0:	48 8d 7d c0          	lea    -0x40(%rbp),%rdi
   3d8c4:	4c 89 e6             	mov    %r12,%rsi
   3d8c7:	4c 89 fa             	mov    %r15,%rdx
   3d8ca:	4d 89 f0             	mov    %r14,%r8
   3d8cd:	ff 15 a5 52 04 00    	call   *0x452a5(%rip)        # 82b78 <_GLOBAL_OFFSET_TABLE_+0x480>
   3d8d3:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   3d8d7:	48 89 43 10          	mov    %rax,0x10(%rbx)
   3d8db:	0f 10 45 c0          	movups -0x40(%rbp),%xmm0
   3d8df:	0f 11 03             	movups %xmm0,(%rbx)
   3d8e2:	4d 85 ed             	test   %r13,%r13
   3d8e5:	74 16                	je     3d8fd <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   3d8e7:	4c 89 ff             	mov    %r15,%rdi
   3d8ea:	ff 15 60 4e 04 00    	call   *0x44e60(%rip)        # 82750 <free@GLIBC_2.2.5>
   3d8f0:	eb 0b                	jmp    3d8fd <<star_core::serv::enclave::Enclave>::issue_token+0xad>
   3d8f2:	c6 43 08 01          	movb   $0x1,0x8(%rbx)
   3d8f6:	48 c7 03 ff ff ff ff 	movq   $0xffffffffffffffff,(%rbx)
   3d8fd:	48 89 d8             	mov    %rbx,%rax
   3d900:	48 83 c4 18          	add    $0x18,%rsp
   3d904:	5b                   	pop    %rbx
   3d905:	41 5c                	pop    %r12
   3d907:	41 5d                	pop    %r13
   3d909:	41 5e                	pop    %r14
   3d90b:	41 5f                	pop    %r15
   3d90d:	5d                   	pop    %rbp
   3d90e:	c3                   	ret
   3d90f:	48 89 c3             	mov    %rax,%rbx
   3d912:	4d 85 ed             	test   %r13,%r13
   3d915:	74 09                	je     3d920 <<star_core::serv::enclave::Enclave>::issue_token+0xd0>
   3d917:	4c 89 ff             	mov    %r15,%rdi
   3d91a:	ff 15 30 4e 04 00    	call   *0x44e30(%rip)        # 82750 <free@GLIBC_2.2.5>
   3d920:	48 89 df             	mov    %rbx,%rdi
   3d923:	e8 18 87 fc ff       	call   6040 <_Unwind_Resume@plt>
   3d928:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   3d92f:	00 

000000000003d930 <<star_core::serv::enclave::Enclave>::issue_token_plain>:
   3d930:	55                   	push   %rbp
   3d931:	48 89 e5             	mov    %rsp,%rbp
   3d934:	41 57                	push   %r15
   3d936:	41 56                	push   %r14
   3d938:	41 54                	push   %r12
   3d93a:	53                   	push   %rbx
   3d93b:	48 81 ec b0 00 00 00 	sub    $0xb0,%rsp
   3d942:	48 89 fb             	mov    %rdi,%rbx
   3d945:	48 83 f9 50          	cmp    $0x50,%rcx
   3d949:	72 62                	jb     3d9ad <<star_core::serv::enclave::Enclave>::issue_token_plain+0x7d>
   3d94b:	49 89 f1             	mov    %rsi,%r9
   3d94e:	0f 10 42 20          	movups 0x20(%rdx),%xmm0
   3d952:	0f 10 4a 30          	movups 0x30(%rdx),%xmm1
   3d956:	0f 29 8d 60 ff ff ff 	movaps %xmm1,-0xa0(%rbp)
   3d95d:	0f 29 85 50 ff ff ff 	movaps %xmm0,-0xb0(%rbp)
   3d964:	f3 0f 6f 02          	movdqu (%rdx),%xmm0
   3d968:	f3 0f 6f 4a 10       	movdqu 0x10(%rdx),%xmm1
   3d96d:	66 0f 7f 85 30 ff ff 	movdqa %xmm0,-0xd0(%rbp)
   3d974:	ff 
   3d975:	66 0f 7f 8d 40 ff ff 	movdqa %xmm1,-0xc0(%rbp)
   3d97c:	ff 
   3d97d:	4c 8b 72 40          	mov    0x40(%rdx),%r14
   3d981:	49 89 d4             	mov    %rdx,%r12
   3d984:	4c 8b 7a 48          	mov    0x48(%rdx),%r15
   3d988:	48 8b 46 58          	mov    0x58(%rsi),%rax
   3d98c:	49 83 c1 60          	add    $0x60,%r9
   3d990:	48 8d bd 30 ff ff ff 	lea    -0xd0(%rbp),%rdi
   3d997:	4c 89 fe             	mov    %r15,%rsi
   3d99a:	4c 89 f2             	mov    %r14,%rdx
   3d99d:	4c 89 c1             	mov    %r8,%rcx
   3d9a0:	49 89 c0             	mov    %rax,%r8
   3d9a3:	ff 15 67 52 04 00    	call   *0x45267(%rip)        # 82c10 <_GLOBAL_OFFSET_TABLE_+0x518>
   3d9a9:	84 c0                	test   %al,%al
   3d9ab:	75 3d                	jne    3d9ea <<star_core::serv::enclave::Enclave>::issue_token_plain+0xba>
   3d9ad:	bf 01 00 00 00       	mov    $0x1,%edi
   3d9b2:	ff 15 b8 52 04 00    	call   *0x452b8(%rip)        # 82c70 <malloc@GLIBC_2.2.5>
   3d9b8:	48 85 c0             	test   %rax,%rax
   3d9bb:	0f 84 1c 01 00 00    	je     3dadd <<star_core::serv::enclave::Enclave>::issue_token_plain+0x1ad>
   3d9c1:	c6 00 00             	movb   $0x0,(%rax)
   3d9c4:	48 c7 03 01 00 00 00 	movq   $0x1,(%rbx)
   3d9cb:	48 89 43 08          	mov    %rax,0x8(%rbx)
   3d9cf:	48 c7 43 10 01 00 00 	movq   $0x1,0x10(%rbx)
   3d9d6:	00 
   3d9d7:	48 89 d8             	mov    %rbx,%rax
   3d9da:	48 81 c4 b0 00 00 00 	add    $0xb0,%rsp
   3d9e1:	5b                   	pop    %rbx
   3d9e2:	41 5c                	pop    %r12
   3d9e4:	41 5e                	pop    %r14
   3d9e6:	41 5f                	pop    %r15
   3d9e8:	5d                   	pop    %rbp
   3d9e9:	c3                   	ret
   3d9ea:	41 0f 10 04 24       	movups (%r12),%xmm0
   3d9ef:	41 0f 10 4c 24 10    	movups 0x10(%r12),%xmm1
   3d9f5:	0f 11 4d b8          	movups %xmm1,-0x48(%rbp)
   3d9f9:	0f 11 45 a8          	movups %xmm0,-0x58(%rbp)
   3d9fd:	0f 10 05 9c 06 03 00 	movups 0x3069c(%rip),%xmm0        # 6e0a0 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x130>
   3da04:	0f 29 85 70 ff ff ff 	movaps %xmm0,-0x90(%rbp)
   3da0b:	f3 0f 6f 05 9d 06 03 	movdqu 0x3069d(%rip),%xmm0        # 6e0b0 <anon.dc58466c5fe28f75dac3d707d3f985ee.27.llvm.15461068194027807234+0x140>
   3da12:	00 
   3da13:	66 0f 7f 45 80       	movdqa %xmm0,-0x80(%rbp)
   3da18:	48 c7 45 90 00 00 00 	movq   $0x0,-0x70(%rbp)
   3da1f:	00 
   3da20:	48 8d 75 98          	lea    -0x68(%rbp),%rsi
   3da24:	4c 89 7d 98          	mov    %r15,-0x68(%rbp)
   3da28:	4c 89 75 a0          	mov    %r14,-0x60(%rbp)
   3da2c:	c6 45 d8 30          	movb   $0x30,-0x28(%rbp)
   3da30:	c6 45 c8 80          	movb   $0x80,-0x38(%rbp)
   3da34:	c7 45 c9 00 00 00 00 	movl   $0x0,-0x37(%rbp)
   3da3b:	c7 45 cc 00 00 00 00 	movl   $0x0,-0x34(%rbp)
   3da42:	48 b8 00 00 00 00 00 	movabs $0x8001000000000000,%rax
   3da49:	00 01 80 
   3da4c:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   3da50:	48 8d bd 70 ff ff ff 	lea    -0x90(%rbp),%rdi
   3da57:	ba 01 00 00 00       	mov    $0x1,%edx
   3da5c:	ff 15 76 52 04 00    	call   *0x45276(%rip)        # 82cd8 <_GLOBAL_OFFSET_TABLE_+0x5e0>
   3da62:	66 0f ef c0          	pxor   %xmm0,%xmm0
   3da66:	66 0f 6f 8d 70 ff ff 	movdqa -0x90(%rbp),%xmm1
   3da6d:	ff 
   3da6e:	66 0f 6f 55 80       	movdqa -0x80(%rbp),%xmm2
   3da73:	66 0f 6f d9          	movdqa %xmm1,%xmm3
   3da77:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   3da7b:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   3da80:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   3da85:	66 0f 60 c8          	punpcklbw %xmm0,%xmm1
   3da89:	f2 0f 70 c9 1b       	pshuflw $0x1b,%xmm1,%xmm1
   3da8e:	f3 0f 70 c9 1b       	pshufhw $0x1b,%xmm1,%xmm1
   3da93:	66 0f 67 cb          	packuswb %xmm3,%xmm1
   3da97:	66 0f 6f da          	movdqa %xmm2,%xmm3
   3da9b:	66 0f 68 d8          	punpckhbw %xmm0,%xmm3
   3da9f:	f2 0f 70 db 1b       	pshuflw $0x1b,%xmm3,%xmm3
   3daa4:	f3 0f 70 db 1b       	pshufhw $0x1b,%xmm3,%xmm3
   3daa9:	66 0f 60 d0          	punpcklbw %xmm0,%xmm2
   3daad:	f2 0f 70 c2 1b       	pshuflw $0x1b,%xmm2,%xmm0
   3dab2:	f3 0f 70 c0 1b       	pshufhw $0x1b,%xmm0,%xmm0
   3dab7:	66 0f 67 c3          	packuswb %xmm3,%xmm0
   3dabb:	66 0f 7f 8d 70 ff ff 	movdqa %xmm1,-0x90(%rbp)
   3dac2:	ff 
   3dac3:	66 0f 7f 45 80       	movdqa %xmm0,-0x80(%rbp)
   3dac8:	48 8d b5 70 ff ff ff 	lea    -0x90(%rbp),%rsi
   3dacf:	48 89 df             	mov    %rbx,%rdi
   3dad2:	ff 15 b8 53 04 00    	call   *0x453b8(%rip)        # 82e90 <_GLOBAL_OFFSET_TABLE_+0x798>
   3dad8:	e9 fa fe ff ff       	jmp    3d9d7 <<star_core::serv::enclave::Enclave>::issue_token_plain+0xa7>
   3dadd:	bf 01 00 00 00       	mov    $0x1,%edi
   3dae2:	be 01 00 00 00       	mov    $0x1,%esi
   3dae7:	ff 15 d3 52 04 00    	call   *0x452d3(%rip)        # 82dc0 <_GLOBAL_OFFSET_TABLE_+0x6c8>
   3daed:	0f 1f 00             	nopl   (%rax)

000000000003daf0 <<star_core::serv::core::cred::Credential>::is_valid>:
   3daf0:	55                   	push   %rbp
   3daf1:	48 89 e5             	mov    %rsp,%rbp
   3daf4:	41 57                	push   %r15
   3daf6:	41 56                	push   %r14
   3daf8:	41 55                	push   %r13
   3dafa:	41 54                	push   %r12
   3dafc:	53                   	push   %rbx
   3dafd:	48 81 ec e8 02 00 00 	sub    $0x2e8,%rsp
   3db04:	4c 89 4d c8          	mov    %r9,-0x38(%rbp)
   3db08:	4d 89 c5             	mov    %r8,%r13
   3db0b:	49 89 d7             	mov    %rdx,%r15
   3db0e:	48 89 7d c0          	mov    %rdi,-0x40(%rbp)
   3db12:	48 89 d0             	mov    %rdx,%rax
   3db15:	48 c1 e8 08          	shr    $0x8,%rax
   3db19:	48 89 85 68 ff ff ff 	mov    %rax,-0x98(%rbp)
   3db20:	48 89 d0             	mov    %rdx,%rax
   3db23:	48 c1 e8 10          	shr    $0x10,%rax
   3db27:	48 89 85 70 ff ff ff 	mov    %rax,-0x90(%rbp)
   3db2e:	48 89 d0             	mov    %rdx,%rax
   3db31:	48 c1 e8 18          	shr    $0x18,%rax
   3db35:	48 89 85 78 ff ff ff 	mov    %rax,-0x88(%rbp)
   3db3c:	48 89 d0             	mov    %rdx,%rax
   3db3f:	48 c1 e8 20          	shr    $0x20,%rax
   3db43:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
   3db47:	48 89 d0             	mov    %rdx,%rax
   3db4a:	48 c1 e8 28          	shr    $0x28,%rax
   3db4e:	48 89 45 88          	mov    %rax,-0x78(%rbp)
   3db52:	48 89 d0             	mov    %rdx,%rax
   3db55:	48 c1 e8 30          	shr    $0x30,%rax
   3db59:	48 89 45 90          	mov    %rax,-0x70(%rbp)
   3db5d:	48 89 d0             	mov    %rdx,%rax
   3db60:	48 c1 e8 38          	shr    $0x38,%rax
   3db64:	48 89 45 a0          	mov    %rax,-0x60(%rbp)
   3db68:	4c 89 c3             	mov    %r8,%rbx
   3db6b:	48 c1 eb 08          	shr    $0x8,%rbx
   3db6f:	48 89 9d b8 fe ff ff 	mov    %rbx,-0x148(%rbp)
   3db76:	4d 89 c4             	mov    %r8,%r12
   3db79:	49 c1 ec 10          	shr    $0x10,%r12
   3db7d:	4d 89 c6             	mov    %r8,%r14
   3db80:	49 c1 ee 18          	shr    $0x18,%r14
   3db84:	4c 89 c0             	mov    %r8,%rax
   3db87:	48 c1 e8 20          	shr    $0x20,%rax
   3db8b:	48 89 45 80          	mov    %rax,-0x80(%rbp)
   3db8f:	4c 89 c0             	mov    %r8,%rax
   3db92:	48 c1 e8 28          	shr    $0x28,%rax
   3db96:	48 89 45 98          	mov    %rax,-0x68(%rbp)
   3db9a:	4c 89 c0             	mov    %r8,%rax
   3db9d:	48 c1 e8 30          	shr    $0x30,%rax
   3dba1:	48 89 45 a8          	mov    %rax,-0x58(%rbp)
   3dba5:	4c 89 c0             	mov    %r8,%rax
   3dba8:	48                   	rex.W
   3dba9:	c1                   	.byte 0xc1
   3dbaa:	e8                   	.byte 0xe8

=== A=0x48CC7D0 F=0x8cc7d0 ===

./target/release/analysis:     file format elf64-x86-64


