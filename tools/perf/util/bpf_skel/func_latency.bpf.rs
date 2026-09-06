//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/func_latency.bpf.c
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
// Copyright (c) 2021 Google

// This should be in sync with "util/ftrace.h"
pub const NUM_BUCKET: c_int = 22;
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u64));
    __uint(value_size, sizeof(__u64));
    __uint(max_entries, 10000);
    } functime SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } cpu_filter SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } task_filter SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u64));
    __uint(max_entries, NUM_BUCKET);
    } latency SEC(".maps");
    let mut enabled: c_int = 0;
// stats
    __s64 total;
    __s64 count;
    __s64 max;
    __s64 min;
    let mut has_cpu: volatile int = 0;
    let mut has_task: volatile int = 0;
    let mut use_nsec: volatile int = 0;
    const volatile unsigned int bucket_range;
    const volatile unsigned int min_latency;
    const volatile unsigned int max_latency;
    let mut bucket_num: volatile unsigned int = NUM_BUCKET;
#[no_mangle]
unsafe extern "C" fn can_record() -> bool {
    static bool can_record(void)
    {
    if (has_cpu) {
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    __u8 *ok;
    ok = bpf_map_lookup_elem(&cpu_filter, &cpu);
    if (!ok)
    return false;
    }
    if (has_task) {
    let mut pid: __u32 = bpf_get_current_pid_tgid();
    __u8 *ok;
    ok = bpf_map_lookup_elem(&task_filter, &pid);
    if (!ok)
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn update_latency(delta: __s64) {
    static void update_latency(__s64 delta)
    {
    let mut val: __u64 = delta;
    let mut key: __u32 = 0;
    __u64 *hist;
    let mut cmp_base: __u64 = use_nsec ? 1 : 1000;
    if (delta < 0)
    return;
    if (bucket_range != 0) {
    val = delta / cmp_base;
    if (min_latency > 0) {
    if (val > min_latency)
    val -= min_latency;
    else
    goto do_lookup;
    }
// Less than 1 unit (ms or ns), or, in the future,
// than the min latency desired.
    if (val > 0) { // 1st entry: [ 1 unit .. bucket_range units )
    key = val / bucket_range + 1;
    if (key >= bucket_num)
    key = bucket_num - 1;
    }
    goto do_lookup;
    }
// calculate index using delta
    for (key = 0; key < (bucket_num - 1); key++) {
    if (delta < (cmp_base << key))
    break;
    }
    do_lookup:
    hist = bpf_map_lookup_elem(&latency, &key);
    if (!hist)
    return;
    __sync_fetch_and_add(hist, 1);
    __sync_fetch_and_add(&total, delta); // always in nsec
    __sync_fetch_and_add(&count, 1);
    if (delta > max)
    max = delta;
    if (delta < min)
    min = delta;
    }
    SEC("kprobe/func")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: func_begin) -> c_int {
    int BPF_PROG(func_begin)
    {
    __u64 key, now;
    if (!enabled || !can_record())
    return 0;
    key = bpf_get_current_pid_tgid();
    now = bpf_ktime_get_ns();
// overwrite timestamp for nested functions
    bpf_map_update_elem(&functime, &key, &now, BPF_ANY);
    return 0;
    }
    SEC("kretprobe/func")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: func_end) -> c_int {
    int BPF_PROG(func_end)
    {
    __u64 tid;
    __u64 *start;
    if (!enabled)
    return 0;
    tid = bpf_get_current_pid_tgid();
    start = bpf_map_lookup_elem(&functime, &tid);
    if (start) {
    update_latency(bpf_ktime_get_ns() - *start);
    bpf_map_delete_elem(&functime, &tid);
    }
    return 0;
    }
    SEC("raw_tp")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: event_begin) -> c_int {
    int BPF_PROG(event_begin)
    {
    __u64 key, now;
    if (!enabled || !can_record())
    return 0;
    key = bpf_get_current_pid_tgid();
    now = bpf_ktime_get_ns();
// overwrite timestamp for nested events
    bpf_map_update_elem(&functime, &key, &now, BPF_ANY);
    return 0;
    }
    SEC("raw_tp")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: event_end) -> c_int {
    int BPF_PROG(event_end)
    {
    __u64 tid;
    __u64 *start;
    if (!enabled)
    return 0;
    tid = bpf_get_current_pid_tgid();
    start = bpf_map_lookup_elem(&functime, &tid);
    if (start) {
    update_latency(bpf_ktime_get_ns() - *start);
    bpf_map_delete_elem(&functime, &tid);
    }
    return 0;
    }
