//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/res_spin_lock.c
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
// Copyright (c) 2024-2025 Meta Platforms, Inc. and affiliates.

pub const EDEADLK: c_int = 35;
pub const ETIMEDOUT: c_int = 110;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arr_elem {
    pub lock: bpf_res_spin_lock,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 64);
    __type(key, int);
    __type(value, struct arr_elem);
    } arrmap SEC(".maps");
    struct bpf_res_spin_lock lockA __hidden SEC(".data.A");
    struct bpf_res_spin_lock lockB __hidden SEC(".data.B");
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_test(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_test(struct __sk_buff *ctx)
    {
    struct arr_elem *elem1, *elem2;
    int r;
    elem1 = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem1)
    return -1;
    elem2 = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem2)
    return -1;
    r = bpf_res_spin_lock(&elem1.lock);
    if (r)
    return r;
    r = bpf_res_spin_lock(&elem2.lock);
    if (!r) {
    bpf_res_spin_unlock(&elem2.lock);
    bpf_res_spin_unlock(&elem1.lock);
    return -1;
    }
    bpf_res_spin_unlock(&elem1.lock);
    return r != -EDEADLK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_test_AB(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_test_AB(struct __sk_buff *ctx)
    {
    int r;
    r = bpf_res_spin_lock(&lockA);
    if (r)
    return !r;
// Only unlock if we took the lock.
    if (!bpf_res_spin_lock(&lockB))
    bpf_res_spin_unlock(&lockB);
    bpf_res_spin_unlock(&lockA);
    return 0;
    }
    int err;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_test_BA(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_test_BA(struct __sk_buff *ctx)
    {
    int r;
    r = bpf_res_spin_lock(&lockB);
    if (r)
    return !r;
    if (!bpf_res_spin_lock(&lockA))
    bpf_res_spin_unlock(&lockA);
    else
    err = -EDEADLK;
    bpf_res_spin_unlock(&lockB);
    return err ?: 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_test_held_lock_max(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_test_held_lock_max(struct __sk_buff *ctx)
    {
    struct bpf_res_spin_lock *locks[48] = {};
    struct arr_elem *e;
    u64 time_beg, time;
    let mut ret: c_int = 0, i;
    _Static_assert(ARRAY_SIZE(((struct rqspinlock_held){}).locks) == 31,
    "RES_NR_HELD assumed to be 31");
    for (i = 0; i < 34; i++) {
    let mut key: c_int = i;
// We cannot pass in i as it will get spilled/filled by the compiler and
// loses bounds in verifier state.
//
    e = bpf_map_lookup_elem(&arrmap, &key);
    if (!e)
    return 1;
    locks[i] = &e.lock;
    }
    for (; i < 48; i++) {
    let mut key: c_int = i - 2;
// We cannot pass in i as it will get spilled/filled by the compiler and
// loses bounds in verifier state.
//
    e = bpf_map_lookup_elem(&arrmap, &key);
    if (!e)
    return 1;
    locks[i] = &e.lock;
    }
    time_beg = bpf_ktime_get_ns();
    for (i = 0; i < 34; i++) {
    if (bpf_res_spin_lock(locks[i]))
    goto end;
    }
// Trigger AA, after exhausting entries in the held lock table. This
// time, only the timeout can save us, as AA detection won't succeed.
//
    ret = bpf_res_spin_lock(locks[34]);
    if (!ret) {
    bpf_res_spin_unlock(locks[34]);
    ret = 1;
    goto end;
    }
    ret = ret != -ETIMEDOUT ? 2 : 0;
    end:
    for (i = i - 1; i >= 0; i--)
    bpf_res_spin_unlock(locks[i]);
    time = bpf_ktime_get_ns() - time_beg;
// Time spent should be easily above our limit (1/4 s), since AA
// detection won't be expedited due to lack of held lock entry.
//
    return ret ?: (time > 1000000000 / 4 ? 0 : 1);
    }
    char _license[] SEC("license") = "GPL";
