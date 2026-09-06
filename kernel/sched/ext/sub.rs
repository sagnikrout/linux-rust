//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/ext/sub.h
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
// BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
//
// Sub-scheduler hierarchy support.
//
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2026 Tejun Heo <tj@kernel.org>
//

extern "C" {
    pub fn scx_set_task_sched(p: *mut task_struct, sch: *mut scx_sched);
}
extern "C" {
    pub fn set_cgroup_sched(cgrp: *mut cgroup, sch: *mut scx_sched);
}
extern "C" {
    pub fn scx_pstack_recursion_on_dispatch(prog: *mut bpf_prog);
}
extern "C" {
    pub fn scx_pstack_recursion_on_caps_updated(prog: *mut bpf_prog);
}
extern "C" {
    pub fn drain_descendants(sch: *mut scx_sched);
}
extern "C" {
    pub fn scx_sub_disable(sch: *mut scx_sched);
}
extern "C" {
    pub fn scx_sub_enable_workfn(work: *mut kthread_work);
}
extern "C" {
    pub fn scx_bpf_sub_dispatch(cgroup_id: u64, aux: *const bpf_prog_aux) -> bool;
}
extern "C" {
    pub fn scx_free_pshards(sch: *mut scx_sched);
}
extern "C" {
    pub fn scx_alloc_pshards(sch: *mut scx_sched) -> i32;
}
extern "C" {
    pub fn scx_init_root_caps(sch: *mut scx_sched);
}
extern "C" {
    pub fn scx_process_sync_ecaps(rq: *mut rq, prev: *mut task_struct);
}
extern "C" {
    pub fn scx_unbypass_replay_ecaps(rq: *mut rq, sch: *mut scx_sched);
}
extern "C" {
    pub fn scx_online_ecaps(rq: *mut rq);
}
extern "C" {
    pub fn scx_offline_ecaps(rq: *mut rq);
}
extern "C" {
    pub fn scx_discard_ecaps_to_sync(cpu: i32, pcpu: *mut scx_sched_pcpu);
}
extern "C" {
    pub fn scx_discard_stale_ecaps_syncs();
}
extern "C" {
    pub fn scx_task_reenq_on_cap_revoke(rq: *mut rq, p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn scx_reenq_reject(rq: *mut rq);
}
extern "C" {
    pub fn scx_rescue_charge(rq: *mut rq, delta_exec: i64);
}
extern "C" {
    pub fn scx_rescue_end(rq: *mut rq);
}
extern "C" {
    pub fn scx_rescue_keep(rq: *mut rq, p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn scx_rescue_flush(rq: *mut rq);
}
extern "C" {
    pub fn scx_rescue_dump(s: *mut seq_buf, rq: *mut rq);
}
extern "C" {
    pub fn scx_rescue_set_knobs(sch: *mut scx_sched);
}
extern "C" {
    pub fn scx_rescue_init(rq: *mut rq);
}
//
// cgrp->scx_sched is written by root/sub enable/disable under all of
// scx_enable_mutex, scx_fork_rwsem and cgroup_mutex. A new cgroup inherits the
// parent's sched under just cgroup_mutex but is not yet reachable by the other
// two lock holders. Any one of the three locks stabilizes the association.
//
// a dying sub's hot-path influence ends in scx_sched_free_rcu_work()

//
// scx_for_each_descendant_pre - pre-order walk of a sched's descendants
// @pos: iteration cursor
// @root: sched to walk the descendants of
//
// Walk @root's descendants. @root is included in the iteration and the first
// node to be visited. Must be called with scx_enable_mutex, scx_sched_lock, or
// RCU read lock.
//

//
// scx_missing_caps - The caps in @needed that @sch lacks on @cpu
// @sch: sched to test
// @cpu: cpu to test on
// @needed: bitmask of SCX_CAP_* values
//
// Return the caps in @needed that @sch lacks for @cpu, 0 if it holds them all.
//
// no sub-scheds, no missing caps
// root holds every cap on every cpu
//
// Cap semantics: which caps an action requires, and which caps a cap implies.
// Keep all such mappings collected here.
//
// map @enq_flags to the SCX_CAP_* bit required for the local-DSQ insert
// a restored task must be put into the local DSQ regardless of caps
// map queued @p to the SCX_CAP_* bit required to stay on its local DSQ
// the cap @sch needs to preempt @rq's current task, 0 if none
// a kernel-forced placement preempts regardless of caps
// a non-ext task can't be preempted by ext, own-subtree needs no cap
// caps implied by holding @cap
// may @p keep running on @rq's cpu? requires baseline cpu access
// a migration-disabled task is let in without caps, keep it likewise
extern "C" {
    pub fn likely(_arg: !scx_missing_caps(scx_task_sched(p), _arg: cpu_of(rq), _arg: SCX_CAP_BASE)) -> return;
}
// the task admitted for rescue on @rq, NULL if none

