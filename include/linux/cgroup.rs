//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cgroup.h
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
// cgroup interface
//
// Copyright (C) 2003 BULL SA
// Copyright (C) 2004-2006 Silicon Graphics, Inc.
//

//
// All weight knobs on the default hierarchy should use the following min,
// default and max values.  The default value is the logarithmic center of
// MIN and MAX and allows 100x to be expressed in both directions.
//
pub const CGROUP_WEIGHT_MIN: c_int = 1;
pub const CGROUP_WEIGHT_DFL: c_int = 100;
pub const CGROUP_WEIGHT_MAX: c_int = 10000;

//
// To avoid confusing the compiler (and generating warnings) with code
// that attempts to access what would be a 0-element array (i.e. sized
// to a potentially empty array when CGROUP_SUBSYS_COUNT == 0), this
// constant expression can be added.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum css_task_iter_flags {
    CSS_TASK_ITER_PROCS    = (1U << 0),  /* walk only threadgroup leaders */
    CSS_TASK_ITER_THREADED = (1U << 1),  /* walk all threaded css_sets in the domain */
    CSS_TASK_ITER_WITH_DEAD = (1U << 2),  /* include exiting tasks */
    CSS_TASK_ITER_SKIPPED  = (1U << 16), /* internal flags */
}

// a css_task_iter should be treated as an opaque object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct css_task_iter {
    pub ss: *mut cgroup_subsys,
    pub flags: c_uint,
    pub cset_pos: *mut list_head,
    pub cset_head: *mut list_head,
    pub tcset_pos: *mut list_head,
    pub tcset_head: *mut list_head,
    pub task_pos: *mut list_head,
    pub cur_tasks_head: *mut list_head,
    pub cur_cset: *mut css_set,
    pub cur_dcset: *mut css_set,
    pub cur_task: *mut task_struct,
    pub /: *mut *mut list_head iters_node; / css_set->task_iters,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgroup_lifetime_events {
    CGROUP_LIFETIME_ONLINE,
    CGROUP_LIFETIME_OFFLINE,
}

//
// Events on cgroup_task_notifier, data is struct cgroup_task_migrate_ctx.
// MIGRATING fires per task before the migration commits and an error return
// from the chain fails the migration, in which case tasks that were already
// notified receive MIGRATE_CANCELED. MIGRATED fires per task after the
// migration is committed and can't fail. Only migrations that change a task's
// dfl cgroup are reported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgroup_task_events {
    CGROUP_TASK_MIGRATING,
    CGROUP_TASK_MIGRATED,
    CGROUP_TASK_MIGRATE_CANCELED,
}

//
// @src_dcgrp and @dst_dcgrp are @task's dfl cgroups before and after the
// migration. @src_dcgrp is NULL for CGROUP_TASK_MIGRATED as per-task sources
// are not tracked past the commit point.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_task_migrate_ctx {
    pub task: *mut task_struct,
    pub src_dcgrp: *mut cgroup,
    pub dst_dcgrp: *mut cgroup,
}

//
// cgroup_subsys_enabled - fast test on whether a subsys is enabled
// @ss: subsystem in question
//

//
// cgroup_subsys_on_dfl - fast test on whether a subsys is on default hierarchy
// @ss: subsystem in question
//

extern "C" {
    pub fn cgroup_on_dfl(cgrp: *const cgroup) -> bool;
}
extern "C" {
    pub fn css_has_online_children(css: *mut cgroup_subsys_state) -> bool;
}
extern "C" {
    pub fn cgroup_attach_task_all(from: *mut task_struct, : *mut task_struct) -> c_int;
}
extern "C" {
    pub fn cgroup_transfer_tasks(to: *mut cgroup, from: *mut cgroup) -> c_int;
}
extern "C" {
    pub fn cgroup_add_dfl_cftypes(ss: *mut cgroup_subsys, cfts: *mut cftype) -> c_int;
}
extern "C" {
    pub fn cgroup_add_legacy_cftypes(ss: *mut cgroup_subsys, cfts: *mut cftype) -> c_int;
}
extern "C" {
    pub fn cgroup_add_cftypes(ss: *mut cgroup_subsys, cfts: *mut cftype) -> c_int;
}
extern "C" {
    pub fn cgroup_rm_cftypes(cfts: *mut cftype) -> c_int;
}
extern "C" {
    pub fn cgroup_file_notify(cfile: *mut cgroup_file);
}
extern "C" {
    pub fn cgroup_file_show(cfile: *mut cgroup_file, show: bool);
}
extern "C" {
    pub fn cgroupstats_build(stats: *mut cgroupstats, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn cgroup_fork(p: *mut task_struct);
}
extern "C" {
    pub fn cgroup_task_exit(p: *mut task_struct);
}
extern "C" {
    pub fn cgroup_task_dead(p: *mut task_struct);
}
extern "C" {
    pub fn cgroup_task_release(p: *mut task_struct);
}
extern "C" {
    pub fn cgroup_task_free(p: *mut task_struct);
}
extern "C" {
    pub fn cgroup_init_early() -> c_int;
}
extern "C" {
    pub fn cgroup_init() -> c_int;
}
extern "C" {
    pub fn cgroup_parse_float(input: *const c_char, dec_shift: unsigned, v: *mut i64) -> c_int;
}
//
// Iteration helpers and macros.
//
extern "C" {
    pub fn css_task_iter_end(it: *mut css_task_iter);
}
//
// css_for_each_child - iterate through children of a css
// @pos: the css * to use as the loop cursor
// @parent: css whose children to walk
//
// Walk @parent's children.  Must be called under rcu_read_lock().
//
// If a subsystem synchronizes ->css_online() and the start of iteration, a
// css which finished ->css_online() is guaranteed to be visible in the
// future iterations and will stay visible until the last reference is put.
// A css which hasn't finished ->css_online() or already finished
// ->css_offline() may show up during traversal.  It's each subsystem's
// responsibility to synchronize against on/offlining.
//
// It is allowed to temporarily drop RCU read lock during iteration.  The
// caller is responsible for ensuring that @pos remains accessible until
// the start of the next iteration by, for example, bumping the css refcnt.
//

//
// css_for_each_descendant_pre - pre-order walk of a css's descendants
// @pos: the css * to use as the loop cursor
// @root: css whose descendants to walk
//
// Walk @root's descendants.  @root is included in the iteration and the
// first node to be visited.  Must be called under rcu_read_lock().
//
// If a subsystem synchronizes ->css_online() and the start of iteration, a
// css which finished ->css_online() is guaranteed to be visible in the
// future iterations and will stay visible until the last reference is put.
// A css which hasn't finished ->css_online() or already finished
// ->css_offline() may show up during traversal.  It's each subsystem's
// responsibility to synchronize against on/offlining.
//
// For example, the following guarantees that a descendant can't escape
// state updates of its ancestors.
//
// my_online(@css)
// {
// Lock @css's parent and @css;
// Inherit state from the parent;
// Unlock both.
// }
//
// my_update_state(@css)
// {
// css_for_each_descendant_pre(@pos, @css) {
// Lock @pos;
// if (@pos == @css)
// Update @css's state;
// else
// Verify @pos is alive and inherit state from its parent;
// Unlock @pos;
// }
//
// As long as the inheriting step, including checking the parent state, is
// enclosed inside @pos locking, double-locking the parent isn't necessary
// while inheriting.  The state update to the parent is guaranteed to be
// visible by walking order and, as long as inheriting operations to the
// same @pos are atomic to each other, multiple updates racing each other
// still result in the correct state.  It's guaranateed that at least one
// inheritance happens for any css after the latest update to its parent.
//
// If checking parent's state requires locking the parent, each inheriting
// iteration should lock and unlock both @pos->parent and @pos.
//
// Alternatively, a subsystem may choose to use a single global lock to
// synchronize ->css_online() and ->css_offline() against tree-walking
// operations.
//
// It is allowed to temporarily drop RCU read lock during iteration.  The
// caller is responsible for ensuring that @pos remains accessible until
// the start of the next iteration by, for example, bumping the css refcnt.
//

//
// css_for_each_descendant_post - post-order walk of a css's descendants
// @pos: the css * to use as the loop cursor
// @css: css whose descendants to walk
//
// Similar to css_for_each_descendant_pre() but performs post-order
// traversal instead.  @root is included in the iteration and the last
// node to be visited.
//
// If a subsystem synchronizes ->css_online() and the start of iteration, a
// css which finished ->css_online() is guaranteed to be visible in the
// future iterations and will stay visible until the last reference is put.
// A css which hasn't finished ->css_online() or already finished
// ->css_offline() may show up during traversal.  It's each subsystem's
// responsibility to synchronize against on/offlining.
//
// Note that the walk visibility guarantee example described in pre-order
// walk doesn't apply the same to post-order walks.
//

// iterate over child cgrps, lock should be held throughout iteration

// walk live descendants in pre order

// walk live descendants in postorder

//
// cgroup_taskset_for_each - iterate cgroup_taskset
// @task: the loop cursor
// @dst_css: the destination css
// @tset: taskset to iterate
//
// @tset may contain multiple tasks and they may belong to multiple
// processes.
//
// On the v2 hierarchy, there may be tasks from multiple processes and they
// may not share the source or destination csses.
//
// On traditional hierarchies, when there are multiple tasks in @tset, if a
// task of a process is in @tset, all tasks of the process are in @tset.
// Also, all are guaranteed to share the same source and destination csses.
//
// Iteration is not in any specific order.
//

//
// cgroup_taskset_for_each_leader - iterate group leaders in a cgroup_taskset
// @leader: the loop cursor
// @dst_css: the destination css
// @tset: taskset to iterate
//
// Iterate threadgroup leaders of @tset.  For single-task migrations, @tset
// may not contain any.
//

//
// Inline functions.
//

extern "C" {
    pub fn css_get(css: *mut cgroup_subsys_state);
}
extern "C" {
    pub fn css_get_many(css: *mut cgroup_subsys_state, n: c_uint);
}
extern "C" {
    pub fn css_tryget(css: *mut cgroup_subsys_state) -> bool;
}
extern "C" {
    pub fn css_tryget_online(css: *mut cgroup_subsys_state) -> bool;
}
extern "C" {
    pub fn css_put(css: *mut cgroup_subsys_state);
}
extern "C" {
    pub fn css_put_many(css: *mut cgroup_subsys_state, n: c_uint);
}

// Macro flag: #define CGROUP_REF_EXPORT(fn)

//
// cgroup_css - obtain a cgroup's css for the specified subsystem
// @cgrp: the cgroup of interest
// @ss: the subsystem of interest (%NULL returns @cgrp->self)
//
// Return @cgrp's css (cgroup_subsys_state) associated with @ss.  This
// function must be called either under cgroup_mutex or rcu_read_lock() and
// the caller is responsible for pinning the returned css if it wants to
// keep accessing it outside the said locks.  This function may return
// %NULL if @cgrp doesn't have @subsys_id enabled.
//
// css_is_dying - test whether the specified css is dying
// @css: target css
//
// Test whether @css is in the process of offlining or already offline.  In
// most cases, ->css_online() and ->css_offline() callbacks should be
// enough; however, the actual offline operations are RCU delayed and this
// test returns %true also when @css is scheduled to be offlined.
//
// This is useful, for example, when the use case requires synchronous
// behavior with respect to cgroup removal.  cgroup removal schedules css
// offlining but the css can seem alive while the operation is being
// delayed.  If the delay affects user visible semantics, this test can be
// used to resolve the situation.
//
// cgroup::self should not have subsystem association
extern "C" {
    pub fn css_tryget(_arg: &cgrp->self) -> return;
}
//
// task_css_set_check - obtain a task's css_set with extra access conditions
// @task: the task to obtain css_set for
// @__c: extra condition expression to be passed to rcu_dereference_check()
//
// A task's css_set is RCU protected, initialized and exited while holding
// task_lock(), and can only be modified while holding both cgroup_mutex
// and task_lock() while the task is alive.  This macro verifies that the
// caller is inside proper critical section and returns @task's css_set.
//
// The caller can also specify additional allowed conditions via @__c, such
// as locks used during the cgroup_subsys::attach() methods.
//

//
// task_css_check - obtain css for (task, subsys) w/ extra access conds
// @task: the target task
// @subsys_id: the target subsystem ID
// @__c: extra condition expression to be passed to rcu_dereference_check()
//
// Return the cgroup_subsys_state for the (@task, @subsys_id) pair.  The
// synchronization rules are the same as task_css_set_check().
//

//
// task_css_set - obtain a task's css_set
// @task: the task to obtain css_set for
//
// See task_css_set_check().
//
extern "C" {
    pub fn task_css_set_check(_arg: task, _arg: false) -> return;
}
//
// task_css - obtain css for (task, subsys)
// @task: the target task
// @subsys_id: the target subsystem ID
//
// See task_css_check().
//
extern "C" {
    pub fn task_css_check(_arg: task, _arg: subsys_id, _arg: false) -> return;
}
//
// task_get_css - find and get the css for (task, subsys)
// @task: the target task
// @subsys_id: the target subsystem ID
//
// Find the css for the (@task, @subsys_id) combination, increment a
// reference on and return it.  This function is guaranteed to return a
// valid css.  The returned css may already have been offlined.
//
// Can't use css_tryget_online() here.  A task which has
// PF_EXITING set may stay associated with an offline css.
// If such task calls this function, css_tryget_online()
// will keep failing.
//
// task_css_is_root - test whether a task belongs to the root css
// @task: the target task
// @subsys_id: the target subsystem ID
//
// Test whether @task belongs to the root css on the specified subsystem.
// May be invoked in any context.
//
extern "C" {
    pub fn container_of(_arg: parent_css, cgroup: struct, _arg: self) -> return;
}
//
// cgroup_is_descendant - test ancestry
// @cgrp: the cgroup to be tested
// @ancestor: possible ancestor of @cgrp
//
// Test whether @cgrp is a descendant of @ancestor.  It also returns %true
// if @cgrp == @ancestor.  This function is safe to call as long as @cgrp
// and @ancestor are accessible.
//
// cgroup_ancestor - find ancestor of cgroup
// @cgrp: cgroup to find ancestor of
// @ancestor_level: level of ancestor to find starting from root
//
// Find ancestor of cgroup at specified level starting from root if it exists
// and return pointer to it. Return NULL if @cgrp doesn't have ancestor at
// @ancestor_level.
//
// This function is safe to call as long as @cgrp is accessible.
//
// cgroup_common_ancestor - find common ancestor of two cgroups
// @a: first cgroup to find common ancestor of
// @b: second cgroup to find common ancestor of
//
// Find the first cgroup that is an ancestor of both @a and @b, if it exists
// and return a pointer to it. If such a cgroup doesn't exist, return NULL.
//
// This function is safe to call as long as both @a and @b are accessible.
//
// task_under_cgroup_hierarchy - test task's membership of cgroup ancestry
// @task: the task to be tested
// @ancestor: possible ancestor of @task's cgroup
//
// Tests whether @task's default cgroup hierarchy is a descendant of @ancestor.
// It follows all the same rules as cgroup_is_descendant, and only applies
// to the default hierarchy.
//
extern "C" {
    pub fn cgroup_is_descendant(_arg: cset->dfl_cgrp, _arg: ancestor) -> return;
}
//
// Populated counters: writes happen under css_set_lock. The accessors below
// may read unlocked. What an unpopulated result means depends on context:
//
// - No lock held. Just a snapshot. May race with concurrent updates and is
// useful only as a hint.
//
// - cgroup_mutex held. Migration into the cgroup is blocked, so an observed
// !populated stays !populated until cgroup_mutex is dropped.
//
// - CSS_DYING set. The css can no longer be repopulated, so !populated is
// sticky once observed.
//
extern "C" {
    pub fn READ_ONCE(_arg: cgrp->self.nr_populated_csets) -> return;
}
extern "C" {
    pub fn READ_ONCE(READ_ONCE(css->nr_populated_children: css->nr_populated_csets) ||) -> return;
}
extern "C" {
    pub fn css_is_populated(_arg: &cgrp->self) -> return;
}
// returns ino associated with a cgroup
extern "C" {
    pub fn kernfs_ino(_arg: cgrp->kn) -> return;
}
// cft/css accessors for cftype->write() operation
// cft/css accessors for cftype->seq_*() operations
extern "C" {
    pub fn of_cft(_arg: seq->private) -> return;
}
extern "C" {
    pub fn of_css(_arg: seq->private) -> return;
}
//
// Name / path handling functions.  All are thin wrappers around the kernfs
// counterparts and can be called under any context.
//
extern "C" {
    pub fn kernfs_name(_arg: cgrp->kn, _arg: buf, _arg: buflen) -> return;
}
extern "C" {
    pub fn kernfs_path(_arg: cgrp->kn, _arg: buf, _arg: buflen) -> return;
}
extern "C" {
    pub fn cgroup_psi_enabled() -> bool;
}
//
// kthreadd is inherited by all kthreads, keep it in the root so
// that the new kthreads are guaranteed to stay in the root until
// initialization is finished.
//
// This kthread finished initialization.  The creator should have
// set PF_NO_SETAFFINITY if this kthread should stay in the root.
//
extern "C" {
    pub fn cgroup_path_from_kernfs_id(id: u64, buf: *mut c_char, buflen: usize);
}

//
// cgroup scalable recursive statistics.
//
extern "C" {
    pub fn __css_rstat_updated(css: *mut cgroup_subsys_state, cpu: c_int);
}
extern "C" {
    pub fn css_rstat_updated(css: *mut cgroup_subsys_state, cpu: c_int);
}
extern "C" {
    pub fn css_rstat_flush(css: *mut cgroup_subsys_state);
}
//
// Basic resource stats.
//

extern "C" {
    pub fn cpuacct_charge(tsk: *mut task_struct, cputime: u64);
}
extern "C" {
    pub fn cpuacct_account_field(tsk: *mut task_struct, index: c_int, val: u64);
}

extern "C" {
    pub fn __cgroup_account_cputime(cgrp: *mut cgroup, delta_exec: u64);
}

//
// sock->sk_cgrp_data handling.  For more info, see sock_cgroup_data
// definition in cgroup-defs.h.
//

extern "C" {
    pub fn cgroup_sk_alloc(skcd: *mut sock_cgroup_data);
}
extern "C" {
    pub fn cgroup_sk_clone(skcd: *mut sock_cgroup_data);
}
extern "C" {
    pub fn cgroup_sk_free(skcd: *mut sock_cgroup_data);
}

extern "C" {
    pub fn cgroup_enter_frozen();
}
extern "C" {
    pub fn cgroup_leave_frozen(always_leave: bool);
}
extern "C" {
    pub fn cgroup_update_frozen(cgrp: *mut cgroup);
}
extern "C" {
    pub fn cgroup_freeze(cgrp: *mut cgroup, freeze: bool);
}

