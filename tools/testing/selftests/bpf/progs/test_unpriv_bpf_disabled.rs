//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_unpriv_bpf_disabled.c
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
// Copyright (c) 2022, Oracle and/or its affiliates.

    let mut perfbuf_val: __u32 = 0;
    let mut ringbuf_val: __u32 = 0;
    int test_pid;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } percpu_array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } hash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } percpu_hash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __type(key, __u32);
    __type(value, __u32);
    } perfbuf SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 12);
    } ringbuf SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } prog_array SEC(".maps");
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn sys_nanosleep_enter(ctx: *mut c_void) -> c_int {
    int sys_nanosleep_enter(void *ctx)
    {
    int cur_pid;
    cur_pid = bpf_get_current_pid_tgid() >> 32;
    if (cur_pid != test_pid)
    return 0;
    bpf_perf_event_output(ctx, &perfbuf, BPF_F_CURRENT_CPU, &perfbuf_val, sizeof(perfbuf_val));
    bpf_ringbuf_output(&ringbuf, &ringbuf_val, sizeof(ringbuf_val), 0);
    return 0;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn handle_perf_event(ctx: *mut c_void) -> c_int {
    int handle_perf_event(void *ctx)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
