//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_gotol.c
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
    __description("gotol, small_imm")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn gotol_small_imm() -> __naked void {
    __naked void gotol_small_imm(void)
    {
    asm volatile ("					\
    call %[bpf_ktime_get_ns];			\
    if r0 == 0 goto l0_%=;				\
    gotol l1_%=;					\
    l2_%=:							\
    gotol l3_%=;					\
    l1_%=:							\
    r0 = 1;						\
    gotol l2_%=;					\
    l0_%=:							\
    r0 = 2;						\
    l3_%=:							\
    exit;						\
    "	:
    : __imm(bpf_ktime_get_ns)
    : __clobber_all);
    }
    SEC("socket")
    __description("gotol, large_imm")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 40000) -> __success __failure_unpriv {
    __success __failure_unpriv __retval(40000)
#[no_mangle]
pub unsafe extern "C" fn gotol_large_imm() -> __naked void {
    __naked void gotol_large_imm(void)
    {
    asm volatile ("					\
    gotol 1f;					\
    0:							\
    r0 = 0;						\
    .rept 40000;					\
    r0 += 1;					\
    .endr;						\
    exit;						\
    1:	gotol 0b;					\
    "	:
    :
    : __clobber_all);
    }

    SEC("socket")
    __description("cpuv4 is not supported by compiler or jit, use a dummy test")
    __success
#[no_mangle]
pub unsafe extern "C" fn dummy_test() -> c_int {
    int dummy_test(void)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
