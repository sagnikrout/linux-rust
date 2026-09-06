//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_direct_stack_access_wraparound.c
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
// Converted from tools/testing/selftests/bpf/verifier/direct_stack_access_wraparound.c

    SEC("socket")
    __description("direct stack access with 32-bit wraparound. test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(2147483647": "fp pointer and) -> __failure {
    __failure __msg("fp pointer and 2147483647")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn with_32_bit_wraparound_test1() -> __naked void {
    __naked void with_32_bit_wraparound_test1(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += 0x7fffffff;				\
    r1 += 0x7fffffff;				\
    w0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("direct stack access with 32-bit wraparound. test2")
#[no_mangle]
pub unsafe extern "C" fn __msg(1073741823": "fp pointer and) -> __failure {
    __failure __msg("fp pointer and 1073741823")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn with_32_bit_wraparound_test2() -> __naked void {
    __naked void with_32_bit_wraparound_test2(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += 0x3fffffff;				\
    r1 += 0x3fffffff;				\
    w0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("direct stack access with 32-bit wraparound. test3")
#[no_mangle]
pub unsafe extern "C" fn __msg(1073741822": "fp pointer offset) -> __failure {
    __failure __msg("fp pointer offset 1073741822")
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
#[no_mangle]
pub unsafe extern "C" fn with_32_bit_wraparound_test3() -> __naked void {
    __naked void with_32_bit_wraparound_test3(void)
    {
    asm volatile ("					\
    r1 = r10;					\
    r1 += 0x1fffffff;				\
    r1 += 0x1fffffff;				\
    w0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
