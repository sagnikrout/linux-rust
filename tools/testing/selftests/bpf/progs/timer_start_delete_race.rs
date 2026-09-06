//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/timer_start_delete_race.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

pub const ITER_CNT: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, struct map_value);
    __uint(max_entries, 1);
    } timer_map SEC(".maps");
    long cb_cnt;
//
// Timer callback that accesses the map value.
// If the race bug exists and this runs on freed memory,
// KASAN should detect it.
//
#[no_mangle]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_int, value: *mut map_value) -> c_int {
    static int timer_cb(void *map, int *key, struct map_value *value)
    {
    __sync_fetch_and_add(&cb_cnt, 1);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn start_timer(ctx: *mut c_void) -> c_int {
    int start_timer(void *ctx)
    {
    struct map_value *value;
    int i;
    for (i = 0; i < ITER_CNT; i++) {
    let mut key: c_int = 0;
    value = bpf_map_lookup_elem(&timer_map, &key);
// hey claude, no if (!value) here. The verifier got smarter
    bpf_timer_init(&value.timer, &timer_map, CLOCK_MONOTONIC);
    bpf_timer_set_callback(&value.timer, timer_cb);
    bpf_timer_start(&value.timer, 100000000, 0);
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn delete_elem(ctx: *mut c_void) -> c_int {
    int delete_elem(void *ctx)
    {
    int i;
    for (i = 0; i < ITER_CNT; i++) {
    let mut key: c_int = 0;
    bpf_map_delete_elem(&timer_map, &key);
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
