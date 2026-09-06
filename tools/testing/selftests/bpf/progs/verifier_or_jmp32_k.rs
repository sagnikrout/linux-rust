//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_or_jmp32_k.c
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

    SEC("socket")
    __description("or_jmp32_k: bit ops + branch on unknown value")
    __failure
    __msg("R0 invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn or_jmp32_k() -> __naked void {
    __naked void or_jmp32_k(void)
    {
    asm volatile ("					\
    r0 = 0xffffffff;				\
    r0 /= 1;					\
    r1 = 0;						\
    w1 = -1;					\
    w1 >>= 1;					\
    w0 &= w1;					\
    w0 |= 2;					\
    if w0 != 0x7ffffffd goto l1;			\
    r0 = 1;						\
    exit;						\
    l3:							\
    r0 = 5;						\
// (u64*)(r0 - 8) = r0;				\
    exit;						\
    l2:							\
    w0 -= 0xe;					\
    if w0 == 1 goto l3;				\
    r0 = 4;						\
    exit;						\
    l1:							\
    w0 -= 0x7ffffff0;				\
    if w0 s>= 0xe goto l2;				\
    r0 = 3;						\
    exit;						\
    "	::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
