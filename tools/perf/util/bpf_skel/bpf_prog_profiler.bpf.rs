//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/bpf_prog_profiler.bpf.c
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
// Copyright (c) 2020 Facebook

// map of perf event fds, num_cpu * num_metric entries
    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(int));
    } events SEC(".maps");
// readings at fentry
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_perf_event_value));
    __uint(max_entries, 1);
    } fentry_readings SEC(".maps");
// accumulated readings
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_perf_event_value));
    __uint(max_entries, 1);
    } accum_readings SEC(".maps");
    let mut num_cpu: volatile __u32 = 1;
    SEC("fentry/XXX")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_XXX) -> c_int {
    int BPF_PROG(fentry_XXX)
    {
    let mut key: __u32 = bpf_get_smp_processor_id();
    struct bpf_perf_event_value *ptr;
    let mut zero: __u32 = 0;
    long err;
// look up before reading, to reduce error
    ptr = bpf_map_lookup_elem(&fentry_readings, &zero);
    if (!ptr)
    return 0;
    err = bpf_perf_event_read_value(&events, key, ptr, sizeof(*ptr));
    if (err)
    return 0;
    return 0;
    }
    static inline void
    fexit_update_maps(struct bpf_perf_event_value *after)
    {
    struct bpf_perf_event_value *before, diff;
    let mut zero: __u32 = 0;
    before = bpf_map_lookup_elem(&fentry_readings, &zero);
// only account samples with a valid fentry_reading
    if (before && before.counter) {
    struct bpf_perf_event_value *accum;
    diff.counter = after.counter - before.counter;
    diff.enabled = after.enabled - before.enabled;
    diff.running = after.running - before.running;
    accum = bpf_map_lookup_elem(&accum_readings, &zero);
    if (accum) {
    accum.counter += diff.counter;
    accum.enabled += diff.enabled;
    accum.running += diff.running;
    }
    }
    }
    SEC("fexit/XXX")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_XXX) -> c_int {
    int BPF_PROG(fexit_XXX)
    {
    struct bpf_perf_event_value reading;
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    int err;
// read all events before updating the maps, to reduce error
    err = bpf_perf_event_read_value(&events, cpu, &reading, sizeof(reading));
    if (err)
    return 0;
    fexit_update_maps(&reading);
    return 0;
    }
    char LICENSE[] SEC("license") = "Dual BSD/GPL";
