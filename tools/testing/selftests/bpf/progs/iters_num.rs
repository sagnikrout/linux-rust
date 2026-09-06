//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_num.c
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

    let mut exp_empty_zero: volatile __s64 = 0 + 1;
    __s64 res_empty_zero;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_empty_zero(ctx: *const c_void) -> c_int {
    int num_empty_zero(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, 0, 0) sum += i;
    res_empty_zero = 1 + sum;
    return 0;
    }
    let mut exp_empty_int_min: volatile __s64 = 0 + 2;
    __s64 res_empty_int_min;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_empty_int_min(ctx: *const c_void) -> c_int {
    int num_empty_int_min(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, INT_MIN, INT_MIN) sum += i;
    res_empty_int_min = 2 + sum;
    return 0;
    }
    let mut exp_empty_int_max: volatile __s64 = 0 + 3;
    __s64 res_empty_int_max;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_empty_int_max(ctx: *const c_void) -> c_int {
    int num_empty_int_max(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, INT_MAX, INT_MAX) sum += i;
    res_empty_int_max = 3 + sum;
    return 0;
    }
    let mut exp_empty_minus_one: volatile __s64 = 0 + 4;
    __s64 res_empty_minus_one;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_empty_minus_one(ctx: *const c_void) -> c_int {
    int num_empty_minus_one(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, -1, -1) sum += i;
    res_empty_minus_one = 4 + sum;
    return 0;
    }
    let mut exp_simple_sum: volatile __s64 = 9 * 10 / 2;
    __s64 res_simple_sum;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_simple_sum(ctx: *const c_void) -> c_int {
    int num_simple_sum(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, 0, 10) sum += i;
    res_simple_sum = sum;
    return 0;
    }
    let mut exp_neg_sum: volatile __s64 = -11 * 10 / 2;
    __s64 res_neg_sum;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_neg_sum(ctx: *const c_void) -> c_int {
    int num_neg_sum(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, -10, 0) sum += i;
    res_neg_sum = sum;
    return 0;
    }
    let mut exp_very_neg_sum: volatile __s64 = INT_MIN + (__s64)(INT_MIN + 1);
    __s64 res_very_neg_sum;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_very_neg_sum(ctx: *const c_void) -> c_int {
    int num_very_neg_sum(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, INT_MIN, INT_MIN + 2) sum += i;
    res_very_neg_sum = sum;
    return 0;
    }
    let mut exp_very_big_sum: volatile __s64 = (__s64)(INT_MAX - 1) + (__s64)(INT_MAX - 2);
    __s64 res_very_big_sum;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_very_big_sum(ctx: *const c_void) -> c_int {
    int num_very_big_sum(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, INT_MAX - 2, INT_MAX) sum += i;
    res_very_big_sum = sum;
    return 0;
    }
    let mut exp_neg_pos_sum: volatile __s64 = -3;
    __s64 res_neg_pos_sum;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_neg_pos_sum(ctx: *const c_void) -> c_int {
    int num_neg_pos_sum(const void *ctx)
    {
    let mut sum: __s64 = 0, i;
    bpf_for(i, -3, 3) sum += i;
    res_neg_pos_sum = sum;
    return 0;
    }
    let mut exp_invalid_range: volatile __s64 = -EINVAL;
    __s64 res_invalid_range;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_invalid_range(ctx: *const c_void) -> c_int {
    int num_invalid_range(const void *ctx)
    {
    struct bpf_iter_num it;
    res_invalid_range = bpf_iter_num_new(&it, 1, 0);
    bpf_iter_num_destroy(&it);
    return 0;
    }
    let mut exp_max_range: volatile __s64 = 0 + 10;
    __s64 res_max_range;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_max_range(ctx: *const c_void) -> c_int {
    int num_max_range(const void *ctx)
    {
    struct bpf_iter_num it;
    res_max_range = 10 + bpf_iter_num_new(&it, 0, BPF_MAX_LOOPS);
    bpf_iter_num_destroy(&it);
    return 0;
    }
    let mut exp_e2big_range: volatile __s64 = -E2BIG;
    __s64 res_e2big_range;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_e2big_range(ctx: *const c_void) -> c_int {
    int num_e2big_range(const void *ctx)
    {
    struct bpf_iter_num it;
    res_e2big_range = bpf_iter_num_new(&it, -1, BPF_MAX_LOOPS);
    bpf_iter_num_destroy(&it);
    return 0;
    }
    let mut exp_succ_elem_cnt: volatile __s64 = 10;
    __s64 res_succ_elem_cnt;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_succ_elem_cnt(ctx: *const c_void) -> c_int {
    int num_succ_elem_cnt(const void *ctx)
    {
    struct bpf_iter_num it;
    let mut cnt: c_int = 0, *v;
    bpf_iter_num_new(&it, 0, 10);
    while ((v = bpf_iter_num_next(&it))) {
    cnt++;
    }
    bpf_iter_num_destroy(&it);
    res_succ_elem_cnt = cnt;
    return 0;
    }
    let mut exp_overfetched_elem_cnt: volatile __s64 = 5;
    __s64 res_overfetched_elem_cnt;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_overfetched_elem_cnt(ctx: *const c_void) -> c_int {
    int num_overfetched_elem_cnt(const void *ctx)
    {
    struct bpf_iter_num it;
    let mut cnt: c_int = 0, *v, i;
    bpf_iter_num_new(&it, 0, 5);
    for (i = 0; i < 10; i++) {
    v = bpf_iter_num_next(&it);
    if (v)
    cnt++;
    }
    bpf_iter_num_destroy(&it);
    res_overfetched_elem_cnt = cnt;
    return 0;
    }
    let mut exp_fail_elem_cnt: volatile __s64 = 20 + 0;
    __s64 res_fail_elem_cnt;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn num_fail_elem_cnt(ctx: *const c_void) -> c_int {
    int num_fail_elem_cnt(const void *ctx)
    {
    struct bpf_iter_num it;
    let mut cnt: c_int = 0, *v, i;
    bpf_iter_num_new(&it, 100, 10);
    for (i = 0; i < 10; i++) {
    v = bpf_iter_num_next(&it);
    if (v)
    cnt++;
    }
    bpf_iter_num_destroy(&it);
    res_fail_elem_cnt = 20 + cnt;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
