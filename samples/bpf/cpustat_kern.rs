//! Automatically rewritten from C to Rust
//! Source: samples/bpf/cpustat_kern.c
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
// The CPU number, cstate number and pstate number are based
// on 96boards Hikey with octa CA53 CPUs.
//
// Every CPU have three idle states for cstate:
// WFI, CPU_OFF, CLUSTER_OFF
//
// Every CPU have 5 operating points:
// 208MHz, 432MHz, 729MHz, 960MHz, 1200MHz
//
// This code is based on these assumption and other platforms
// need to adjust these definitions.
//
pub const MAX_CPU: c_int = 8;
pub const MAX_PSTATE_ENTRIES: c_int = 5;
pub const MAX_CSTATE_ENTRIES: c_int = 3;
    static int cpu_opps[] = { 208000, 432000, 729000, 960000, 1200000 };
//
// my_map structure is used to record cstate and pstate index and
// timestamp (Idx, Ts), when new event incoming we need to update
// combination for new state index and timestamp (Idx`, Ts`).
//
// Based on (Idx, Ts) and (Idx`, Ts`) we can calculate the time
// interval for the previous state: Duration(Idx) = Ts` - Ts.
//
// Every CPU has one below array for recording state index and
// timestamp, and record for cstate and pstate saperately:
//
// +--------------------------+
// | cstate timestamp         |
// +--------------------------+
// | cstate index             |
// +--------------------------+
// | pstate timestamp         |
// +--------------------------+
// | pstate index             |
// +--------------------------+
//
pub const MAP_OFF_CSTATE_TIME: c_int = 0;
pub const MAP_OFF_CSTATE_IDX: c_int = 1;
pub const MAP_OFF_PSTATE_TIME: c_int = 2;
pub const MAP_OFF_PSTATE_IDX: c_int = 3;
pub const MAP_OFF_NUM: c_int = 4;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __type(value, u64);
    __uint(max_entries, MAX_CPU * MAP_OFF_NUM);
    } my_map SEC(".maps");
// cstate_duration records duration time for every idle state per CPU
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __type(value, u64);
    __uint(max_entries, MAX_CPU * MAX_CSTATE_ENTRIES);
    } cstate_duration SEC(".maps");
// pstate_duration records duration time for every operating point per CPU
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __type(value, u64);
    __uint(max_entries, MAX_CPU * MAX_PSTATE_ENTRIES);
    } pstate_duration SEC(".maps");
//
// The trace events for cpu_idle and cpu_frequency are taken from:
// /sys/kernel/tracing/events/power/cpu_idle/format
// /sys/kernel/tracing/events/power/cpu_frequency/format
//
// These two events have same format, so define one common structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_args {
    pub pad: u64,
    pub state: u32,
    pub cpu_id: u32,
}

// calculate pstate index, returns MAX_PSTATE_ENTRIES for failure
#[no_mangle]
unsafe extern "C" fn find_cpu_pstate_idx(frequency: u32) -> u32 {
    static u32 find_cpu_pstate_idx(u32 frequency)
    {
    u32 i;
    for (i = 0; i < sizeof(cpu_opps) / sizeof(u32); i++) {
    if (frequency == cpu_opps[i])
    return i;
    }
    return i;
    }
    SEC("tracepoint/power/cpu_idle")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut cpu_args) -> c_int {
    int bpf_prog1(struct cpu_args *ctx)
    {
    u64 *cts, *pts, *cstate, *pstate, prev_state, cur_ts, delta;
    u32 key, cpu, pstate_idx;
    u64 *val;
    if (ctx.cpu_id > MAX_CPU)
    return 0;
    cpu = ctx.cpu_id;
    key = cpu * MAP_OFF_NUM + MAP_OFF_CSTATE_TIME;
    cts = bpf_map_lookup_elem(&my_map, &key);
    if (!cts)
    return 0;
    key = cpu * MAP_OFF_NUM + MAP_OFF_CSTATE_IDX;
    cstate = bpf_map_lookup_elem(&my_map, &key);
    if (!cstate)
    return 0;
    key = cpu * MAP_OFF_NUM + MAP_OFF_PSTATE_TIME;
    pts = bpf_map_lookup_elem(&my_map, &key);
    if (!pts)
    return 0;
    key = cpu * MAP_OFF_NUM + MAP_OFF_PSTATE_IDX;
    pstate = bpf_map_lookup_elem(&my_map, &key);
    if (!pstate)
    return 0;
    prev_state = *cstate;
// cstate = ctx->state;
    if (!*cts) {
// cts = bpf_ktime_get_ns();
    return 0;
    }
    cur_ts = bpf_ktime_get_ns();
    delta = cur_ts - *cts;
// cts = cur_ts;
//
// When state doesn't equal to (u32)-1, the cpu will enter
// one idle state; for this case we need to record interval
// for the pstate.
//
// OPP2
// +---------------------+
// OPP1   |                     |
// ---------+                     |
// |  Idle state
// +---------------
//
// |<- pstate duration ->|
// ^                     ^
// pts                  cur_ts
//
    if (ctx.state != (u32)-1) {
// record pstate after have first cpu_frequency event
    if (!*pts)
    return 0;
    delta = cur_ts - *pts;
    pstate_idx = find_cpu_pstate_idx(*pstate);
    if (pstate_idx >= MAX_PSTATE_ENTRIES)
    return 0;
    key = cpu * MAX_PSTATE_ENTRIES + pstate_idx;
    val = bpf_map_lookup_elem(&pstate_duration, &key);
    if (val)
    __sync_fetch_and_add((long *)val, delta);
//
// When state equal to (u32)-1, the cpu just exits from one
// specific idle state; for this case we need to record
// interval for the pstate.
//
// OPP2
// -----------+
// |                          OPP1
// |                     +-----------
// |     Idle state      |
// +---------------------+
//
// |<- cstate duration ->|
// ^                     ^
// cts                  cur_ts
//
    } else {
    key = cpu * MAX_CSTATE_ENTRIES + prev_state;
    val = bpf_map_lookup_elem(&cstate_duration, &key);
    if (val)
    __sync_fetch_and_add((long *)val, delta);
    }
// Update timestamp for pstate as new start time
    if (*pts)
// pts = cur_ts;
    return 0;
    }
    SEC("tracepoint/power/cpu_frequency")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut cpu_args) -> c_int {
    int bpf_prog2(struct cpu_args *ctx)
    {
    u64 *pts, *cstate, *pstate, cur_ts, delta;
    u32 key, cpu, pstate_idx;
    u64 *val;
    cpu = ctx.cpu_id;
    key = cpu * MAP_OFF_NUM + MAP_OFF_PSTATE_TIME;
    pts = bpf_map_lookup_elem(&my_map, &key);
    if (!pts)
    return 0;
    key = cpu * MAP_OFF_NUM + MAP_OFF_PSTATE_IDX;
    pstate = bpf_map_lookup_elem(&my_map, &key);
    if (!pstate)
    return 0;
    key = cpu * MAP_OFF_NUM + MAP_OFF_CSTATE_IDX;
    cstate = bpf_map_lookup_elem(&my_map, &key);
    if (!cstate)
    return 0;
// pstate = ctx->state;
    if (!*pts) {
// pts = bpf_ktime_get_ns();
    return 0;
    }
    cur_ts = bpf_ktime_get_ns();
    delta = cur_ts - *pts;
// pts = cur_ts;
// When CPU is in idle, bail out to skip pstate statistics
    if (*cstate != (u32)(-1))
    return 0;
//
// The cpu changes to another different OPP (in below diagram
// change frequency from OPP3 to OPP1), need recording interval
// for previous frequency OPP3 and update timestamp as start
// time for new frequency OPP1.
//
// OPP3
// +---------------------+
// OPP2   |                     |
// ---------+                     |
// |    OPP1
// +---------------
//
// |<- pstate duration ->|
// ^                     ^
// pts                  cur_ts
//
    pstate_idx = find_cpu_pstate_idx(*pstate);
    if (pstate_idx >= MAX_PSTATE_ENTRIES)
    return 0;
    key = cpu * MAX_PSTATE_ENTRIES + pstate_idx;
    val = bpf_map_lookup_elem(&pstate_duration, &key);
    if (val)
    __sync_fetch_and_add((long *)val, delta);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
    u32 _version SEC("version") = LINUX_VERSION_CODE;
