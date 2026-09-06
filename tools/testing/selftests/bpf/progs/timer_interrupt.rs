//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/timer_interrupt.c
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
pub const CLOCK_MONOTONIC: c_int = 1;
    int preempt_count;
    int in_interrupt;
    int in_interrupt_cb;
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
    } array SEC(".maps");
#[no_mangle]
unsafe extern "C" fn timer_in_interrupt(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_in_interrupt(void *map, int *key, struct bpf_timer *timer)
    {
    preempt_count = get_preempt_count();
    in_interrupt_cb = bpf_in_interrupt();
    return 0;
    }
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_timer_interrupt) -> c_int {
    int BPF_PROG(test_timer_interrupt)
    {
    struct bpf_timer *timer;
    let mut key: c_int = 0;
    timer = bpf_map_lookup_elem(&array, &key);
    if (!timer)
    return 0;
    in_interrupt = bpf_in_interrupt();
    bpf_timer_init(timer, &array, CLOCK_MONOTONIC);
    bpf_timer_set_callback(timer, timer_in_interrupt);
    bpf_timer_start(timer, 0, 0);
    return 0;
    }
