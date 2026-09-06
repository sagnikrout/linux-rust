//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_async_cb_context.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
// Timer tests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_elem {
    pub t: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct timer_elem);
    } timer_map SEC(".maps");
#[no_mangle]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_cb(void *map, int *key, struct bpf_timer *timer)
    {
    u32 data;
// Timer callbacks are never sleepable, even from non-sleepable programs
    bpf_copy_from_user(&data, sizeof(data), core::ptr::null_mut());
    return 0;
    }
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __msg(prog": "sleepable helper bpf_copy_from_user#{{[0-9]+}} in non-sleepable) -> __failure {
    __failure __msg("sleepable helper bpf_copy_from_user#{{[0-9]+}} in non-sleepable prog")
#[no_mangle]
pub unsafe extern "C" fn timer_non_sleepable_prog(ctx: *mut c_void) -> c_int {
    int timer_non_sleepable_prog(void *ctx)
    {
    struct timer_elem *val;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&timer_map, &key);
    if (!val)
    return 0;
    bpf_timer_init(&val.t, &timer_map, 0);
    bpf_timer_set_callback(&val.t, timer_cb);
    return 0;
    }
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn __msg(prog": "sleepable helper bpf_copy_from_user#{{[0-9]+}} in non-sleepable) -> __failure {
    __failure __msg("sleepable helper bpf_copy_from_user#{{[0-9]+}} in non-sleepable prog")
#[no_mangle]
pub unsafe extern "C" fn timer_sleepable_prog(ctx: *mut c_void) -> c_int {
    int timer_sleepable_prog(void *ctx)
    {
    struct timer_elem *val;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&timer_map, &key);
    if (!val)
    return 0;
    bpf_timer_init(&val.t, &timer_map, 0);
    bpf_timer_set_callback(&val.t, timer_cb);
    return 0;
    }
// Workqueue tests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wq_elem {
    pub w: bpf_wq,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct wq_elem);
    } wq_map SEC(".maps");
#[no_mangle]
unsafe extern "C" fn wq_cb(map: *mut c_void, key: *mut c_int, value: *mut c_void) -> c_int {
    static int wq_cb(void *map, int *key, void *value)
    {
    u32 data;
// Workqueue callbacks are always sleepable, even from non-sleepable programs
    bpf_copy_from_user(&data, sizeof(data), core::ptr::null_mut());
    return 0;
    }
    SEC("fentry/bpf_fentry_test1")
    __success
#[no_mangle]
pub unsafe extern "C" fn wq_non_sleepable_prog(ctx: *mut c_void) -> c_int {
    int wq_non_sleepable_prog(void *ctx)
    {
    struct wq_elem *val;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&wq_map, &key);
    if (!val)
    return 0;
    if (bpf_wq_init(&val.w, &wq_map, 0) != 0)
    return 0;
    if (bpf_wq_set_callback(&val.w, wq_cb, 0) != 0)
    return 0;
    return 0;
    }
    SEC("lsm.s/file_open")
    __success
#[no_mangle]
pub unsafe extern "C" fn wq_sleepable_prog(ctx: *mut c_void) -> c_int {
    int wq_sleepable_prog(void *ctx)
    {
    struct wq_elem *val;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&wq_map, &key);
    if (!val)
    return 0;
    if (bpf_wq_init(&val.w, &wq_map, 0) != 0)
    return 0;
    if (bpf_wq_set_callback(&val.w, wq_cb, 0) != 0)
    return 0;
    return 0;
    }
// Task work tests
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_work_elem {
    pub tw: bpf_task_work,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct task_work_elem);
    } task_work_map SEC(".maps");
#[no_mangle]
unsafe extern "C" fn task_work_cb(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> c_int {
    static int task_work_cb(struct bpf_map *map, void *key, void *value)
    {
    u32 data;
// Task work callbacks are always sleepable, even from non-sleepable programs
    bpf_copy_from_user(&data, sizeof(data), core::ptr::null_mut());
    return 0;
    }
    SEC("fentry/bpf_fentry_test1")
    __success
#[no_mangle]
pub unsafe extern "C" fn task_work_non_sleepable_prog(ctx: *mut c_void) -> c_int {
    int task_work_non_sleepable_prog(void *ctx)
    {
    struct task_work_elem *val;
    struct task_struct *task;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&task_work_map, &key);
    if (!val)
    return 0;
    task = bpf_get_current_task_btf();
    if (!task)
    return 0;
    bpf_task_work_schedule_resume(task, &val.tw, &task_work_map, task_work_cb);
    return 0;
    }
    SEC("lsm.s/file_open")
    __success
#[no_mangle]
pub unsafe extern "C" fn task_work_sleepable_prog(ctx: *mut c_void) -> c_int {
    int task_work_sleepable_prog(void *ctx)
    {
    struct task_work_elem *val;
    struct task_struct *task;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&task_work_map, &key);
    if (!val)
    return 0;
    task = bpf_get_current_task_btf();
    if (!task)
    return 0;
    bpf_task_work_schedule_resume(task, &val.tw, &task_work_map, task_work_cb);
    return 0;
    }
