//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_tailcall_jit.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

    int main(void);
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __array(values, void (void));
    } jmp_table SEC(".maps") = {
    .values = {
    [0] = (void *) &main,
    },
    };
    __noinline __auxiliary
#[no_mangle]
unsafe extern "C" fn sub() -> __naked int {
    static __naked int sub(void)
    {
    asm volatile (
    "r2 = %[jmp_table] ll;"
    "r3 = 0;"
    "call 12;"
    "exit;"
    :
    : __imm_addr(jmp_table)
    : __clobber_all);
    }
    __success
    __arch_x86_64
// program entry for main(), regular function prologue
    __jited("	endbr64")
    __jited("	nopl	(%rax,%rax)")
    __jited("	xorq	%rax, %rax")
    __jited("	pushq	%rbp")
    __jited("	movq	%rsp, %rbp")
// tail call prologue for program:
// - establish memory location for tail call counter at &rbp[-8];
// - spill tail_call_cnt_ptr at &rbp[-16];
// - expect tail call counter to be passed in rax;
// - for entry program rax is a raw counter, value < 33;
// - for tail called program rax is tail_call_cnt_ptr (value > 33).
//
    __jited("	endbr64")
    __jited("	cmpq	$0x21, %rax")
    __jited("	ja	L0")
    __jited("	pushq	%rax")
    __jited("	movq	%rsp, %rax")
    __jited("	jmp	L1")
    __jited("L0:	pushq	%rax")			/* rbp[-8]  = rax         */
    __jited("L1:	pushq	%rax")			/* rbp[-16] = rax         */
// on subprogram call restore rax to be tail_call_cnt_ptr from rbp[-16]
// (cause original rax might be clobbered by this point)
//
    __jited("	movq	-0x10(%rbp), %rax")
    __jited("...")
    __jited("	callq	0x{{.*}}")		/* call to sub()          */
    __jited("	xorl	%eax, %eax")
    __jited("	leave")
    __jited("	{{(retq|jmp	0x)}}")		/* return or jump to rethunk */
    __jited("...")
// subprogram entry for sub(), regular function prologue
    __jited("	endbr64")
    __jited("	nopl	(%rax,%rax)")
    __jited("	nopl	(%rax)")
    __jited("	pushq	%rbp")
    __jited("	movq	%rsp, %rbp")
// tail call prologue for subprogram address of tail call counter
// stored at rbp[-16].
//
    __jited("	endbr64")
    __jited("	pushq	%rax")			/* rbp[-8]  = rax          */
    __jited("	pushq	%rax")			/* rbp[-16] = rax          */
    __jited("	movabsq	${{.*}}, %rsi")		/* r2 = &jmp_table         */
    __jited("	xorl	%edx, %edx")		/* r3 = 0                  */
// bpf_tail_call implementation:
// - load tail_call_cnt_ptr from rbp[-16];
// - if *tail_call_cnt_ptr < 33, increment it and jump to target;
// - otherwise do nothing.
//
    __jited("	movq	-0x10(%rbp), %rax")
    __jited("	cmpq	$0x21, (%rax)")
    __jited("	jae	L0")
    __jited("	nopl	(%rax,%rax)")
    __jited("	addq	$0x1, (%rax)")		/* *tail_call_cnt_ptr += 1 */
    __jited("	popq	%rax")
    __jited("	popq	%rax")
    __jited("	jmp	{{.*}}")		/* jump to tail call tgt   */
    __jited("L0:	leave")
    __jited("	{{(retq|jmp	0x)}}")		/* return or jump to rethunk */
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn main() -> __naked int {
    __naked int main(void)
    {
    asm volatile (
    "call %[sub];"
    "r0 = 0;"
    "exit;"
    :
    : __imm(sub)
    : __clobber_all);
    }
    char __license[] SEC("license") = "GPL";
