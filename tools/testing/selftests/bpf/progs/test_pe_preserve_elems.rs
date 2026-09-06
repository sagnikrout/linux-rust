//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_pe_preserve_elems.c
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
// Copyright (c) 2020 Facebook

    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } array_1 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    __uint(map_flags, BPF_F_PRESERVE_ELEMS);
    } array_2 SEC(".maps");
    SEC("raw_tp/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: read_array_1) -> c_int {
    int BPF_PROG(read_array_1)
    {
    struct bpf_perf_event_value val;
    return bpf_perf_event_read_value(&array_1, 0, &val, sizeof(val));
    }
    SEC("raw_tp/task_rename")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: read_array_2) -> c_int {
    int BPF_PROG(read_array_2)
    {
    struct bpf_perf_event_value val;
    return bpf_perf_event_read_value(&array_2, 0, &val, sizeof(val));
    }
    char LICENSE[] SEC("license") = "GPL";
