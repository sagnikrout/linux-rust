//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/tree_stall.h
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
// RCU CPU stall warnings for normal RCU grace periods
//
// Copyright IBM Corporation, 2019
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
//

//
// Controlling CPU stall warnings, including delay calculation.
// panic() on RCU Stall sysctl.

extern "C" {
    pub fn sysfs_emit(_arg: page, _arg: "%u\n", _arg: rcu_stall_count) -> return;
}

pub const RCU_STALL_DELAY_DELTA: c_int = 0;

pub const RCU_STALL_MIGHT_DIV: c_int = 8;

// Zero says to use rcu_cpu_stall_timeout, but in milliseconds.
// Limit check must be consistent with the Kconfig limits for
// CONFIG_RCU_EXP_CPU_STALL_TIMEOUT, so check the allowed range.
// The minimum clamped value is "2UL", because at least one full
// tick has to be guaranteed.

// Add extra ~25% out of till_stall_check.

// Limit-check stall timeouts specified at boottime and runtime.
//
// Limit check must be consistent with the Kconfig limits
// for CONFIG_RCU_CPU_STALL_TIMEOUT.
//
// Don't do RCU CPU stall warnings during long sysrq printouts.
// Don't print RCU CPU stall warnings during a kernel panic.
// If so specified via sysctl, panic, yielding cleaner stall-warning output.
//
// Attempt to kick out the BPF scheduler if it's installed and defer
// the panic to give the system a chance to recover.
//
// rcu_cpu_stall_reset - restart stall-warning timeout for current grace period
//
// To perform the reset request from the caller, disable stall detection until
// 3 fqs loops have passed. This is required to ensure a fresh jiffies is
// loaded.  It should be safe to do from the fqs loop as enough timer
// interrupts and context switches should have passed.
//
// The caller must disable hard irqs.
//
// Interaction with RCU grace periods
// Start of new grace period, so record stall time (and forcing times).
// Zero ->ticks_this_gp and snapshot the number of RCU softirq handlers.
//
// If too much time has passed in the current grace period, and if
// so configured, go kick the relevant kthreads.
//
// Handler for the irq_work request posted about halfway into the RCU CPU
// stall timeout, and used to detect excessive irq disabling.  Set state
// appropriately, but just complain if there is unexpected state on entry.
//
// Printing RCU CPU stall warnings

//
// Dump detailed information for all tasks blocking the current RCU
// grace period on the specified rcu_node structure.
//
// We could be printing a lot while holding a spinlock.
// Avoid triggering hard lockup.
//
// Communicate task state back to the RCU CPU stall warning request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_stall_chk_rdr {
    pub nesting: c_int,
    pub rs: rcu_special,
    pub on_blkd_list: bool,
}

//
// Report out the state of a not-running task that is stalling the
// current RCU grace period.
//
// Scan the current list of tasks blocked within RCU read-side critical
// sections, printing out the tid of each of the first few of them.
//

//
// Because preemptible RCU does not exist, we never have to check for
// tasks blocked within RCU read-side critical sections.
//
// Because preemptible RCU does not exist, we never have to check for
// tasks blocked within RCU read-side critical sections.
//

//
// Dump stacks of all tasks running on stalled CPUs.  First try using
// NMIs, but fall back to manual remote stack tracing on architectures
// that don't support NMI-based stack dumps.  The NMI-triggered stack
// traces are more accurate because they are printed by the target CPU.
//
// Convert a ->gp_state value to a character string.
//
// Is the RCU grace-period kthread being starved of CPU time?
// jp = j;
//
// Print out diagnostic information for the specified stalled CPU.
//
// If the specified CPU is aware of the current RCU grace period, then
// print the number of scheduling clock interrupts the CPU has taken
// during the time that it has been aware.  Otherwise, print the number
// of RCU grace periods that this CPU is ignorant of, for example, "1"
// if the CPU was aware of the previous grace period.
//
// Also print out idle info.
//
// We could be printing a lot while holding a spinlock.  Avoid
// triggering hard lockup.
//
// Print signed value, as negative values indicate a probable bug.
// Complain about starvation of grace-period kthread.
// Complain about missing wakeups from expired fqs wait timer
//
// Order reads of .gp_state and .jiffies_force_qs.
// Matching smp_wmb() is present in rcu_gp_fqs_loop().
//
// Kick and suppress, if so configured.
//
// OK, time to rat on our buddy...
// See Documentation/RCU/stallwarn.rst for info on how to debug
// RCU CPU stall warnings.
//
// Complain about tasks blocking the grace period.
// Rewrite if needed in case of slow consoles.
// Kick and suppress, if so configured.
//
// OK, time to rat on ourselves...
// See Documentation/RCU/stallwarn.rst for info on how to debug
// RCU CPU stall warnings.
//
// Rewrite if needed in case of slow consoles.
//
// Attempt to revive the RCU machinery by forcing a context switch.
//
// A context switch would normally allow the RCU state machine to make
// progress and it could be we're stuck in kernel space without context
// switches for an entirely unreasonable amount of time.
//
// Check if it was requested (via rcu_cpu_stall_reset()) that the FQS
// loop has to set jiffies to ensure a non-stale jiffies value. This
// is required to have good jiffies value after coming out of long
// breaks of jiffies updates. Not doing so can cause false positives.
//
// Lots of memory barriers to reject false positives.
//
// The idea is to pick up rcu_state.gp_seq, then
// rcu_state.jiffies_stall, then rcu_state.gp_start, and finally
// another copy of rcu_state.gp_seq.  These values are updated in
// the opposite order with memory barriers (or equivalent) during
// grace-period initialization and cleanup.  Now, a false positive
// can occur if we get an new value of rcu_state.gp_start and a old
// value of rcu_state.jiffies_stall.  But given the memory barriers,
// the only way that this can happen is if one grace period ends
// and another starts between these two fetches.  This is detected
// by comparing the second fetch of rcu_state.gp_seq with the
// previous fetch from rcu_state.gp_seq.
//
// Given this check, comparisons of jiffies, rcu_state.jiffies_stall,
// and rcu_state.gp_start suffice to forestall false positives.
//
// If a virtual machine is stopped by the host it can look to
// the watchdog like an RCU stall. Check to see if the host
// stopped the vm.
//

// We haven't checked in, so go dump stack.
// They had a few time units to dump stack, so complain.
//
// RCU forward-progress mechanisms, including for callback invocation.
//
// Check to see if a failure to end RCU priority inversion was due to
// a CPU not passing through a quiescent state.  When this happens, there
// is nothing that RCU priority boosting can do to help, so we shouldn't
// count this as an RCU priority boosting failure.  A return of true says
// RCU priority boosting is to blame, and false says otherwise.  If false
// is returned, the first of the CPUs to blame is stored through cpup.
// If there was no CPU blocking the current grace period, but also nothing
// in need of being boosted, *cpup is set to -1.  This can happen in case
// of vCPU preemption while the last CPU is reporting its quiscent state,
// for example.
//
// If cpup is NULL, then a lockless quick check is carried out, suitable
// for high-rate usage.  On the other hand, if cpup is non-NULL, each
// rcu_node structure's ->lock is acquired, ruling out high-rate usage.
//
// cpup = -1;
// No CPUs without quiescent states for this rnp.
// Find the first holdout CPU.
// cpup = cpu;
// Can't blame CPUs, so must blame RCU priority boosting.
//
// Show the state of the grace-period kthreads.
//
// This function checks for grace-period requests that fail to motivate
// RCU to come out of its idle mode.
//
// Hold onto the leaf lock to make others see warned==1.
// irqs remain disabled.
//
// Do a forward-progress check for rcutorture.  This is normally invoked
// due to an OOM event.  The argument "j" gives the time period during
// which rcutorture would like progress to have been made.
//
// Commandeer a sysrq key to dump RCU's tree.
// Dump grace-period-request information due to commandeered sysrq.
extern "C" {
    pub fn register_sysrq_key(_arg: 'y', _arg: &sysrq_rcudump_op) -> return;
}

//
// RCU CPU stall-warning notifiers
extern "C" {
    pub fn ATOMIC_NOTIFIER_HEAD(_arg: rcu_cpu_stall_notifier_list) -> static;
}
//
// rcu_stall_chain_notifier_register - Add an RCU CPU stall notifier
// @n: Entry to add.
//
// Adds an RCU CPU stall notifier to an atomic notifier chain.
// The @action passed to a notifier will be @RCU_STALL_NOTIFY_NORM or
// friends.  The @data will be the duration of the stalled grace period,
// in jiffies, coerced to a void* pointer.
//
// Returns 0 on success, %-EEXIST on error.
//
extern "C" {
    pub fn atomic_notifier_chain_register(_arg: &rcu_cpu_stall_notifier_list, _arg: n) -> return;
}
//
// rcu_stall_chain_notifier_unregister - Remove an RCU CPU stall notifier
// @n: Entry to add.
//
// Removes an RCU CPU stall notifier from an atomic notifier chain.
//
// Returns zero on success, %-ENOENT on failure.
//
extern "C" {
    pub fn atomic_notifier_chain_unregister(_arg: &rcu_cpu_stall_notifier_list, _arg: n) -> return;
}
//
// rcu_stall_notifier_call_chain - Call functions in an RCU CPU stall notifier chain
// @val: Value passed unmodified to notifier function
// @v: Pointer passed unmodified to notifier function
//
// Calls each function in the RCU CPU stall notifier chain in turn, which
// is an atomic call chain.  See atomic_notifier_call_chain() for more
// information.
//
// This is for use within RCU, hence the omission of the extra asterisk
// to indicate a non-kerneldoc format header comment.
//
extern "C" {
    pub fn atomic_notifier_call_chain(_arg: &rcu_cpu_stall_notifier_list, _arg: val, _arg: v) -> return;
}