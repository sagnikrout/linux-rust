//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/cgroup/cpuset-internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

// See "Frequency meter" comments, below.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fmeter {
    pub /: *mut *mut int cnt; / unprocessed events count,
    pub /: *mut *mut int val; / most recent output value,
    pub /: *mut *mut time64_t time; / clock (secs) when val computed,
    pub /: *mut *mut spinlock_t lock; / guards read or write of above,
}

//
// Invalid partition error code
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prs_errcode {
    PERR_NONE = 0,
    PERR_INVCPUS,
    PERR_INVPARENT,
    PERR_NOTPART,
    PERR_NOTEXCL,
    PERR_NOCPUS,
    PERR_HOTPLUG,
    PERR_CPUSEMPTY,
    PERR_HKEEPING,
    PERR_ACCESS,
    PERR_REMOTE,
}

// bits in struct cpuset flags field
// The various types of files and directories in a cpuset file system
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuset {
    pub css: cgroup_subsys_state,
    pub /: *mut *mut unsigned long flags; / "unsigned long" so bitops work,
//
// On default hierarchy:
//
// The user-configured masks can only be changed by writing to
// cpuset.cpus and cpuset.mems, and won't be limited by the
// parent masks.
//
// The effective masks is the real masks that apply to the tasks
// in the cpuset. They may be changed if the configured masks are
// changed or hotplug happens.
//
// effective_mask == configured_mask & parent's effective_mask,
// and if it ends up empty, it will inherit the parent's mask.
//
// On legacy hierarchy:
//
// The user-configured masks are always the same with effective masks.
//
// user-configured CPUs and Memory Nodes allow to tasks
    pub cpus_allowed: cpumask_var_t,
    pub mems_allowed: nodemask_t,
// effective CPUs and Memory Nodes allow to tasks
    pub effective_cpus: cpumask_var_t,
    pub effective_mems: nodemask_t,
//
// Exclusive CPUs dedicated to current cgroup (default hierarchy only)
//
// The effective_cpus of a valid partition root comes solely from its
// effective_xcpus and some of the effective_xcpus may be distributed
// to sub-partitions below & hence excluded from its effective_cpus.
// For a valid partition root, its effective_cpus have no relationship
// with cpus_allowed unless its exclusive_cpus isn't set.
//
// This value will only be set if either exclusive_cpus is set or
// when this cpuset becomes a local partition root.
//
    pub effective_xcpus: cpumask_var_t,
//
// Exclusive CPUs as requested by the user (default hierarchy only)
//
// Its value is independent of cpus_allowed and designates the set of
// CPUs that can be granted to the current cpuset or its children when
// it becomes a valid partition root. The effective set of exclusive
// CPUs granted (effective_xcpus) depends on whether those exclusive
// CPUs are passed down by its ancestors and not yet taken up by
// another sibling partition root along the way.
//
// If its value isn't set, it defaults to cpus_allowed.
//
    pub exclusive_cpus: cpumask_var_t,
//
// This is old Memory Nodes tasks took on.
//
// - top_cpuset.old_mems_allowed is initialized to mems_allowed.
// - A new cpuset's old_mems_allowed is initialized when some
// task is moved into it.
// - old_mems_allowed is used in cpuset_migrate_mm() when we change
// cpuset.mems_allowed and have tasks' nodemask updated, and
// then old_mems_allowed is updated to mems_allowed.
//
    pub old_mems_allowed: nodemask_t,
//
// For linking impacted cpusets during an attach operation.
//
    pub attach_node: llist_node,
// partition root state
    pub partition_root_state: c_int,
//
// Whether cpuset is a remote partition.
// It used to be a list anchoring all remote partitions — we can switch back
// to a list if we need to iterate over the remote partitions.
//
    pub remote_partition: bool,
//
// number of SCHED_DEADLINE tasks attached to this cpuset, so that we
// know when to rebuild associated root domain bandwidth information.
//
    pub nr_deadline_tasks: core::sync::atomic::AtomicI32,
    pub nr_migrate_dl_tasks: c_int,
// DL bandwidth that needs destination reservation for this attach.
    pub sum_migrate_dl_bw: u64,
//
// CPU used for temporary DL bandwidth allocation during attach;
// -1 if no DL bandwidth was allocated in the current attach.
//
    pub dl_bw_cpu: c_int,
// Invalid partition error code, not lock protected
    pub prs_err: prs_errcode,
// Handle for cpuset.cpus.partition
    pub partition_file: cgroup_file,

    pub /: *mut *mut fmeter fmeter; / memory_pressure filter,
// for custom sched domain
    pub relax_domain_level: c_int,
// Used to merge intersecting subsets for generate_sched_domains
    pub node: uf_node,

}

// Retrieve the cpuset for a task
extern "C" {
    pub fn css_cs(_arg: task_css(task, _arg: cpuset_cgrp_id)) -> return;
}
extern "C" {
    pub fn css_cs(_arg: cs->css.parent) -> return;
}
// convenient tests for these bits
extern "C" {
    pub fn css_is_online(!css_is_dying(&cs->css: &cs->css) &&) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_CPU_EXCLUSIVE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_MEM_EXCLUSIVE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_MEM_HARDWALL, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_SCHED_LOAD_BALANCE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_MEMORY_MIGRATE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_SPREAD_PAGE, _arg: &cs->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: CS_SPREAD_SLAB, _arg: &cs->flags) -> return;
}
//
// Helper routine for generate_sched_domains().
// Do cpusets a, b have overlapping effective cpus_allowed masks?
//
extern "C" {
    pub fn cpumask_intersects(_arg: a->effective_cpus, _arg: b->effective_cpus) -> return;
}
// jump label reference count + the top-level cpuset
extern "C" {
    pub fn cgroup_is_populated(_arg: cs->css.cgroup) -> return;
}
//
// cpuset_for_each_child - traverse online children of a cpuset
// @child_cs: loop cursor pointing to the current child
// @pos_css: used for iteration
// @parent_cs: target cpuset to walk children of
//
// Walk @child_cs through the online children of @parent_cs.  Must be used
// with RCU read locked.
//

//
// cpuset_for_each_descendant_pre - pre-order walk of a cpuset's descendants
// @des_cs: loop cursor pointing to the current descendant
// @pos_css: used for iteration
// @root_cs: target cpuset to walk ancestor of
//
// Walk @des_cs through the online descendants of @root_cs.  Must be used
// with RCU read locked.  The caller may modify @pos_css by calling
// css_rightmost_descendant() to skip subtree.  @root_cs is included in the
// iteration and the first node to be visited.
//

extern "C" {
    pub fn rebuild_sched_domains_locked();
}
extern "C" {
    pub fn cpuset_callback_lock_irq();
}
extern "C" {
    pub fn cpuset_callback_unlock_irq();
}
extern "C" {
    pub fn cpuset_update_tasks_cpumask(cs: *mut cpuset, new_cpus: *mut cpumask);
}
extern "C" {
    pub fn cpuset_update_tasks_nodemask(cs: *mut cpuset);
}
extern "C" {
    pub fn cpuset_update_flag(bit: cpuset_flagbits_t, cs: *mut cpuset, turning_on: c_int) -> c_int;
}
extern "C" {
    pub fn cpuset_common_seq_show(sf: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cpuset_full_lock();
}
extern "C" {
    pub fn cpuset_full_unlock();
}
//
// cpuset-v1.c
//

extern "C" {
    pub fn cpuset1_update_tasks_flags(cs: *mut cpuset);
}
extern "C" {
    pub fn cpuset1_validate_change(cur: *mut cpuset, trial: *mut cpuset) -> c_int;
}
extern "C" {
    pub fn cpuset1_cpus_excl_conflict(cs1: *mut cpuset, cs2: *mut cpuset) -> bool;
}
extern "C" {
    pub fn cpuset1_init(cs: *mut cpuset);
}
extern "C" {
    pub fn cpuset1_online_css(css: *mut cgroup_subsys_state);
}

