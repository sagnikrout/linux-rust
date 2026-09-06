//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/tree.h
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
// Internal non-public definitions.
//
// Copyright IBM Corporation, 2008
//
// Author: Ingo Molnar <mingo@elte.hu>
// Paul E. McKenney <paulmck@linux.ibm.com>
//

// Communicate arguments to a kthread worker handler.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_exp_work {
    pub rew_s: c_ulong,
    pub rew_work: kthread_work,
}

// RCU's kthread states for tracing.
pub const RCU_KTHREAD_STOPPED: c_int = 0;
pub const RCU_KTHREAD_RUNNING: c_int = 1;
pub const RCU_KTHREAD_WAITING: c_int = 2;
pub const RCU_KTHREAD_OFFCPU: c_int = 3;
pub const RCU_KTHREAD_YIELDING: c_int = 4;
pub const RCU_KTHREAD_MAX: c_int = 4;
//
// Definition for node within the RCU grace-period-detection hierarchy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_node {
//     pub /: *mut *mut raw_spinlock_t __private lock; / Root rcu_node's lock protects,
// some rcu_state fields as well as
// following.
//     pub /: *mut *mut unsigned long gp_seq; / Track rsp->gp_seq.,
//     pub /: *mut *mut unsigned long gp_seq_needed; / Track furthest future GP request.,
//     pub /: *mut *mut unsigned long completedqs; / All QSes done for this node.,
//     pub /: *mut *mut unsigned long qsmask; / CPUs or groups that need to switch in,
// order for current grace period to proceed.
// In leaf rcu_node, each bit corresponds to
// an rcu_data structure, otherwise, each
// bit corresponds to a child rcu_node
// structure.
//     pub /: *mut *mut unsigned long rcu_gp_init_mask; / Mask of offline CPUs at GP init.,
    pub qsmaskinit: c_ulong,
// Per-GP initial value for qsmask.
// Initialized from ->qsmaskinitnext at the
// beginning of each grace period.
    pub qsmaskinitnext: c_ulong,
//     pub /: *mut *mut unsigned long expmask; / CPUs or groups that need to check in,
// to allow the current expedited GP
// to complete.
    pub expmaskinit: c_ulong,
// Per-GP initial values for expmask.
// Initialized from ->expmaskinitnext at the
// beginning of each expedited GP.
    pub expmaskinitnext: c_ulong,
// Online CPUs for next expedited GP.
// Any CPU that has ever been online will
// have its bit set.
    pub exp_kworker: *mut kthread_worker,
// Workers performing per node expedited GP
// initialization.
    pub cbovldmask: c_ulong,
// CPUs experiencing callback overload.
//     pub /: *mut *mut unsigned long ffmask; / Fully functional CPUs.,
//     pub /: *mut *mut unsigned long grpmask; / Mask to apply to parent qsmask.,
// Only one bit will be set in this mask.
//     pub /: *mut *mut int grplo; / lowest-numbered CPU here.,
//     pub /: *mut *mut int grphi; / highest-numbered CPU here.,
//     pub /: *mut *mut u8 grpnum; / group number for next level up.,
//     pub /: *mut *mut u8 level; / root is at level 0.,
//     pub /: *mut *mut bool wait_blkd_tasks;/ Necessary to wait for blocked tasks to,
// exit RCU read-side critical sections
// before propagating offline up the
// rcu_node tree?
    pub parent: *mut rcu_node,
    pub blkd_tasks: list_head,
// Tasks blocked in RCU read-side critical
// section.  Tasks are placed at the head
// of this list and age towards the tail.
    pub gp_tasks: *mut list_head,
// Pointer to the first task blocking the
// current grace period, or NULL if there
// is no such task.
    pub exp_tasks: *mut list_head,
// Pointer to the first task blocking the
// current expedited grace period, or NULL
// if there is no such task.  If there
// is no current expedited grace period,
// then there can cannot be any such task.
    pub boost_tasks: *mut list_head,
// Pointer to first task that needs to be
// priority boosted, or NULL if no priority
// boosting is needed for this rcu_node
// structure.  If there are no tasks
// queued on this rcu_node structure that
// are blocking the current grace period,
// there can be no such task.
    pub boost_mtx: rt_mutex,
// Used only for the priority-boosting
// side effect, not as a lock.
    pub boost_time: c_ulong,
// When to start boosting (jiffies).
    pub kthread_mutex: mutex,
// Exclusion for thread spawning and affinity
// manipulation.
    pub boost_kthread_task: *mut task_struct,
// kthread that takes care of priority
// boosting for this rcu_node structure.
    pub boost_kthread_status: c_uint,
// State of boost_kthread_task for tracing.
//     pub /: *mut *mut unsigned long n_boosts; / Number of boosts for this rcu_node structure.,
    pub nocb_gp_wq: [swait_queue_head; 2],
// Place for rcu_nocb_kthread() to wait GP.

    pub ____cacheline_internodealigned_in_smp: raw_spinlock_t fqslock,
    pub ____cacheline_internodealigned_in_smp: spinlock_t exp_lock,
    pub exp_seq_rq: c_ulong,
    pub exp_wq: [wait_queue_head_t; 4],
    pub rew: rcu_exp_work,
//     pub /: *mut *mut bool exp_need_flush; / Need to flush workitem?,
    pub exp_poll_lock: raw_spinlock_t,
// Lock and data for polled expedited grace periods.
    pub exp_seq_poll_rq: c_ulong,
    pub exp_poll_wq: work_struct,
}
//
// Bitmasks in an rcu_node cover the interval [grplo, grphi] of CPU IDs, and
// are indexed relative to this interval rather than the global CPU ID space.
// This generates the bit for a CPU in node-local masks.
//

//
// Union to allow "aggregate OR" operation on the need for a quiescent
// state by the normal and expedited grace periods.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union rcu_noqs {
    pub norm: u8,
    pub exp: u8,
//     pub /: *mut *mut } b; / Bits.,
//     pub /: *mut *mut u16 s; / Set of bits, aggregate OR here.,
}

//
// Record the snapshot of the core stats at half of the first RCU stall timeout.
// The member gp_seq is used to ensure that all members are updated only once
// during the sampling period. The snapshot is taken only if this gp_seq is not
// equal to rdp->gp_seq.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_snap_record {
//     pub /: *mut *mut unsigned long gp_seq; / Track rdp->gp_seq counter,
//     pub /: *mut *mut u64 cputime_irq; / Accumulated cputime of hard irqs,
//     pub /: *mut *mut u64 cputime_softirq;/ Accumulated cputime of soft irqs,
//     pub /: *mut *mut u64 cputime_system; / Accumulated cputime of kernel tasks,
//     pub /: *mut *mut u64 nr_hardirqs; / Accumulated number of hard irqs,
//     pub /: *mut *mut unsigned int nr_softirqs; / Accumulated number of soft irqs,
//     pub /: *mut *mut unsigned long long nr_csw; / Accumulated number of task switches,
//     pub /: *mut *mut unsigned long jiffies; / Track jiffies value,
}

//
// An IRQ work (deferred_qs_iw) is used by RCU to get the scheduler's attention.
// to report quiescent states at the soonest possible time.
// The request can be in one of the following states:
// - DEFER_QS_IDLE: An IRQ work is yet to be scheduled.
// - DEFER_QS_PENDING: An IRQ work was scheduled but either not yet run, or it
// ran and we still haven't reported a quiescent state.
//
pub const DEFER_QS_IDLE: c_int = 0;
pub const DEFER_QS_PENDING: c_int = 1;
// Per-CPU data for read-copy update.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_data {
// 1) quiescent-state and grace-period handling :
//     pub /: *mut *mut unsigned long gp_seq; / Track rsp->gp_seq counter.,
//     pub /: *mut *mut unsigned long gp_seq_needed; / Track furthest future GP request.,
//     pub /: *mut *mut rcu_noqs cpu_no_qs; / No QSes yet for this CPU.,
//     pub /: *mut *mut bool core_needs_qs; / Core waits for quiescent state.,
//     pub /: *mut *mut bool beenonline; / CPU online at least once.,
//     pub /: *mut *mut bool gpwrap; / Possible ->gp_seq wrap.,
//     pub /: *mut *mut unsigned int gpwrap_count; / Count of GP sequence wrap.,
//     pub /: *mut *mut bool cpu_started; / RCU watching this onlining CPU.,
//     pub /: *mut *mut *mut rcu_node mynode; / This CPU's leaf of hierarchy,
//     pub /: *mut *mut unsigned long grpmask; / Mask to apply to leaf qsmask.,
//     pub /: *mut *mut unsigned long ticks_this_gp; / The number of scheduling-clock,
// ticks this CPU has handled
// during and after the last grace
// period it is aware of.
//     pub /: *mut *mut irq_work defer_qs_iw; / Obtain later scheduler attention.,
//     pub /: *mut *mut int defer_qs_pending; / irqwork or softirq pending?,
//     pub /: *mut *mut work_strict_work; / Schedule readers for strict GPs.,
// 2) batch handling
//     pub /: *mut *mut rcu_segcblist cblist; / Segmented callback list, with,
// different callbacks waiting for
// different grace periods.
    pub qlen_last_fqs_check: c_long,
// qlen at last check for QS forcing
//     pub /: *mut *mut unsigned long n_cbs_invoked; / # callbacks invoked since boot.,
    pub n_force_qs_snap: c_ulong,
// did other CPU force QS recently?
//     pub /: *mut *mut long blimit; / Upper limit on a processed batch,
// 3) dynticks interface.
//     pub /: *mut *mut int watching_snap; / Per-GP tracking for dynticks.,
//     pub /: *mut *mut bool rcu_need_heavy_qs; / GP old, so heavy quiescent state!,
//     pub /: *mut *mut bool rcu_urgent_qs; / GP old need light quiescent state.,
//     pub /: *mut *mut bool rcu_forced_tick; / Forced tick to provide QS.,
//     pub /: *mut *mut bool rcu_forced_tick_exp; / ... provide QS to expedited GP.,
// 4) rcu_barrier(), OOM callbacks, and expediting.
//     pub /: *mut *mut unsigned long barrier_seq_snap; / Snap of rcu_state.barrier_sequence.,
    pub barrier_head: rcu_head,
//     pub /: *mut *mut int exp_watching_snap; / Double-check need for IPI.,
// 5) Callback offloading.

//     pub /: *mut *mut swait_queue_head nocb_cb_wq; / For nocb kthreads to sleep on.,
//     pub /: *mut *mut swait_queue_head nocb_state_wq; / For offloading state changes,
    pub nocb_gp_kthread: *mut task_struct,
//     pub /: *mut *mut raw_spinlock_t nocb_lock; / Guard following pair of fields.,
//     pub /: *mut *mut int nocb_defer_wakeup; / Defer wakeup of nocb_kthread.,
//     pub /: *mut *mut timer_list nocb_timer; / Enforce finite deferral.,
//     pub /: *mut *mut unsigned long nocb_gp_adv_time; / Last call_rcu() CB adv (jiffies).,
//     pub /: *mut *mut mutex nocb_gp_kthread_mutex; / Exclusion for nocb gp kthread,
// spawning
// The following fields are used by call_rcu, hence own cacheline.
    pub ____cacheline_internodealigned_in_smp: raw_spinlock_t nocb_bypass_lock,
//     pub /: *mut *mut rcu_cblist nocb_bypass; / Lock-contention-bypass CB list.,
//     pub /: *mut *mut unsigned long nocb_bypass_first; / Time (jiffies) of first enqueue.,
//     pub /: *mut *mut unsigned long nocb_nobypass_last; / Last ->cblist enqueue (jiffies).,
//     pub /: *mut *mut int nocb_nobypass_count; / # ->cblist enqueues at ^^^ time.,
// The following fields are used by GP kthread, hence own cacheline.
    pub ____cacheline_internodealigned_in_smp: raw_spinlock_t nocb_gp_lock,
//     pub /: *mut *mut u8 nocb_gp_sleep; / Is the nocb GP thread asleep?,
//     pub /: *mut *mut u8 nocb_gp_bypass; / Found a bypass on last scan?,
//     pub /: *mut *mut u8 nocb_gp_gp; / GP to wait for on last scan?,
//     pub /: *mut *mut rcu_gp_seq nocb_gp_seq; / If so, GP state to wait for.,
//     pub /: *mut *mut unsigned long nocb_gp_loops; / # passes through wait code.,
//     pub /: *mut *mut swait_queue_head nocb_gp_wq; / For nocb kthreads to sleep on.,
//     pub /: *mut *mut bool nocb_cb_sleep; / Is the nocb CB thread asleep?,
    pub nocb_cb_kthread: *mut task_struct,
    pub //: *mut list_head nocb_head_rdp;,
// Head of rcu_data list in wakeup chain,
// if rdp_gp.
//
//     pub /: *mut *mut list_head nocb_entry_rdp; / rcu_data node in wakeup chain.,
//     pub /: *mut *mut *mut rcu_data nocb_toggling_rdp; / rdp queued for (de-)offloading,
// The following fields are used by CB kthread, hence new cacheline.
    pub ____cacheline_internodealigned_in_smp: *mut *mut rcu_data nocb_gp_rdp,
// GP rdp takes GP-end wakeups.

// 6) RCU priority boosting.
    pub rcu_cpu_kthread_task: *mut task_struct,
// rcuc per-CPU kthread or NULL.
    pub rcu_cpu_kthread_status: c_uint,
    pub rcu_cpu_has_work: c_char,
    pub rcuc_activity: c_ulong,
// 7) Diagnostic data, including RCU CPU stall warnings.
//     pub /: *mut *mut unsigned int softirq_snap; / Snapshot of softirq activity.,
// ->rcu_iw* fields protected by leaf rcu_node ->lock.
//     pub /: *mut *mut irq_work rcu_iw; / Check for non-irq activity.,
//     pub /: *mut *mut bool rcu_iw_pending; / Is ->rcu_iw pending?,
//     pub /: *mut *mut unsigned long rcu_iw_gp_seq; / ->gp_seq associated with ->rcu_iw.,
//     pub /: *mut *mut unsigned long rcu_ofl_gp_seq; / ->gp_seq at last offline.,
//     pub /: *mut *mut short rcu_ofl_gp_state; / ->gp_state at last offline.,
//     pub /: *mut *mut unsigned long rcu_onl_gp_seq; / ->gp_seq at last online.,
//     pub /: *mut *mut short rcu_onl_gp_state; / ->gp_state at last online.,
//     pub /: *mut *mut unsigned long last_fqs_resched; / Time of last rcu_resched().,
//     pub /: *mut *mut unsigned long last_sched_clock; / Jiffies of last rcu_sched_clock_irq().,
//     pub /: *mut *mut rcu_snap_record snap_record; / Snapshot of core stats at half of,
// the first RCU stall timeout
//     pub /: *mut *mut long lazy_len; / Length of buffered lazy callbacks.,
    pub cpu: c_int,
}

// Values for nocb_defer_wakeup field in struct rcu_data.
pub const RCU_NOCB_WAKE_NOT: c_int = 0;
pub const RCU_NOCB_WAKE_BYPASS: c_int = 1;
pub const RCU_NOCB_WAKE_LAZY: c_int = 2;
pub const RCU_NOCB_WAKE: c_int = 3;

// For jiffies_till_first_fqs and
// and jiffies_till_next_fqs.

// delay between bouts of
// quiescent-state forcing.

// at least one scheduling clock
// irq before ratting on them.

//
// A max threshold for synchronize_rcu() users which are
// awaken directly by the rcu_gp_kthread(). Left part is
// deferred to the main worker.
//
pub const SR_MAX_USERS_WAKE_FROM_GP: c_int = 5;
pub const SR_NORMAL_GP_WAIT_HEAD_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sr_wait_node {
    pub inuse: core::sync::atomic::AtomicI32,
    pub node: llist_node,
}

//
// RCU global state, including node hierarchy.  This hierarchy is
// represented in "heap" form in a dense array.  The root (first level)
// of the hierarchy is in ->node[0] (referenced by ->level[0]), the second
// level in ->node[1] through ->node[m] (->node[1] referenced by ->level[1]),
// and the third level in ->node[m+1] and following (->node[m+1] referenced
// by ->level[2]).  The number of levels is determined by the number of
// CPUs and by CONFIG_RCU_FANOUT.  Small systems will have a "hierarchy"
// consisting of a single rcu_node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_state {
//     pub /: *mut *mut rcu_node node[NUM_RCU_NODES]; / Hierarchy.,
    pub 1]: *mut *mut rcu_node level[RCU_NUM_LVLS +,
// Hierarchy levels (+1 to
// shut bogus gcc warning)
//     pub /: *mut *mut int ncpus; / # CPUs seen so far.,
//     pub /: *mut *mut int n_online_cpus; / # CPUs online for RCU.,
// The following fields are guarded by the root rcu_node's lock.
    pub ____cacheline_internodealigned_in_smp: unsigned long gp_seq,
// Grace-period sequence #.
//     pub /: *mut *mut unsigned long gp_max; / Maximum GP duration in,
// jiffies.
//     pub /: *mut *mut *mut task_gp_kthread; / Task for grace periods.,
//     pub /: *mut *mut swait_queue_head gp_wq; / Where GP task waits.,
//     pub /: *mut *mut short gp_flags; / Commands for GP task.,
//     pub /: *mut *mut short gp_state; / GP kthread sleep state.,
//     pub /: *mut *mut unsigned long gp_wake_time; / Last GP kthread wake.,
//     pub /: *mut *mut unsigned long gp_wake_seq; / ->gp_seq at ^^^.,
//     pub /: *mut *mut unsigned long gp_seq_polled; / GP seq for polled API.,
//     pub /: *mut *mut unsigned long gp_seq_polled_snap; / ->gp_seq_polled at normal GP start.,
//     pub /: *mut *mut unsigned long gp_seq_polled_exp_snap; / ->gp_seq_polled at expedited GP start.,
// End of fields guarded by root rcu_node's lock.
//     pub /: *mut *mut mutex barrier_mutex; / Guards barrier fields.,
//     pub /: *mut *mut atomic_t barrier_cpu_count; / # CPUs waiting on.,
//     pub /: *mut *mut completion barrier_completion; / Wake at barrier end.,
//     pub /: *mut *mut unsigned long barrier_sequence; / ++ at start and end of,
// rcu_barrier().
// End of fields guarded by barrier_mutex.
//     pub /: *mut *mut raw_spinlock_t barrier_lock; / Protects ->barrier_seq_snap.,
//     pub /: *mut *mut mutex exp_mutex; / Serialize expedited GP.,
//     pub /: *mut *mut mutex exp_wake_mutex; / Serialize wakeup.,
//     pub /: *mut *mut unsigned long expedited_sequence; / Take a ticket.,
//     pub /: *mut *mut swait_queue_head expedited_wq; / Wait for check-ins.,
//     pub /: *mut *mut int ncpus_snap; / # CPUs seen last time.,
//     pub /: *mut *mut u8 cbovld; / Callback overload now?,
//     pub /: *mut *mut u8 cbovldnext; / ^ ^ next time?,
//     pub /: *mut *mut unsigned long jiffies_force_qs; / Time at which to invoke,
// force_quiescent_state().
//     pub /: *mut *mut unsigned long jiffies_kick_kthreads; / Time at which to kick,
// kthreads, if configured.
//     pub /: *mut *mut unsigned long n_force_qs; / Number of calls to,
// force_quiescent_state().
//     pub /: *mut *mut unsigned long gp_start; / Time at which GP started,,
// but in jiffies.
//     pub /: *mut *mut unsigned long gp_end; / Time last GP ended, again,
// in jiffies.
//     pub /: *mut *mut unsigned long gp_activity; / Time of last GP kthread,
// activity in jiffies.
//     pub /: *mut *mut unsigned long gp_req_activity; / Time of last GP request,
// in jiffies.
//     pub /: *mut *mut unsigned long jiffies_stall; / Time at which to check,
// for CPU stalls.
    pub after: *mut *mut int nr_fqs_jiffies_stall; / Number of fqs loops,
// which read jiffies and set
// jiffies_stall. Stall
// warnings disabled if !0.
//     pub /: *mut *mut unsigned long jiffies_resched; / Time at which to resched,
// a reluctant CPU.
//     pub /: *mut *mut unsigned long n_force_qs_gpstart; / Snapshot of n_force_qs at,
// GP start.
//     pub /: *const *const *const char name; / Name of structure.,
//     pub /: *mut *mut char abbr; / Abbreviated name.,
    pub ____cacheline_internodealigned_in_smp: arch_spinlock_t ofl_lock,
// Synchronize offline with
// GP pre-initialization.
// synchronize_rcu() part.
//     pub /: *mut *mut llist_head srs_next; / request a GP users.,
//     pub /: *mut *mut *mut llist_node srs_wait_tail; / wait for GP users.,
//     pub /: *mut *mut *mut llist_node srs_done_tail; / ready for GP users.,
    pub srs_wait_nodes: [sr_wait_node; SR_NORMAL_GP_WAIT_HEAD_MAX],
    pub srs_cleanup_work: work_struct,
//     pub /: *mut *mut atomic_t srs_cleanups_pending; / srs inflight worker cleanups.,

//     pub /: *mut *mut mutex nocb_mutex; / Guards (de-)offloading,
//     pub /: *mut *mut int nocb_is_setup; / nocb is setup from boot,

}

// Values for rcu_state structure's gp_flags field.
pub const RCU_GP_FLAG_INIT: c_uint = 0x1	/* Need grace-period initialization. */;
pub const RCU_GP_FLAG_FQS: c_uint = 0x2	/* Need grace-period quiescent-state forcing. */;
pub const RCU_GP_FLAG_OVLD: c_uint = 0x4	/* Experiencing callback overload. */;
// Values for rcu_state structure's gp_state field.

//
// In order to export the rcu_state name to the tracing tools, it
// needs to be added in the __tracepoint_string section.
// This requires defining a separate variable tp_<sname>_varname
// that points to the string being used, and this will allow
// the tracing userspace tools to be able to decipher the string
// address to the matching string.
//

// Forward declarations for tree_plugin.h
extern "C" {
    pub fn rcu_bootup_announce() -> static void;
}
extern "C" {
    pub fn rcu_qs() -> static void;
}
extern "C" {
    pub fn rcu_preempt_blocked_readers_cgp(rnp: *mut rcu_node) -> static int;
}

extern "C" {
    pub fn rcu_preempt_has_tasks(rnp: *mut rcu_node) -> static bool;
}

extern "C" {
    pub fn rcu_print_task_exp_stall(rnp: *mut rcu_node) -> static int;
}
extern "C" {
    pub fn rcu_preempt_check_blocked_tasks(rnp: *mut rcu_node) -> static void;
}
extern "C" {
    pub fn rcu_flavor_sched_clock_irq(user: c_int) -> static void;
}
extern "C" {
    pub fn dump_blkd_tasks(rnp: *mut rcu_node, ncheck: c_int) -> static void;
}
extern "C" {
    pub fn rcu_preempt_deferred_qs_init(rdp: *mut rcu_data) -> static void;
}
extern "C" {
    pub fn rcu_initiate_boost(rnp: *mut rcu_node, flags: c_ulong) -> static void;
}
extern "C" {
    pub fn rcu_preempt_boost_start_gp(rnp: *mut rcu_node) -> static void;
}
extern "C" {
    pub fn rcu_is_callbacks_kthread(rdp: *mut rcu_data) -> static bool;
}
extern "C" {
    pub fn rcu_cpu_kthread_setup(cpu: c_uint) -> static void;
}
extern "C" {
    pub fn rcu_spawn_one_boost_kthread(rnp: *mut rcu_node) -> static void;
}
extern "C" {
    pub fn rcu_preempt_has_tasks(rnp: *mut rcu_node) -> static bool;
}
extern "C" {
    pub fn rcu_preempt_need_deferred_qs(t: *mut task_struct) -> static bool;
}
extern "C" {
    pub fn zero_cpu_stall_ticks(rdp: *mut rcu_data) -> static void;
}
extern "C" {
    pub fn rcu_nocb_gp_cleanup(sq: *mut swait_queue_head) -> static void;
}
extern "C" {
    pub fn rcu_nocb_exp_cleanup(rnp: *mut rcu_node) -> static void;
}
extern "C" {
    pub fn rcu_init_one_nocb(rnp: *mut rcu_node) -> static void;
}
extern "C" {
    pub fn wake_nocb_gp(rdp: *mut rcu_data) -> static bool;
}
extern "C" {
    pub fn rcu_nocb_need_deferred_wakeup(rdp: *mut rcu_data, level: c_int) -> static int;
}
extern "C" {
    pub fn do_nocb_deferred_wakeup(rdp: *mut rcu_data) -> static bool;
}
extern "C" {
    pub fn rcu_boot_init_nocb_percpu_data(rdp: *mut rcu_data) -> static void;
}
extern "C" {
    pub fn rcu_spawn_cpu_nocb_kthread(cpu: c_int) -> static void;
}
extern "C" {
    pub fn show_rcu_nocb_state(rdp: *mut rcu_data) -> static void;
}
extern "C" {
    pub fn rcu_nocb_lock(rdp: *mut rcu_data) -> static void;
}
extern "C" {
    pub fn rcu_nocb_unlock(rdp: *mut rcu_data) -> static void;
}
extern "C" {
    pub fn rcu_lockdep_assert_cblist_protected(rdp: *mut rcu_data) -> static void;
}

extern "C" {
    pub fn rcu_organize_nocb_kthreads() -> static void __init;
}
//
// Disable IRQs before checking offloaded state so that local
// locking is safe against concurrent de-offloading.
//

extern "C" {
    pub fn rcu_bind_gp_kthread() -> static void;
}
extern "C" {
    pub fn rcu_nohz_full_cpu() -> static bool;
}
// Forward declarations for tree_stall.h
extern "C" {
    pub fn record_gp_stall_check_time() -> static void;
}
extern "C" {
    pub fn rcu_iw_handler(iwp: *mut irq_work) -> static void;
}
extern "C" {
    pub fn check_cpu_stall(rdp: *mut rcu_data) -> static void;
}
extern "C" {
    pub fn rcu_check_gp_start_stall(rnp: *mut rcu_node, gpssdelay: c_ulong) -> static void;
}
// Forward declarations for tree_exp.h.
extern "C" {
    pub fn sync_rcu_do_polled_gp(wp: *mut work_struct) -> static void;