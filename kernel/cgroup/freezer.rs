//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/freezer.c
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


// SPDX-License-Identifier: GPL-2.0

//
// Update CGRP_FROZEN of cgroup.flag
// Return true if flags is updated; false if flags has no change
//
#[no_mangle]
unsafe extern "C" fn cgroup_update_frozen_flag(cgrp: *mut cgroup, frozen: bool) -> bool {
    lockdep_assert_held(&css_set_lock);
// Already there?
    if (test_bit(CGRP_FROZEN, &cgrp.flags) == frozen) {
    return false;
    }
    if (frozen) {
    set_bit(CGRP_FROZEN, &cgrp.flags);
    }
    else {
    clear_bit(CGRP_FROZEN, &cgrp.flags);
    }
    cgroup_file_notify(&cgrp.events_file);
    TRACE_CGROUP_PATH(notify_frozen, cgrp, frozen);
    return true;
    }
//
// Propagate the cgroup frozen state upwards by the cgroup tree.
//
#[no_mangle]
unsafe extern "C" fn cgroup_propagate_frozen(cgrp: *mut cgroup, frozen: bool) {
pub static mut desc: c_int = 1;
//
// If the new state is frozen, some freezing ancestor cgroups may change
// their state too, depending on if all their descendants are frozen.
//
// Otherwise, all ancestor cgroups are forced into the non-frozen state.
//
    while ((cgrp = cgroup_parent(cgrp))) {
    if (frozen) {
    cgrp.freezer.nr_frozen_descendants += desc;
    if (!test_bit(CGRP_FREEZE, &cgrp.flags) ||
    (cgrp.freezer.nr_frozen_descendants !=
    cgrp.nr_descendants)) {
    continue;
    }
    } else {
    cgrp.freezer.nr_frozen_descendants -= desc;
    }
    if (cgroup_update_frozen_flag(cgrp, frozen)) {
    desc += 1;
    }
    }
    }
//
// Revisit the cgroup frozen state.
// Checks if the cgroup is really frozen and perform all state transitions.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_update_frozen(cgrp: *mut cgroup) {
    let mut frozen = 0;
//
// If the cgroup has to be frozen (CGRP_FREEZE bit set),
// and all tasks are frozen and/or stopped, let's consider
// the cgroup frozen. Otherwise it's not frozen.
//
    frozen = test_bit(CGRP_FREEZE, &cgrp.flags) &&
    cgrp.freezer.nr_frozen_tasks == __cgroup_task_count(cgrp);
// If flags is updated, update the state of ancestor cgroups.
    if (cgroup_update_frozen_flag(cgrp, frozen)) {
    cgroup_propagate_frozen(cgrp, frozen);
    }
    }
//
// Increment cgroup's nr_frozen_tasks.
//
#[no_mangle]
unsafe extern "C" fn cgroup_inc_frozen_cnt(cgrp: *mut cgroup) {
    cgrp.freezer.nr_frozen_tasks += 1;
    }
//
// Decrement cgroup's nr_frozen_tasks.
//
#[no_mangle]
unsafe extern "C" fn cgroup_dec_frozen_cnt(cgrp: *mut cgroup) {
    cgrp.freezer.nr_frozen_tasks -= 1;
    WARN_ON_ONCE!(cgrp.freezer.nr_frozen_tasks < 0);
    }
//
// Enter frozen/stopped state, if not yet there. Update cgroup's counters,
// and revisit the state of the cgroup, if necessary.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_enter_frozen() {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    if (current.frozen) {
    return;
    }
    spin_lock_irq(&css_set_lock);
    current.frozen = true;
    cgrp = task_dfl_cgroup(current);
    cgroup_inc_frozen_cnt(cgrp);
    cgroup_update_frozen(cgrp);
    spin_unlock_irq(&css_set_lock);
    }
//
// Conditionally leave frozen/stopped state. Update cgroup's counters,
// and revisit the state of the cgroup, if necessary.
//
// If always_leave is not set, and the cgroup is freezing,
// we're racing with the cgroup freezing. In this case, we don't
// drop the frozen counter to avoid a transient switch to
// the unfrozen state.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_leave_frozen(always_leave: bool) {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    spin_lock_irq(&css_set_lock);
    cgrp = task_dfl_cgroup(current);
    if (always_leave || !test_bit(CGRP_FREEZE, &cgrp.flags)) {
    cgroup_dec_frozen_cnt(cgrp);
    cgroup_update_frozen(cgrp);
    WARN_ON_ONCE!(!current.frozen);
    current.frozen = false;
    } else if (!(current.jobctl & JOBCTL_TRAP_FREEZE)) {
    spin_lock(&current.sighand.siglock);
    current.jobctl |= JOBCTL_TRAP_FREEZE;
    set_thread_flag(TIF_SIGPENDING);
    spin_unlock(&current.sighand.siglock);
    }
    spin_unlock_irq(&css_set_lock);
    }
//
// Freeze or unfreeze the task by setting or clearing the JOBCTL_TRAP_FREEZE
// jobctl bit.
//
#[no_mangle]
unsafe extern "C" fn cgroup_freeze_task(task: *mut task_struct, freeze: bool) {
    let mut flags = 0;
// If the task is about to die, don't bother with freezing it.
    if (!lock_task_sighand(task, &flags)) {
    return;
    }
    if (freeze) {
    task.jobctl |= JOBCTL_TRAP_FREEZE;
    signal_wake_up(task, false);
    } else {
    task.jobctl &= ~JOBCTL_TRAP_FREEZE;
    wake_up_process(task);
    }
    unlock_task_sighand(task, &flags);
    }
//
// Freeze or unfreeze all tasks in the given cgroup.
//
#[no_mangle]
unsafe extern "C" fn cgroup_do_freeze(cgrp: *mut cgroup, freeze: bool, ts_nsec: u64) {
pub static mut it: usize = 0;
pub static mut task: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&cgroup_mutex);
    spin_lock_irq(&css_set_lock);
    write_seqcount_begin(&cgrp.freezer.freeze_seq);
    if (freeze) {
    set_bit(CGRP_FREEZE, &cgrp.flags);
    cgrp.freezer.freeze_start_nsec = ts_nsec;
    } else {
    clear_bit(CGRP_FREEZE, &cgrp.flags);
    cgrp.freezer.frozen_nsec += (ts_nsec -
    cgrp.freezer.freeze_start_nsec);
    }
    write_seqcount_end(&cgrp.freezer.freeze_seq);
    spin_unlock_irq(&css_set_lock);
    if (freeze) {
    TRACE_CGROUP_PATH(freeze, cgrp);
    }
    else {
    TRACE_CGROUP_PATH(unfreeze, cgrp);
    }
    css_task_iter_start(&cgrp.self, 0, &it);
    while ((task = css_task_iter_next(&it))) {
//
// Ignore kernel threads here. Freezing cgroups containing
// kthreads isn't supported.
//
    if (task.flags & PF_KTHREAD) {
    continue;
    }
    cgroup_freeze_task(task, freeze);
    }
    css_task_iter_end(&it);
//
// Cgroup state should be revisited here to cover empty leaf cgroups
// and cgroups which descendants are already in the desired state.
//
    spin_lock_irq(&css_set_lock);
    if (cgrp.nr_descendants == cgrp.freezer.nr_frozen_descendants) {
    cgroup_update_frozen(cgrp);
    }
    spin_unlock_irq(&css_set_lock);
    }
//
// Adjust the task state (freeze or unfreeze) and revisit the state of
// source and destination cgroups.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_freezer_migrate_task(task: *mut task_struct, src: *mut cgroup, dst: *mut cgroup) {
    lockdep_assert_held(&css_set_lock);
//
// Kernel threads are not supposed to be frozen at all.
//
    if (task.flags & PF_KTHREAD) {
    return;
    }
//
// It's not necessary to do changes if both of the src and dst cgroups
// are not freezing and task is not frozen.
//
    if (!test_bit(CGRP_FREEZE, &src.flags) &&
    !test_bit(CGRP_FREEZE, &dst.flags) &&
    !task.frozen) {
    return;
    }
//
// Adjust counters of freezing and frozen tasks.
// Note, that if the task is frozen, but the destination cgroup is not
// frozen, we bump both counters to keep them balanced.
//
    if (task.frozen) {
    cgroup_inc_frozen_cnt(dst);
    cgroup_dec_frozen_cnt(src);
    }
    cgroup_update_frozen(dst);
    cgroup_update_frozen(src);
//
// Force the task to the desired state.
//
    cgroup_freeze_task(task, test_bit(CGRP_FREEZE, &dst.flags));
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_freeze(cgrp: *mut cgroup, freeze: bool) {
pub static mut css: *mut c_void = core::ptr::null_mut();
pub static mut parent: *mut c_void = core::ptr::null_mut();
pub static mut dsct: *mut c_void = core::ptr::null_mut();
pub static mut applied: bool = false;
    let mut ts_nsec = 0;
    let mut old_e = 0;
    lockdep_assert_held(&cgroup_mutex);
//
// Nothing changed? Just exit.
//
    if (cgrp.freezer.freeze == freeze) {
    return;
    }
    cgrp.freezer.freeze = freeze;
    ts_nsec = ktime_get_ns();
//
// Propagate changes downwards the cgroup tree.
//
    css_for_each_descendant_pre(css, &cgrp.self) {
    dsct = css.cgroup;
    if (cgroup_is_dead(dsct)) {
    continue;
    }
//
// e_freeze is affected by parent's e_freeze and dst's freeze.
// If old e_freeze eq new e_freeze, no change, its children
// will not be affected. So do nothing and skip the subtree
//
    old_e = dsct.freezer.e_freeze;
    parent = cgroup_parent(dsct);
    dsct.freezer.e_freeze = (dsct.freezer.freeze ||
    parent.freezer.e_freeze);
    if (dsct.freezer.e_freeze == old_e) {
    css = css_rightmost_descendant(css);
    continue;
    }
//
// Do change actual state: freeze or unfreeze.
//
    cgroup_do_freeze(dsct, freeze, ts_nsec);
    applied = true;
    }
//
// Even if the actual state hasn't changed, let's notify a user.
// The state can be enforced by an ancestor cgroup: the cgroup
// can already be in the desired state or it can be locked in the
// opposite state, so that the transition will never happen.
// In both cases it's better to notify a user, that there is
// nothing to wait for.
//
    if (!applied) {
    TRACE_CGROUP_PATH(notify_frozen, cgrp,
    test_bit(CGRP_FROZEN, &cgrp.flags));
    cgroup_file_notify(&cgrp.events_file);
    }
    }