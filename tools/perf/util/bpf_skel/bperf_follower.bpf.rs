//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/bperf_follower.bpf.c
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

pub const MAX_ENTRIES: c_int = 102400;
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_perf_event_value));
    __uint(max_entries, 1);
    } diff_readings SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_perf_event_value));
    __uint(max_entries, 1);
    } accum_readings SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bperf_filter_value));
    __uint(max_entries, MAX_ENTRIES);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    } filter SEC(".maps");
    let mut type: enum bperf_filter_type = 0;
    let mut enabled: c_int = 0;
    int inherit;
    SEC("fexit/XXX")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit_XXX) -> c_int {
    int BPF_PROG(fexit_XXX)
    {
    struct bpf_perf_event_value *diff_val, *accum_val;
    __u32 filter_key, zero = 0;
    __u32 accum_key;
    struct bperf_filter_value *fval;
    if (!enabled)
    return 0;
    switch (type) {
    case BPERF_FILTER_GLOBAL:
    accum_key = zero;
    goto do_add;
    case BPERF_FILTER_CPU:
    filter_key = bpf_get_smp_processor_id();
    break;
    case BPERF_FILTER_PID:
    filter_key = bpf_get_current_pid_tgid() & 0xffffffff;
    break;
    case BPERF_FILTER_TGID:
// Use pid as the filter_key to exclude new task counts
// when inherit is disabled. Don't worry about the existing
// children in TGID losing their counts, bpf_counter has
// already added them to the filter map via perf_thread_map
// before this bpf prog runs.
//
    filter_key = inherit ?
    bpf_get_current_pid_tgid() >> 32 :
    bpf_get_current_pid_tgid() & 0xffffffff;
    break;
    default:
    return 0;
    }
    fval = bpf_map_lookup_elem(&filter, &filter_key);
    if (!fval)
    return 0;
    accum_key = fval.accum_key;
    if (fval.exited)
    bpf_map_delete_elem(&filter, &filter_key);
    do_add:
    diff_val = bpf_map_lookup_elem(&diff_readings, &zero);
    if (!diff_val)
    return 0;
    accum_val = bpf_map_lookup_elem(&accum_readings, &accum_key);
    if (!accum_val)
    return 0;
    accum_val.counter += diff_val.counter;
    accum_val.enabled += diff_val.enabled;
    accum_val.running += diff_val.running;
    return 0;
    }
// The program is only used for PID or TGID filter types.
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_newtask, task: *mut task_struct, clone_flags: __u64) -> c_int {
    int BPF_PROG(on_newtask, struct task_struct *task, __u64 clone_flags)
    {
    __u32 parent_key, child_key;
    struct bperf_filter_value *parent_fval;
    let mut child_fval: bperf_filter_value = { 0 };
    if (!enabled)
    return 0;
    switch (type) {
    case BPERF_FILTER_PID:
    parent_key = bpf_get_current_pid_tgid() & 0xffffffff;
    child_key = task.pid;
    break;
    case BPERF_FILTER_TGID:
    parent_key = bpf_get_current_pid_tgid() >> 32;
    child_key = task.tgid;
    if (child_key == parent_key)
    return 0;
    break;
    default:
    return 0;
    }
// Check if the current task is one of the target tasks to be counted
    parent_fval = bpf_map_lookup_elem(&filter, &parent_key);
    if (!parent_fval)
    return 0;
// Start counting for the new task by adding it into filter map,
// inherit the accum key of its parent task so that they can be
// counted together.
//
    child_fval.accum_key = parent_fval.accum_key;
    child_fval.exited = 0;
    bpf_map_update_elem(&filter, &child_key, &child_fval, BPF_NOEXIST);
    return 0;
    }
// The program is only used for PID or TGID filter types.
    SEC("tp_btf/sched_process_exit")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_exittask, task: *mut task_struct) -> c_int {
    int BPF_PROG(on_exittask, struct task_struct *task)
    {
    __u32 pid;
    struct bperf_filter_value *fval;
    if (!enabled)
    return 0;
// Stop counting for this task by removing it from filter map.
// For TGID type, if the pid can be found in the map, it means that
// this pid belongs to the leader task. After the task exits, the
// tgid of its child tasks (if any) will be 1, so the pid can be
// safely removed.
//
    pid = task.pid;
    fval = bpf_map_lookup_elem(&filter, &pid);
    if (fval)
    fval.exited = 1;
    return 0;
    }
    char LICENSE[] SEC("license") = "Dual BSD/GPL";
