//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_netfilter_retcode.c
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

    SEC("netfilter")
    __description("bpf_exit with invalid return code. test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "R0 is not a known) -> __failure {
    __failure __msg("R0 is not a known value")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_return_code_test1() -> __naked void {
    __naked void with_invalid_return_code_test1(void)
    {
    asm volatile ("					\
    r0 = *(u64*)(r1 + 0);				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("netfilter")
    __description("bpf_exit with valid return code. test2")
    __success
#[no_mangle]
pub unsafe extern "C" fn with_valid_return_code_test2() -> __naked void {
    __naked void with_valid_return_code_test2(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("netfilter")
    __description("bpf_exit with valid return code. test3")
    __success
#[no_mangle]
pub unsafe extern "C" fn with_valid_return_code_test3() -> __naked void {
    __naked void with_valid_return_code_test3(void)
    {
    asm volatile ("					\
    r0 = 1;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("netfilter")
    __description("bpf_exit with invalid return code. test4")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "R0 has smin=2 smax=2 should have been in, _arg: 1]") -> __failure {
    __failure __msg("R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_return_code_test4() -> __naked void {
    __naked void with_invalid_return_code_test4(void)
    {
    asm volatile ("					\
    r0 = 2;						\
    exit;						\
    "	::: __clobber_all);
    }
