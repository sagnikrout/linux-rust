//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/srcutiny.c
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
// Sleepable Read-Copy Update mechanism for mutual exclusion,
// tiny version for non-preemptible single-CPU use.
//
// Copyright (C) IBM Corporation, 2017
//
// Author: Paul McKenney <paulmck@linux.ibm.com>
//

    let mut rcu_scheduler_active = 0;

extern "C" { pub static mut rcu_scheduler_active: usize; }

pub static mut srcu_boot_list: usize = 0;
    static bool srcu_init_done;
#[no_mangle]
unsafe extern "C" fn init_srcu_struct_fields(ssp: *mut srcu_struct) -> c_int {
    ssp.srcu_lock_nesting[0] = 0;
    ssp.srcu_lock_nesting[1] = 0;
    init_swait_queue_head(&ssp.srcu_wq);
    ssp.srcu_cb_head = core::ptr::null_mut();
    ssp.srcu_cb_tail = &ssp.srcu_cb_head;
    ssp.srcu_gp_running = false;
    ssp.srcu_gp_waiting = false;
    ssp.srcu_idx = 0;
    ssp.srcu_idx_max = 0;
    INIT_WORK(&ssp.srcu_work, srcu_drive_gp);
    INIT_LIST_HEAD(&ssp.srcu_work.entry);
    init_irq_work(&ssp.srcu_irq_work, srcu_tiny_irq_work);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn init_srcu_struct_lockdep(ssp: *mut srcu_struct, name: *mut c_char, key: *mut lock_class_key) -> c_int {
// Don't re-initialize a lock while it is held.
    debug_check_no_locks_freed(ssp, sizeof!(*ssp));
    lockdep_init_map(&ssp.dep_map, name, key, 0);
    return init_srcu_struct_fields(ssp);
    }
    EXPORT_SYMBOL_GPL(init_srcu_struct_lockdep);

//
// init_srcu_struct_generic - initialize a sleep-RCU structure
// @ssp: structure to initialize.
//
// Must invoke this on a given srcu_struct before passing that srcu_struct
// to any other function.  Each srcu_struct represents a separate domain
// of SRCU protection.
//
#[no_mangle]
pub unsafe extern "C" fn init_srcu_struct_generic(ssp: *mut srcu_struct) -> c_int {
    return init_srcu_struct_fields(ssp);
    }
    EXPORT_SYMBOL_GPL(init_srcu_struct_generic);

//
// cleanup_srcu_struct - deconstruct a sleep-RCU structure
// @ssp: structure to clean up.
//
// Must invoke this after you are finished using a given srcu_struct that
// was initialized via init_srcu_struct(), else you leak memory.
//
#[no_mangle]
pub unsafe extern "C" fn cleanup_srcu_struct(ssp: *mut srcu_struct) {
    WARN_ON!(srcu_readers_active(ssp));
    irq_work_sync(&ssp.srcu_irq_work);
    flush_work(&ssp.srcu_work);
    WARN_ON!(ssp.srcu_gp_running);
    WARN_ON!(ssp.srcu_gp_waiting);
    WARN_ON!(ssp.srcu_cb_head);
    WARN_ON!(&ssp.srcu_cb_head != ssp.srcu_cb_tail);
    WARN_ON!(ssp.srcu_idx != ssp.srcu_idx_max);
    WARN_ON!(ssp.srcu_idx & 0x1);
    }
    EXPORT_SYMBOL_GPL(cleanup_srcu_struct);
//
// Removes the count for the old reader from the appropriate element of
// the srcu_struct.
//
#[no_mangle]
pub unsafe extern "C" fn __srcu_read_unlock(ssp: *mut srcu_struct, idx: c_int) {
    let mut newval = 0;
    preempt_disable();  // Needed for PREEMPT_LAZY
    newval = READ_ONCE(ssp.srcu_lock_nesting[idx]) - 1;
    WRITE_ONCE(ssp.srcu_lock_nesting[idx], newval);
    preempt_enable();
    if (!newval && READ_ONCE(ssp.srcu_gp_waiting) && in_task() && !irqs_disabled()) {
    swake_up_one(&ssp.srcu_wq);
    }
    }
    EXPORT_SYMBOL_GPL(__srcu_read_unlock);
//
// Workqueue handler to drive one grace period and invoke any callbacks
// that become ready as a result.  Single-CPU operation and preemption
// disabling mean that we get away with murder on synchronization.  ;-)
//
#[no_mangle]
pub unsafe extern "C" fn srcu_drive_gp(wp: *mut work_struct) {
    let mut idx = 0;
pub static mut lh: *mut c_void = core::ptr::null_mut();
pub static mut rhp: *mut c_void = core::ptr::null_mut();
pub static mut ssp: *mut c_void = core::ptr::null_mut();
    ssp = container_of!(wp, srcu_struct, srcu_work);
    preempt_disable();  // Needed for PREEMPT_LAZY
    if (ssp.srcu_gp_running || ULONG_CMP_GE(ssp.srcu_idx, READ_ONCE(ssp.srcu_idx_max))) {
    preempt_enable();
    return; /* Already running or nothing to do. */
    }
// Remove recently arrived callbacks and wait for readers.
    WRITE_ONCE(ssp.srcu_gp_running, true);
    local_irq_disable();
    lh = ssp.srcu_cb_head;
    ssp.srcu_cb_head = core::ptr::null_mut();
    ssp.srcu_cb_tail = &ssp.srcu_cb_head;
    local_irq_enable();
    idx = (ssp.srcu_idx & 0x2) / 2;
    WRITE_ONCE(ssp.srcu_idx, ssp.srcu_idx + 1);
    WRITE_ONCE(ssp.srcu_gp_waiting, true);  /* srcu_read_unlock() wakes! */
    preempt_enable();
    do {
// Deadlock issues prevent __srcu_read_unlock() from
// doing an unconditional wakeup, so polling is required.
    swait_event_timeout_exclusive(ssp.srcu_wq,
    !READ_ONCE(ssp.srcu_lock_nesting[idx]), HZ / 10);
    } while (READ_ONCE(ssp.srcu_lock_nesting[idx]));
    preempt_disable();  // Needed for PREEMPT_LAZY
    WRITE_ONCE(ssp.srcu_gp_waiting, false); /* srcu_read_unlock() cheap. */
    WRITE_ONCE(ssp.srcu_idx, ssp.srcu_idx + 1);
    preempt_enable();
// Invoke the callbacks we removed above.
    while (lh) {
    rhp = lh;
    lh = lh.next;
    debug_rcu_head_callback(rhp);
    local_bh_disable();
    rhp.func(rhp);
    local_bh_enable();
    }
//
// Enable rescheduling, and if there are more callbacks,
reschedule ourselves.  This can race with a call_srcu()
// at interrupt level, but the ->srcu_gp_running checks will
// straighten that out.
//
    preempt_disable();  // Needed for PREEMPT_LAZY
    WRITE_ONCE(ssp.srcu_gp_running, false);
    idx = ULONG_CMP_LT(ssp.srcu_idx, READ_ONCE(ssp.srcu_idx_max));
    preempt_enable();
    if (idx) {
    schedule_work(&ssp.srcu_work);
    }
    }
    EXPORT_SYMBOL_GPL(srcu_drive_gp);
//
// Use an irq_work to defer schedule_work() to avoid acquiring the workqueue
// pool->lock while the caller might hold scheduler locks, causing lockdep
// splats due to workqueue_init() doing a wakeup.
//
#[no_mangle]
pub unsafe extern "C" fn srcu_tiny_irq_work(irq_work: *mut irq_work) {
pub static mut ssp: *mut c_void = core::ptr::null_mut();
    ssp = container_of!(irq_work, srcu_struct, srcu_irq_work);
    schedule_work(&ssp.srcu_work);
    }
    EXPORT_SYMBOL_GPL(srcu_tiny_irq_work);
#[no_mangle]
unsafe extern "C" fn srcu_gp_start_if_needed(ssp: *mut srcu_struct) {
    let mut cookie = 0;
    lockdep_assert_preemption_disabled(); // Needed for PREEMPT_LAZY
    cookie = get_state_synchronize_srcu(ssp);
    if (ULONG_CMP_GE(READ_ONCE(ssp.srcu_idx_max), cookie)) {
    return;
    }
    WRITE_ONCE(ssp.srcu_idx_max, cookie);
    if (!READ_ONCE(ssp.srcu_gp_running)) {
    if (likely(srcu_init_done)) {
    irq_work_queue(&ssp.srcu_irq_work);
    }

    else if (list_empty(&ssp.srcu_work.entry)) {
    list_add(&ssp.srcu_work.entry, &srcu_boot_list);
    }
    }
    }
//
// Enqueue an SRCU callback on the specified srcu_struct structure,
// initiating grace-period processing if it is not already running.
//
#[no_mangle]
pub unsafe extern "C" fn call_srcu(ssp: *mut srcu_struct, rhp: *mut rcu_head, func: rcu_callback_t) {
    let mut flags = 0;
    rhp.func = func;
    rhp.next = core::ptr::null_mut();
    preempt_disable();  // Needed for PREEMPT_LAZY
    local_irq_save(flags);
// ssp->srcu_cb_tail = rhp;
    ssp.srcu_cb_tail = &rhp.next;
    local_irq_restore(flags);
    srcu_gp_start_if_needed(ssp);
    preempt_enable();
    }
    EXPORT_SYMBOL_GPL(call_srcu);
//
// synchronize_srcu - wait for prior SRCU read-side critical-section completion
//
#[no_mangle]
pub unsafe extern "C" fn synchronize_srcu(ssp: *mut srcu_struct) {
pub static mut rs: usize = 0;
    srcu_lock_sync(&ssp.dep_map);
    RCU_LOCKDEP_WARN(lockdep_is_held(ssp) ||
    lock_is_held(&rcu_bh_lock_map) ||
    lock_is_held(&rcu_lock_map) ||
    lock_is_held(&rcu_sched_lock_map),
    "Illegal synchronize_srcu() in same-type SRCU (or in RCU) read-side critical section");
    if (rcu_scheduler_active == RCU_SCHEDULER_INACTIVE) {
    return;
    }
    might_sleep();
    init_rcu_head_on_stack(&rs.head);
    init_completion(&rs.completion);
    call_srcu(ssp, &rs.head, wakeme_after_rcu);
    wait_for_completion(&rs.completion);
    destroy_rcu_head_on_stack(&rs.head);
    }
    EXPORT_SYMBOL_GPL(synchronize_srcu);
//
// get_state_synchronize_srcu - Provide an end-of-grace-period cookie
//
#[no_mangle]
pub unsafe extern "C" fn get_state_synchronize_srcu(ssp: *mut srcu_struct) -> c_ulong {
    let mut ret = 0;
    barrier();
    ret = (READ_ONCE(ssp.srcu_idx) + 3) & ~0x1;
    barrier();
    return ret;
    }
    EXPORT_SYMBOL_GPL(get_state_synchronize_srcu);
//
// start_poll_synchronize_srcu - Provide cookie and start grace period
//
// The difference between this and get_state_synchronize_srcu() is that
// this function ensures that the poll_state_synchronize_srcu() will
// eventually return the value true.
//
#[no_mangle]
pub unsafe extern "C" fn start_poll_synchronize_srcu(ssp: *mut srcu_struct) -> c_ulong {
    let mut ret = 0;
    preempt_disable();  // Needed for PREEMPT_LAZY
    ret = get_state_synchronize_srcu(ssp);
    srcu_gp_start_if_needed(ssp);
    preempt_enable();
    return ret;
    }
    EXPORT_SYMBOL_GPL(start_poll_synchronize_srcu);
//
// poll_state_synchronize_srcu - Has cookie's grace period ended?
//
#[no_mangle]
pub unsafe extern "C" fn poll_state_synchronize_srcu(ssp: *mut srcu_struct, cookie: c_ulong) -> bool {
pub static mut cur_s: c_ulong = 0;
    barrier();
    return cookie == SRCU_GET_STATE_COMPLETED ||
    ULONG_CMP_GE(cur_s, cookie) || ULONG_CMP_LT(cur_s, cookie - 3);
    }
    EXPORT_SYMBOL_GPL(poll_state_synchronize_srcu);

// Lockdep diagnostics.
#[no_mangle]
pub unsafe extern "C" fn rcu_scheduler_starting()  {
    rcu_scheduler_active = RCU_SCHEDULER_RUNNING;
    }

//
// Queue work for srcu_struct structures with early boot callbacks.
// The work won't actually execute until the workqueue initialization
// phase that takes place after the scheduler starts.
//
#[no_mangle]
pub unsafe extern "C" fn srcu_init()  {
pub static mut ssp: *mut c_void = core::ptr::null_mut();
    srcu_init_done = true;
    while (!list_empty(&srcu_boot_list)) {
    ssp = list_first_entry(&srcu_boot_list, srcu_struct, srcu_work.entry);
    list_del_init(&ssp.srcu_work.entry);
    schedule_work(&ssp.srcu_work);
    }
    }