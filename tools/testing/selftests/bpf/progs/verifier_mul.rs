//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_mul.c
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
// Copyright (c) 2025 Nandakumar Edamana

// Intended to test the abstract multiplication technique(s) used by
// the verifier. Using assembly to avoid compiler optimizations.
//
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: mul_precise, x: c_int) {
    void BPF_PROG(mul_precise, int x)
    {
// First, force the verifier to be uncertain about the value:
// unsigned int a = (bpf_get_prandom_u32() & 0x2) | 0x1;
//
// Assuming the verifier is using tnum, a must be tnum{.v=0x1, .m=0x2}.
// Then a * 0x3 would be m0m1 (m for uncertain). Added imprecision
// would cause the following to fail, because the required return value
// is 0:
// return (a * 0x3) & 0x4);
//
    asm volatile ("\
    call %[bpf_get_prandom_u32];\
    r0 &= 0x2;\
    r0 |= 0x1;\
    r0 *= 0x3;\
    r0 &= 0x4;\
    if r0 != 0 goto l0_%=;\
    r0 = 0;\
    goto l1_%=;\
    l0_%=:\
    r0 = 1;\
    l1_%=:\
    "	:
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
