//! Automatically rewritten from C to Rust
//! Source: tools/tracing/rtla/src/timerlat.bpf.c
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

pub const MAX_ENTRIES_DEFAULT: c_int = 4096;
    char LICENSE[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_raw_timerlat_sample {
    pub timer_latency: c_ulonglong,
    pub context: c_int,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub MAX_ENTRIES_DEFAULT): __uint(max_entries,,
    pub int): __type(key, unsigned,
    pub long): __type(value, unsigned long,
    pub SEC(".maps"): } hist_irq SEC(".maps"), hist_thread SEC(".maps"), hist_user,
    struct {
    pub BPF_MAP_TYPE_PERCPU_ARRAY): __uint(type,,
    pub SUMMARY_FIELD_N): __uint(max_entries,,
    pub int): __type(key, unsigned,
    pub long): __type(value, unsigned long,
    pub SEC(".maps"): } summary_irq SEC(".maps"), summary_thread SEC(".maps"), summary_user,
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key, unsigned,
    pub long): __type(value, unsigned long,
    pub SEC(".maps"): } stop_tracing,
    struct {
    pub BPF_MAP_TYPE_RINGBUF): __uint(type,,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } signal_stop_tracing,
    struct {
    pub BPF_MAP_TYPE_PROG_ARRAY): __uint(type,,
    pub int)): __uint(key_size, sizeof(unsigned,
    pub 1): __uint(max_entries,,
    pub )): *mut __array(values, unsigned int (void,
    } bpf_action SEC(".maps") = {
    .values = {
    [0] = 0
    },
}

// Params to be set by rtla
    let mut bucket_size: volatile int = 1;
    let mut output_divisor: volatile int = 1000;
    let mut entries: volatile int = 256;
    const volatile int irq_threshold;
    const volatile int thread_threshold;
    const volatile bool aa_only;
    nosubprog unsigned long long map_get(void *map,
    unsigned int key)
    {
    unsigned long long *value_ptr;
    value_ptr = bpf_map_lookup_elem(map, &key);
    return !value_ptr ? 0 : *value_ptr;
    }
    nosubprog void map_set(void *map,
    unsigned int key,
    unsigned long long value)
    {
    bpf_map_update_elem(map, &key, &value, BPF_ANY);
    }
    nosubprog void map_increment(void *map,
    unsigned int key)
    {
    map_set(map, key, map_get(map, key) + 1);
    }
    nosubprog void update_main_hist(void *map,
    int bucket)
    {
    if (entries == 0)
// No histogram
    return;
    if (bucket >= entries)
// Overflow
    return;
    map_increment(map, bucket);
    }
    nosubprog void update_summary(void *map,
    unsigned long long latency,
    int bucket)
    {
    if (aa_only)
// Auto-analysis only, nothing to be done here
    return;
    map_set(map, SUMMARY_CURRENT, latency);
    if (bucket >= entries)
// Overflow
    map_increment(map, SUMMARY_OVERFLOW);
    if (latency > map_get(map, SUMMARY_MAX))
    map_set(map, SUMMARY_MAX, latency);
    if (latency < map_get(map, SUMMARY_MIN) || map_get(map, SUMMARY_COUNT) == 0)
    map_set(map, SUMMARY_MIN, latency);
    map_increment(map, SUMMARY_COUNT);
    map_set(map, SUMMARY_SUM, map_get(map, SUMMARY_SUM) + latency);
    }
#[no_mangle]
pub unsafe extern "C" fn set_stop_tracing(tp_args: *mut trace_event_raw_timerlat_sample) -> nosubprog void {
    nosubprog void set_stop_tracing(struct trace_event_raw_timerlat_sample *tp_args)
    {
    let mut value: c_int = 0;
// Suppress further sample processing
    map_set(&stop_tracing, 0, 1);
// Signal to userspace
    bpf_ringbuf_output(&signal_stop_tracing, &value, sizeof(value), 0);
//
// Call into BPF action program, if attached.
// Otherwise, just silently fail.
//
    bpf_tail_call(tp_args, &bpf_action, 0);
    }
    SEC("tp/osnoise/timerlat_sample")
#[no_mangle]
pub unsafe extern "C" fn handle_timerlat_sample(tp_args: *mut trace_event_raw_timerlat_sample) -> c_int {
    int handle_timerlat_sample(struct trace_event_raw_timerlat_sample *tp_args)
    {
    unsigned long long latency, latency_us;
    int bucket;
    if (map_get(&stop_tracing, 0))
    return 0;
    latency = tp_args.timer_latency / output_divisor;
    latency_us = tp_args.timer_latency / 1000;
    bucket = latency / bucket_size;
    if (tp_args.context == 0) {
    update_main_hist(&hist_irq, bucket);
    update_summary(&summary_irq, latency, bucket);
    if (irq_threshold != 0 && latency_us >= irq_threshold)
    set_stop_tracing(tp_args);
    } else if (tp_args.context == 1) {
    update_main_hist(&hist_thread, bucket);
    update_summary(&summary_thread, latency, bucket);
    if (thread_threshold != 0 && latency_us >= thread_threshold)
    set_stop_tracing(tp_args);
    } else {
    update_main_hist(&hist_user, bucket);
    update_summary(&summary_user, latency, bucket);
    if (thread_threshold != 0 && latency_us >= thread_threshold)
    set_stop_tracing(tp_args);
    }
    return 0;
    }
