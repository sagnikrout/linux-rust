//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/wq_failures.c
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
// Copyright (c) 2024 Benjamin Tissoires
//

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub w: bpf_wq,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 2);
    __type(key, int);
    __type(value, struct elem);
    } array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 4);
    __type(key, int);
    __type(value, struct elem);
    } lru SEC(".maps");
// callback for non sleepable workqueue
#[no_mangle]
unsafe extern "C" fn wq_callback(map: *mut c_void, key: *mut c_int, value: *mut c_void) -> c_int {
    static int wq_callback(void *map, int *key, void *value)
    {
    bpf_kfunc_common_test();
    return 0;
    }
// callback for sleepable workqueue
#[no_mangle]
unsafe extern "C" fn wq_cb_sleepable(map: *mut c_void, key: *mut c_int, value: *mut c_void) -> c_int {
    static int wq_cb_sleepable(void *map, int *key, void *value)
    {
    bpf_kfunc_call_test_sleepable();
    return 0;
    }
    SEC("tc")
// test that bpf_wq_init takes a map as a second argument
//
    __log_level(2)
    __flag(BPF_F_TEST_STATE_FREQ)
    __failure
    __msg(": (85) call bpf_wq_init#") /* anchor message */
    __msg("pointer in R2 isn't map pointer")
#[no_mangle]
pub unsafe extern "C" fn test_wq_init_nomap(ctx: *mut c_void) -> c_long {
    long test_wq_init_nomap(void *ctx)
    {
    struct bpf_wq *wq;
    struct elem *val;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&array, &key);
    if (!val)
    return -1;
    wq = &val.w;
    if (bpf_wq_init(wq, &key, 0) != 0)
    return -3;
    return 0;
    }
    SEC("tc")
// test that the workqueue is part of the map in bpf_wq_init
//
    __log_level(2)
    __flag(BPF_F_TEST_STATE_FREQ)
    __failure
    __msg(": (85) call bpf_wq_init#") /* anchor message */
    __msg("workqueue pointer in R1 map_uid=0 doesn't match map pointer in R2 map_uid=0")
#[no_mangle]
pub unsafe extern "C" fn test_wq_init_wrong_map(ctx: *mut c_void) -> c_long {
    long test_wq_init_wrong_map(void *ctx)
    {
    struct bpf_wq *wq;
    struct elem *val;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&array, &key);
    if (!val)
    return -1;
    wq = &val.w;
    if (bpf_wq_init(wq, &lru, 0) != 0)
    return -3;
    return 0;
    }
    SEC("?tc")
    __log_level(2)
    __failure
// check that the first argument of bpf_wq_set_callback()
// is a correct bpf_wq pointer.
//
    __msg(": (85) call bpf_wq_set_callback#") /* anchor message */
    __msg("R1 doesn't point to a map value")
#[no_mangle]
pub unsafe extern "C" fn test_wrong_wq_pointer(ctx: *mut c_void) -> c_long {
    long test_wrong_wq_pointer(void *ctx)
    {
    let mut key: c_int = 0;
    struct bpf_wq *wq;
    wq = bpf_map_lookup_elem(&array, &key);
    if (!wq)
    return 1;
    if (bpf_wq_init(wq, &array, 0))
    return 2;
    if (bpf_wq_set_callback((void *)&wq, wq_callback, 0))
    return 3;
    return -22;
    }
    SEC("?tc")
    __log_level(2)
    __failure
// check that the first argument of bpf_wq_set_callback()
// is a correct bpf_wq pointer.
//
    __msg(": (85) call bpf_wq_set_callback#") /* anchor message */
    __msg("off 1 doesn't point to 'struct bpf_wq' that is at 0")
#[no_mangle]
pub unsafe extern "C" fn test_wrong_wq_pointer_offset(ctx: *mut c_void) -> c_long {
    long test_wrong_wq_pointer_offset(void *ctx)
    {
    let mut key: c_int = 0;
    struct bpf_wq *wq;
    wq = bpf_map_lookup_elem(&array, &key);
    if (!wq)
    return 1;
    if (bpf_wq_init(wq, &array, 0))
    return 2;
    if (bpf_wq_set_callback((void *)wq + 1, wq_cb_sleepable, 0))
    return 3;
    return -22;
    }
    SEC("tc")
    __log_level(2)
    __failure
    __msg(": (85) call bpf_wq_init#")
    __msg("R1 doesn't have constant offset. bpf_wq has to be at the constant offset")
#[no_mangle]
pub unsafe extern "C" fn test_bad_wq_off(ctx: *mut c_void) -> c_long {
    long test_bad_wq_off(void *ctx)
    {
    struct elem *val;
    struct bpf_wq *wq;
    let mut key: c_int = 42;
    u64 unknown;
    val = bpf_map_lookup_elem(&array, &key);
    if (!val)
    return -2;
    unknown = bpf_get_prandom_u32();
    wq = &val.w + unknown;
    if (bpf_wq_init(wq, &array, 0) != 0)
    return -3;
    return 0;
    }
