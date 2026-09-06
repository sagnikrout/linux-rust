//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/timer_lockup.c
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

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub t: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } timer1_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } timer2_map SEC(".maps");
    int timer1_err;
    int timer2_err;
#[no_mangle]
unsafe extern "C" fn timer_cb1(map: *mut c_void, k: *mut c_int, v: *mut elem) -> c_int {
    static int timer_cb1(void *map, int *k, struct elem *v)
    {
    struct bpf_timer *timer;
    let mut key: c_int = 0;
    timer = bpf_map_lookup_elem(&timer2_map, &key);
    if (timer)
    timer2_err = bpf_timer_cancel(timer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timer_cb2(map: *mut c_void, k: *mut c_int, v: *mut elem) -> c_int {
    static int timer_cb2(void *map, int *k, struct elem *v)
    {
    struct bpf_timer *timer;
    let mut key: c_int = 0;
    timer = bpf_map_lookup_elem(&timer1_map, &key);
    if (timer)
    timer1_err = bpf_timer_cancel(timer);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn timer1_prog(ctx: *mut c_void) -> c_int {
    int timer1_prog(void *ctx)
    {
    struct bpf_timer *timer;
    let mut key: c_int = 0;
    timer = bpf_map_lookup_elem(&timer1_map, &key);
    if (timer) {
    bpf_timer_init(timer, &timer1_map, CLOCK_BOOTTIME);
    bpf_timer_set_callback(timer, timer_cb1);
    bpf_timer_start(timer, 1, BPF_F_TIMER_CPU_PIN);
    }
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn timer2_prog(ctx: *mut c_void) -> c_int {
    int timer2_prog(void *ctx)
    {
    struct bpf_timer *timer;
    let mut key: c_int = 0;
    timer = bpf_map_lookup_elem(&timer2_map, &key);
    if (timer) {
    bpf_timer_init(timer, &timer2_map, CLOCK_BOOTTIME);
    bpf_timer_set_callback(timer, timer_cb2);
    bpf_timer_start(timer, 1, BPF_F_TIMER_CPU_PIN);
    }
    return 0;
    }
