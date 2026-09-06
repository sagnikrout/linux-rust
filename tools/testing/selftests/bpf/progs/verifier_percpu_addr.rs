//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_percpu_addr.c
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

    int percpu_data SEC(".percpu");
//
// An ld_imm64 of a per-CPU map value is followed by a mov_percpu_addr that
// reuses the same register, so check that the add resolves into the register
// the address was loaded into, for every register.
//
    SEC("raw_tp")
    __description("per-CPU address resolution")
    __success
    __arch_x86_64
    __jited("	movabsq	$0x{{.*}}, %rax")
    __jited("	addq	%gs:{{.*}}, %rax")
    __jited("	movabsq	$0x{{.*}}, %rdi")
    __jited("	addq	%gs:{{.*}}, %rdi")
    __jited("	movabsq	$0x{{.*}}, %rsi")
    __jited("	addq	%gs:{{.*}}, %rsi")
    __jited("	movabsq	$0x{{.*}}, %rdx")
    __jited("	addq	%gs:{{.*}}, %rdx")
    __jited("	movabsq	$0x{{.*}}, %rcx")
    __jited("	addq	%gs:{{.*}}, %rcx")
    __jited("	movabsq	$0x{{.*}}, %r8")
    __jited("	addq	%gs:{{.*}}, %r8")
    __jited("	movabsq	$0x{{.*}}, %rbx")
    __jited("	addq	%gs:{{.*}}, %rbx")
    __jited("	movabsq	$0x{{.*}}, %r13")
    __jited("	addq	%gs:{{.*}}, %r13")
    __jited("	movabsq	$0x{{.*}}, %r14")
    __jited("	addq	%gs:{{.*}}, %r14")
    __jited("	movabsq	$0x{{.*}}, %r15")
    __jited("	addq	%gs:{{.*}}, %r15")
#[no_mangle]
pub unsafe extern "C" fn percpu_addr() -> __naked void {
    __naked void percpu_addr(void)
    {
    asm volatile ("					\
    r0 = %[percpu_data] ll;				\
    r1 = %[percpu_data] ll;				\
    r2 = %[percpu_data] ll;				\
    r3 = %[percpu_data] ll;				\
    r4 = %[percpu_data] ll;				\
    r5 = %[percpu_data] ll;				\
    r6 = %[percpu_data] ll;				\
    r7 = %[percpu_data] ll;				\
    r8 = %[percpu_data] ll;				\
    r9 = %[percpu_data] ll;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_addr(percpu_data)
    : __clobber_all);
    }

    SEC("raw_tp")
    __description("percpu addr dummy")
    __success
#[no_mangle]
pub unsafe extern "C" fn dummy_test() -> c_int {
    int dummy_test(void)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
