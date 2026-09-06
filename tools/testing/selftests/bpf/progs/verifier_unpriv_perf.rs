//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_unpriv_perf.c
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
// Converted from tools/testing/selftests/bpf/verifier/unpriv.c

    SEC("perf_event")
    __description("unpriv: spill/fill of different pointers ldx")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointers": "same insn cannot be used with different) -> __failure {
    __failure __msg("same insn cannot be used with different pointers")
#[no_mangle]
pub unsafe extern "C" fn fill_of_different_pointers_ldx() -> __naked void {
    __naked void fill_of_different_pointers_ldx(void)
    {
    asm volatile ("					\
    r6 = r10;					\
    r6 += -8;					\
    if r1 == 0 goto l0_%=;				\
    r2 = r10;					\
    r2 += %[__imm_0];				\
// (u64*)(r6 + 0) = r2;				\
    l0_%=:	if r1 != 0 goto l1_%=;				\
// (u64*)(r6 + 0) = r1;				\
    l1_%=:	r1 = *(u64*)(r6 + 0);				\
    r1 = *(u64*)(r1 + %[sample_period]);		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__imm_0,
    -(__s32) offsetof(struct bpf_perf_event_data, sample_period) - 8),
    __imm_const(sample_period,
    offsetof(struct bpf_perf_event_data, sample_period))
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
