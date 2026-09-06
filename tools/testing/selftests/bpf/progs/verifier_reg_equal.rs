//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_reg_equal.c
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
    __description("check w reg equal if r reg upper32 bits 0")
    __success
#[no_mangle]
pub unsafe extern "C" fn subreg_equality_1() -> __naked void {
    __naked void subreg_equality_1(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
// (u64 *)(r10 - 8) = r0;				\
    r2 = *(u32 *)(r10 - 8);				\
// At this point upper 4-bytes of r2 are 0,	\
// thus insn w3 = w2 should propagate reg id,	\
// and w2 < 9 comparison would also propagate	\
// the range for r3.				\
// \
    w3 = w2;					\
    if w2 < 9 goto l0_%=;				\
    exit;						\
    l0_%=:	if r3 < 9 goto l1_%=;				\
// r1 read is illegal at this point */		\
    r0 -= r1;					\
    l1_%=:	exit;						\
    "	:
    : __imm(bpf_ktime_get_ns)
    : __clobber_all);
    }
    SEC("socket")
    __description("check w reg not equal if r reg upper32 bits not 0")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R1) -> __failure {
    __failure __msg("R1 !read_ok")
#[no_mangle]
pub unsafe extern "C" fn subreg_equality_2() -> __naked void {
    __naked void subreg_equality_2(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
    r2 = r0;					\
// Upper 4-bytes of r2 may not be 0, thus insn	\
// w3 = w2 should not propagate reg id,	and	\
// w2 < 9 comparison should not propagate	\
// the range for r3 either.			\
// \
    w3 = w2;					\
    if w2 < 9 goto l0_%=;				\
    exit;						\
    l0_%=:	if r3 < 9 goto l1_%=;				\
// r1 read is illegal at this point */		\
    r0 -= r1;					\
    l1_%=:	exit;						\
    "	:
    : __imm(bpf_ktime_get_ns)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
