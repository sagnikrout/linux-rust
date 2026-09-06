//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/sync.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// RCU-based infrastructure for lightweight reader-writer locking
//
// Copyright (c) 2015, Red Hat, Inc.
//
// Author: Oleg Nesterov <oleg@redhat.com>
//

    enum { GP_IDLE = 0, GP_ENTER, GP_PASSED, GP_EXIT, GP_REPLAY };

//
// rcu_sync_init() - Initialize an rcu_sync structure
// @rsp: Pointer to rcu_sync structure to be initialized
//
#[no_mangle]
pub unsafe extern "C" fn rcu_sync_init(rsp: *mut rcu_sync) {
    memset(rsp, 0, sizeof!(*rsp));
    init_waitqueue_head(&rsp.gp_wait);
    }
// forward_decl: rcu_sync_func;
#[no_mangle]
unsafe extern "C" fn rcu_sync_call(rsp: *mut rcu_sync) {
    call_rcu_hurry(&rsp.cb_head, rcu_sync_func);
    }
//
// rcu_sync_func() - Callback function managing reader access to fastpath
// @rhp: Pointer to rcu_head in rcu_sync structure to use for synchronization
//
// This function is passed to call_rcu() function by rcu_sync_enter() and
// rcu_sync_exit(), so that it is invoked after a grace period following the
// that invocation of enter/exit.
//
// If it is called by rcu_sync_enter() it signals that all the readers were
// switched onto slow path.
//
// If it is called by rcu_sync_exit() it takes action based on events that
// have taken place in the meantime, so that closely spaced rcu_sync_enter()
// and rcu_sync_exit() pairs need not wait for a grace period.
//
// If another rcu_sync_enter() is invoked before the grace period
// ended, reset state to allow the next rcu_sync_exit() to let the
// readers back onto their fastpaths (after a grace period).  If both
// another rcu_sync_enter() and its matching rcu_sync_exit() are invoked
// before the grace period ended, re-invoke call_rcu() on behalf of that
// rcu_sync_exit().  Otherwise, set all state back to idle so that readers
// can again use their fastpaths.
//
#[no_mangle]
unsafe extern "C" fn rcu_sync_func(rhp: *mut rcu_head) {
    let mut rsp = container_of!(rhp, rcu_sync, cb_head);
    let mut flags = 0;
    WARN_ON_ONCE!(READ_ONCE(rsp.gp_state) == GP_IDLE);
    WARN_ON_ONCE!(READ_ONCE(rsp.gp_state) == GP_PASSED);
    spin_lock_irqsave(&rsp.rss_lock, flags);
    if (rsp.gp_count) {
//
// We're at least a GP after the GP_IDLE->GP_ENTER transition.
//
    WRITE_ONCE(rsp.gp_state, GP_PASSED);
    wake_up_locked(&rsp.gp_wait);
    } else if (rsp.gp_state == GP_REPLAY) {
//
// A new rcu_sync_exit() has happened; requeue the callback to
// catch a later GP.
//
    WRITE_ONCE(rsp.gp_state, GP_EXIT);
    rcu_sync_call(rsp);
    } else {
//
// We're at least a GP after the last rcu_sync_exit(); everybody
// will now have observed the write side critical section.
// Let 'em rip!
//
    WRITE_ONCE(rsp.gp_state, GP_IDLE);
    }
    spin_unlock_irqrestore(&rsp.rss_lock, flags);
    }
//
// rcu_sync_enter() - Force readers onto slowpath
// @rsp: Pointer to rcu_sync structure to use for synchronization
//
// This function is used by updaters who need readers to make use of
// a slowpath during the update.  After this function returns, all
// subsequent calls to rcu_sync_is_idle() will return false, which
// tells readers to stay off their fastpaths.  A later call to
// rcu_sync_exit() re-enables reader fastpaths.
//
// When called in isolation, rcu_sync_enter() must wait for a grace
// period, however, closely spaced calls to rcu_sync_enter() can
// optimize away the grace-period wait via a state machine implemented
// by rcu_sync_enter(), rcu_sync_exit(), and rcu_sync_func().
//
#[no_mangle]
pub unsafe extern "C" fn rcu_sync_enter(rsp: *mut rcu_sync) {
    let mut gp_state = 0;
    spin_lock_irq(&rsp.rss_lock);
    gp_state = rsp.gp_state;
    if (gp_state == GP_IDLE) {
    WRITE_ONCE(rsp.gp_state, GP_ENTER);
    WARN_ON_ONCE!(rsp.gp_count);
//
// Note that we could simply do rcu_sync_call(rsp) here and
// avoid the "if (gp_state == GP_IDLE)" block below.
//
// However, synchronize_rcu() can be faster if rcu_expedited
// or rcu_blocking_is_gp() is true.
//
// Another reason is that we can't wait for rcu callback if
// we are called at early boot time but this shouldn't happen.
//
    }
    rsp.gp_count += 1;
    spin_unlock_irq(&rsp.rss_lock);
    if (gp_state == GP_IDLE) {
//
// See the comment above, this simply does the "synchronous"
// call_rcu(rcu_sync_func) which does GP_ENTER -> GP_PASSED.
//
    synchronize_rcu();
    rcu_sync_func(&rsp.cb_head);
// Not really needed, wait_event() would see GP_PASSED.
    return;
    }
    wait_event(rsp.gp_wait, READ_ONCE(rsp.gp_state) >= GP_PASSED);
    }
//
// rcu_sync_exit() - Allow readers back onto fast path after grace period
// @rsp: Pointer to rcu_sync structure to use for synchronization
//
// This function is used by updaters who have completed, and can therefore
// now allow readers to make use of their fastpaths after a grace period
// has elapsed.  After this grace period has completed, all subsequent
// calls to rcu_sync_is_idle() will return true, which tells readers that
// they can once again use their fastpaths.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_sync_exit(rsp: *mut rcu_sync) {
    WARN_ON_ONCE!(READ_ONCE(rsp.gp_state) == GP_IDLE);
    spin_lock_irq(&rsp.rss_lock);
    WARN_ON_ONCE!(rsp.gp_count == 0);
    if (!--rsp.gp_count) {
    if (rsp.gp_state == GP_PASSED) {
    WRITE_ONCE(rsp.gp_state, GP_EXIT);
    rcu_sync_call(rsp);
    } else if (rsp.gp_state == GP_EXIT) {
    WRITE_ONCE(rsp.gp_state, GP_REPLAY);
    }
    }
    spin_unlock_irq(&rsp.rss_lock);
    }
//
// rcu_sync_dtor() - Clean up an rcu_sync structure
// @rsp: Pointer to rcu_sync structure to be cleaned up
//
#[no_mangle]
pub unsafe extern "C" fn rcu_sync_dtor(rsp: *mut rcu_sync) {
    let mut gp_state = 0;
    WARN_ON_ONCE!(READ_ONCE(rsp.gp_state) == GP_PASSED);
    spin_lock_irq(&rsp.rss_lock);
    WARN_ON_ONCE!(rsp.gp_count);
    if (rsp.gp_state == GP_REPLAY) {
    WRITE_ONCE(rsp.gp_state, GP_EXIT);
    }
    gp_state = rsp.gp_state;
    spin_unlock_irq(&rsp.rss_lock);
    if (gp_state != GP_IDLE) {
    rcu_barrier();
    WARN_ON_ONCE!(rsp.gp_state != GP_IDLE);
    }
    }