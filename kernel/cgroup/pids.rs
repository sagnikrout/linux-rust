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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


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

#[no_mangle]
pub unsafe extern "C" fn css_pids(css: *mut cgroup_subsys_state) -> *mut c_void {
    return container_of!(css, pids_cgroup, css);
    }
#[no_mangle]
pub unsafe extern "C" fn parent_pids(pids: *mut pids_cgroup) -> *mut c_void {
    return css_pids(pids.css.parent);
    }
#[no_mangle]
pub unsafe extern "C" fn pids_css_alloc(parent: *mut cgroup_subsys_state) -> *mut c_void {
pub static mut pids: *mut c_void = core::ptr::null_mut();
    pids = kzalloc_obj(pids_cgroup);
    if (!pids) {
    return ERR_PTR(-ENOMEM);
    }
    atomic64_set(&pids.limit, PIDS_MAX);
    return &pids.css;
    }
#[no_mangle]
unsafe extern "C" fn pids_css_free(css: *mut cgroup_subsys_state) {
    kfree(css_pids(css));
    }
#[no_mangle]
unsafe extern "C" fn pids_update_watermark(p: *mut pids_cgroup, nr_pids: i64) {
//
// This is racy, but we don't need perfectly accurate tallying of
// the watermark, and this lets us avoid extra atomic overhead.
//
    if (nr_pids > READ_ONCE(p.watermark)) {
    WRITE_ONCE(p.watermark, nr_pids);
    }
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
//
// A negative count (or overflow for that matter) is invalid,
// and indicates a bug in the `pids` controller proper.
//
    WARN_ON_ONCE!(atomic64_add_negative(-num, &pids.counter));
    }
//
// pids_uncharge - hierarchically uncharge the pid count
// @pids: the pid cgroup state
// @num: the number of pids to uncharge
//
#[no_mangle]
unsafe extern "C" fn pids_uncharge(pids: *mut pids_cgroup, num: c_int) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    for (p = pids; parent_pids(p); p = parent_pids(p)) {
    pids_cancel(p, num);
    }
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
pub static mut p: *mut c_void = core::ptr::null_mut();
    for (p = pids; parent_pids(p); p = parent_pids(p)) {
pub static mut new: i64 = 0;
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
    let mut p = core::ptr::null_mut();
    let mut q = core::ptr::null_mut();
    for (p = pids; parent_pids(p); p = parent_pids(p)) {
pub static mut new: i64 = 0;
pub static mut limit: i64 = 0;
//
// Since new is capped to the maximum number of pid_t, if
// p->limit is %PIDS_MAX then we know that this test will never
// fail.
//
    if (new > limit) {
// fail = p;
// goto;
    }
//
// Not technically accurate if we go over limit somewhere up
// the hierarchy, but that's tolerable for the watermark.
//
    pids_update_watermark(p, new);
    }
    return 0;
// label;
    for (q = pids; q != p; q = parent_pids(q)) {
    pids_cancel(q, num);
    }
    pids_cancel(p, num);
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn pids_can_attach(tset: *mut cgroup_taskset) -> c_int {
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut dst_css: *mut c_void = core::ptr::null_mut();
    cgroup_taskset_for_each(task, dst_css, tset) {
    let mut pids = css_pids(dst_css);
pub static mut old_css: *mut c_void = core::ptr::null_mut();
pub static mut old_pids: *mut c_void = core::ptr::null_mut();
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
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut dst_css: *mut c_void = core::ptr::null_mut();
    cgroup_taskset_for_each(task, dst_css, tset) {
    let mut pids = css_pids(dst_css);
pub static mut old_css: *mut c_void = core::ptr::null_mut();
pub static mut old_pids: *mut c_void = core::ptr::null_mut();
    old_css = task_css(task, pids_cgrp_id);
    old_pids = css_pids(old_css);
    pids_charge(old_pids, 1);
    pids_uncharge(pids, 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pids_event(pids_forking: *mut pids_cgroup, pids_over_limit: *mut pids_cgroup) {
    let mut p = pids_forking;
// Only log the first time limit is hit.
    if (atomic64_inc_return(&p.events_local[PIDCG_FORKFAIL]) == 1) {
    pr_info!("cgroup: fork rejected by pids controller in ");
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
    let mut pids = core::ptr::null_mut();
    let mut pids_over_limit = core::ptr::null_mut();
    let mut err = 0;
    pids = css_pids(cset.subsys[pids_cgrp_id]);
    err = pids_try_charge(pids, 1, &pids_over_limit);
    if (err) {
    pids_event(pids, pids_over_limit);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pids_cancel_fork(task: *mut task_struct, cset: *mut css_set) {
pub static mut pids: *mut c_void = core::ptr::null_mut();
    pids = css_pids(cset.subsys[pids_cgrp_id]);
    pids_uncharge(pids, 1);
    }
#[no_mangle]
unsafe extern "C" fn pids_release(task: *mut task_struct) {
    let mut pids = css_pids(task_css(task, pids_cgrp_id));
    pids_uncharge(pids, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn pids_max_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    let mut css = of_css(of);
    let mut pids = css_pids(css);
    let mut limit;
    let mut err = 0;
    buf = strstrip(buf);
    if (!strcmp(buf, PIDS_MAX_STR)) {
    limit = PIDS_MAX;
// goto;
    }
    err = kstrtoll(buf, 0, &limit);
    if (err) {
    return err;
    }
    if (limit < 0 || limit >= PIDS_MAX) {
    return -EINVAL;
    }
// label;
//
// Limit updates don't need to be mutex'd, since it isn't
// critical that any racing fork()s follow the new limit.
//
    atomic64_set(&pids.limit, limit);
    return nbytes;
    }
#[no_mangle]
unsafe extern "C" fn pids_max_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut css = seq_css(sf);
    let mut pids = css_pids(css);
pub static mut limit: i64 = 0;
    if (limit >= PIDS_MAX) {
    seq_printf(sf, "%s\n", PIDS_MAX_STR);
    }
    else {
    seq_printf(sf, "%lld\n", limit);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pids_current_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> s64 {
    let mut pids = css_pids(css);
    return atomic64_read(&pids.counter);
    }
#[no_mangle]
pub unsafe extern "C" fn pids_peak_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> s64 {
    let mut pids = css_pids(css);
    return READ_ONCE(pids.watermark);
    }
#[no_mangle]
unsafe extern "C" fn __pids_events_show(sf: *mut seq_file, local: bool) -> c_int {
    let mut pids = css_pids(seq_css(sf));
pub static mut pe: pidcg_event = 0;
pub static mut events: *mut c_void = core::ptr::null_mut();
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
    __pids_events_show(sf, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pids_events_local_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    __pids_events_show(sf, true);
    return 0;
    }
pub static mut cftype: usize = 0;
pub static mut cftype: usize = 0;
pub static mut cgroup_subsys: usize = 0;