//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/cgroup/cgroup-internal.h
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

pub const TRACE_CGROUP_PATH_LEN: c_int = 1024;
extern "C" {
    pub fn enable_debug_cgroup() -> void __init;
}
//
// cgroup_path() takes a spin lock. It is good practice not to take
// spin locks within trace point handlers, as they are mostly hidden
// from normal view. As cgroup_path() can take the kernfs_rename_lock
// spin lock, it is best to not call that function from the trace event
// handler.
//
// Note: trace_cgroup_##type##_enabled() is a static branch that will only
// be set when the trace event is enabled.
//

//
// The cgroup filesystem superblock creation/mount context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_fs_context {
    pub kfc: kernfs_fs_context,
    pub root: *mut cgroup_root,
    pub ns: *mut cgroup_namespace,
    pub /: *mut *mut *mut unsigned int flags; / CGRP_ROOT_ flags,
// cgroup1 bits
    pub cpuset_clone_children: bool,
    pub /: *mut *mut bool none; / User explicitly requested empty subsystem,
    pub /: *mut *mut bool all_ss; / Seen 'all' option,
    pub /: *mut *mut u32 subsys_mask; / Selected subsystems,
    pub /: *mut *mut *mut char name; / Hierarchy name,
    pub /: *mut *mut *mut char release_agent; / Path for release notifications,
}

extern "C" {
    pub fn container_of(_arg: kfc, cgroup_fs_context: struct, _arg: kfc) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_file_ctx {
    pub ns: *mut cgroup_namespace,
    pub trigger: *mut c_void,
    pub psi: },
    pub started: bool,
    pub iter: css_task_iter,
    pub procs: },
    pub pidlist: *mut cgroup_pidlist,
    pub procs1: },
    pub peak: cgroup_of_peak,
}

//
// A cgroup can be associated with multiple css_sets as different tasks may
// belong to different cgroups on different hierarchies.  In the other
// direction, a css_set is naturally associated with multiple cgroups.
// This M:N relationship is represented by the following link structure
// which exists for each association and allows traversing the associations
// from both sides.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgrp_cset_link {
// the cgroup and css_set this link associates
    pub cgrp: *mut cgroup,
    pub cset: *mut css_set,
// list of cgrp_cset_links anchored at cgrp->cset_links
    pub cset_link: list_head,
// list of cgrp_cset_links anchored at css_set->cgrp_links
    pub cgrp_link: list_head,
}

// used to track tasks and csets during migration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_taskset {
// the src and dst cset list running through cset->mg_node
    pub src_csets: list_head,
    pub dst_csets: list_head,
// the number of tasks in the set
    pub nr_tasks: c_int,
// the subsys currently being processed
    pub ssid: c_int,
//
// Fields for cgroup_taskset_*() iteration.
//
// Before migration is committed, the target migration tasks are on
// ->mg_tasks of the csets on ->src_csets.  After, on ->mg_tasks of
// the csets on ->dst_csets.  ->csets point to either ->src_csets
// or ->dst_csets depending on whether migration is committed.
//
// ->cur_csets and ->cur_task point to the current task position
// during iteration.
//
    pub csets: *mut list_head,
    pub cur_cset: *mut css_set,
    pub cur_task: *mut task_struct,
}

// migration context also tracks preloading
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_mgctx {
//
// Preloaded source and destination csets.  Used to guarantee
// atomic success or failure on actual migration.
//
    pub preloaded_src_csets: list_head,
    pub preloaded_dst_csets: list_head,
// tasks and csets to migrate
    pub tset: cgroup_taskset,
// subsystems affected by migration
    pub ss_mask: u32,
}

// iterate across the hierarchies

//
// for_each_subsys - iterate all enabled cgroup subsystems
// @ss: the iteration cursor
// @ssid: the index of @ss, CGROUP_SUBSYS_COUNT after reaching the end
//

extern "C" {
    pub fn test_bit(_arg: CGRP_NOTIFY_ON_RELEASE, _arg: &cgrp->flags) -> return;
}
extern "C" {
    pub fn put_css_set_locked(cset: *mut css_set);
}
//
// Ensure that the refcount doesn't hit zero while any readers
// can see it. Similar to atomic_dec_and_lock(), but for an
// rwlock
//
// refcounted get/put for css_set objects
//
extern "C" {
    pub fn cgroup_ssid_enabled(ssid: c_int) -> bool;
}
extern "C" {
    pub fn cgroup_kn_unlock(kn: *mut kernfs_node);
}
extern "C" {
    pub fn cgroup_favor_dynmods(root: *mut cgroup_root, favor: bool);
}
extern "C" {
    pub fn cgroup_free_root(root: *mut cgroup_root);
}
extern "C" {
    pub fn init_cgroup_root(ctx: *mut cgroup_fs_context);
}
extern "C" {
    pub fn cgroup_setup_root(root: *mut cgroup_root, ss_mask: u32) -> c_int;
}
extern "C" {
    pub fn rebind_subsystems(dst_root: *mut cgroup_root, ss_mask: u32) -> c_int;
}
extern "C" {
    pub fn cgroup_do_get_tree(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn cgroup_migrate_vet_dst(dst_cgrp: *mut cgroup) -> c_int;
}
extern "C" {
    pub fn cgroup_migrate_finish(mgctx: *mut cgroup_mgctx);
}
extern "C" {
    pub fn cgroup_migrate_prepare_dst(mgctx: *mut cgroup_mgctx) -> c_int;
}
extern "C" {
    pub fn cgroup_lock_and_drain_offline(cgrp: *mut cgroup);
}
extern "C" {
    pub fn cgroup_mkdir(parent_kn: *mut kernfs_node, name: *const c_char, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn cgroup_rmdir(kn: *mut kernfs_node) -> c_int;
}
extern "C" {
    pub fn __cgroup_task_count(cgrp: *const cgroup) -> c_int;
}
extern "C" {
    pub fn cgroup_task_count(cgrp: *const cgroup) -> c_int;
}
//
// rstat.c
//
extern "C" {
    pub fn css_rstat_init(css: *mut cgroup_subsys_state) -> c_int;
}
extern "C" {
    pub fn css_rstat_exit(css: *mut cgroup_subsys_state);
}
extern "C" {
    pub fn ss_rstat_init(ss: *mut cgroup_subsys) -> c_int;
}
extern "C" {
    pub fn cgroup_base_stat_cputime_show(seq: *mut seq_file);
}
//
// namespace.c
//
// cgroup-v1.c
//
extern "C" {
    pub fn proc_cgroupstats_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cgroup1_ssid_disabled(ssid: c_int) -> bool;
}
extern "C" {
    pub fn cgroup1_pidlist_destroy_all(cgrp: *mut cgroup);
}
extern "C" {
    pub fn cgroup1_release_agent(work: *mut work_struct);
}
extern "C" {
    pub fn cgroup1_check_for_release(cgrp: *mut cgroup);
}
extern "C" {
    pub fn cgroup1_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
}
extern "C" {
    pub fn cgroup1_get_tree(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn cgroup1_reconfigure(ctx: *mut fs_context) -> c_int;
}
