//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/bperf_leader.bpf.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2021 Facebook

    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(int));
    __uint(map_flags, BPF_F_PRESERVE_ELEMS);
    } events SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_perf_event_value));
    __uint(max_entries, 1);
    } prev_readings SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_perf_event_value));
    __uint(max_entries, 1);
    } diff_readings SEC(".maps");
    SEC("raw_tp/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_switch) -> c_int {
    int BPF_PROG(on_switch)
    {
    struct bpf_perf_event_value val, *prev_val, *diff_val;
    let mut key: __u32 = bpf_get_smp_processor_id();
    let mut zero: __u32 = 0;
    long err;
    prev_val = bpf_map_lookup_elem(&prev_readings, &zero);
    if (!prev_val)
    return 0;
    diff_val = bpf_map_lookup_elem(&diff_readings, &zero);
    if (!diff_val)
    return 0;
    err = bpf_perf_event_read_value(&events, key, &val, sizeof(val));
    if (err)
    return 0;
    diff_val.counter = val.counter - prev_val.counter;
    diff_val.enabled = val.enabled - prev_val.enabled;
    diff_val.running = val.running - prev_val.running;
// prev_val = val;
    return 0;
    }
    char LICENSE[] SEC("license") = "Dual BSD/GPL";
