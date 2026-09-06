//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/libarena/src/spmc.bpf.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
//
// Copyright (c) 2025-2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2025-2026 Emil Tsalapatis <etsal@meta.com>
//

    static inline
#[no_mangle]
pub unsafe extern "C" fn spmc_arr_size(spmc_arr: *mut volatile struct spmc_arr __arena) -> u64 {
    u64 spmc_arr_size(volatile struct spmc_arr __arena *spmc_arr)
    {
    return SPMC_ARR_BASESZ << spmc_arr.order;
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn spmc_arr_get(spmc_arr: *mut volatile struct spmc_arr __arena, ind: u64) -> u64 {
    u64 spmc_arr_get(volatile struct spmc_arr __arena *spmc_arr, u64 ind)
    {
    let mut ret: u64 = READ_ONCE(spmc_arr.data[ind % spmc_arr_size(spmc_arr)]);
    return ret;
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn spmc_arr_put(spmc_arr: *mut volatile struct spmc_arr __arena, ind: u64, value: u64) {
    void spmc_arr_put(volatile struct spmc_arr __arena *spmc_arr, u64 ind, u64 value)
    {
    WRITE_ONCE(spmc_arr.data[ind % spmc_arr_size(spmc_arr)], value);
    }
    static inline
    void spmc_arr_copy(volatile struct spmc_arr __arena *dst,
    volatile struct spmc_arr __arena *src, u64 b, u64 t)
    {
    u64 i;
    for (i = t; i < b && can_loop; i++)
    spmc_arr_put(dst, i, spmc_arr_get(src, i));
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn spmc_order_init(spmc: *mut spmc __arena, order: c_int) -> c_int {
    int spmc_order_init(struct spmc __arena *spmc, int order)
    {
    volatile struct spmc_arr __arena *arr = &spmc.arr[order];
    if (unlikely(!spmc))
    return -EINVAL;
    if (order >= SPMC_ARR_ORDERS)
    return -E2BIG;
// Already allocated?
    if (arr.data)
    return 0;
    arr.data = arena_malloc((SPMC_ARR_BASESZ << order) * sizeof(*arr.data));
    if (!arr.data)
    return -ENOMEM;
    return 0;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn spmc_owned_add(spmc: *mut spmc __arena, val: u64) -> c_int {
    int spmc_owned_add(struct spmc __arena *spmc, u64 val)
    {
    volatile struct spmc_arr __arena *newarr;
    volatile struct spmc_arr __arena *arr;
    ssize_t sz;
    u64 b, t;
    int ret;
    if (unlikely(!spmc))
    return -EINVAL;
//
// Bottom must always be read first, also
// see spmc_steal().
//
    b = smp_load_acquire(&spmc.bottom);
    t = READ_ONCE(spmc.top);
    arr = READ_ONCE(spmc.cur);
    sz = b - t;
    if (sz >= spmc_arr_size(arr) - 1) {
    ret = spmc_order_init(spmc, arr.order + 1);
    if (ret)
    return ret;
    newarr = &spmc.arr[arr.order + 1];
    spmc_arr_copy(newarr, arr, b, t);
    smp_store_release(&spmc.cur, newarr);
    arr = newarr;
    }
    spmc_arr_put(arr, b, val);
    smp_store_release(&spmc.bottom, b + 1);
    return 0;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn spmc_owned_remove(spmc: *mut spmc __arena, val: *mut u64) -> c_int {
    int spmc_owned_remove(struct spmc __arena *spmc, u64 *val)
    {
    volatile struct spmc_arr __arena *arr;
    let mut ret: c_int = 0;
    ssize_t sz;
    u64 value;
    u64 b, t;
    if (unlikely(!spmc || !val))
    return -EINVAL;
    b = READ_ONCE(spmc.bottom) - 1;
    WRITE_ONCE(spmc.bottom, b);
    smp_mb();
    t = READ_ONCE(spmc.top);
    arr = READ_ONCE(spmc.cur);
    sz = b - t;
    if (sz < 0) {
    WRITE_ONCE(spmc.bottom, t);
    return -ENOENT;
    }
    value = spmc_arr_get(arr, b);
    if (sz > 0) {
// val = value;
    return 0;
    }
    if (cmpxchg(&spmc.top, t, t + 1) != t)
    ret = -EAGAIN;
    WRITE_ONCE(spmc.bottom, t + 1);
    if (ret)
    return ret;
// val = value;
    return 0;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn spmc_steal(spmc: *mut spmc __arena, val: *mut u64) -> c_int {
    int spmc_steal(struct spmc __arena *spmc, u64 *val)
    {
    volatile struct spmc_arr __arena *arr;
    ssize_t sz;
    u64 value;
    u64 b, t;
    if (unlikely(!spmc || !val))
    return -EINVAL;
//
// It is important that t is read before b for
// stealers to avoid racing with the owner.
// Races between stealers are dealt with using
// CAS to increment the top value below.
//
    t = smp_load_acquire(&spmc.top);
    b = smp_load_acquire(&spmc.bottom);
    sz = b - t;
    if (sz <= 0)
    return -ENOENT;
    arr = smp_load_acquire(&spmc.cur);
    value = spmc_arr_get(arr, t);
    if (cmpxchg(&spmc.top, t, t + 1) != t)
    return -EAGAIN;
// val = value;
    return 0;
    }
    __weak
    struct spmc __arena *spmc_create(void)
    {
//
// Marked as volatile because otherwise the array
// reference in the internal loop gets demoted to
// scalar and the program fails verification.
//
    struct spmc __arena *volatile spmc;
    int ret, i;
    spmc = arena_malloc(sizeof(*spmc));
    if (!spmc)
    return core::ptr::null_mut();
    spmc.bottom = 0;
    spmc.top = 0;
    for (i = 0; i < SPMC_ARR_ORDERS && can_loop; i++) {
    spmc.arr[i].data = core::ptr::null_mut();
    spmc.arr[i].order = i;
    }
    ret = spmc_order_init((struct spmc __arena *)spmc, 0);
    if (ret) {
    arena_free(spmc);
    return core::ptr::null_mut();
    }
    spmc.cur = &spmc.arr[0];
    return (struct spmc __arena *)spmc;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn spmc_destroy(spmc: *mut spmc __arena) -> c_int {
    int spmc_destroy(struct spmc __arena *spmc)
    {
    int i;
    if (unlikely(!spmc))
    return -EINVAL;
    for (i = 0; i < SPMC_ARR_ORDERS && can_loop; i++)
    arena_free(spmc.arr[i].data);
    arena_free(spmc);
    return 0;
    }
