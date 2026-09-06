//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/legacy_freezer.c
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


// SPDX-License-Identifier: LGPL-2.1
//
// cgroup_freezer.c -  control group freezer subsystem
//
// Copyright IBM Corporation, 2007
//
// Author : Cedric Le Goater <clg@fr.ibm.com>
//

//
// A cgroup is freezing if any FREEZING flags are set.  FREEZING_SELF is
// set if "FROZEN" is written to freezer.state cgroupfs file, and cleared
// for "THAWED".  FREEZING_PARENT is set if the parent freezer is FREEZING
// for whatever reason.  IOW, a cgroup has FREEZING_PARENT set if one of
// its ancestors has FREEZING_SELF set.
//
    enum freezer_state_flags {
    CGROUP_FREEZER_ONLINE	= (1 << 0), /* freezer is fully online */
    CGROUP_FREEZING_SELF	= (1 << 1), /* this freezer is freezing */
    CGROUP_FREEZING_PARENT	= (1 << 2), /* the parent freezer is freezing */
    CGROUP_FROZEN		= (1 << 3), /* this and its descendants frozen */
// mask for all FREEZING flags
    CGROUP_FREEZING		= CGROUP_FREEZING_SELF | CGROUP_FREEZING_PARENT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct freezer {
    pub css: cgroup_subsys_state,
    pub state: c_uint,
}

pub static mut freezer_mutex: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn css_freezer(css: *mut cgroup_subsys_state) -> *mut c_void {
    return css ? container_of!(css, freezer, css) : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn task_freezer(task: *mut task_struct) -> *mut c_void {
    return css_freezer(task_css(task, freezer_cgrp_id));
    }
#[no_mangle]
pub unsafe extern "C" fn parent_freezer(freezer: *mut freezer) -> *mut c_void {
    return css_freezer(freezer.css.parent);
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup1_freezing(task: *mut task_struct) -> bool {
    let mut ret = 0;
    rcu_read_lock();
    ret = task_freezer(task).state & CGROUP_FREEZING;
    rcu_read_unlock();
    return ret;
    }
    static const char *freezer_state_strs(unsigned int state)
    {
    if (state & CGROUP_FROZEN) {
    return "FROZEN";
    }
    if (state & CGROUP_FREEZING) {
    return "FREEZING";
    }
    return "THAWED";
    };
#[no_mangle]
pub unsafe extern "C" fn freezer_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut c_void {
pub static mut freezer: *mut c_void = core::ptr::null_mut();
    freezer = kzalloc_obj(freezer);
    if (!freezer) {
    return ERR_PTR(-ENOMEM);
    }
    return &freezer.css;
    }
//
// freezer_css_online - commit creation of a freezer css
// @css: css being created
//
// We're committing to creation of @css.  Mark it online and inherit
// parent's freezing state while holding cpus read lock and freezer_mutex.
//
#[no_mangle]
unsafe extern "C" fn freezer_css_online(css: *mut cgroup_subsys_state) -> c_int {
    let mut freezer = css_freezer(css);
    let mut parent = parent_freezer(freezer);
    cpus_read_lock();
    mutex_lock(&freezer_mutex);
    freezer.state |= CGROUP_FREEZER_ONLINE;
    if (parent && (parent.state & CGROUP_FREEZING)) {
    freezer.state |= CGROUP_FREEZING_PARENT | CGROUP_FROZEN;
    static_branch_inc_cpuslocked(&freezer_active);
    }
    mutex_unlock(&freezer_mutex);
    cpus_read_unlock();
    return 0;
    }
//
// freezer_css_offline - initiate destruction of a freezer css
// @css: css being destroyed
//
// @css is going away.  Mark it dead and decrement freezer_active if
// it was holding one.
//
#[no_mangle]
unsafe extern "C" fn freezer_css_offline(css: *mut cgroup_subsys_state) {
    let mut freezer = css_freezer(css);
    cpus_read_lock();
    mutex_lock(&freezer_mutex);
    if (freezer.state & CGROUP_FREEZING) {
    static_branch_dec_cpuslocked(&freezer_active);
    }
    freezer.state = 0;
    mutex_unlock(&freezer_mutex);
    cpus_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn freezer_css_free(css: *mut cgroup_subsys_state) {
    kfree(css_freezer(css));
    }
//
// Tasks can be migrated into a different freezer anytime regardless of its
// current state.  freezer_attach() is responsible for making new tasks
// conform to the current state.
//
// Freezer state changes and task migration are synchronized via
// @freezer->lock.  freezer_attach() makes the new tasks conform to the
// current state and all following state changes can see the new tasks.
//
#[no_mangle]
unsafe extern "C" fn freezer_attach(tset: *mut cgroup_taskset) {
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut new_css: *mut c_void = core::ptr::null_mut();
    mutex_lock(&freezer_mutex);
//
// Make the new tasks conform to the current state of @new_css.
// For simplicity, when migrating any task to a FROZEN cgroup, we
// revert it to FREEZING and let update_if_frozen() determine the
// correct state later.
//
// Tasks in @tset are on @new_css but may not conform to its
// current state before executing the following - !frozen tasks may
// be visible in a FROZEN cgroup and frozen tasks in a THAWED one.
//
    cgroup_taskset_for_each(task, new_css, tset) {
    let mut freezer = css_freezer(new_css);
    if (!(freezer.state & CGROUP_FREEZING)) {
    __thaw_task(task);
    } else {
// clear FROZEN and propagate upwards
    while (freezer && (freezer.state & CGROUP_FROZEN)) {
    freezer.state &= ~CGROUP_FROZEN;
    freezer = parent_freezer(freezer);
    }
    freeze_task(task);
    }
    }
    mutex_unlock(&freezer_mutex);
    }
//
// freezer_fork - cgroup post fork callback
// @task: a task which has just been forked
//
// @task has just been created and should conform to the current state of
// the cgroup_freezer it belongs to.  This function may race against
// freezer_attach().  Losing to freezer_attach() means that we don't have
// to do anything as freezer_attach() will put @task into the appropriate
// state.
//
#[no_mangle]
unsafe extern "C" fn freezer_fork(task: *mut task_struct) {
pub static mut freezer: *mut c_void = core::ptr::null_mut();
//
// The root cgroup is non-freezable, so we can skip locking the
// freezer.  This is safe regardless of race with task migration.
// If we didn't race or won, skipping is obviously the right thing
// to do.  If we lost and root is the new cgroup, noop is still the
// right thing to do.
//
    if (task_css_is_root(task, freezer_cgrp_id)) {
    return;
    }
    mutex_lock(&freezer_mutex);
    rcu_read_lock();
    freezer = task_freezer(task);
    if (freezer.state & CGROUP_FREEZING) {
    freeze_task(task);
    }
    rcu_read_unlock();
    mutex_unlock(&freezer_mutex);
    }
//
// update_if_frozen - update whether a cgroup finished freezing
// @css: css of interest
//
// Once FREEZING is initiated, transition to FROZEN is lazily updated by
// calling this function.  If the current state is FREEZING but not FROZEN,
// this function checks whether all tasks of this cgroup and the descendant
// cgroups finished freezing and, if so, sets FROZEN.
//
// The caller is responsible for grabbing RCU read lock and calling
// update_if_frozen() on all descendants prior to invoking this function.
//
// Task states and freezer state might disagree while tasks are being
// migrated into or out of @css, so we can't verify task states against
// @freezer state here.  See freezer_attach() for details.
//
#[no_mangle]
unsafe extern "C" fn update_if_frozen(css: *mut cgroup_subsys_state) {
    let mut freezer = css_freezer(css);
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut it: usize = 0;
pub static mut task: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&freezer_mutex);
    if (!(freezer.state & CGROUP_FREEZING) ||
    (freezer.state & CGROUP_FROZEN)) {
    return;
    }
// are all (live) children frozen?
    rcu_read_lock();
    css_for_each_child(pos, css) {
    let mut child = css_freezer(pos);
    if ((child.state & CGROUP_FREEZER_ONLINE) &&
    !(child.state & CGROUP_FROZEN)) {
    rcu_read_unlock();
    return;
    }
    }
    rcu_read_unlock();
// are all tasks frozen?
    css_task_iter_start(css, 0, &it);
    while ((task = css_task_iter_next(&it))) {
    if (freezing(task) && !frozen(task)) {
// goto;
    }
    }
    freezer.state |= CGROUP_FROZEN;
// label;
    css_task_iter_end(&it);
    }
#[no_mangle]
unsafe extern "C" fn freezer_read(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut css = seq_css(m), *pos;
    mutex_lock(&freezer_mutex);
    rcu_read_lock();
// update states bottom-up
    css_for_each_descendant_post(pos, css) {
    if (!css_tryget_online(pos)) {
    continue;
    }
    rcu_read_unlock();
    update_if_frozen(pos);
    rcu_read_lock();
    css_put(pos);
    }
    rcu_read_unlock();
    mutex_unlock(&freezer_mutex);
    seq_puts(m, freezer_state_strs(css_freezer(css).state));
    seq_putc(m, '\n');
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn freeze_cgroup(freezer: *mut freezer) {
pub static mut it: usize = 0;
pub static mut task: *mut c_void = core::ptr::null_mut();
    css_task_iter_start(&freezer.css, 0, &it);
    while ((task = css_task_iter_next(&it))) {
    freeze_task(task);
    }
    css_task_iter_end(&it);
    }
#[no_mangle]
unsafe extern "C" fn unfreeze_cgroup(freezer: *mut freezer) {
pub static mut it: usize = 0;
pub static mut task: *mut c_void = core::ptr::null_mut();
    css_task_iter_start(&freezer.css, 0, &it);
    while ((task = css_task_iter_next(&it))) {
    __thaw_task(task);
    }
    css_task_iter_end(&it);
    }
//
// freezer_apply_state - apply state change to a single cgroup_freezer
// @freezer: freezer to apply state change to
// @freeze: whether to freeze or unfreeze
// @state: CGROUP_FREEZING_* flag to set or clear
//
// Set or clear @state on @cgroup according to @freeze, and perform
// freezing or thawing as necessary.
//
#[no_mangle]
pub unsafe extern "C" fn freezer_apply_state(freezer: *mut freezer, freeze: bool, state: c_uint) {
// also synchronizes against task migration, see freezer_attach()
    lockdep_assert_held(&freezer_mutex);
    if (!(freezer.state & CGROUP_FREEZER_ONLINE)) {
    return;
    }
    if (freeze) {
    if (!(freezer.state & CGROUP_FREEZING)) {
    static_branch_inc_cpuslocked(&freezer_active);
    }
    freezer.state |= state;
    freeze_cgroup(freezer);
    } else {
pub static mut was_freezing: bool = false;
    freezer.state &= ~state;
    if (!(freezer.state & CGROUP_FREEZING)) {
    freezer.state &= ~CGROUP_FROZEN;
    if (was_freezing) {
    static_branch_dec_cpuslocked(&freezer_active);
    }
    unfreeze_cgroup(freezer);
    }
    }
    }
//
// freezer_change_state - change the freezing state of a cgroup_freezer
// @freezer: freezer of interest
// @freeze: whether to freeze or thaw
//
// Freeze or thaw @freezer according to @freeze.  The operations are
// recursive - all descendants of @freezer will be affected.
//
#[no_mangle]
unsafe extern "C" fn freezer_change_state(freezer: *mut freezer, freeze: bool) {
pub static mut pos: *mut c_void = core::ptr::null_mut();
    cpus_read_lock();
//
// Update all its descendants in pre-order traversal.  Each
// descendant will try to inherit its parent's FREEZING state as
// CGROUP_FREEZING_PARENT.
//
    mutex_lock(&freezer_mutex);
    rcu_read_lock();
    css_for_each_descendant_pre(pos, &freezer.css) {
    let mut pos_f = css_freezer(pos);
    let mut parent = parent_freezer(pos_f);
    if (!css_tryget_online(pos)) {
    continue;
    }
    rcu_read_unlock();
    if (pos_f == freezer) {
    freezer_apply_state(pos_f, freeze,
    CGROUP_FREEZING_SELF);
    }
    else {
    freezer_apply_state(pos_f,
    parent.state & CGROUP_FREEZING,
    CGROUP_FREEZING_PARENT);
    }
    rcu_read_lock();
    css_put(pos);
    }
    rcu_read_unlock();
    mutex_unlock(&freezer_mutex);
    cpus_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn freezer_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    let mut freeze = 0;
    buf = strstrip(buf);
    if (strcmp(buf, freezer_state_strs(0)) == 0) {
    freeze = false;
    }
if true {
    pr_info_once!("Freezing with imperfect legacy cgroup freezer. "
    "See cgroup.freeze of cgroup v2\n");
    freeze = true;
    } else {
    return -EINVAL;
    }
    freezer_change_state(css_freezer(of_css(of)), freeze);
    return nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn freezer_self_freezing_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    let mut freezer = css_freezer(css);
    return (bool)(freezer.state & CGROUP_FREEZING_SELF);
    }
#[no_mangle]
pub unsafe extern "C" fn freezer_parent_freezing_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    let mut freezer = css_freezer(css);
    return (bool)(freezer.state & CGROUP_FREEZING_PARENT);
    }
pub static mut cftype: usize = 0;
pub static mut cgroup_subsys: usize = 0;