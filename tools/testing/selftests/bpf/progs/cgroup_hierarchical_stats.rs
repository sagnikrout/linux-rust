//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgroup_hierarchical_stats.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2022 Google LLC.
//

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_attach_counter {
// Previous percpu state, to figure out if we have new updates
    pub prev: __u64,
// Current percpu state
    pub state: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct attach_counter {
// State propagated through children, pending aggregation
    pub pending: __u64,
// Total state, including all cpus and all children
    pub state: __u64,
}

    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(max_entries, 1024);
    __type(key, __u64);
    __type(value, struct percpu_attach_counter);
    } percpu_attach_counters SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, __u64);
    __type(value, struct attach_counter);
    } attach_counters SEC(".maps");
    extern void css_rstat_updated(
    struct cgroup_subsys_state *css, int cpu) __ksym;
    extern void css_rstat_flush(struct cgroup_subsys_state *css) __ksym;
#[no_mangle]
unsafe extern "C" fn cgroup_id(cgrp: *mut cgroup) -> u64 {
    static uint64_t cgroup_id(struct cgroup *cgrp)
    {
    return cgrp.kn.id;
    }
#[no_mangle]
unsafe extern "C" fn create_percpu_attach_counter(cg_id: __u64, state: __u64) -> c_int {
    static int create_percpu_attach_counter(__u64 cg_id, __u64 state)
    {
    let mut pcpu_init: percpu_attach_counter = {.state = state, .prev = 0};
    return bpf_map_update_elem(&percpu_attach_counters, &cg_id,
    &pcpu_init, BPF_NOEXIST);
    }
#[no_mangle]
unsafe extern "C" fn create_attach_counter(cg_id: __u64, state: __u64, pending: __u64) -> c_int {
    static int create_attach_counter(__u64 cg_id, __u64 state, __u64 pending)
    {
    let mut init: attach_counter = {.state = state, .pending = pending};
    return bpf_map_update_elem(&attach_counters, &cg_id,
    &init, BPF_NOEXIST);
    }
    SEC("tp_btf/cgroup_attach_task")
    int BPF_PROG(counter, struct cgroup *dst_cgrp, const char *path,
    struct task_struct *task, bool threadgroup)
    {
    let mut cg_id: __u64 = cgroup_id(dst_cgrp);
    struct percpu_attach_counter *pcpu_counter = bpf_map_lookup_elem(
    &percpu_attach_counters,
    &cg_id);
    if (pcpu_counter)
    pcpu_counter.state += 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: create_percpu_attach_counter(cg_id, _arg: 1)) -> else {
    else if (create_percpu_attach_counter(cg_id, 1))
    return 0;
    css_rstat_updated(&dst_cgrp.self, bpf_get_smp_processor_id());
    return 0;
    }
    SEC("fentry/bpf_rstat_flush")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: flusher, cgrp: *mut cgroup, parent: *mut cgroup, cpu: c_int) -> c_int {
    int BPF_PROG(flusher, struct cgroup *cgrp, struct cgroup *parent, int cpu)
    {
    struct percpu_attach_counter *pcpu_counter;
    struct attach_counter *total_counter, *parent_counter;
    let mut cg_id: __u64 = cgroup_id(cgrp);
    let mut parent_cg_id: __u64 = parent ? cgroup_id(parent) : 0;
    __u64 state;
    let mut delta: __u64 = 0;
// Add CPU changes on this level since the last flush
    pcpu_counter = bpf_map_lookup_percpu_elem(&percpu_attach_counters,
    &cg_id, cpu);
    if (pcpu_counter) {
    state = pcpu_counter.state;
    delta += state - pcpu_counter.prev;
    pcpu_counter.prev = state;
    }
    total_counter = bpf_map_lookup_elem(&attach_counters, &cg_id);
    if (!total_counter) {
    if (create_attach_counter(cg_id, delta, 0))
    return 0;
    goto update_parent;
    }
// Collect pending stats from subtree
    if (total_counter.pending) {
    delta += total_counter.pending;
    total_counter.pending = 0;
    }
// Propagate changes to this cgroup's total
    total_counter.state += delta;
    update_parent:
// Skip if there are no changes to propagate, or no parent
    if (!delta || !parent_cg_id)
    return 0;
// Propagate changes to cgroup's parent
    parent_counter = bpf_map_lookup_elem(&attach_counters,
    &parent_cg_id);
    if (parent_counter)
    parent_counter.pending += delta;
    else
    create_attach_counter(parent_cg_id, 0, delta);
    return 0;
    }
    SEC("iter.s/cgroup")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: dumper, meta: *mut bpf_iter_meta, cgrp: *mut cgroup) -> c_int {
    int BPF_PROG(dumper, struct bpf_iter_meta *meta, struct cgroup *cgrp)
    {
    struct seq_file *seq = meta.seq;
    struct attach_counter *total_counter;
    let mut cg_id: __u64 = cgrp ? cgroup_id(cgrp) : 0;
// Do nothing for the terminal call
    if (!cg_id)
    return 1;
// Flush the stats to make sure we get the most updated numbers
    css_rstat_flush(&cgrp.self);
    total_counter = bpf_map_lookup_elem(&attach_counters, &cg_id);
    if (!total_counter) {
    BPF_SEQ_PRINTF(seq, "cg_id: %llu, attach_counter: 0\n",
    cg_id);
    } else {
    BPF_SEQ_PRINTF(seq, "cg_id: %llu, attach_counter: %llu\n",
    cg_id, total_counter.state);
    }
    return 0;
    }
