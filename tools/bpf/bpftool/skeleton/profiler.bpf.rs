//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpftool/skeleton/profiler.bpf.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_perf_event_value___local {
    pub counter: __u64,
    pub enabled: __u64,
    pub running: __u64,
    pub __attribute__((preserve_access_index)): },
// map of perf event fds, num_cpu * num_metric entries
    struct {
    pub BPF_MAP_TYPE_PERF_EVENT_ARRAY): __uint(type,,
    pub sizeof(u32)): __uint(key_size,,
    pub sizeof(int)): __uint(value_size,,
    pub SEC(".maps"): } events,
// readings at fentry
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub sizeof(u32)): __uint(key_size,,
    pub bpf_perf_event_value___local)): __uint(value_size, sizeof(struct,
    pub SEC(".maps"): } fentry_readings,
// accumulated readings
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub sizeof(u32)): __uint(key_size,,
    pub bpf_perf_event_value___local)): __uint(value_size, sizeof(struct,
    pub SEC(".maps"): } accum_readings,
// sample counts, one per cpu
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub sizeof(u32)): __uint(key_size,,
    pub sizeof(u64)): __uint(value_size,,
    pub SEC(".maps"): } counts,
    pub 1: volatile __u32 num_cpu =,
    pub 1: volatile __u32 num_metric =,
pub const MAX_NUM_METRICS: c_int = 4;
    SEC("fentry/XXX")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_XXX) -> c_int {
    int BPF_PROG(fentry_XXX)
    {
    pub ptrs: [*mut bpf_perf_event_value___local; MAX_NUM_METRICS],
    pub bpf_get_smp_processor_id(): u32 key =,
    pub i: u32,
// look up before reading, to reduce error
    pub {: for (i = 0; i < num_metric && i < MAX_NUM_METRICS; i++),
    pub i: u32 flag =,
    pub &flag): ptrs[i] = bpf_map_lookup_elem(&fentry_readings,,
    if (!ptrs[i])
    pub 0: return,
    }
    pub {: for (i = 0; i < num_metric && i < MAX_NUM_METRICS; i++),
    pub reading: bpf_perf_event_value___local,
    pub err: c_int,
    err = bpf_perf_event_read_value(&events, key, (void *)&reading,
    if (err)
    pub 0: return,
// (ptrs[i]) = reading;
    pub num_cpu: key +=,
    }
    pub 0: return,
    }
    static inline void
    fexit_update_maps(u32 id, struct bpf_perf_event_value___local *after)
    {
    pub diff: *mut *mut bpf_perf_event_value___local before,,
    pub &id): before = bpf_map_lookup_elem(&fentry_readings,,
// only account samples with a valid fentry_reading
    if (before && before.counter) {
    pub accum: *mut bpf_perf_event_value___local,
    pub before->counter: diff.counter = after->counter -,
    pub before->enabled: diff.enabled = after->enabled -,
    pub before->running: diff.running = after->running -,
    pub &id): accum = bpf_map_lookup_elem(&accum_readings,,
    if (accum) {
    pub diff.counter: accum->counter +=,
    pub diff.enabled: accum->enabled +=,
    pub diff.running: accum->running +=,
    }
    }
    }
    SEC("fexit/XXX")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_XXX) -> c_int {
    int BPF_PROG(fexit_XXX)
    {
    pub readings: [bpf_perf_event_value___local; MAX_NUM_METRICS],
    pub bpf_get_smp_processor_id(): u32 cpu =,
    pub 0: u32 i, zero =,
    pub err: c_int,
    pub count: *mut u64,
// read all events before updating the maps, to reduce error
    pub {: for (i = 0; i < num_metric && i < MAX_NUM_METRICS; i++),
    err = bpf_perf_event_read_value(&events, cpu + i * num_cpu,
    (void *)(readings + i),
    if (err)
    pub 0: return,
    }
    pub &zero): count = bpf_map_lookup_elem(&counts,,
    if (count) {
// count += 1;
    pub i++): for (i = 0; i < num_metric && i < MAX_NUM_METRICS;,
    pub &readings[i]): fexit_update_maps(i,,
    }
    pub 0: return,
    }
    pub BSD/GPL": char LICENSE[] SEC("license") = "Dual,
