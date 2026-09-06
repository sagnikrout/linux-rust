//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/rcu.h
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
// Read-Copy Update definitions shared among RCU implementations.
//
// Copyright IBM Corporation, 2011
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
//

//
// Grace-period counter management.
//
// The two least significant bits contain the control flags.
// The most significant bits contain the grace-period sequence counter.
//
// When both control flags are zero, no grace period is in progress.
// When either bit is non-zero, a grace period has started and is in
// progress. When the grace period completes, the control flags are reset
// to 0 and the grace-period sequence counter is incremented.
//
// However some specific RCU usages make use of custom values.
//
// SRCU special control values:
//
// SRCU_SNP_INIT_SEQ	:	Invalid/init value set when SRCU node
// is initialized.
//
// SRCU_STATE_IDLE		:	No SRCU gp is in progress
//
// SRCU_STATE_SCAN1	:	State set by rcu_seq_start(). Indicates
// we are scanning the readers on the slot
// defined as inactive (there might well
// be pending readers that will use that
// index, but their number is bounded).
//
// SRCU_STATE_SCAN2	:	State set manually via rcu_seq_set_state()
// Indicates we are flipping the readers
// index and then scanning the readers on the
// slot newly designated as inactive (again,
// the number of pending readers that will use
// this inactive index is bounded).
//
// RCU polled GP special control values:
//
// RCU_GET_STATE_COMPLETED :	State value indicating an already-completed
// polled GP has completed.  This value covers
// both the state and the counter of the
// grace-period sequence number.
//
// RCU_GET_STATE_NOT_TRACKED :	State value indicating that a GP component
// is not tracked by this subsystem and should
// not be checked.  Used by SRCU and RCU Tasks
// which do not track expedited GPs, to prevent
// false-positive completion when their
// gp_seq entries are checked via
// poll_state_synchronize_rcu_full().
//
// Low-order bit definitions for polled grace-period APIs.
pub const RCU_GET_STATE_COMPLETED: c_uint = 0x1;
pub const RCU_GET_STATE_NOT_TRACKED: c_uint = 0x2;
// A complete grace period count

//
// Return the counter portion of a sequence number previously returned
// by rcu_seq_snap() or rcu_seq_current().
//
// Return the state portion of a sequence number previously returned
// by rcu_seq_snap() or rcu_seq_current().
//
// Set the state portion of the pointed-to sequence number.
// The caller is responsible for preventing conflicting updates.
//
// Adjust sequence number for start of update-side operation.
// Compute the end-of-grace-period value for the specified sequence number.
// Adjust sequence number for end of update-side operation.
//
// rcu_seq_snap - Take a snapshot of the update side's sequence number.
//
// This function returns the earliest value of the grace-period sequence number
// that will indicate that a full grace period has elapsed since the current
// time.  Once the grace-period sequence number has reached this value, it will
// be safe to invoke all callbacks that have been registered prior to the
// current time. This value is the current grace-period number plus two to the
// power of the number of low-order bits reserved for state, then rounded up to
// the next value in which the state bits are all zero.
//
// Return the current value the update side's sequence number, no ordering.
extern "C" {
    pub fn READ_ONCE(_arg: *mut sp) -> return;
}
//
// Given a snapshot from rcu_seq_snap(), determine whether or not the
// corresponding update-side operation has started.
//
extern "C" {
    pub fn ULONG_CMP_LT(~RCU_SEQ_STATE_MASK: (s - 1) &, _arg: *mut READ_ONCE(sp)) -> return;
}
//
// Given a snapshot from rcu_seq_snap(), determine whether or not a
// full update-side operation has occurred.
//
extern "C" {
    pub fn ULONG_CMP_GE(_arg: *mut READ_ONCE(sp), _arg: s) -> return;
}
//
// Given a snapshot from rcu_seq_snap(), determine whether or not a
// full update-side operation has occurred, but do not allow the
// (ULONG_MAX / 2) safety-factor/guard-band.
//
// The token returned by get_state_synchronize_rcu_full() is based on
// rcu_state.gp_seq but it is tested in poll_state_synchronize_rcu_full()
// against the root rnp->gp_seq. Since rcu_seq_start() is first called
// on rcu_state.gp_seq and only later reflected on the root rnp->gp_seq,
// it is possible that rcu_seq_snap(rcu_state.gp_seq) returns 2 full grace
// periods ahead of the root rnp->gp_seq. To prevent false-positives with the
// full polling API that a wrap around instantly completed the GP, when nothing
// like that happened, adjust for the 2 GPs in the ULONG_CMP_LT().
//
extern "C" {
    pub fn ULONG_CMP_GE(_arg: cur_s, ULONG_CMP_LT(cur_s: s) ||, RCU_SEQ_GP): *mut *mut s - (2) -> return;
}
//
// Has a grace period completed since the time the old gp_seq was collected?
//
extern "C" {
    pub fn ULONG_CMP_LT(_arg: old, ~RCU_SEQ_STATE_MASK: new &) -> return;
}
//
// Has a grace period started since the time the old gp_seq was collected?
//
// Roughly how many full grace periods have elapsed between the collection
// of the two specified grace periods?
//
// Compute the number of grace periods (still shifted up), plus
// one if either of new and old is not an exact grace period.
//
// debug_rcu_head_queue()/debug_rcu_head_unqueue() are used internally
// by call_rcu() and rcu callback execution, and are therefore not part
// of the RCU API. These are in rcupdate.h because they are used by all
// RCU implementations.
//

extern "C" {
    pub fn rcu_jiffies_till_stall_check() -> c_int;
}
extern "C" {
    pub fn rcu_exp_jiffies_till_stall_check() -> c_int;
}

extern "C" {
    pub fn rcu_stall_is_suppressed_at_boot() -> return;
}
// Macro flag: #define rcu_ftrace_dump_stall_suppress()
// Macro flag: #define rcu_ftrace_dump_stall_unsuppress()

//
// Strings used in tracepoints need to be exported via the
// tracing system such that tools like perf and trace-cmd can
// translate the string address pointers to actual text.
//

//
// Dump the ftrace buffer, but only one time per callsite per boot.
//

extern "C" {
    pub fn rcu_early_boot_tests();
}
extern "C" {
    pub fn rcu_test_sync_prims();
}
//
// This function really isn't for public consumption, but RCU is special in
// that context switches can allow the state machine to make progress.
//
extern "C" {
    pub fn resched_cpu(cpu: c_int);
}

//
// Compute the per-level fanout, either using the exact fanout specified
// or balancing the tree, depending on the rcu_fanout_exact boot parameter.
//
extern "C" {
    pub fn rcu_init_geometry();
}
// Returns a pointer to the first leaf rcu_node structure.

// Is this rcu_node a leaf?

// Is this rcu_node the last leaf?

//
// Do a full breadth-first scan of the {s,}rcu_node structures for the
// specified state structure (for SRCU) or the only rcu_state structure
// (for RCU).
//

//
// Scan the leaves of the rcu_node hierarchy for the rcu_state structure.
// Note that if there is a singleton rcu_node tree with but one rcu_node
// structure, this loop -will- visit the rcu_node structure.  It is still
// a leaf node, even if it is also the root node.
//

//
// Iterate over all possible CPUs in a leaf RCU node.
//

//
// Iterate over all CPUs in a leaf RCU node's specified mask.
//

//
// Wrappers for the rcu_node::lock acquire and release.
//
// Because the rcu_nodes form a tree, the tree traversal locking will observe
// different lock values, this in turn means that an UNLOCK of one level
// followed by a LOCK of another level does not imply a full memory barrier;
// and most importantly transitivity is lost.
//
// In order to restore full ordering between tree levels, augment the regular
// lock acquire functions with smp_mb__after_unlock_lock().
//
// As ->lock of struct rcu_node is a __private field, therefore one should use
// these wrappers rather than directly call raw_spin_{lock,unlock}* on ->lock.
//

// Tiny RCU doesn't expedite, as its purpose in life is instead to be tiny.

extern "C" {
    pub fn rcu_expedite_gp();
}
extern "C" {
    pub fn rcu_unexpedite_gp();
}
extern "C" {
    pub fn rcu_async_hurry();
}
extern "C" {
    pub fn rcu_async_relax();
}
extern "C" {
    pub fn rcupdate_announce_bootup_oddness();
}
extern "C" {
    pub fn rcu_cpu_online(cpu: c_int) -> bool;
}

extern "C" {
    pub fn show_rcu_tasks_gp_kthreads();
}

extern "C" {
    pub fn rcu_tasks_get_gp_data(flags: *mut c_int, gp_seq: *mut c_ulong);
}

extern "C" {
    pub fn rcu_tasks_rude_get_gp_data(flags: *mut c_int, gp_seq: *mut c_ulong);
}

extern "C" {
    pub fn tasks_cblist_init_generic();
}

pub const RCU_SCHEDULER_INACTIVE: c_int = 0;
pub const RCU_SCHEDULER_INIT: c_int = 1;
pub const RCU_SCHEDULER_RUNNING: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rcutorture_type {
    RCU_FLAVOR,
    RCU_TASKS_FLAVOR,
    RCU_TASKS_RUDE_FLAVOR,
    RCU_TASKS_TRACING_FLAVOR,
    RCU_TRIVIAL_FLAVOR,
    SRCU_FLAVOR,
    INVALID_RCU_FLAVOR
}

extern "C" {
    pub fn rcu_get_jiffies_lazy_flush() -> c_ulong;
}
extern "C" {
    pub fn rcu_set_jiffies_lazy_flush(j: c_ulong);
}

extern "C" {
    pub fn rcutorture_get_gp_data(flags: *mut c_int, gp_seq: *mut c_ulong);
}
extern "C" {
    pub fn rcu_gp_set_torture_wait(duration: c_int);
}
extern "C" {
    pub fn rcu_set_gpwrap_lag(lag: c_ulong);
}
extern "C" {
    pub fn rcu_get_gpwrap_count(cpu: c_int) -> c_int;
}

// flags = 0;
// gp_seq = 0;

extern "C" {
    pub fn rcutorture_gather_gp_seqs() -> c_ulonglong;
}
extern "C" {
    pub fn rcutorture_format_gp_seqs(seqs: c_ulonglong, cp: *mut c_char, len: usize);
}

// flags = 0;
// gp_seq = sp->srcu_idx;

extern "C" {
    pub fn rcu_watching_zero_in_eqs(cpu: c_int, vp: *mut c_int) -> bool;
}
extern "C" {
    pub fn rcu_get_gp_seq() -> c_ulong;
}
extern "C" {
    pub fn rcu_exp_batches_completed() -> c_ulong;
}
extern "C" {
    pub fn rcu_check_boost_fail(gp_state: c_ulong, cpup: *mut c_int) -> bool;
}
extern "C" {
    pub fn show_rcu_gp_kthreads();
}
extern "C" {
    pub fn rcu_get_gp_kthreads_prio() -> c_int;
}
extern "C" {
    pub fn rcu_fwd_progress_check(j: c_ulong);
}
extern "C" {
    pub fn rcu_force_quiescent_state();
}
extern "C" {
    pub fn rcu_gp_slow_register(rgssp: *mut core::sync::atomic::AtomicI32);
}
extern "C" {
    pub fn rcu_gp_slow_unregister(rgssp: *mut core::sync::atomic::AtomicI32);
}

extern "C" {
    pub fn srcu_batches_completed(sp: *mut srcu_struct) -> c_ulong;
}

extern "C" {
    pub fn rcu_bind_current_to_nocb();
}

extern "C" {
    pub fn show_rcu_tasks_classic_gp_kthread();
}

extern "C" {
    pub fn show_rcu_tasks_rude_gp_kthread();
}

extern "C" {
    pub fn rcu_cpu_beenfullyonline(cpu: c_int) -> bool;
}

extern "C" {
    pub fn rcu_stall_notifier_call_chain(val: c_ulong, v: *mut c_void) -> c_int;
}

extern "C" {
    pub fn synchronize_rcu_trivial_preempt();
}

extern "C" {
    pub fn rcu_is_task_rcu_boosted() -> bool;
}