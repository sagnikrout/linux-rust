//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_helper_restricted.c
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


// SPDX-License-Identifier: GPL-2.0-only

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer {
    pub t: bpf_timer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock {
    pub l: bpf_spin_lock,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct timer);
    } timers SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct lock);
    } locks SEC(".maps");
#[no_mangle]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_int, timer: *mut timer) -> c_int {
    static int timer_cb(void *map, int *key, struct timer *timer)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timer_work() {
    static void timer_work(void)
    {
    struct timer *timer;
    let mut key: c_int = 0;
    timer  = bpf_map_lookup_elem(&timers, &key);
    if (timer) {
    bpf_timer_init(&timer.t, &timers, CLOCK_MONOTONIC);
    bpf_timer_set_callback(&timer.t, timer_cb);
    bpf_timer_start(&timer.t, 10E9, 0);
    bpf_timer_cancel(&timer.t);
    }
    }
#[no_mangle]
unsafe extern "C" fn spin_lock_work() {
    static void spin_lock_work(void)
    {
    let mut key: c_int = 0;
    struct lock *lock;
    lock = bpf_map_lookup_elem(&locks, &key);
    if (lock) {
    bpf_spin_lock(&lock.l);
    bpf_spin_unlock(&lock.l);
    }
    }
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn raw_tp_timer(ctx: *mut c_void) -> c_int {
    int raw_tp_timer(void *ctx)
    {
    timer_work();
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn tp_timer(ctx: *mut c_void) -> c_int {
    int tp_timer(void *ctx)
    {
    timer_work();
    return 0;
    }
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn kprobe_timer(ctx: *mut c_void) -> c_int {
    int kprobe_timer(void *ctx)
    {
    timer_work();
    return 0;
    }
    SEC("?perf_event")
#[no_mangle]
pub unsafe extern "C" fn perf_event_timer(ctx: *mut c_void) -> c_int {
    int perf_event_timer(void *ctx)
    {
    timer_work();
    return 0;
    }
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn raw_tp_spin_lock(ctx: *mut c_void) -> c_int {
    int raw_tp_spin_lock(void *ctx)
    {
    spin_lock_work();
    return 0;
    }
    SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn tp_spin_lock(ctx: *mut c_void) -> c_int {
    int tp_spin_lock(void *ctx)
    {
    spin_lock_work();
    return 0;
    }
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn kprobe_spin_lock(ctx: *mut c_void) -> c_int {
    int kprobe_spin_lock(void *ctx)
    {
    spin_lock_work();
    return 0;
    }
    SEC("?perf_event")
#[no_mangle]
pub unsafe extern "C" fn perf_event_spin_lock(ctx: *mut c_void) -> c_int {
    int perf_event_spin_lock(void *ctx)
    {
    spin_lock_work();
    return 0;
    }
    const char LICENSE[] SEC("license") = "GPL";
