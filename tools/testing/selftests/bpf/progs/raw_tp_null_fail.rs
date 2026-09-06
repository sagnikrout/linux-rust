//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/raw_tp_null_fail.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
// Ensure module parameter has PTR_MAYBE_NULL
    SEC("tp_btf/bpf_testmod_test_raw_tp_null_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('trusted_ptr_or_null_'": "R1 invalid mem access) -> __failure {
    __failure __msg("R1 invalid mem access 'trusted_ptr_or_null_'")
#[no_mangle]
pub unsafe extern "C" fn test_raw_tp_null_bpf_testmod_test_raw_tp_null_arg_1(ctx: *mut c_void) -> c_int {
    asm volatile("r1 = *(u64 *)(r1 +0); r1 = *(u64 *)(r1 +0);" ::: __clobber_all);
    return 0;
    }
// Check NULL marking
    SEC("tp_btf/sched_pi_setprio")
#[no_mangle]
pub unsafe extern "C" fn __msg('trusted_ptr_or_null_'": "R1 invalid mem access) -> __failure {
    __failure __msg("R1 invalid mem access 'trusted_ptr_or_null_'")
#[no_mangle]
pub unsafe extern "C" fn test_raw_tp_null_sched_pi_setprio_arg_2(ctx: *mut c_void) -> c_int {
    asm volatile("r1 = *(u64 *)(r1 +8); r1 = *(u64 *)(r1 +0);" ::: __clobber_all);
    return 0;
    }
