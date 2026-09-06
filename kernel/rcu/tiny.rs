//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/tiny.c
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
// Read-Copy Update mechanism for mutual exclusion, the Bloatwatch edition.
//
// Copyright IBM Corporation, 2008
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
//
// For detailed explanation of Read-Copy Update mechanism see -
// Documentation/RCU
//

// Global control variables for rcupdate callback mechanism.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_ctrlblk {
//     pub /: *mut *mut *mut rcu_head rcucblist; / List of pending callbacks (CBs).,
//     pub /: *mut *mut *mut *mut rcu_head donetail; / ->next pointer of last "done" CB.,
//     pub /: *mut *mut *mut *mut rcu_head curtail; / ->next pointer of last CB.,
//     pub /: *mut *mut unsigned long gp_seq; / Grace-period counter.,
}

// Definition for rcupdate control block.
pub static mut rcu_ctrlblk: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn rcu_barrier() {
    wait_rcu_gp(call_rcu_hurry);
    }
    EXPORT_SYMBOL(rcu_barrier);
// Record an rcu quiescent state.
#[no_mangle]
pub unsafe extern "C" fn rcu_qs() {
    let mut flags = 0;
    local_irq_save(flags);
    if (rcu_ctrlblk.donetail != rcu_ctrlblk.curtail) {
    rcu_ctrlblk.donetail = rcu_ctrlblk.curtail;
    raise_softirq_irqoff(RCU_SOFTIRQ);
    }
    WRITE_ONCE(rcu_ctrlblk.gp_seq, rcu_ctrlblk.gp_seq + 2);
    local_irq_restore(flags);
    }
//
// Check to see if the scheduling-clock interrupt came from an extended
// quiescent state, and, if so, tell RCU about it.  This function must
// be called from hardirq context.  It is normally called from the
// scheduling-clock interrupt.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_sched_clock_irq(user: c_int) {
    if (user) {
    rcu_qs();
    }

    else if (rcu_ctrlblk.donetail != rcu_ctrlblk.curtail) {
    set_need_resched_current();
    }
    }
//
// Reclaim the specified callback, either by invoking it for non-kfree cases or
// freeing it directly (for kfree). Return true if kfreeing, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_reclaim_tiny(head: *mut rcu_head) -> bool {
    let mut f;
    rcu_lock_acquire(&rcu_callback_map);
    trace_rcu_invoke_callback("", head);
    f = head.func;
    debug_rcu_head_callback(head);
    WRITE_ONCE(head.func, (rcu_callback_t)0L);
    f(head);
    rcu_lock_release(&rcu_callback_map);
    return false;
    }
// Invoke the RCU callbacks whose grace period has elapsed.
#[no_mangle]
unsafe extern "C" fn rcu_process_callbacks() -> __latent_entropy void {
    let mut next = core::ptr::null_mut();
    let mut list = core::ptr::null_mut();
    let mut flags = 0;
// Move the ready-to-invoke callbacks to a local list.
    local_irq_save(flags);
    if (rcu_ctrlblk.donetail == &rcu_ctrlblk.rcucblist) {
// No callbacks ready, so just leave.
    local_irq_restore(flags);
    return;
    }
    list = rcu_ctrlblk.rcucblist;
    rcu_ctrlblk.rcucblist = *rcu_ctrlblk.donetail;
// rcu_ctrlblk.donetail = NULL;
    if (rcu_ctrlblk.curtail == rcu_ctrlblk.donetail) {
    rcu_ctrlblk.curtail = &rcu_ctrlblk.rcucblist;
    }
    rcu_ctrlblk.donetail = &rcu_ctrlblk.rcucblist;
    local_irq_restore(flags);
// Invoke the callbacks on the local list.
    while (list) {
    next = list.next;
    prefetch(next);
    debug_rcu_head_unqueue(list);
    rcu_reclaim_tiny(list);
    list = next;
    }
    }
//
// Wait for a grace period to elapse.  But it is illegal to invoke
// synchronize_rcu() from within an RCU read-side critical section.
// Therefore, any legal call to synchronize_rcu() is a quiescent state,
// and so on a UP system, synchronize_rcu() need do nothing, other than
// let the polled APIs know that another grace period elapsed.
//
// (But Lai Jiangshan points out the benefits of doing might_sleep()
// to reduce latency.)
//
// Cool, huh?  (Due to Josh Triplett.)
//
#[no_mangle]
pub unsafe extern "C" fn synchronize_rcu() {
    RCU_LOCKDEP_WARN(lock_is_held(&rcu_bh_lock_map) ||
    lock_is_held(&rcu_lock_map) ||
    lock_is_held(&rcu_sched_lock_map),
    "Illegal synchronize_rcu() in RCU read-side critical section");
    preempt_disable();
    WRITE_ONCE(rcu_ctrlblk.gp_seq, rcu_ctrlblk.gp_seq + 2);
    preempt_enable();
    }
    EXPORT_SYMBOL_GPL(synchronize_rcu);
//
// Post an RCU callback to be invoked after the end of an RCU grace
// period.  But since we have but one CPU, that would be after any
// quiescent state.
//
#[no_mangle]
pub unsafe extern "C" fn call_rcu(head: *mut rcu_head, func: rcu_callback_t) {
    static atomic_t doublefrees;
    let mut flags = 0;
    if (debug_rcu_head_queue(head)) {
    if (atomic_inc_return(&doublefrees) < 4) {
    pr_err!("%s(): Double-freed CB %p.%pS()!!!  ", __func__, head, head.func);
    mem_dump_obj(head);
    }
    return;
    }
    head.func = func;
    head.next = core::ptr::null_mut();
    local_irq_save(flags);
// rcu_ctrlblk.curtail = head;
    rcu_ctrlblk.curtail = &head.next;
    local_irq_restore(flags);
    if (unlikely(is_idle_task(current))) {
// force scheduling for rcu_qs()
    resched_cpu(0);
    }
    }
    EXPORT_SYMBOL_GPL(call_rcu);
//
// Store a grace-period-counter "cookie".  For more information,
// see the Tree RCU header comment.
//
#[no_mangle]
pub unsafe extern "C" fn get_completed_synchronize_rcu_full(gsp: *mut rcu_gp_seq) {
    gsp.norm = RCU_GET_STATE_COMPLETED;
    }
    EXPORT_SYMBOL_GPL(get_completed_synchronize_rcu_full);
//
// Return a grace-period-counter "cookie".  For more information,
// see the Tree RCU header comment.
//
#[no_mangle]
pub unsafe extern "C" fn get_state_synchronize_rcu() -> c_ulong {
    return READ_ONCE(rcu_ctrlblk.gp_seq);
    }
    EXPORT_SYMBOL_GPL(get_state_synchronize_rcu);
//
// Return a grace-period-counter "cookie" and ensure that a future grace
// period completes.  For more information, see the Tree RCU header comment.
//
#[no_mangle]
pub unsafe extern "C" fn start_poll_synchronize_rcu() -> c_ulong {
pub static mut gp_seq: c_ulong = 0;
    if (unlikely(is_idle_task(current))) {
// force scheduling for rcu_qs()
    resched_cpu(0);
    }
    return gp_seq;
    }
    EXPORT_SYMBOL_GPL(start_poll_synchronize_rcu);
//
// Return true if the grace period corresponding to oldstate has completed
// and false otherwise.  For more information, see the Tree RCU header
// comment.
//
#[no_mangle]
pub unsafe extern "C" fn poll_state_synchronize_rcu(oldstate: c_ulong) -> bool {
pub static mut oldstate: return = 0;
    }
    EXPORT_SYMBOL_GPL(poll_state_synchronize_rcu);

#[no_mangle]
pub unsafe extern "C" fn rcutorture_gather_gp_seqs() -> c_ulonglong {
    return READ_ONCE(rcu_ctrlblk.gp_seq) & 0xffffULL;
    }
    EXPORT_SYMBOL_GPL(rcutorture_gather_gp_seqs);
#[no_mangle]
pub unsafe extern "C" fn rcutorture_format_gp_seqs(seqs: c_ulonglong, cp: *mut c_char, len: usize) {
    snprintf(cp, len, "g%04llx", seqs & 0xffffULL);
    }
    EXPORT_SYMBOL_GPL(rcutorture_format_gp_seqs);

#[no_mangle]
pub unsafe extern "C" fn rcu_init()  {
    open_softirq(RCU_SOFTIRQ, rcu_process_callbacks);
    rcu_early_boot_tests();
    tasks_cblist_init_generic();
    }