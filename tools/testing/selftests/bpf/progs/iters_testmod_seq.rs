//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_testmod_seq.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_testmod_seq {
    pub :64: u64,
    pub :64: u64,
}

    extern int bpf_iter_testmod_seq_new(struct bpf_iter_testmod_seq *it, s64 value, int cnt) __ksym;
    extern s64 *bpf_iter_testmod_seq_next(struct bpf_iter_testmod_seq *it) __ksym;
    extern s64 bpf_iter_testmod_seq_value(int blah, struct bpf_iter_testmod_seq *it) __ksym;
    extern void bpf_iter_testmod_seq_destroy(struct bpf_iter_testmod_seq *it) __ksym;
    let mut exp_empty: volatile __s64 = 0 + 1;
    __s64 res_empty;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("fp-16=iter_testmod_seq(id=1,state=active,depth=0)")
    __msg("fp-16=iter_testmod_seq(id=1,state=drained,depth=0)")
    __msg("call bpf_iter_testmod_seq_destroy")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_empty(ctx: *const c_void) -> c_int {
    int testmod_seq_empty(const void *ctx)
    {
    let mut sum: __s64 = 0, *i;
    bpf_for_each(testmod_seq, i, 1000, 0) sum += *i;
    res_empty = 1 + sum;
    return 0;
    }
    let mut exp_full: volatile __s64 = 1000000;
    __s64 res_full;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("fp-16=iter_testmod_seq(id=1,state=active,depth=0)")
    __msg("fp-16=iter_testmod_seq(id=1,state=drained,depth=0)")
    __msg("call bpf_iter_testmod_seq_destroy")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_full(ctx: *const c_void) -> c_int {
    int testmod_seq_full(const void *ctx)
    {
    let mut sum: __s64 = 0, *i;
    bpf_for_each(testmod_seq, i, 1000, 1000) sum += *i;
    res_full = sum;
    return 0;
    }
    let mut exp_truncated: volatile __s64 = 10 * 1000000;
    __s64 res_truncated;
    let mut zero: static volatile int = 0;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("fp-16=iter_testmod_seq(id=1,state=active,depth=0)")
    __msg("fp-16=iter_testmod_seq(id=1,state=drained,depth=0)")
    __msg("call bpf_iter_testmod_seq_destroy")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_truncated(ctx: *const c_void) -> c_int {
    int testmod_seq_truncated(const void *ctx)
    {
    let mut sum: __s64 = 0, *i;
    let mut cnt: c_int = zero;
    bpf_for_each(testmod_seq, i, 10, 2000000) {
    sum += *i;
    cnt++;
    if (cnt >= 1000000)
    break;
    }
    res_truncated = sum;
    return 0;
    }
    SEC("?raw_tp")
    __failure
    __msg("expected an initialized iter_testmod_seq as R2")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_getter_before_bad(ctx: *const c_void) -> c_int {
    int testmod_seq_getter_before_bad(const void *ctx)
    {
    struct bpf_iter_testmod_seq it;
    return bpf_iter_testmod_seq_value(0, &it);
    }
    SEC("?raw_tp")
    __failure
    __msg("expected an initialized iter_testmod_seq as R2")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_getter_after_bad(ctx: *const c_void) -> c_int {
    int testmod_seq_getter_after_bad(const void *ctx)
    {
    struct bpf_iter_testmod_seq it;
    let mut sum: i64 = 0, *v;
    bpf_iter_testmod_seq_new(&it, 100, 100);
    while ((v = bpf_iter_testmod_seq_next(&it))) {
    sum += *v;
    }
    bpf_iter_testmod_seq_destroy(&it);
    return sum + bpf_iter_testmod_seq_value(0, &it);
    }
    SEC("?socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1000000) -> __success {
    __success __retval(1000000)
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_getter_good(ctx: *const c_void) -> c_int {
    int testmod_seq_getter_good(const void *ctx)
    {
    struct bpf_iter_testmod_seq it;
    let mut sum: i64 = 0, *v;
    bpf_iter_testmod_seq_new(&it, 100, 100);
    while ((v = bpf_iter_testmod_seq_next(&it))) {
    sum += *v;
    }
    sum *= bpf_iter_testmod_seq_value(0, &it);
    bpf_iter_testmod_seq_destroy(&it);
    return sum;
    }
    char _license[] SEC("license") = "GPL";
