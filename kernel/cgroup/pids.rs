//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/pids.c
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
// Process number limiting controller for cgroups.
//
// Used to allow a cgroup hierarchy to stop any new processes from fork()ing
// after a certain limit is reached.
//
// Since it is trivial to hit the task limit without hitting any kmemcg limits
// in place, PIDs are a fundamental resource. As such, PID exhaustion must be
// preventable in the scope of a cgroup hierarchy by allowing resource limiting
// of the number of tasks in a cgroup.
//
// In order to use the `pids` controller, set the maximum number of tasks in
// pids.max (this is not available in the root cgroup for obvious reasons). The
// number of processes currently in the cgroup is given by pids.current.
// Organisational operations are not blocked by cgroup policies, so it is
// possible to have pids.current > pids.max. However, it is not possible to
// violate a cgroup policy through fork(). fork() will return -EAGAIN if forking
// would cause a cgroup policy to be violated.
//
// To set a cgroup to have no limit, set pids.max to "max". This is the default
// for all new cgroups (N.B. that PID limits are hierarchical, so the most
// stringent limit in the hierarchy is followed).
//
// pids.current tracks all child cgroup hierarchies, so parent/pids.current is
// a superset of parent/child/pids.current.
//
// Copyright (C) 2015 Aleksa Sarai <cyphar@cyphar.com>
//

    enum pidcg_event {
// Fork failed in subtree because this pids_cgroup limit was hit.
    PIDCG_MAX,
// Fork failed in this pids_cgroup because ancestor limit was hit.
    PIDCG_FORKFAIL,
    NR_PIDCG_EVENTS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pids_cgroup {
    pub css: cgroup_subsys_state,
//
// Use 64-bit types so that we can safely represent "max" as
// %PIDS_MAX = (%PID_MAX_LIMIT + 1).
//
    pub counter: core::sync::atomic::AtomicI64,
    pub limit: core::sync::atomic::AtomicI64,
    pub watermark: i64,
// Handles for pids.events[.local]
    pub events_file: cgroup_file,
    pub events_local_file: cgroup_file,
    pub events: [core::sync::atomic::AtomicI64; NR_PIDCG_EVENTS],
    pub events_local: [core::sync::atomic::AtomicI64; NR_PIDCG_EVENTS],
}

    static struct pids_cgroup *css_pids(struct cgroup_subsys_state *css)
    {
    return container_of(css, struct pids_cgroup, css);
    }
    static struct pids_cgroup *parent_pids(struct pids_cgroup *pids)
    {
    return css_pids(pids.css.parent);
    }
    static struct cgroup_subsys_state *
    pids_css_alloc(struct cgroup_subsys_state *parent)
    {
    struct pids_cgroup *pids;
    pids = kzalloc_obj(struct pids_cgroup);
    if (!pids)
    return ERR_PTR(-ENOMEM);
    atomic64_set(&pids.limit, PIDS_MAX);
    return &pids.css;
    }
#[no_mangle]
unsafe extern "C" fn pids_css_free(css: *mut cgroup_subsys_state) {
    static void pids_css_free(struct cgroup_subsys_state *css)
    {
    kfree(css_pids(css));
    }
#[no_mangle]
unsafe extern "C" fn pids_update_watermark(p: *mut pids_cgroup, nr_pids: i64) {
    static void pids_update_watermark(struct pids_cgroup *p, int64_t nr_pids)
    {
//
// This is racy, but we don't need perfectly accurate tallying of
// the watermark, and this lets us avoid extra atomic overhead.
//
    if (nr_pids > READ_ONCE(p.watermark))
    WRITE_ONCE(p.watermark, nr_pids);
    }
//
// pids_cancel - uncharge the local pid count
// @pids: the pid cgroup state
// @num: the number of pids to cancel
//
// This function will WARN if the pid count goes under 0, because such a case is
// a bug in the pids controller proper.
//
#[no_mangle]
unsafe extern "C" fn pids_cancel(pids: *mut pids_cgroup, num: c_int) {
    static void pids_cancel(struct pids_cgroup *pids, int num)
    {
//
// A negative count (or overflow for that matter) is invalid,
// and indicates a bug in the `pids` controller proper.
//
    WARN_ON_ONCE(atomic64_add_negative(-num, &pids.counter));
    }
//
// pids_uncharge - hierarchically uncharge the pid count
// @pids: the pid cgroup state
// @num: the number of pids to uncharge
//
#[no_mangle]
unsafe extern "C" fn pids_uncharge(pids: *mut pids_cgroup, num: c_int) {
    static void pids_uncharge(struct pids_cgroup *pids, int num)
    {
    struct pids_cgroup *p;
    for (p = pids; parent_pids(p); p = parent_pids(p))
    pids_cancel(p, num);
    }
//
// pids_charge - hierarchically charge the pid count
// @pids: the pid cgroup state
// @num: the number of pids to charge
//
// This function does *not* follow the pid limit set. It cannot fail and the new
// pid count may exceed the limit. This is only used for reverting failed
// attaches, where there is no other way out than violating the limit.
//
#[no_mangle]
unsafe extern "C" fn pids_charge(pids: *mut pids_cgroup, num: c_int) {
    static void pids_charge(struct pids_cgroup *pids, int num)
    {
    struct pids_cgroup *p;
    for (p = pids; parent_pids(p); p = parent_pids(p)) {
    let mut new: i64 = atomic64_add_return(num, &p.counter);
    pids_update_watermark(p, new);
    }
    }
//
// pids_try_charge - hierarchically try to charge the pid count
// @pids: the pid cgroup state
// @num: the number of pids to charge
// @fail: storage of pid cgroup causing the fail
//
// This function follows the set limit. It will fail if the charge would cause
// the new value to exceed the hierarchical limit. Returns 0 if the charge
// succeeded, otherwise -EAGAIN.
//
#[no_mangle]
unsafe extern "C" fn pids_try_charge(pids: *mut pids_cgroup, num: c_int, fail: *mut pids_cgroup) -> c_int {
    static int pids_try_charge(struct pids_cgroup *pids, int num, struct pids_cgroup **fail)
    {
    struct pids_cgroup *p, *q;
    for (p = pids; parent_pids(p); p = parent_pids(p)) {
    let mut new: i64 = atomic64_add_return(num, &p.counter);
    let mut limit: i64 = atomic64_read(&p.limit);
//
// Since new is capped to the maximum number of pid_t, if
// p->limit is %PIDS_MAX then we know that this test will never
// fail.
//
    if (new > limit) {
// fail = p;
    goto revert;
    }
//
// Not technically accurate if we go over limit somewhere up
// the hierarchy, but that's tolerable for the watermark.
//
    pids_update_watermark(p, new);
    }
    return 0;
    revert:
    for (q = pids; q != p; q = parent_pids(q))
    pids_cancel(q, num);
    pids_cancel(p, num);
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn pids_can_attach(tset: *mut cgroup_taskset) -> c_int {
    static int pids_can_attach(struct cgroup_taskset *tset)
    {
    struct task_struct *task;
    struct cgroup_subsys_state *dst_css;
    cgroup_taskset_for_each(task, dst_css, tset) {
    struct pids_cgroup *pids = css_pids(dst_css);
    struct cgroup_subsys_state *old_css;
    struct pids_cgroup *old_pids;
//
// No need to pin @old_css between here and cancel_attach()
// because cgroup core protects it from being freed before
// the migration completes or fails.
//
    old_css = task_css(task, pids_cgrp_id);
    old_pids = css_pids(old_css);
    pids_charge(pids, 1);
    pids_uncharge(old_pids, 1);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pids_cancel_attach(tset: *mut cgroup_taskset) {
    static void pids_cancel_attach(struct cgroup_taskset *tset)
    {
    struct task_struct *task;
    struct cgroup_subsys_state *dst_css;
    cgroup_taskset_for_each(task, dst_css, tset) {
    struct pids_cgroup *pids = css_pids(dst_css);
    struct cgroup_subsys_state *old_css;
    struct pids_cgroup *old_pids;
    old_css = task_css(task, pids_cgrp_id);
    old_pids = css_pids(old_css);
    pids_charge(old_pids, 1);
    pids_uncharge(pids, 1);
    }
    }
    static void pids_event(struct pids_cgroup *pids_forking,
    struct pids_cgroup *pids_over_limit)
    {
    struct pids_cgroup *p = pids_forking;
// Only log the first time limit is hit.
    if (atomic64_inc_return(&p.events_local[PIDCG_FORKFAIL]) == 1) {
    pr_info("cgroup: fork rejected by pids controller in ");
    pr_cont_cgroup_path(p.css.cgroup);
    pr_cont("\n");
    }
    if (!cgroup_subsys_on_dfl(pids_cgrp_subsys) ||
    cgrp_dfl_root.flags & CGRP_ROOT_PIDS_LOCAL_EVENTS) {
    cgroup_file_notify(&p.events_local_file);
    return;
    }
    atomic64_inc(&pids_over_limit.events_local[PIDCG_MAX]);
    cgroup_file_notify(&pids_over_limit.events_local_file);
    for (p = pids_over_limit; parent_pids(p); p = parent_pids(p)) {
    atomic64_inc(&p.events[PIDCG_MAX]);
    cgroup_file_notify(&p.events_file);
    }
    }
//
// task_css_check(true) in pids_can_fork() and pids_cancel_fork() relies
// on cgroup_threadgroup_change_begin() held by the copy_process().
//
#[no_mangle]
unsafe extern "C" fn pids_can_fork(task: *mut task_struct, cset: *mut css_set) -> c_int {
    static int pids_can_fork(struct task_struct *task, struct css_set *cset)
    {
    struct pids_cgroup *pids, *pids_over_limit;
    int err;
    pids = css_pids(cset.subsys[pids_cgrp_id]);
    err = pids_try_charge(pids, 1, &pids_over_limit);
    if (err)
    pids_event(pids, pids_over_limit);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pids_cancel_fork(task: *mut task_struct, cset: *mut css_set) {
    static void pids_cancel_fork(struct task_struct *task, struct css_set *cset)
    {
    struct pids_cgroup *pids;
    pids = css_pids(cset.subsys[pids_cgrp_id]);
    pids_uncharge(pids, 1);
    }
#[no_mangle]
unsafe extern "C" fn pids_release(task: *mut task_struct) {
    static void pids_release(struct task_struct *task)
    {
    struct pids_cgroup *pids = css_pids(task_css(task, pids_cgrp_id));
    pids_uncharge(pids, 1);
    }
    static ssize_t pids_max_write(struct kernfs_open_file *of, char *buf,
    size_t nbytes, loff_t off)
    {
    struct cgroup_subsys_state *css = of_css(of);
    struct pids_cgroup *pids = css_pids(css);
    int64_t limit;
    int err;
    buf = strstrip(buf);
    if (!strcmp(buf, PIDS_MAX_STR)) {
    limit = PIDS_MAX;
    goto set_limit;
    }
    err = kstrtoll(buf, 0, &limit);
    if (err)
    return err;
    if (limit < 0 || limit >= PIDS_MAX)
    return -EINVAL;
    set_limit:
//
// Limit updates don't need to be mutex'd, since it isn't
// critical that any racing fork()s follow the new limit.
//
    atomic64_set(&pids.limit, limit);
    return nbytes;
    }
#[no_mangle]
unsafe extern "C" fn pids_max_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int pids_max_show(struct seq_file *sf, void *v)
    {
    struct cgroup_subsys_state *css = seq_css(sf);
    struct pids_cgroup *pids = css_pids(css);
    let mut limit: i64 = atomic64_read(&pids.limit);
    if (limit >= PIDS_MAX)
    seq_printf(sf, "%s\n", PIDS_MAX_STR);
    else
    seq_printf(sf, "%lld\n", limit);
    return 0;
    }
    static s64 pids_current_read(struct cgroup_subsys_state *css,
    struct cftype *cft)
    {
    struct pids_cgroup *pids = css_pids(css);
    return atomic64_read(&pids.counter);
    }
    static s64 pids_peak_read(struct cgroup_subsys_state *css,
    struct cftype *cft)
    {
    struct pids_cgroup *pids = css_pids(css);
    return READ_ONCE(pids.watermark);
    }
#[no_mangle]
unsafe extern "C" fn __pids_events_show(sf: *mut seq_file, local: bool) -> c_int {
    static int __pids_events_show(struct seq_file *sf, bool local)
    {
    struct pids_cgroup *pids = css_pids(seq_css(sf));
    let mut pe: enum pidcg_event = PIDCG_MAX;
    atomic64_t *events;
    if (!cgroup_subsys_on_dfl(pids_cgrp_subsys) ||
    cgrp_dfl_root.flags & CGRP_ROOT_PIDS_LOCAL_EVENTS) {
    pe = PIDCG_FORKFAIL;
    local = true;
    }
    events = local ? pids.events_local : pids.events;
    seq_printf(sf, "max %lld\n", (s64)atomic64_read(&events[pe]));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pids_events_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int pids_events_show(struct seq_file *sf, void *v)
    {
    __pids_events_show(sf, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pids_events_local_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int pids_events_local_show(struct seq_file *sf, void *v)
    {
    __pids_events_show(sf, true);
    return 0;
    }
    static struct cftype pids_files[] = {
    {
    .name = "max",
    .write = pids_max_write,
    .seq_show = pids_max_show,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "current",
    .read_s64 = pids_current_read,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "peak",
    .flags = CFTYPE_NOT_ON_ROOT,
    .read_s64 = pids_peak_read,
    },
    {
    .name = "events",
    .seq_show = pids_events_show,
    .file_offset = offsetof(struct pids_cgroup, events_file),
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "events.local",
    .seq_show = pids_events_local_show,
    .file_offset = offsetof(struct pids_cgroup, events_local_file),
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    { }	/* terminate */
    };
    static struct cftype pids_files_legacy[] = {
    {
    .name = "max",
    .write = pids_max_write,
    .seq_show = pids_max_show,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "current",
    .read_s64 = pids_current_read,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "peak",
    .flags = CFTYPE_NOT_ON_ROOT,
    .read_s64 = pids_peak_read,
    },
    {
    .name = "events",
    .seq_show = pids_events_show,
    .file_offset = offsetof(struct pids_cgroup, events_file),
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    { }	/* terminate */
    };
    struct cgroup_subsys pids_cgrp_subsys = {
    .css_alloc	= pids_css_alloc,
    .css_free	= pids_css_free,
    .can_attach 	= pids_can_attach,
    .cancel_attach 	= pids_cancel_attach,
    .can_fork	= pids_can_fork,
    .cancel_fork	= pids_cancel_fork,
    .release	= pids_release,
    .legacy_cftypes = pids_files_legacy,
    .dfl_cftypes	= pids_files,
    .threaded	= true,
    };
