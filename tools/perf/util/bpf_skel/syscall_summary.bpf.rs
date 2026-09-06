//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/syscall_summary.bpf.c
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
//
// Trace raw_syscalls tracepoints to collect system call statistics.
//

// This is to calculate a delta between sys-enter and sys-exit for each thread
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_trace {
    pub /: *mut *mut int nr; / syscall number is only available at sys-enter,
    pub unused: c_int,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_trace_map {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub /: *mut *mut __type(key, int); / tid,
    pub syscall_trace): __type(value, struct,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub SEC(".maps"): } syscall_trace_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_stats_map {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub syscall_key): __type(key, struct,
    pub syscall_stats): __type(value, struct,
    pub MAX_ENTRIES): __uint(max_entries,,
    pub SEC(".maps"): } syscall_stats_map,
    pub /: *mut *mut int enabled; / controlled from userspace,
    pub aggr_mode: volatile enum syscall_aggr_mode,
    pub use_cgroup_v2: volatile int,
    pub -1: int perf_subsys_id =,
#[no_mangle]
pub unsafe extern "C" fn get_current_cgroup_id() -> __u64 {
    static inline __u64 get_current_cgroup_id(void)
    {
    pub task: *mut task_struct,
    pub cgrp: *mut cgroup,
    if (use_cgroup_v2)
    pub bpf_get_current_cgroup_id(): return,
    pub bpf_get_current_task_btf(): task =,
    if (perf_subsys_id == -1) {

    perf_subsys_id = bpf_core_enum_value(enum cgroup_subsys_id,

    pub perf_event_cgrp_id: perf_subsys_id =,

    }
    pub cgroup): cgrp = BPF_CORE_READ(task, cgroups, subsys[perf_subsys_id],,
    pub id): return BPF_CORE_READ(cgrp, kn,,
    }
    static void update_stats(int cpu_or_tid, u64 cgroup_id, int nr, s64 duration,
    long ret)
    {
    struct syscall_key key = {
    .cpu_or_tid = cpu_or_tid,
    .cgroup = cgroup_id,
    .nr = nr,
}

    struct syscall_stats *stats;
    stats = bpf_map_lookup_elem(&syscall_stats_map, &key);
    if (stats == core::ptr::null_mut()) {
    let mut zero: syscall_stats = {};
    bpf_map_update_elem(&syscall_stats_map, &key, &zero, BPF_NOEXIST);
    stats = bpf_map_lookup_elem(&syscall_stats_map, &key);
    if (stats == core::ptr::null_mut())
    return;
    }
    __sync_fetch_and_add(&stats.count, 1);
    if (ret < 0)
    __sync_fetch_and_add(&stats.error, 1);
    if (duration > 0) {
    __sync_fetch_and_add(&stats.total_time, duration);
    __sync_fetch_and_add(&stats.squared_sum, duration * duration);
    if (stats.max_time < duration)
    stats.max_time = duration;
    if (stats.min_time > duration || stats.min_time == 0)
    stats.min_time = duration;
    }
    return;
    }
    SEC("tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn sys_enter(ctx: *mut u64) -> c_int {
    int sys_enter(u64 *ctx)
    {
    int tid;
    struct syscall_trace st;
    if (!enabled)
    return 0;
    st.nr = ctx[1]; /* syscall number */
    st.unused = 0;
    st.timestamp = bpf_ktime_get_ns();
    tid = bpf_get_current_pid_tgid();
    bpf_map_update_elem(&syscall_trace_map, &tid, &st, BPF_ANY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_exit(ret: c_long) -> c_int {
    static int do_exit(long ret)
    {
    int tid;
    let mut key: c_int = 0;
    let mut cgroup: u64 = 0;
    struct syscall_trace *st;
    s64 delta;
    if (!enabled)
    return 0;
    tid = bpf_get_current_pid_tgid();
    st = bpf_map_lookup_elem(&syscall_trace_map, &tid);
    if (st == core::ptr::null_mut())
    return 0;
    if (aggr_mode == SYSCALL_AGGR_THREAD)
    key = tid;
#[no_mangle]
pub unsafe extern "C" fn if(SYSCALL_AGGR_CGROUP: aggr_mode ==) -> else {
    else if (aggr_mode == SYSCALL_AGGR_CGROUP)
    cgroup = get_current_cgroup_id();
    else
    key = bpf_get_smp_processor_id();
    delta = bpf_ktime_get_ns() - st.timestamp;
    update_stats(key, cgroup, st.nr, delta, ret);
    bpf_map_delete_elem(&syscall_trace_map, &tid);
    return 0;
    }
    SEC("tp_btf/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn sys_exit(ctx: *mut u64) -> c_int {
    int sys_exit(u64 *ctx)
    {
    long ret = ctx[1]; /* return value of the syscall */
    return do_exit(ret);
    }
    SEC("tp_btf/sched_process_exit")
#[no_mangle]
pub unsafe extern "C" fn process_exit(ctx: *mut u64) -> c_int {
    int process_exit(u64 *ctx)
    {
    return do_exit(0);
    }
    char _license[] SEC("license") = "GPL";
