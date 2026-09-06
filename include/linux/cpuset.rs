//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpuset.h
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
// cpuset interface
//
// Copyright (C) 2003 BULL SA
// Copyright (C) 2004-2006 Silicon Graphics, Inc.
//

extern "C" {
    pub fn lockdep_is_cpuset_held() -> bool;
}

//
// Static branch rewrites can happen in an arbitrary order for a given
// key. In code paths where we need to loop with read_mems_allowed_begin() and
// read_mems_allowed_retry() to get a consistent view of mems_allowed, we need
// to ensure that begin() always gets rewritten before retry() in the
// disabled -> enabled transition. If not, then if local irqs are disabled
// around the loop, we can deadlock since retry() would always be
// comparing the latest value of the mems_allowed seqcount against 0 as
// begin() still would see cpusets_enabled() as false. The enabled -> disabled
// transition should happen in reverse order for the same reasons (want to stop
// looking at real value of mems_allowed.sequence in retry() first).
//
extern "C" {
    pub fn static_branch_unlikely(_arg: &cpusets_enabled_key) -> return;
}
//
// This will get enabled whenever a cpuset configuration is considered
// unsupportable in general. E.g. movable only node which cannot satisfy
// any non movable allocations (see update_nodemask). Page allocator
// needs to make additional checks for those configurations and this
// check is meant to guard those checks without any overhead for sane
// configurations.
//
extern "C" {
    pub fn static_branch_unlikely(_arg: &cpusets_insane_config_key) -> return;
}
extern "C" {
    pub fn cpuset_init() -> c_int;
}
extern "C" {
    pub fn cpuset_init_smp();
}
extern "C" {
    pub fn cpuset_force_rebuild();
}
extern "C" {
    pub fn cpuset_update_active_cpus();
}
extern "C" {
    pub fn inc_dl_tasks_cs(task: *mut task_struct);
}
extern "C" {
    pub fn dec_dl_tasks_cs(task: *mut task_struct);
}
extern "C" {
    pub fn cpuset_lock();
}
extern "C" {
    pub fn cpuset_unlock();
}
extern "C" {
    pub fn lockdep_assert_cpuset_lock_held();
}
extern "C" {
    pub fn cpuset_cpus_allowed_locked(p: *mut task_struct, mask: *mut cpumask);
}
extern "C" {
    pub fn cpuset_cpus_allowed(p: *mut task_struct, mask: *mut cpumask);
}
extern "C" {
    pub fn cpuset_cpus_allowed_fallback(p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn cpuset_num_cpus(cgroup: *mut cgroup) -> c_int;
}
extern "C" {
    pub fn cpuset_mems_allowed(p: *mut task_struct) -> nodemask_t;
}

extern "C" {
    pub fn cpuset_init_current_mems_allowed();
}
extern "C" {
    pub fn cpuset_nodemask_valid_mems_allowed(nodemask: *const nodemask_t) -> c_int;
}
extern "C" {
    pub fn cpuset_current_node_allowed(node: c_int, gfp_mask: gfp_t) -> bool;
}
extern "C" {
    pub fn cpuset_current_node_allowed(_arg: zone_to_nid(z), _arg: gfp_mask) -> return;
}
extern "C" {
    pub fn __cpuset_zone_allowed(_arg: z, _arg: gfp_mask) -> return;
}

extern "C" {
    pub fn __cpuset_memory_pressure_bump();
}

extern "C" {
    pub fn cpuset_mem_spread_node() -> c_int;
}
extern "C" {
    pub fn task_spread_page(_arg: current) -> return;
}
extern "C" {
    pub fn current_cpuset_is_being_rebound() -> bool;
}
extern "C" {
    pub fn dl_rebuild_rd_accounting();
}
extern "C" {
    pub fn rebuild_sched_domains();
}
extern "C" {
    pub fn cpuset_print_current_mems_allowed();
}
extern "C" {
    pub fn cpuset_reset_sched_domains();
}
//
// read_mems_allowed_begin is required when making decisions involving
// mems_allowed such as during page allocation. mems_allowed can be updated in
// parallel and depending on the new value an operation can fail potentially
// causing process failure. A retry loop with read_mems_allowed_begin and
// read_mems_allowed_retry prevents these artificial failures.
//
extern "C" {
    pub fn read_seqcount_begin(_arg: &current->mems_allowed_seq) -> return;
}
//
// If this returns true, the operation that took place after
// read_mems_allowed_begin may have failed artificially due to a concurrent
// update of mems_allowed. It is up to the caller to retry the operation if
// appropriate.
//
extern "C" {
    pub fn read_seqcount_retry(_arg: &current->mems_allowed_seq, _arg: seq) -> return;
}
extern "C" {
    pub fn cpuset_nodes_allowed(cgroup: *mut cgroup, mask: *mut nodemask_t);
}

extern "C" {
    pub fn num_online_cpus() -> return;
}

