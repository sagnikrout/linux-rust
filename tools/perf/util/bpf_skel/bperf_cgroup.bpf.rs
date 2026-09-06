//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/bperf_cgroup.bpf.c
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
// Copyright (c) 2021 Google

// NOTE: many of map and global data will be modified before loading
// from the userspace (perf tool) using the skeleton helpers.
// single set of global perf events to measure
    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(int));
    __uint(max_entries, 1);
    } events SEC(".maps");
// from cgroup id to event index
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u64));
    __uint(value_size, sizeof(__u32));
    __uint(max_entries, 1);
    } cgrp_idx SEC(".maps");
// per-cpu event snapshots to calculate delta
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_perf_event_value));
    } prev_readings SEC(".maps");
// aggregated event values for each cgroup (per-cpu)
// will be read from the user-space
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct bpf_perf_event_value));
    } cgrp_readings SEC(".maps");
// new kernel cgroup definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup___new {
    pub level: c_int,
    pub ancestors: [*mut cgroup; ],
    pub __attribute__((preserve_access_index)): },
// old kernel cgroup definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup___old {
    pub level: c_int,
    pub ancestor_ids: [u64; ],
    pub __attribute__((preserve_access_index)): },
    pub 1: volatile __u32 num_events =,
    pub 1: volatile __u32 num_cpus =,
    pub 0: volatile int use_cgroup_v2 =,
    pub 0: int enabled =,
    pub -1: int perf_subsys_id =,
#[no_mangle]
pub unsafe extern "C" fn get_cgroup_v1_ancestor_id(cgrp: *mut cgroup, level: c_int) -> __u64 {
    static inline __u64 get_cgroup_v1_ancestor_id(struct cgroup *cgrp, int level)
    {
// recast pointer to capture new type for compiler
    pub )cgrp: *mut *mut cgroup___new cgrp_new = (void,
    if (bpf_core_field_exists(cgrp_new.ancestors)) {
    pub id): return BPF_CORE_READ(cgrp_new, ancestors[level], kn,,
    } else {
// recast pointer to capture old type for compiler
    pub )cgrp: *mut *mut cgroup___old cgrp_old = (void,
    pub ancestor_ids[level]): return BPF_CORE_READ(cgrp_old,,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_cgroup_v1_idx(cgrps: *mut __u32, size: c_int) -> c_int {
    static inline int get_cgroup_v1_idx(__u32 *cgrps, int size)
    {
    pub )bpf_get_current_task(): *mut *mut task_p = (void,
    pub cgrp: *mut cgroup,
    pub 0: register int i =,
    pub elem: *mut __u32,
    pub level: c_int,
    pub cnt: c_int,
    if (perf_subsys_id == -1) {

    perf_subsys_id = bpf_core_enum_value(enum cgroup_subsys_id,

    pub perf_event_cgrp_id: perf_subsys_id =,

    }
    pub cgroup): cgrp = BPF_CORE_READ(p, cgroups, subsys[perf_subsys_id],,
    pub level): level = BPF_CORE_READ(cgrp,,
    pub {: for (cnt = 0; i < BPERF_CGROUP__MAX_LEVELS; i++),
    pub cgrp_id: __u64,
    if (i > level)
// convert cgroup-id to a map index
    pub i): cgrp_id = get_cgroup_v1_ancestor_id(cgrp,,
    pub &cgrp_id): elem = bpf_map_lookup_elem(&cgrp_idx,,
    if (!elem)
    pub elem: *mut cgrps[cnt++] =,
    if (cnt == size)
    }
    pub cnt: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_cgroup_v2_idx(cgrps: *mut __u32, size: c_int) -> c_int {
    static inline int get_cgroup_v2_idx(__u32 *cgrps, int size)
    {
    pub 0: register int i =,
    pub elem: *mut __u32,
    pub cnt: c_int,
    pub {: for (cnt = 0; i < BPERF_CGROUP__MAX_LEVELS; i++),
    pub bpf_get_current_ancestor_cgroup_id(i): __u64 cgrp_id =,
    if (cgrp_id == 0)
// convert cgroup-id to a map index
    pub &cgrp_id): elem = bpf_map_lookup_elem(&cgrp_idx,,
    if (!elem)
    pub elem: *mut cgrps[cnt++] =,
    if (cnt == size)
    }
    pub cnt: return,
    }
#[no_mangle]
unsafe extern "C" fn bperf_cgroup_count() -> c_int {
    static int bperf_cgroup_count(void)
    {
    pub verifier: register __u32 idx = 0; // to have it in a register to pass BPF,
    pub 0: register int c =,
    pub cgrp_val: *mut *mut bpf_perf_event_value val, delta, prev_val,,
    pub bpf_get_smp_processor_id(): __u32 cpu =,
    pub cgrp_idx: [__u32; BPERF_CGROUP__MAX_LEVELS],
    pub cgrp_cnt: c_int,
    pub cgrp: __u32 key,,
    pub err: c_long,
    if (use_cgroup_v2)
    pub BPERF_CGROUP__MAX_LEVELS): cgrp_cnt = get_cgroup_v2_idx(cgrp_idx,,
    else
    pub BPERF_CGROUP__MAX_LEVELS): cgrp_cnt = get_cgroup_v1_idx(cgrp_idx,,
    pub {: for ( ; idx < BPERF_CGROUP__MAX_EVENTS; idx++),
    if (idx == num_events)
// XXX: do not pass idx directly (for verifier)
    pub idx: key =,
// this is per-cpu array for diff
    pub &key): prev_val = bpf_map_lookup_elem(&prev_readings,,
    if (!prev_val) {
    pub 0: val.counter = val.enabled = val.running =,
    pub BPF_ANY): bpf_map_update_elem(&prev_readings, &key, &val,,
    pub &key): prev_val = bpf_map_lookup_elem(&prev_readings,,
    if (!prev_val)
    }
// read from global perf_event array
    pub cpu: *mut *mut key = idx  num_cpus +,
    pub sizeof(val)): err = bpf_perf_event_read_value(&events, key, &val,,
    if (err)
    if (enabled) {
    pub prev_val->counter: delta.counter = val.counter -,
    pub prev_val->enabled: delta.enabled = val.enabled -,
    pub prev_val->running: delta.running = val.running -,
    pub {: for (c = 0; c < BPERF_CGROUP__MAX_LEVELS; c++),
    if (c == cgrp_cnt)
    pub cgrp_idx: [cgrp =; c],
// aggregate the result by cgroup
    pub idx: *mut *mut key = cgrp  num_events +,
    pub &key): cgrp_val = bpf_map_lookup_elem(&cgrp_readings,,
    if (cgrp_val) {
    pub delta.counter: cgrp_val->counter +=,
    pub delta.enabled: cgrp_val->enabled +=,
    pub delta.running: cgrp_val->running +=,
    } else {
    bpf_map_update_elem(&cgrp_readings, &key,
    pub BPF_ANY): &delta,,
    }
    }
    }
// prev_val = val;
    }
    pub 0: return,
    }
// This will be attached to cgroup-switches event for each cpu
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_cgrp_switch) -> c_int {
    int BPF_PROG(on_cgrp_switch)
    {
    pub bperf_cgroup_count(): return,
    }
    SEC("raw_tp/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trigger_read) -> c_int {
    int BPF_PROG(trigger_read)
    {
    pub bperf_cgroup_count(): return,
    }
    pub BSD/GPL": char LICENSE[] SEC("license") = "Dual,
