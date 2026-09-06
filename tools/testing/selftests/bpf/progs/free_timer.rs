//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/free_timer.c
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
// Copyright (C) 2025. Huawei Technologies Co., Ltd

pub const MAX_ENTRIES: c_int = 8;
// clang considers 'sum += 1' as usage but 'sum++' as non-usage.  GCC
// is more consistent and considers both 'sum += 1' and 'sum++' as
// non-usage.  This triggers warnings in the functions below.
//
// Starting with GCC 16 -Wunused-but-set-variable=2 can be used to
// mimic clang's behavior.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, int);
    __type(value, struct map_value);
    __uint(max_entries, MAX_ENTRIES);
    } map SEC(".maps");
#[no_mangle]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_void, value: *mut map_value) -> c_int {
    static int timer_cb(void *map, void *key, struct map_value *value)
    {
    let mut sum: volatile int = 0;
    int i;
    bpf_for(i, 0, 1024 * 1024) sum += i;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn start_cb(key: c_int) -> c_int {
    static int start_cb(int key)
    {
    struct map_value *value;
    value = bpf_map_lookup_elem(&map, (void *)&key);
    if (!value)
    return 0;
    bpf_timer_init(&value.timer, &map, CLOCK_MONOTONIC);
    bpf_timer_set_callback(&value.timer, timer_cb);
// Hope 100us will be enough to wake-up and run the overwrite thread
    bpf_timer_start(&value.timer, 100000, BPF_F_TIMER_CPU_PIN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn overwrite_cb(key: c_int) -> c_int {
    static int overwrite_cb(int key)
    {
    let mut zero: map_value = {};
// Free the timer which may run on other CPU
    bpf_map_update_elem(&map, (void *)&key, &zero, BPF_ANY);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: start_timer) -> c_int {
    int BPF_PROG(start_timer)
    {
    bpf_loop(MAX_ENTRIES, start_cb, core::ptr::null_mut(), 0);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: overwrite_timer) -> c_int {
    int BPF_PROG(overwrite_timer)
    {
    bpf_loop(MAX_ENTRIES, overwrite_cb, core::ptr::null_mut(), 0);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
