//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/tree_nocb.h
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
// Read-Copy Update mechanism for mutual exclusion (tree-based version)
// Internal non-public definitions that provide either classic
// or preemptible semantics.
//
// Copyright Red Hat, 2009
// Copyright IBM Corporation, 2009
// Copyright SUSE, 2021
//
// Author: Ingo Molnar <mingo@elte.hu>
// Paul E. McKenney <paulmck@linux.ibm.com>
// Frederic Weisbecker <frederic@kernel.org>
//

// Race on early boot between thread creation and assignment
//
// Offload callback processing from the boot-time-specified set of CPUs
// specified by rcu_nocb_mask.  For the CPUs in the set, there are kthreads
// created that pull the callbacks from the corresponding CPU, wait for
// a grace period to elapse, and invoke the callbacks.  These kthreads
// are organized into GP kthreads, which manage incoming callbacks, wait for
// grace periods, and awaken CB kthreads, and the CB kthreads, which only
// invoke callbacks.  Each GP kthread invokes its own CBs.  The no-CBs CPUs
// do a wake_up() on their GP kthread when they insert a callback into any
// empty list, unless the rcu_nocb_poll boot parameter has been specified,
// in which case each kthread actively polls its CPU.  (Which isn't so great
// for energy efficiency, but which does reduce RCU's overhead on that CPU.)
//
// This is intended to be used in conjunction with Frederic Weisbecker's
// adaptive-idle work, which would seriously reduce OS jitter on CPUs
// running CPU-bound user-mode computations.
//
// Offloading of callbacks can also be used as an energy-efficiency
// measure because CPUs with no RCU callbacks queued are more aggressive
// about entering dyntick-idle mode.
//
// Parse the boot-time rcu_nocb_mask CPU list from the kernel parameters.
// If the list is invalid, a warning is emitted and all CPUs are offloaded.
//
// Don't bother bypassing ->cblist if the call_rcu() rate is low.
// After all, the main point of bypassing is to avoid lock contention
// on ->nocb_lock, which only can happen at high call_rcu() rates.
//
// Acquire the specified rcu_data structure's ->nocb_bypass_lock.  If the
// lock isn't immediately available, perform minimal sanity check.
//
// Contention expected only when local enqueue collide with
// remote flush from kthreads.
//
// Conditionally acquire the specified rcu_data structure's
// ->nocb_bypass_lock.
//
extern "C" {
    pub fn raw_spin_trylock(_arg: &rdp->nocb_bypass_lock) -> return;
}
//
// Release the specified rcu_data structure's ->nocb_bypass_lock.
//
// Acquire the specified rcu_data structure's ->nocb_lock, but only
// if it corresponds to a no-CBs CPU.
//
// Release the specified rcu_data structure's ->nocb_lock, but only
// if it corresponds to a no-CBs CPU.
//
// Release the specified rcu_data structure's ->nocb_lock and restore
// interrupts, but only if it corresponds to a no-CBs CPU.
//
// Lockdep check that ->cblist may be safely accessed.
//
// Wake up any no-CBs CPUs' kthreads that were waiting on the just-ended
// grace period.
//
// swait_active() can be checked first because of the following
// ordering, which pairs the smp_mb() in rcu_gp_cleanup() against
// the implicit barrier in prepare_to_swait()/set_current_state()
// on the nocb_gp_wait() side:
//
// rcu_gp_cleanup()                          nocb_gp_wait()
// ---------------                           --------------
// WRITE_ONCE(root->gp_seq, new_gp_seq);     swait_event_interruptible_exclusive(sq)
// smp_mb()                                     prepare_to_swait()
// if swait_active(sq)                             list_add_tail(...)
// swake_up_all(sq)                            set_current_state()
// smp_mb()
// if (poll_state_synchronize_rcu_full())
// ...
//
// Wake NOCB rcuog kthreads on a leaf node so that they can advance
// callbacks that were waiting for the just-completed expedited GP.
//
// The rcuog kthread waiting for a grace period sleeps on the per-leaf-node
// ->nocb_gp_wq[] (not on its rdp_gp's ->nocb_gp_wq, which only signals that
// new callbacks have shown up), so this is the queue that must be woken.
// Both the even and odd waitqueues are woken because the expedited sequence
// does not share parity with the normal ->gp_seq the waiter indexed with.
//
// swait_active() can be checked first because of the following
// ordering, which pairs the smp_mb() in rcu_exp_wait_wake() against
// the implicit barrier in prepare_to_swait()/set_current_state()
// on the nocb_gp_wait() side:
//
// rcu_exp_wait_wake()                          nocb_gp_wait()
// ---------------                              --------------
// rcu_seq_end(&rcu_state.expedited_sequence);  swait_event_interruptible_exclusive(sq)
// smp_mb()                                         prepare_to_swait()
// if swait_active(sq)                                 list_add_tail(...)
// swake_up_all(sq)                                set_current_state()
// smp_mb()
// if (poll_state_synchronize_rcu_full())
// ...
//
// Clear any pending deferred wakeup timer (nocb_gp_lock must be held).
//
// Kick the GP kthread for this NOCB group.
//
extern "C" {
    pub fn __wake_nocb_gp(_arg: rdp_gp, _arg: rdp, _arg: flags) -> return;
}

//
// LAZY_FLUSH_JIFFIES decides the maximum amount of time that
// can elapse before lazy callbacks are flushed. Lazy callbacks
// could be flushed much earlier for a number of other reasons
// however, LAZY_FLUSH_JIFFIES will ensure no lazy callbacks are
// left unsubmitted to RCU after those many jiffies.
//

// To be called only from test code.

//
// Arrange to wake the GP kthread for this NOCB group at some future
// time when it is safe to do so.
//
// Bypass wakeup overrides previous deferments. In case of
// callback storms, no need to wake up too early.
//
// Flush the ->nocb_bypass queue into ->cblist, enqueuing rhp if non-NULL.
// However, if there is a callback to be enqueued and if ->nocb_bypass
// proves to be initially empty, just return false because the no-CB GP
// kthread may need to be awakened in this case.
//
// Return true if there was something to be flushed and it succeeded, otherwise
// false.
//
// Note that this function always returns true if rhp is NULL.
//
// Note: ->cblist.len already accounts for ->nocb_bypass contents.
//
// If the new CB requested was a lazy one, queue it onto the main
// ->cblist so that we can take advantage of the grace-period that will
// happen regardless. But queue it onto the bypass list first so that
// the lazy CB is ordered with the existing CBs in the bypass list.
//
// Flush the ->nocb_bypass queue into ->cblist, enqueuing rhp if non-NULL.
// However, if there is a callback to be enqueued and if ->nocb_bypass
// proves to be initially empty, just return false because the no-CB GP
// kthread may need to be awakened in this case.
//
// Note that this function always returns true if rhp is NULL.
//
extern "C" {
    pub fn rcu_nocb_do_flush_bypass(_arg: rdp, _arg: rhp, _arg: j, _arg: lazy) -> return;
}
//
// If the ->nocb_bypass_lock is immediately available, flush the
// ->nocb_bypass queue into ->cblist.
//
// Determine if the bypass queue needs to be flushed based on time and size.
// For lazy-only bypass queues, use the lazy flush timeout; otherwise flush
// based on jiffy advancement. The flush_faster controls flush aggressiveness.
//
extern "C" {
    pub fn time_after(_arg: j, flush_timeout: bypass_first +) -> return;
}
//
// See whether it is appropriate to use the ->nocb_bypass list in order
// to control contention on ->nocb_lock.  A limited number of direct
// enqueues are permitted into ->cblist per jiffy.  If ->nocb_bypass
// is non-empty, further callbacks must be placed into ->nocb_bypass,
// otherwise rcu_barrier() breaks.  Use rcu_nocb_flush_bypass() to switch
// back to direct use of ->cblist.  However, ->nocb_bypass should not be
// used if ->cblist is empty, because otherwise callbacks can be stranded
// on ->nocb_bypass because we cannot count on the current CPU ever again
// invoking call_rcu().  The general rule is that if ->nocb_bypass is
// non-empty, the corresponding no-CBs grace-period kthread must not be
// in an indefinite sleep state.
//
// Finally, it is not permitted to use the bypass during early boot,
// as doing so would confuse the auto-initialization code.  Besides
// which, there is no point in worrying about lock contention while
// there is only one CPU in operation.
//
// Pure softirq/rcuc based processing: no bypassing, no
// locking.
// was_alldone = !rcu_segcblist_pend_cbs(&rdp->cblist);
// Don't use ->nocb_bypass during early boot.
// was_alldone = !rcu_segcblist_pend_cbs(&rdp->cblist);
// If we have advanced to a new jiffy, reset counts to allow
// moving back from ->nocb_bypass to ->cblist.
// If there hasn't yet been all that many ->cblist enqueues
// this jiffy, tell the caller to enqueue onto ->cblist.  But flush
// ->nocb_bypass first.
// Lazy CBs throttle this back and do immediate bypass queuing.
// was_alldone = !rcu_segcblist_pend_cbs(&rdp->cblist);
// If ->nocb_bypass has been used too long or is too full,
// flush ->nocb_bypass to ->cblist.
// was_alldone = !rcu_segcblist_pend_cbs(&rdp->cblist);
// The flush succeeded and we moved CBs into the regular list.
// Don't wait for the wake up timer as it may be too far ahead.
// Wake up the GP thread now instead, if the cblist was empty.
// We need to use the bypass.
// A wake up of the grace period kthread or timer adjustment
// needs to be done only if:
// 1. Bypass list was fully empty before (this is the first
// bypass list entry), or:
// 2. Both of these conditions are met:
// a. The bypass list previously had only lazy CBs, and:
// b. The new CB is non-lazy.
// No-CBs GP kthread might be indefinitely asleep, if so, wake.
//
// Awaken the no-CBs grace-period kthread if needed due to it legitimately
// being asleep.
//
// If we are being polled or there is no kthread, just leave.
// Need to actually to a wakeup.
// Only lazy CBs in bypass list
// ... if queue was empty ...
// Not enqueued on bypass but locked, do regular enqueue
//
// Locking orders future de-offloaded callbacks enqueue against previous
// handling of this rdp. Ie: Make sure rcuog is done with this rdp before
// deoffloaded callbacks can be enqueued.
//
// Offloading. Set our flag and notify the offload worker.
// We will handle this rdp until it ever gets de-offloaded.
//
// De-offloading. Clear our flag and notify the de-offload worker.
// We will ignore this rdp until it ever gets re-offloaded.
//
// No-CBs GP kthreads come here to wait for additional callbacks to show up
// or for grace periods to end.
//
// Each pass through the following loop checks for CBs and for the
// nearest grace period (if any) to wait for next.  The CB kthreads
// and the global grace-period kthread are awakened if needed.
//
// An rcu_data structure is removed from the list after its
// CPU is de-offloaded and added to the list before that CPU is
// (re-)offloaded.  If the following loop happens to be referencing
// that rcu_data structure during the time that the corresponding
// CPU is de-offloaded and then immediately re-offloaded, this
// loop's rdp pointer will be carried to the end of the list by
// the resulting pair of list operations.  This can cause the loop
// to skip over some of the rcu_data structures that were supposed
// to have been scanned.  Fortunately a new iteration through the
// entire loop is forced after a given CPU's rcu_data structure
// is added to the list, so the skipped-over rcu_data structures
// won't be ignored for long.
//
// Bypass full or old, so flush it.
// Advance callbacks if helpful and low contention.
// Need to wait on some grace period?
//
// Track the earliest pending normal and expedited GP
// across the group so the wait below can be released by
// whichever completes first.
//
// At least one child with non-empty ->nocb_bypass, so set
// timer in order to avoid stranding its callbacks.
// If bypass list only has lazy CBs. Add a deferred lazy wake up.
// Otherwise add a deferred bypass wake up.
// Polling, so trace if first poll in the series.
// Wait for any offloading rdp
// Wait for callbacks to appear.
// (De-)queue an rdp to/from the group if its nocb state is changing
//
// Paranoid locking to make sure nocb_toggling_rdp is well
// reset *before* we (re)set SEGCBLIST_KTHREAD_GP or we could
// race with another round of nocb toggling for this rdp.
// Nocb locking should prevent from that already but we stick
// to paranoia, especially in rare path.
//
// No-CBs grace-period-wait kthread.  There is one of these per group
// of CPUs, but only once at least one CPU in that group has come online
// at least once since boot.  This kthread checks for newly posted
// callbacks from any of the CPUs it is responsible for, waits for a
// grace period, then awakens all of the rcu_nocb_cb_kthread() instances
// that then have callback-invocation work to do.
//
// Invoke any ready callbacks from the corresponding no-CBs CPU,
// then, if there are no more, wait for more to appear.
//
// kthread_park() must be preceded by an rcu_barrier().
// But yet another rcu_barrier() might have sneaked in between
// the barrier callback execution and the callbacks counter
// decrement.
//
// Disable BH to provide the expected environment.  Also, when
// transitioning to/from NOCB mode, a self-requeuing callback might
// be invoked from softirq.  A short grace period could cause both
// instances of this callback would execute concurrently.
//
// Per-rcu_data kthread, but only for no-CBs CPUs.  Repeatedly invoke
// nocb_cb_wait() to do the dirty work.
//
// Each pass through this loop does one callback batch, and,
// if there are no more ready callbacks, waits for them.
// Is a deferred wakeup of rcu_nocb_kthread() required?
// Do a deferred wakeup of rcu_nocb_kthread().
// Do a deferred wakeup of rcu_nocb_kthread() from a timer handler.
//
// Do a deferred wakeup of rcu_nocb_kthread() from fastpath.
// This means we do an inexact common-case check.  Note that if
// we miss, ->nocb_timer will eventually clean things up.
//
extern "C" {
    pub fn do_nocb_deferred_wakeup_common(_arg: rdp_gp, _arg: rdp, _arg: RCU_NOCB_WAKE, _arg: flags) -> return;
}
// Queue this rdp for add/del to/from the list to iterate on rcuog
//
// Locking makes sure rcuog is done handling this rdp before deoffloaded
// enqueue can happen. Also it keeps the SEGCBLIST_OFFLOADED flag stable
// while the ->nocb_lock is held.
//
// CPU must be offline, unless it's early boot
// Flush all callbacks from segcblist and bypass
//
// Make sure the rcuoc kthread isn't in the middle of a nocb locked
// sequence while offloading is deactivated, along with nocb locking.
//
// No kthread to clear the flags for us or remove the rdp from the nocb list
// to iterate. Do it here instead. Locking doesn't look stricly necessary
// but we stick to paranoia in this rare path.
//
// For now we only support re-offload, ie: the rdp must have been
// offloaded on boot first.
//
// Common helper for CPU offload/deoffload operations.
// Already in desired state, nothing to do.
extern "C" {
    pub fn rcu_nocb_cpu_toggle_offload(_arg: cpu, /: *mut *mut false / de-offload) -> return;
}
extern "C" {
    pub fn rcu_nocb_cpu_toggle_offload(_arg: cpu, /: *mut *mut true / offload) -> return;
}

// Protect rcu_nocb_mask against concurrent (de-)offloading.
// Snapshot count of all CPUs
//
// Protect against concurrent (de-)offloading. Otherwise nocb locking
// may be ignored or imbalanced.
//
// But really don't insist if nocb_mutex is contended since we
// can't guarantee that it will never engage in a dependency
// chain involving memory allocation. The lock is seldom contended
// anyway.
//
// Snapshot count of all CPUs
//
// Recheck under the nocb lock. Since we are not holding the bypass
// lock we may still race with increments from the enqueuer but still
// we know for sure if there is at least one lazy callback.
//

// Initialize per-rcu_data variables for no-CBs CPUs.
//
// If the specified CPU is a no-CBs CPU that does not already have its
// rcuo CB kthread, spawn it.  Additionally, if the rcuo GP kthread
// for this CPU's group has not yet been created, spawn it as well.
//
// If there already is an rcuo kthread, then nothing to do.
// If we didn't spawn the GP kthread first, reorganize!
// Spawn the kthread for this CPU.
//
// No need to protect against concurrent rcu_barrier()
// because the number of callbacks should be 0 for a non-boot CPU,
// therefore rcu_barrier() shouldn't even try to grab the nocb_lock.
// But hold nocb_mutex to avoid nocb_lock imbalance from shrinker.
//
// How many CB CPU IDs per GP kthread?  Default of -1 for sqrt(nr_cpu_ids).
//
// Initialize GP-CB relationships for all no-CBs CPU.
//
// Each pass through this loop sets up one rcu_data structure.
// Should the corresponding CPU come online in the future, then
// we will spawn the needed set of rcu_nocb_kthread() kthreads.
//
// New GP kthread, set up for CBs & next GP.
// Another CB kthread, link to previous GP kthread.
//
// Bind the current task to the offloaded CPUs.  If there are no offloaded
// CPUs, leave the task unbound.  Splat if the bind attempt fails.
//
// The ->on_cpu field is available only in CONFIG_SMP=y, so...

//
// Dump out nocb grace-period kthread state for the specified rcu_data
// structure.
//
// Dump out nocb kthread state for the specified rcu_data structure.
// It is OK for GP kthreads to have GP state.

// No ->nocb_lock to acquire.
// No ->nocb_lock to release.
// Lockdep check that ->cblist may be safely accessed.