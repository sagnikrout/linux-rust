//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Define 'struct task_struct' and provide the main scheduler
// APIs (schedule(), wakeup variants, etc.)
//

// task_struct member predeclarations (sorted alphabetically):

//
// Task state bitmask. NOTE! These bits are also
// encoded in fs/proc/array.c: get_task_state().
//
// We have two separate sets of flags: task->__state
// is about runnability, while task->exit_state are
// about the task exiting. Confusing, but this way
// modifying one set can't modify the other one by
// mistake.
//
// Used in tsk->__state:
pub const TASK_RUNNING: c_uint = 0x00000000;
pub const TASK_INTERRUPTIBLE: c_uint = 0x00000001;
pub const TASK_UNINTERRUPTIBLE: c_uint = 0x00000002;
pub const __TASK_STOPPED: c_uint = 0x00000004;
pub const __TASK_TRACED: c_uint = 0x00000008;
// Used in tsk->exit_state:
pub const EXIT_DEAD: c_uint = 0x00000010;
pub const EXIT_ZOMBIE: c_uint = 0x00000020;

// Used in tsk->__state again:
pub const TASK_PARKED: c_uint = 0x00000040;
pub const TASK_DEAD: c_uint = 0x00000080;
pub const TASK_WAKEKILL: c_uint = 0x00000100;
pub const TASK_WAKING: c_uint = 0x00000200;
pub const TASK_NOLOAD: c_uint = 0x00000400;
pub const TASK_NEW: c_uint = 0x00000800;
pub const TASK_RTLOCK_WAIT: c_uint = 0x00001000;
pub const TASK_FREEZABLE: c_uint = 0x00002000;

pub const TASK_FROZEN: c_uint = 0x00008000;
pub const TASK_STATE_MAX: c_uint = 0x00010000;

//
// DO NOT ADD ANY NEW USERS !
//

// Convenience macros for the sake of set_current_state:

// Convenience macros for the sake of wake_up():

// get_task_state():

//
// Special states are those that do not use the normal wait-loop pattern. See
// the comment with set_special_state().
//

//
// set_current_state() includes a barrier so that the write of current->__state
// is correctly serialised wrt the caller's subsequent test of whether to
// actually sleep:
//
// for (;;) {
// set_current_state(TASK_UNINTERRUPTIBLE);
// if (CONDITION)
// break;
//
// schedule();
// }
// __set_current_state(TASK_RUNNING);
//
// If the caller does not need such serialisation (because, for instance, the
// CONDITION test and condition change and wakeup are under the same lock) then
// use __set_current_state().
//
// The above is typically ordered against the wakeup, which does:
//
// CONDITION = 1;
// wake_up_state(p, TASK_UNINTERRUPTIBLE);
//
// where wake_up_state()/try_to_wake_up() executes a full memory barrier before
// accessing p->__state.
//
// Wakeup will do: if (@state & p->__state) p->__state = TASK_RUNNING, that is,
// once it observes the TASK_UNINTERRUPTIBLE store the waking CPU can issue a
// TASK_RUNNING store which can collide with __set_current_state(TASK_RUNNING).
//
// However, with slightly different timing the wakeup TASK_RUNNING store can
// also collide with the TASK_UNINTERRUPTIBLE store. Losing that store is not
// a problem either because that will result in one extra go around the loop
// and our @cond test will save the day.
//
// Also see the comments of try_to_wake_up().
//

//
// set_special_state() should be used for those states when the blocking task
// can not use the regular condition based wait-loop. In that case we must
// serialize against wakeups such that any possible in-flight TASK_RUNNING
// stores will not collide with our state change.
//

//
// PREEMPT_RT specific variants for "sleeping" spin/rwlocks
//
// RT's spin/rwlock substitutions are state preserving. The state of the
// task when blocking on the lock is saved in task_struct::saved_state and
// restored after the lock has been acquired.  These operations are
// serialized by task_struct::pi_lock against try_to_wake_up(). Any non RT
// lock related wakeups while the task is blocked on the lock are
// redirected to operate on task_struct::saved_state to ensure that these
// are not dropped. On restore task_struct::saved_state is set to
// TASK_RUNNING so any wakeup attempt redirected to saved_state will fail.
//
// The lock operation looks like this:
//
// current_save_and_set_rtlock_wait_state();
// for (;;) {
// if (try_lock())
// break;
// raw_spin_unlock_irq(&lock->wait_lock);
// schedule_rtlock();
// raw_spin_lock_irq(&lock->wait_lock);
// set_current_state(TASK_RTLOCK_WAIT);
// }
// current_restore_rtlock_saved_state();
//

//
// Define the task command name length as enum, then it can be visible to
// BPF programs.
//
extern "C" {
    pub fn sched_tick();
}

extern "C" {
    pub fn schedule_timeout(timeout: c_long) -> c_long;
}
extern "C" {
    pub fn schedule_timeout_interruptible(timeout: c_long) -> c_long;
}
extern "C" {
    pub fn schedule_timeout_killable(timeout: c_long) -> c_long;
}
extern "C" {
    pub fn schedule_timeout_uninterruptible(timeout: c_long) -> c_long;
}
extern "C" {
    pub fn schedule_timeout_idle(timeout: c_long) -> c_long;
}
extern "C" {
    pub fn schedule() -> asmlinkage void;
}
extern "C" {
    pub fn schedule_preempt_disabled();
}
extern "C" {
    pub fn preempt_schedule_irq() -> asmlinkage void;
}

extern "C" {
    pub fn schedule_rtlock();
}

extern "C" {
    pub fn io_schedule_prepare() -> int __must_check;
}
extern "C" {
    pub fn io_schedule_finish(token: c_int);
}
extern "C" {
    pub fn io_schedule_timeout(timeout: c_long) -> c_long;
}
extern "C" {
    pub fn io_schedule();
}
// wrapper functions to trace from this header file
extern "C" {
    pub fn __trace_set_current_state(state_value: c_int);
}
extern "C" {
    pub fn __trace_set_need_resched(curr: *mut task_struct, tif: c_int);
}
//
// struct prev_cputime - snapshot of system and user cputime
// @utime: time spent in user mode
// @stime: time spent in system mode
// @lock: protects the above two fields
//
// Stores previous user/system time values such that we can guarantee
// monotonicity.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prev_cputime {

    pub utime: u64,
    pub stime: u64,
    pub lock: raw_spinlock_t,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vtime_state {
// Task is sleeping or running in a CPU with VTIME inactive:
    VTIME_INACTIVE = 0,
// Task is idle
    VTIME_IDLE,
// Task runs in kernelspace in a CPU with VTIME active:
    VTIME_SYS,
// Task runs in userspace in a CPU with VTIME active:
    VTIME_USER,
// Task runs as guests in a CPU with VTIME active:
    VTIME_GUEST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vtime {
    pub seqcount: seqcount_t,
    pub starttime: c_ulonglong,
    pub state: vtime_state,
    pub cpu: c_uint,
    pub utime: u64,
    pub stime: u64,
    pub gtime: u64,
}

//
// Utilization clamp constraints.
// @UCLAMP_MIN:	Minimum utilization
// @UCLAMP_MAX:	Maximum utilization
// @UCLAMP_CNT:	Utilization clamp constraints count
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uclamp_id {
    UCLAMP_MIN = 0,
    UCLAMP_MAX,
    UCLAMP_CNT
}

extern "C" {
    pub fn sched_domains_mutex_lock();
}
extern "C" {
    pub fn sched_domains_mutex_unlock();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_param {
    pub sched_priority: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_info {

// Cumulative counters:
// # of times we have run on this CPU:
    pub pcount: c_ulong,
// Time spent waiting on a runqueue:
    pub run_delay: c_ulonglong,
// Max time spent waiting on a runqueue:
    pub max_run_delay: c_ulonglong,
// Min time spent waiting on a runqueue:
    pub min_run_delay: c_ulonglong,
// Timestamps:
// When did we last run on a CPU?
    pub last_arrival: c_ulonglong,
// When were we last queued to run?
    pub last_queued: c_ulonglong,
// Timestamp of max time spent waiting on a runqueue:
    pub max_run_delay_ts: timespec64,

}

//
// Integer metrics need fixed point arithmetic, e.g., sched/fair
// has a few: load, load_avg, util_avg, freq, and capacity.
//
// We define a basic fixed point arithmetic range, and then formalize
// all these metrics based on that basic range.
//

// Increase resolution of cpu_capacity calculations

#[repr(C)]
#[derive(Copy, Clone)]
pub struct load_weight {
    pub weight: c_ulong,
    pub inv_weight: u32,
}

//
// The load/runnable/util_avg accumulates an infinite geometric series
// (see __update_load_avg_cfs_rq() in kernel/sched/pelt.c).
//
// [load_avg definition]
//
// load_avg = runnable% * scale_load_down(load)
//
// [runnable_avg definition]
//
// runnable_avg = runnable% * SCHED_CAPACITY_SCALE
//
// [util_avg definition]
//
// util_avg = running% * SCHED_CAPACITY_SCALE
//
// where runnable% is the time ratio that a sched_entity is runnable and
// running% the time ratio that a sched_entity is running.
//
// For cfs_rq, they are the aggregated values of all runnable and blocked
// sched_entities.
//
// The load/runnable/util_avg doesn't directly factor frequency scaling and CPU
// capacity scaling. The scaling is done through the rq_clock_pelt that is used
// for computing those signals (see update_rq_clock_pelt())
//
// N.B., the above ratios (runnable% and running%) themselves are in the
// range of [0, 1]. To do fixed point arithmetics, we therefore scale them
// to as large a range as necessary. This is for example reflected by
// util_avg's SCHED_CAPACITY_SCALE.
//
// [Overflow issue]
//
// The 64-bit load_sum can have 4353082796 (=2^64/47742/88761) entities
// with the highest load (=88761), always runnable on a single cfs_rq,
// and should not overflow as the number already hits PID_MAX_LIMIT.
//
// For all other cases (including 32-bit kernels), struct load_weight's
// weight will overflow first before we do, because:
//
// Max(load_avg) <= Max(load.weight)
//
// Then it is the load_weight's responsibility to consider overflow
// issues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_avg {
    pub last_update_time: u64,
    pub load_sum: u64,
    pub runnable_sum: u64,
    pub util_sum: u32,
    pub period_contrib: u32,
    pub load_avg: c_ulong,
    pub runnable_avg: c_ulong,
    pub util_avg: c_ulong,
    pub util_est: c_uint,
    pub ____cacheline_aligned: },
//
// The UTIL_AVG_UNCHANGED flag is used to synchronize util_est with util_avg
// updates. When a task is dequeued, its util_est should not be updated if its
// util_avg has not been updated in the meantime.
// This information is mapped into the MSB bit of util_est at dequeue time.
// Since max value of util_est for a task is 1024 (PELT util_avg for a task)
// it is safe to use MSB.
//
pub const UTIL_EST_WEIGHT_SHIFT: c_int = 2;
pub const UTIL_AVG_UNCHANGED: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_statistics {

    pub wait_start: u64,
    pub wait_max: u64,
    pub wait_count: u64,
    pub wait_sum: u64,
    pub iowait_count: u64,
    pub iowait_sum: u64,
    pub sleep_start: u64,
    pub sleep_max: u64,
    pub sum_sleep_runtime: i64,
    pub block_start: u64,
    pub block_max: u64,
    pub sum_block_runtime: i64,
    pub exec_max: i64,
    pub slice_max: u64,
    pub nr_failed_migrations_affine: u64,
    pub nr_failed_migrations_running: u64,
    pub nr_failed_migrations_hot: u64,
    pub nr_forced_migrations: u64,
    pub nr_wakeups: u64,
    pub nr_wakeups_sync: u64,
    pub nr_wakeups_migrate: u64,
    pub nr_wakeups_local: u64,
    pub nr_wakeups_remote: u64,
    pub nr_wakeups_affine: u64,
    pub nr_wakeups_affine_attempts: u64,

    pub core_forceidle_sum: u64,

    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_entity {
// For load-balancing:
    pub load: load_weight,
    pub h_load: load_weight,
    pub run_node: rb_node,
    pub deadline: u64,
    pub min_vruntime: u64,
    pub min_slice: u64,
    pub max_slice: u64,
    pub group_node: list_head,
    pub on_rq: c_uchar,
    pub sched_delayed: c_uchar,
    pub rel_deadline: c_uchar,
    pub custom_slice: c_uchar,
// hole
    pub exec_start: u64,
    pub sum_exec_runtime: u64,
    pub prev_sum_exec_runtime: u64,
    pub vruntime: u64,
// Approximated virtual lag:
    pub vlag: i64,
// 'Protected' deadline, to give out minimum quantums:
    pub vprot: u64,
    pub slice: u64,
    pub nr_migrations: u64,

    pub depth: c_int,
    pub parent: *mut sched_entity,
// rq on which this entity is (to be) queued:
    pub cfs_rq: *mut cfs_rq,
// rq "owned" by this entity/group:
    pub my_q: *mut cfs_rq,
// cached value of my_q->h_nr_running
    pub runnable_weight: c_ulong,

//
// Per entity load average tracking.
//
// Put into separate cache line so it does not
// collide with read-mostly values above.
//
    pub avg: sched_avg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_rt_entity {
    pub run_list: list_head,
    pub timeout: c_ulong,
    pub watchdog_stamp: c_ulong,
    pub time_slice: c_uint,
    pub on_rq: c_ushort,
    pub on_list: c_ushort,
    pub back: *mut sched_rt_entity,

    pub parent: *mut sched_rt_entity,
// rq on which this entity is (to be) queued:
    pub rt_rq: *mut rt_rq,
// rq "owned" by this entity/group:
    pub my_q: *mut rt_rq,

    pub __randomize_layout: },
    pub rq_flags: struct,
    pub rf): *mut *mut *mut *mut typedef struct task_struct (dl_server_pick_f)(struct sched_dl_entity , struct rq_flags,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_dl_entity {
    pub rb_node: rb_node,
//
// Original scheduling parameters. Copied here from sched_attr
// during sched_setattr(), they will remain the same until
// the next sched_setattr().
//
    pub /: *mut *mut u64 dl_runtime; / Maximum runtime for each instance,
    pub /: *mut *mut u64 dl_deadline; / Relative deadline of each instance,
    pub /: *mut *mut u64 dl_period; / Separation of two instances (period),
    pub /: *mut *mut u64 dl_bw; / dl_runtime / dl_period,
    pub /: *mut *mut u64 dl_density; / dl_runtime / dl_deadline,
//
// Actual scheduling parameters. Initialized with the values above,
// they are continuously updated during task execution. Note that
// the remaining runtime could be < 0 in case we are in overrun.
//
    pub /: *mut *mut s64 runtime; / Remaining runtime for this instance,
    pub /: *mut *mut u64 deadline; / Absolute deadline for this instance,
    pub /: *mut *mut unsigned int flags; / Specifying the scheduler behaviour,
//
// Some bool flags:
//
// @dl_throttled tells if we exhausted the runtime. If so, the
// task has to wait for a replenishment to be performed at the
// next firing of dl_timer.
//
// @dl_yielded tells if task gave up the CPU before consuming
// all its available runtime during the last job.
//
// @dl_non_contending tells if the task is inactive while still
// contributing to the active utilization. In other words, it
// indicates if the inactive timer has been armed and its handler
// has not been executed yet. This flag is useful to avoid race
// conditions between the inactive timer handler and the wakeup
// code.
//
// @dl_overrun tells if the task asked to be informed about runtime
// overruns.
//
// @dl_server tells if this is a server entity.
//
// @dl_server_active tells if the dlserver is active(started).
// dlserver is started on first cfs enqueue on an idle runqueue
// and is stopped when a dequeue results in 0 cfs tasks on the
// runqueue. In other words, dlserver is active only when cpu's
// runqueue has atleast one cfs task.
//
// @dl_defer tells if this is a deferred or regular server. For
// now only defer server exists.
//
// @dl_defer_armed tells if the deferrable server is waiting
// for the replenishment timer to activate it.
//
// @dl_defer_running tells if the deferrable server is actually
// running, skipping the defer phase.
//
// @dl_defer_idle tracks idle state
//
// @dl_bw_attached tells if this server's bandwidth currently
// contributes to the root domain's total_bw. Only meaningful for server
// entities (@dl_server == 1). Allows toggling the reservation on/off
// without losing the configured @dl_runtime/@dl_period.
//
    pub 1: unsigned int dl_throttled :,
    pub 1: unsigned int dl_yielded :,
    pub 1: unsigned int dl_non_contending :,
    pub 1: unsigned int dl_overrun :,
    pub 1: unsigned int dl_server :,
    pub 1: unsigned int dl_server_active :,
    pub 1: unsigned int dl_defer :,
    pub 1: unsigned int dl_defer_armed :,
    pub 1: unsigned int dl_defer_running :,
    pub 1: unsigned int dl_defer_idle :,
    pub 1: unsigned int dl_bw_attached :,
//
// Bandwidth enforcement timer. Each -deadline task has its
// own bandwidth to be enforced, thus we need one timer per task.
//
    pub dl_timer: hrtimer,
//
// Inactive timer, responsible for decreasing the active utilization
// at the "0-lag time". When a -deadline task blocks, it contributes
// to GRUB's active utilization until the "0-lag time", hence a
// timer is needed to decrease the active utilization at the correct
// time.
//
    pub inactive_timer: hrtimer,
//
// Bits for DL-server functionality. Also see the comment near
// dl_server_update().
//
// @rq the runqueue this server is for
//
    pub rq: *mut rq,
    pub server_pick_task: dl_server_pick_f,

//
// Priority Inheritance. When a DEADLINE scheduling entity is boosted
// pi_se points to the donor, otherwise points to the dl_se it belongs
// to (the original one/itself).
//
    pub pi_se: *mut sched_dl_entity,

}

// Number of utilization clamp buckets (shorter alias)

//
// Utilization clamp for a scheduling entity
// @value:		clamp value "assigned" to a se
// @bucket_id:		bucket index corresponding to the "assigned" value
// @active:		the se is currently refcounted in a rq's bucket
// @user_defined:	the requested clamp value comes from user-space
//
// The bucket_id is the index of the clamp bucket matching the clamp value
// which is pre-computed and stored to avoid expensive integer divisions from
// the fast path.
//
// The active bit is set whenever a task has got an "effective" value assigned,
// which can be different from the clamp value "requested" from user-space.
// This allows to know a task is refcounted in the rq's bucket corresponding
// to the "effective" bucket_id.
//
// The user_defined bit is set whenever a task has got a task-specific clamp
// value requested from userspace, i.e. the system defaults apply to this task
// just as a restriction. This allows to relax default clamps when a less
// restrictive task-specific value has been requested, thus allowing to
// implement a "nice" semantic. For example, a task running with a 20%
// default boost can still drop its own boosting to 0%.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclamp_se {
    pub bits_per(SCHED_CAPACITY_SCALE): unsigned int value :,
    pub bits_per(UCLAMP_BUCKETS): unsigned int bucket_id :,
    pub 1: unsigned int active :,
    pub 1: unsigned int user_defined :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rcu_special {
    pub blocked: u8,
    pub need_qs: u8,
    pub /: *mut *mut u8 exp_hint; / Hint for performance.,
    pub /: *mut *mut u8 need_mb; / Readers need smp_mb().,
    pub /: *mut *mut } b; / Bits.,
    pub /: *mut *mut u32 s; / Set of bits.,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_task_context {
    perf_invalid_context = -1,
    perf_hw_context = 0,
    perf_sw_context,
    perf_nr_task_contexts,
}

//
// Number of contexts where an event can trigger:
// task, softirq, hardirq, nmi.
//
pub const PERF_NR_CONTEXTS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_node {
    pub next: *mut wake_q_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmap_ctrl {

    pub idx: c_int,
    pub pteval: [pte_t; KM_MAX_IDX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_ipi_mask {
    pub ipi_mask_ptr: *mut cpumask_t,
    pub ipi_mask_val: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_ipi_mask {

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct {

//
// For reasons of header soup (see current_thread_info()), this
// must be the first element of task_struct.
//
    pub thread_info: thread_info,

    pub __state: c_uint,
// saved state for "spinlock sleepers"
    pub saved_state: c_uint,
//
// This begins the randomizable portion of task_struct. Only
// scheduling-critical items should be added above here.
//
    pub stack: *mut c_void,
    pub usage: refcount_t,
// Per task flags (PF_*), defined further below:
    pub flags: c_uint,
    pub ptrace: c_uint,

    pub alloc_tag: *mut alloc_tag,

    pub on_cpu: u8,
    pub on_rq: u8,
    pub is_blocked: u8,
    pub __pad: u8,
    pub wake_entry: __call_single_node,
    pub wakee_flips: c_uint,
    pub wakee_flip_decay_ts: c_ulong,
    pub last_wakee: *mut task_struct,
//
// recent_used_cpu is initially set as the last CPU used by a task
// that wakes affine another task. Waker/wakee relationships can
// push tasks around a CPU where each wakeup moves to the next one.
// Tracking a recently used CPU allows a quick search for a recently
// used CPU that may be idle.
//
    pub recent_used_cpu: c_int,
    pub wake_cpu: c_int,
    pub prio: c_int,
    pub static_prio: c_int,
    pub normal_prio: c_int,
    pub rt_priority: c_uint,
    pub se: sched_entity,
    pub rt: sched_rt_entity,
    pub dl: sched_dl_entity,
    pub dl_server: *mut sched_dl_entity,

    pub scx: sched_ext_entity,

    pub sched_class: *const sched_class,

    pub core_node: rb_node,
    pub core_cookie: c_ulong,
    pub core_occupation: c_uint,

    pub sched_task_group: *mut task_group,

    pub sched_throttle_work: callback_head,
    pub throttle_node: list_head,
    pub throttled: bool,

//
// Clamp values requested for a scheduling entity.
// Must be updated with task_rq_lock() held.
//
    pub uclamp_req: [uclamp_se; UCLAMP_CNT],
//
// Effective clamp values used for a scheduling entity.
// Must be updated with task_rq_lock() held.
//
    pub uclamp: [uclamp_se; UCLAMP_CNT],
    pub stats: sched_statistics,

// List of struct preempt_notifier:
    pub preempt_notifiers: hlist_head,

    pub btrace_seq: c_uint,

    pub policy: c_uint,
    pub max_allowed_capacity: c_ulong,
    pub nr_cpus_allowed: c_int,
    pub cpus_ptr: *const cpumask_t,
    pub user_cpus_ptr: *mut cpumask_t,
    pub cpus_mask: cpumask_t,
    pub migration_pending: *mut c_void,
    pub migration_disabled: c_ushort,
    pub migration_flags: c_ushort,

    pub rcu_read_lock_nesting: c_int,
    pub rcu_read_unlock_special: rcu_special,
    pub rcu_node_entry: list_head,
    pub rcu_blocked_node: *mut rcu_node,

    pub rcu_tasks_nvcsw: c_ulong,
    pub rcu_tasks_holdout: u8,
    pub rcu_tasks_idx: u8,
    pub rcu_tasks_idle_cpu: c_int,
    pub rcu_tasks_holdout_list: list_head,
    pub rcu_tasks_exit_cpu: c_int,
    pub rcu_tasks_exit_list: list_head,

    pub trc_reader_nesting: c_int,
    pub trc_reader_scp: *mut srcu_ctr __percpu,

    pub rcu_trivial_preempt_nesting: c_int,

    pub sched_info: sched_info,
    pub tasks: list_head,
    pub pushable_tasks: plist_node,
    pub pushable_dl_tasks: rb_node,
    pub mm: *mut mm_struct,
    pub active_mm: *mut mm_struct,
    pub exec_state: *mut task_exec_state __rcu,
    pub exit_state: c_int,
    pub exit_code: c_int,
    pub exit_signal: c_int,
// The signal sent when the parent dies:
    pub pdeath_signal: c_int,
// JOBCTL_*, siglock protected:
    pub jobctl: c_ulong,
// Used for emulating ABI behavior of previous Linux versions:
    pub personality: c_uint,
// Scheduler bits, serialized by scheduler locks:
    pub sched_reset_on_fork:1: unsigned,
    pub sched_contributes_to_load:1: unsigned,
    pub sched_migrated:1: unsigned,
    pub sched_task_hot:1: unsigned,
// Force alignment to the next boundary:
    pub :0: unsigned,
// Unserialized, strictly 'current'
//
// This field must not be in the scheduler word above due to wakelist
// queueing no longer being serialized by p->on_cpu. However:
//
// p->XXX = X;			ttwu()
// schedule()			  if (p->on_rq && ..) // false
// smp_mb__after_spinlock();	  if (smp_load_acquire(&p->on_cpu) && //true
// deactivate_task()		      ttwu_queue_wakelist())
// p->on_rq = 0;			p->sched_remote_wakeup = Y;
//
// guarantees all stores of 'current' are visible before
// ->sched_remote_wakeup gets used, so it can be in this word.
//
    pub sched_remote_wakeup:1: unsigned,

    pub sched_rt_mutex:1: unsigned,

// Bit to tell TOMOYO we're in execve():
    pub in_execve:1: unsigned,
    pub in_iowait:1: unsigned,

    pub restore_sigmask:1: unsigned,

    pub in_user_fault:1: unsigned,

// whether the LRU algorithm may apply to this access
    pub in_lru_fault:1: unsigned,

    pub brk_randomized:1: unsigned,

// disallow userland-initiated cgroup migration
    pub no_cgroup_migration:1: unsigned,
// task is frozen/stopped (used by the cgroup freezer)
    pub frozen:1: unsigned,

    pub use_memdelay:1: unsigned,

// Stalled due to lack of memory
    pub in_memstall:1: unsigned,

// Used by page_owner=on to detect recursion in page tracking.
    pub in_page_owner:1: unsigned,

// Recursion prevention for eventfd_signal()
    pub in_eventfd:1: unsigned,

    pub pasid_activated:1: unsigned,

    pub reported_split_lock:1: unsigned,

// delay due to memory thrashing
    pub in_thrashing:1: unsigned,

    pub in_nf_duplicate:1: unsigned,

    pub net_xmit: netdev_xmit,

    pub /: *mut *mut unsigned long atomic_flags; / Flags requiring atomic access.,
    pub restart_block: restart_block,
    pub pid: pid_t,
    pub tgid: pid_t,

// Canary value for the -fstack-protector GCC feature:
    pub stack_canary: c_ulong,

//
// Pointers to the (original) parent process, youngest child, younger sibling,
// older sibling, respectively.  (p->father can be replaced with
// p->real_parent->pid)
//
// Real parent process:
    pub real_parent: *mut task___rcu,
// Recipient of SIGCHLD, wait4() reports:
    pub parent: *mut task___rcu,
//
// Children/sibling form the list of natural children:
//
    pub children: list_head,
    pub sibling: list_head,
    pub group_leader: *mut task_struct,
//
// 'ptraced' is the list of tasks this task is using ptrace() on.
//
// This includes both natural children and PTRACE_ATTACH targets.
// 'ptrace_entry' is this task's link on the p->parent->ptraced list.
//
    pub ptraced: list_head,
    pub ptrace_entry: list_head,
// PID/PID hash table linkage.
    pub thread_pid: *mut pid,
    pub pid_links: [hlist_node; PIDTYPE_MAX],
    pub thread_node: list_head,
    pub vfork_done: *mut completion,
// CLONE_CHILD_SETTID:
    pub set_child_tid: *mut int __user,
// CLONE_CHILD_CLEARTID:
    pub clear_child_tid: *mut int __user,
// PF_KTHREAD | PF_IO_WORKER
    pub worker_private: *mut c_void,
    pub utime: u64,
    pub stime: u64,

    pub utimescaled: u64,
    pub stimescaled: u64,

    pub gtime: u64,
    pub prev_cputime: prev_cputime,

    pub vtime: vtime,

    pub tick_dep_mask: core::sync::atomic::AtomicI32,

// Context switch counts:
    pub nvcsw: c_ulong,
    pub nivcsw: c_ulong,
// Monotonic time in nsecs:
    pub start_time: u64,
// Boot based time in nsecs:
    pub start_boottime: u64,
// MM fault and swap info: this can arguably be seen as either mm-specific or thread-specific:
    pub min_flt: c_ulong,
    pub maj_flt: c_ulong,
// Empty if CONFIG_POSIX_CPUTIMERS=n
    pub posix_cputimers: posix_cputimers,

    pub posix_cputimers_work: posix_cputimers_work,

// Process credentials:
// Tracer's credentials at attach:
    pub ptracer_cred: *const cred __rcu,
// Objective and real subjective task credentials (COW):
    pub real_cred: *const cred __rcu,
// Effective (overridable) subjective task credentials (COW):
    pub cred: *const cred __rcu,

// Cached requested key.
    pub cached_requested_key: *mut key,

//
// executable name, excluding path.
//
// - normally initialized by begin_new_exec()
// - set it with set_task_comm() to ensure it is always
// NUL-terminated and zero-padded
//
    pub comm: [c_char; TASK_COMM_LEN],
    pub nameidata: *mut nameidata,

    pub sysvsem: sysv_sem,
    pub sysvshm: sysv_shm,

    pub last_switch_count: c_ulong,
    pub last_switch_time: c_ulong,

// Filesystem information:
    pub real_fs: *mut fs_struct,
    pub fs: *mut fs_struct,
// Open file information:
    pub files: *mut files_struct,

    pub io_uring: *mut io_uring_task,
    pub io_uring_restrict: *mut io_restriction,

// Namespaces:
    pub nsproxy: *mut nsproxy,
// Signal handlers:
    pub signal: *mut signal_struct,
    pub sighand: *mut sighand___rcu,
    pub blocked: sigset_t,
    pub real_blocked: sigset_t,
// Restored if set_restore_sigmask() was used:
    pub saved_sigmask: sigset_t,
    pub pending: sigpending,
    pub sas_ss_sp: c_ulong,
    pub sas_ss_size: usize,
    pub sas_ss_flags: c_uint,
    pub task_works: *mut callback_head,

    pub audit_context: *mut audit_context,

    pub loginuid: kuid_t,
    pub sessionid: c_uint,

    pub seccomp: seccomp,
    pub syscall_dispatch: syscall_user_dispatch,
// Thread group tracking:
    pub parent_exec_id: u64,
    pub self_exec_id: u64,
// Protection against (de-)allocation: mm, files, fs, tty, keyrings, mems_allowed, mempolicy:
    pub alloc_lock: spinlock_t,
// Protection of the PI data structures:
    pub pi_lock: raw_spinlock_t,
    pub wake_q: wake_q_node,

// PI waiters blocked on a rt_mutex held by this task:
    pub pi_waiters: rb_root_cached,
// Updated under owner's pi_lock and rq lock
    pub pi_top_task: *mut task_struct,
// Deadlock detection and priority inheritance handling:
    pub pi_blocked_on: *mut rt_mutex_waiter,

    pub /: *mut *mut *mut mutex blocked_on; / lock we're blocked on,
    pub blocked_lock: raw_spinlock_t,
//
// The task that is boosting this task; a back link for the current
// donor stack. Set in schedule() -> find_proxy_task() and only stable
// under preempt_disable().
//
    pub blocked_donor: *mut task_struct,

//
// Encoded lock address causing task block (lower 2 bits = type from
// <linux/hung_task.h>). Accessed via hung_task_*() helpers.
//
    pub blocker: c_ulong,

    pub non_block_count: c_int,

    pub irqtrace: irqtrace_events,
    pub hardirq_threaded: c_uint,
    pub hardirq_chain_key: u64,
    pub softirqs_enabled: c_int,
    pub softirq_context: c_int,
    pub irq_config: c_int,

    pub softirq_disable_cnt: c_int,

    pub curr_chain_key: u64,
    pub lockdep_depth: c_int,
    pub lockdep_recursion: c_uint,
    pub lockdep_seq: c_uint,
    pub held_locks: [held_lock; MAX_LOCK_DEPTH],
    pub in_ubsan: c_uint,

// Journalling filesystem info:
    pub journal_info: *mut c_void,
// Stacked block device info:
    pub bio_list: *mut bio_list,
// Stack plugging:
    pub plug: *mut blk_plug,
// VM state:
    pub reclaim_state: *mut reclaim_state,
    pub io_context: *mut io_context,

    pub capture_control: *mut capture_control,

// Ptrace state:
    pub ptrace_message: c_ulong,
    pub last_siginfo: *mut kernel_siginfo_t,
    pub ioac: task_io_accounting,

// Pressure stall state
    pub psi_flags: c_uint,

// Accumulated RSS usage:
    pub acct_rss_mem1: u64,
// Accumulated virtual memory usage:
    pub acct_vm_mem1: u64,
// stime + utime since last update:
    pub acct_timexpd: u64,

// Protected by ->alloc_lock:
    pub mems_allowed: nodemask_t,
// Sequence number to catch updates:
    pub mems_allowed_seq: seqcount_spinlock_t,
    pub cpuset_mem_spread_rotor: c_int,

// Control Group info protected by css_set_lock:
    pub cgroups: *mut css_set __rcu,
// cg_list protected by css_set_lock and tsk->alloc_lock:
    pub cg_list: list_head,

    pub cg_dead_lnode: llist_node,

    pub closid: u32,
    pub rmid: u32,

    pub futex: futex_sched_data,
    pub perf_recursion: [u8; PERF_NR_CONTEXTS],
    pub perf_event_ctxp: *mut perf_event_context,
    pub perf_event_mutex: mutex,
    pub perf_event_list: list_head,
    pub perf_ctx_data: *mut perf_ctx_data __rcu,

    pub ipi_mask: task_ipi_mask __private,

    pub preempt_disable_ip: c_ulong,

// Protected by alloc_lock:
    pub mempolicy: *mut mempolicy,
    pub il_prev: c_short,
    pub il_weight: u8,
    pub pref_node_fork: c_short,

    pub numa_scan_seq: c_int,
    pub numa_scan_period: c_uint,
    pub numa_scan_period_max: c_uint,
    pub numa_preferred_nid: c_int,
    pub numa_migrate_retry: c_ulong,
// Migration stamp:
    pub node_stamp: u64,
    pub last_task_numa_placement: u64,
    pub last_sum_exec_runtime: u64,
    pub numa_work: callback_head,
//
// This pointer is only modified for current in syscall and
// pagefault context (and for tasks being destroyed), so it can be read
// from any of the following contexts:
// - RCU read-side critical section
// - current->numa_group from everywhere
// - task's runqueue locked, task not running
//
    pub numa_group: *mut numa_group __rcu,
//
// numa_faults is an array split into four regions:
// faults_memory, faults_cpu, faults_memory_buffer, faults_cpu_buffer
// in this precise order.
//
// faults_memory: Exponential decaying average of faults on a per-node
// basis. Scheduling placement decisions are made based on these
// counts. The values remain static for the duration of a PTE scan.
// faults_cpu: Track the nodes the process was running on when a NUMA
// hinting fault was incurred.
// faults_memory_buffer and faults_cpu_buffer: Record faults per node
// during the current scan window. When the scan completes, the counts
// in faults_memory and faults_cpu decay and these values are copied.
//
    pub numa_faults: *mut c_ulong,
    pub total_numa_faults: c_ulong,
//
// numa_faults_locality tracks if faults recorded during the last
// scan window were remote/local or failed to migrate. The task scan
// period is adapted based on the locality of the faults with different
// weights depending on whether they were shared or private faults
//
    pub numa_faults_locality: [c_ulong; 3],
    pub numa_pages_migrated: c_ulong,

    pub cache_work: callback_head,
    pub preferred_llc: c_int,
// 1: task was enqueued to its preferred LLC, 0 otherwise
    pub pref_llc_queued: c_int,

    pub rseq: rseq_data,
    pub mm_cid: sched_mm_cid,
    pub tlb_ubc: tlbflush_unmap_batch,
// Cache last used pipe for splice():
    pub splice_pipe: *mut pipe_inode_info,
    pub task_frag: page_frag,

    pub lazy_mmu_state: lazy_mmu_state,

    pub delays: *mut task_delay_info,

    pub make_it_fail: c_int,
    pub fail_nth: c_uint,

//
// When (nr_dirtied >= nr_dirtied_pause), it's time to call
// balance_dirty_pages() for a dirty throttling pause:
//
    pub nr_dirtied: c_int,
    pub nr_dirtied_pause: c_int,
// Start of a write-and-pause period:
    pub dirty_paused_when: c_ulong,

    pub latency_record_count: c_int,
    pub latency_record: [latency_record; LT_SAVECOUNT],
//
// Time slack values; these are used to round up poll() and
// select() etc timeout values. These are in nanoseconds.
//
    pub timer_slack_ns: u64,
    pub default_timer_slack_ns: u64,

    pub kasan_depth: c_uint,

    pub kcsan_ctx: kcsan_ctx,

    pub kcsan_save_irqtrace: irqtrace_events,

    pub kcsan_stack_depth: c_int,

    pub kmsan_ctx: kmsan_ctx,

    pub kunit_test: *mut kunit,

// Index of current stored address in ret_stack:
    pub curr_ret_stack: c_int,
    pub curr_ret_depth: c_int,
// Stack of return addresses for return function tracing:
    pub ret_stack: *mut c_ulong,
// Timestamp for last schedule:
    pub ftrace_timestamp: c_ulonglong,
    pub ftrace_sleeptime: c_ulonglong,
//
// Number of functions that haven't been traced
// because of depth overrun:
//
    pub trace_overrun: core::sync::atomic::AtomicI32,
// Pause tracing:
    pub tracing_graph_pause: core::sync::atomic::AtomicI32,

// Bitmask and counter of trace recursion:
    pub trace_recursion: c_ulong,

// See kernel/kcov.c for more details.
// Coverage collection mode enabled for this task (0 if disabled):
    pub kcov_mode: c_uint,
// Size of the kcov_area:
    pub kcov_size: c_uint,
// Buffer for coverage collection:
    pub kcov_area: *mut c_void,
// KCOV descriptor wired with this task or NULL:
    pub kcov: *mut kcov,
// KCOV descriptor for remote coverage collection from other tasks:
    pub kcov_remote: *mut kcov,
// KCOV common handle for remote coverage collection:
    pub kcov_handle: u64,
// KCOV sequence number:
    pub kcov_sequence: c_int,
// Collect coverage from softirq context:
    pub kcov_softirq: c_uint,
// Temporary storage for preempting remote coverage collection:
    pub kcov_saved_mode: c_uint,
    pub kcov_saved_size: c_uint,
    pub kcov_saved_area: *mut c_void,
    pub kcov_saved_kcov: *mut kcov,
    pub kcov_saved_sequence: c_int,

    pub memcg_in_oom: *mut mem_cgroup,

// Number of pages to reclaim on returning to userland:
    pub memcg_nr_pages_over_high: c_uint,
// Used by memcontrol for targeted memcg charge:
    pub active_memcg: *mut mem_cgroup,
// Cache for current->cgroups->memcg->nodeinfo[nid]->objcg lookups:
    pub objcg: *mut obj_cgroup,

    pub throttle_disk: *mut gendisk,

    pub utask: *mut uprobe_task,

    pub sequential_io: c_uint,
    pub sequential_io_avg: c_uint,

    pub kmap_ctrl: kmap_ctrl,

    pub task_state_change: c_ulong,

    pub saved_state_change: c_ulong,

    pub rcu: rcu_head,
    pub rcu_users: refcount_t,
    pub pagefault_disabled: c_int,

    pub oom_reaper_list: *mut task_struct,
    pub oom_reaper_timer: timer_list,

    pub stack_vm_area: *mut vm_struct,

// A live task holds one reference:
    pub stack_refcount: refcount_t,

    pub patch_state: c_int,

// Used by LSM modules for access restriction:
    pub security: *mut c_void,

// Used by BPF task local storage
    pub bpf_storage: *mut bpf_local_storage __rcu,
// Used for BPF run context
    pub bpf_ctx: *mut bpf_run_ctx,

// Used by BPF for per-TASK xdp storage
    pub bpf_net_context: *mut bpf_net_context,

    pub lowest_stack: c_ulong,

    pub prev_lowest_stack: c_ulong,

    pub mce_vaddr: *mut void __user,
    pub mce_kflags: __u64,
    pub mce_addr: u64,
    pub 62: __mce_reserved :,
    pub mce_kill_me: callback_head,
    pub mce_count: c_int,

    pub kretprobe_instances: llist_head,

    pub rethooks: llist_head,

//
// If L1D flush is supported on mm context switch
// then we use this callback head to queue kill work
// to kill tasks that are not running on SMT disabled
// cores
//
    pub l1d_flush_kill: callback_head,

//
// Per-task RV monitor, fixed in CONFIG_RV_PER_TASK_MONITORS.
// If memory becomes a concern, we can think about a dynamic method.
//
    pub rv: [rv_task_monitor; CONFIG_RV_PER_TASK_MONITORS],
    pub user_event_mm: *mut user_event_mm,

    pub unwind_info: unwind_task_info,

// CPU-specific state of this task:
    pub thread: thread_struct,
//
// New fields for task_struct should be added above here, so that
// they are included in the randomized portion of task_struct.
//
// C attribute field omitted

    pub static_branch_likely(&__sched_proxy_exec): return,

    pub false: return,

    pub TASK_REPORT: unsigned int state = (tsk_state | tsk_exit_state) &,
    pub TASK_REPORT_IDLE: state =,
//
// We're lying here, but rather than expose a completely new task state
// to userspace, we can make this appear as if the task has gone through
// a regular rt_mutex_lock() call.
// Report frozen tasks as uninterruptible.
//
    pub TASK_UNINTERRUPTIBLE: state =,
    pub fls(state): return,
    pub tsk->exit_state): return __task_state_index(READ_ONCE(tsk->__state),,
    pub "RSDTtXZPI": static char state_char[] =,
    pub 1)): *mut *mut BUILD_BUG_ON(TASK_REPORT_MAX  2 != 1 << (sizeof(state_char) -,
    pub state_char: [return; state],
    pub task_index_to_char(task_state_index(tsk)): return,

//
// __task_lazy_mmu_mode_active() - Test the lazy MMU mode state for a task.
// @tsk: The task to check.
//
// Test whether @tsk has its lazy MMU mode state set to active (i.e. enabled
// and not paused).
//
// This function only considers the state saved in task_struct; to test whether
// current actually is in lazy MMU mode, is_lazy_mmu_mode_active() should be
// used instead.
//
// This function is intended for architectures that implement the lazy MMU
// mode; it must not be called from generic code.
//
    pub &tsk->lazy_mmu_state: *mut *mut lazy_mmu_state state =,
    pub 0: return state->enable_count > 0 && state->pause_count ==,
//
// is_lazy_mmu_mode_active() - Test whether we are currently in lazy MMU mode.
//
// Test whether the current context is in lazy MMU mode. This is true if both:
// 1. We are not in interrupt context
// 2. Lazy MMU mode is active for the current task
//
// This function is intended for architectures that implement the lazy MMU
// mode; it must not be called from generic code.
//
    pub false: return,
    pub __task_lazy_mmu_mode_active(current): return,

    pub cad_pid: *mut extern struct pid,
//
// Per process flags
//
pub const PF_VCPU: c_uint = 0x00000001	/* I'm a virtual CPU */;
pub const PF_IDLE: c_uint = 0x00000002	/* I am an IDLE thread */;
pub const PF_EXITING: c_uint = 0x00000004	/* Getting shut down */;
pub const PF_POSTCOREDUMP: c_uint = 0x00000008	/* Coredumps should ignore this task */;
pub const PF_IO_WORKER: c_uint = 0x00000010	/* Task is an IO worker */;
pub const PF_WQ_WORKER: c_uint = 0x00000020	/* I'm a workqueue worker */;
pub const PF_FORKNOEXEC: c_uint = 0x00000040	/* Forked but didn't exec */;
pub const PF_MCE_PROCESS: c_uint = 0x00000080      /* Process policy on mce errors */;
pub const PF_SUPERPRIV: c_uint = 0x00000100	/* Used super-user privileges */;
pub const PF_DUMPCORE: c_uint = 0x00000200	/* Dumped core */;
pub const PF_SIGNALED: c_uint = 0x00000400	/* Killed by a signal */;
pub const PF_MEMALLOC: c_uint = 0x00000800	/* Allocating memory to free memory. See memalloc_noreclaim_save() */;
pub const PF_NPROC_EXCEEDED: c_uint = 0x00001000	/* set_user() noticed that RLIMIT_NPROC was exceeded */;
pub const PF_USED_MATH: c_uint = 0x00002000	/* If unset the fpu must be initialized before use */;
pub const PF_USER_WORKER: c_uint = 0x00004000	/* Kernel thread cloned from userspace thread */;
pub const PF_NOFREEZE: c_uint = 0x00008000	/* This thread should not be frozen */;
pub const PF_KCOMPACTD: c_uint = 0x00010000	/* I am kcompactd */;
pub const PF_KSWAPD: c_uint = 0x00020000	/* I am kswapd */;
pub const PF_MEMALLOC_NOFS: c_uint = 0x00040000	/* All allocations inherit GFP_NOFS. See memalloc_nfs_save() */;
pub const PF_MEMALLOC_NOIO: c_uint = 0x00080000	/* All allocations inherit GFP_NOIO. See memalloc_noio_save() */;
pub const PF_LOCAL_THROTTLE: c_uint = 0x00100000	/* Throttle writes only against the bdi I write to,;
// I am cleaning dirty pages from some other bdi.
pub const PF_KTHREAD: c_uint = 0x00200000	/* I am a kernel thread */;
pub const PF_RANDOMIZE: c_uint = 0x00400000	/* Randomize virtual address space */;
pub const PF__HOLE__00800000: c_uint = 0x00800000;
pub const PF__HOLE__01000000: c_uint = 0x01000000;
pub const PF__HOLE__02000000: c_uint = 0x02000000;
pub const PF_NO_SETAFFINITY: c_uint = 0x04000000	/* Userland is not allowed to meddle with cpus_mask */;
pub const PF_MCE_EARLY: c_uint = 0x08000000      /* Early kill for mce process policy */;
pub const PF_MEMALLOC_PIN: c_uint = 0x10000000	/* Allocations constrained to zones which allow long term pinning.;
// See memalloc_pin_save()
pub const PF_BLOCK_TS: c_uint = 0x20000000	/* plug has ts that needs updating */;
pub const PF__HOLE__40000000: c_uint = 0x40000000;
pub const PF_SUSPEND_TASK: c_uint = 0x80000000      /* This thread called freeze_processes() and should not be frozen */;
//
// Only the _current_ task can read/write to tsk->flags, but other
// tasks can access tsk->flags in readonly mode for example
// with tsk_used_math (like during threaded core dumping).
// There is however an exception to this rule during ptrace
// or during fork: the ptracer task is allowed to write to the
// child->flags of its traced child (same goes for fork, the parent
// can write to the child->flags), because we're guaranteed the
// child is not running and in turn not changing child->flags
// at the same time the parent does it.
//

    pub (0): do { (child)->flags &= ~PF_USED_MATH, (child)->flags |= (condition) ? PF_USED_MATH : 0; } while,

    pub (0): do { (child)->flags &= ~PF_USED_MATH, (child)->flags |= current->flags & PF_USED_MATH; } while,
// NOTE: this will return 0 or PF_USED_MATH, it will never return 1

    pub 1): (current->nr_cpus_allowed ==,
    pub PF_USER_WORKER)): return task->mm && !(task->flags & (PF_KTHREAD |,
// Per-process atomic flags.

    pub }: { return test_bit(PFA_##name, &p->atomic_flags);,

    pub }: { set_bit(PFA_##name, &p->atomic_flags);,

    pub }: { clear_bit(PFA_##name, &p->atomic_flags);,
    pub ~flags: current->flags &=,
    pub flags: current->flags |= orig_flags &,
    pub trial): *const *const extern int cpuset_cpumask_can_shrink(struct cpumask cur, struct cpumask,
    pub p): *mut extern int task_can_attach(struct task_struct,
    pub dl_bw): extern int dl_bw_alloc(int cpu, u64,
    pub dl_bw): extern void dl_bw_free(int cpu, u64,
// set_cpus_allowed_force() - consider using set_cpus_allowed_ptr() instead
    pub new_mask): *const *const extern void set_cpus_allowed_force(struct task_struct p, struct cpumask,
//
// set_cpus_allowed_ptr - set CPU affinity mask of a task
// @p: the task
// @new_mask: CPU affinity mask
//
// Return: zero if successful, or a negative error code
//
    pub new_mask): *const *const extern int set_cpus_allowed_ptr(struct task_struct p, struct cpumask,
    pub node): *mut *mut *mut extern int dup_user_cpus_ptr(struct task_struct dst, struct task_struct src, int,
    pub p): *mut extern void release_user_cpus_ptr(struct task_struct,
    pub mask): *const *const extern int dl_task_check_affinity(struct task_struct p, struct cpumask,
    pub p): *mut extern void force_compatible_cpus_allowed_ptr(struct task_struct,
    pub p): *mut extern void relax_compatible_cpus_allowed_ptr(struct task_struct,
    pub preempt): *mut *mut extern int yield_to(struct task_struct p, bool,
    pub nice): *mut *mut extern void set_user_nice(struct task_struct p, long,
    pub p): *const extern int task_prio(struct task_struct,
//
// task_nice - return the nice value of a given task.
// @p: the task in question.
//
// Return: The nice value [ -20 ... 0 ... 19 ].
//
    pub PRIO_TO_NICE((p)->static_prio): return,
    pub nice): *const *const extern int can_nice(struct task_struct p, int,
    pub p): *const extern int task_curr(struct task_struct,
    pub cpu): extern int idle_cpu(int,
    pub ): *const *const extern int sched_setscheduler(struct task_struct , int, struct sched_param,
    pub ): *const *const extern int sched_setscheduler_nocheck(struct task_struct , int, struct sched_param,
    pub p): *mut extern void sched_set_fifo(struct task_struct,
    pub p): *mut extern void sched_set_fifo_low(struct task_struct,
    pub p): *mut extern void sched_set_fifo_secondary(struct task_struct,
    pub nice): *mut *mut extern void sched_set_normal(struct task_struct p, int,
    pub ): *const *const extern int sched_setattr(struct task_struct , struct sched_attr,
    pub ): *const *const extern int sched_setattr_nocheck(struct task_struct , struct sched_attr,
    pub cpu): *mut *mut extern struct task_struct idle_task(int,
//
// is_idle_task - is the specified task an idle task?
// @p: the task in question.
//
// Return: 1 if @p is an idle task. 0 otherwise.
//
    pub PF_IDLE): return !!(p->flags &,
    pub cpu): *mut *mut extern struct task_struct curr_task(int,
    pub p): *mut extern void ia64_set_curr_task(int cpu, struct task_struct,
    pub yield(void): c_void,
#[repr(C)]
#[derive(Copy, Clone)]
pub union thread_union {
    pub task: task_struct,

    pub thread_info: thread_info,
    pub stack: [c_ulong; THREAD_SIZE/sizeof(long)],
}

//
// find a task by one of its numerical ids
//
// find_task_by_pid_ns():
// finds a task by its pid in the specified namespace
// find_task_by_vpid():
// finds a task by its virtual pid
//
// see also find_vpid() etc in include/linux/pid.h
//
// find a task by its virtual pid and get the task struct
//
extern "C" {
    pub fn wake_up_state(tsk: *mut task_struct, state: c_uint) -> c_int;
}
extern "C" {
    pub fn wake_up_process(tsk: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn wake_up_new_task(tsk: *mut task_struct);
}
extern "C" {
    pub fn kick_process(tsk: *mut task_struct);
}
extern "C" {
    pub fn __set_task_comm(tsk: *mut task_struct, from: *const c_char, exec: bool);
}

//
// - Why not use task_lock()?
// User space can randomly change their names anyway, so locking for readers
// doesn't make sense. For writers, locking is probably necessary, as a race
// condition could lead to long-term mixed results.
// The strscpy_pad() in __set_task_comm() can ensure that the task comm is
// always NUL-terminated and zero-padded. Therefore the race condition between
// reader and writer is not an issue.
//
// - BUILD_BUG_ON() can help prevent the buf from being truncated.
// Since the callers don't perform any return value checks, this safeguard is
// necessary.
//

//
// Fold TIF_NEED_RESCHED into the preempt_count; anybody setting
// TIF_NEED_RESCHED remotely (for the first time) will also send
// this IPI.
//
extern "C" {
    pub fn wait_task_inactive(: *mut task_struct, match_state: c_uint) -> c_ulong;
}
//
// Set thread flags in other task's structures.
// See asm/thread_info.h for TIF_xxxx flags available:
//
extern "C" {
    pub fn test_and_set_ti_thread_flag(_arg: task_thread_info(tsk), _arg: flag) -> return;
}
extern "C" {
    pub fn test_and_clear_ti_thread_flag(_arg: task_thread_info(tsk), _arg: flag) -> return;
}
extern "C" {
    pub fn test_ti_thread_flag(_arg: task_thread_info(tsk), _arg: flag) -> return;
}
extern "C" {
    pub fn unlikely(_arg: test_tsk_thread_flag(tsk, _arg: TIF_NEED_RESCHED)) -> return;
}
//
// cond_resched() and cond_resched_lock(): latency reduction via
// explicit rescheduling in places that are safe. The return
// value indicates whether a reschedule was done in fact.
// cond_resched_lock() will drop the spinlock before scheduling,
//

extern "C" {
    pub fn __cond_resched() -> c_int;
}

extern "C" {
    pub fn static_call_mod(_arg: cond_resched)() -> return;
}

extern "C" {
    pub fn dynamic_cond_resched() -> c_int;
}
extern "C" {
    pub fn dynamic_cond_resched() -> return;
}

extern "C" {
    pub fn __cond_resched() -> return;
}

extern "C" {
    pub fn __cond_resched_lock(__must_hold(lock: *mut *mut spinlock_t lock)) -> c_int;
}
extern "C" {
    pub fn __cond_resched_rwlock_read(__must_hold_shared(lock: *mut *mut rwlock_t lock)) -> c_int;
}
extern "C" {
    pub fn __cond_resched_rwlock_write(__must_hold(lock: *mut *mut rwlock_t lock)) -> c_int;
}
pub const MIGHT_RESCHED_RCU_SHIFT: c_int = 8;

//
// Non RT kernels have an elevated preempt count due to the held lock,
// but are not allowed to be inside a RCU read side critical section
//

//
// spin/rw_lock() on RT implies rcu_read_lock(). The might_sleep() check in
// cond_resched*lock() has to take that into account because it checks for
// preempt_count() and rcu_preempt_depth().
//

// The task should only be setting itself as blocked
// Currently we serialize blocked_on under the task::blocked_lock
//
// Check ensure we don't overwrite existing mutex value
// with a different mutex. Note, setting it to the same
// lock repeatedly is ok.
//
// Currently we serialize blocked_on under the task::blocked_lock
//
// There may be cases where we re-clear already cleared
// blocked_on relationships, but make sure we are not
// clearing the relationship with a different lock.
//

extern "C" {
    pub fn unlikely(_arg: tif_need_resched()) -> return;
}
//
// Wrappers for p->thread_info->cpu access. No-op on UP.
//

extern "C" {
    pub fn READ_ONCE(_arg: task_thread_info(p)->cpu) -> return;
}
extern "C" {
    pub fn set_task_cpu(p: *mut task_struct, cpu: c_uint);
}

extern "C" {
    pub fn sched_task_on_rq(p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn get_wchan(p: *mut task_struct) -> c_ulong;
}
//
// In order to reduce various lock holder preemption latencies provide an
// interface to see if a vCPU is currently running or not.
//
// This allows us to terminate optimistic spin loops and block, analogous to
// the native optimistic spin heuristic of testing if the lock owner task is
// running or not.
//

extern "C" {
    pub fn sched_setaffinity(pid: pid_t, new_mask: *const cpumask) -> c_long;
}
extern "C" {
    pub fn sched_getaffinity(pid: pid_t, mask: *mut cpumask) -> c_long;
}

//
// As lock holder preemption issue, we both skip spinning if
// task is not on cpu or its cpu is preempted
//
extern "C" {
    pub fn READ_ONCE(!vcpu_is_preempted(task_cpu(owner): owner->on_cpu) &&) -> return;
}
// Returns effective CPU energy utilization, as seen by the scheduler
extern "C" {
    pub fn sched_cpu_util(cpu: c_int) -> c_ulong;
}

extern "C" {
    pub fn sched_core_free(tsk: *mut task_struct);
}
extern "C" {
    pub fn sched_core_fork(p: *mut task_struct);
}
extern "C" {
    pub fn sched_core_idle_cpu(cpu: c_int) -> c_int;
}

extern "C" {
    pub fn sched_set_stop_task(cpu: c_int, stop: *mut task_struct);
}

// Avoids recursive inclusion hell

extern "C" {
    pub fn sched_mm_cid_before_execve(t: *mut task_struct);
}
extern "C" {
    pub fn sched_mm_cid_after_execve(t: *mut task_struct);
}
extern "C" {
    pub fn sched_mm_cid_exit(t: *mut task_struct);
}

//
// Use the processor id as a fall-back when the mm cid feature is
// disabled. This provides functional per-cpu data structure accesses
// in user-space, althrough it won't provide the memory usage benefits.
//
extern "C" {
    pub fn task_cpu(_arg: t) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_cache_time {
    pub runtime: u64,
    pub epoch: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_cache_stat {
    pub pcpu_sched: *mut sched_cache_time __percpu,
    pub lock: raw_spinlock_t,
    pub epoch: c_ulong,
    pub nr_running_avg: u64,
    pub next_scan: c_ulong,
    pub footprint: c_ulong,
    pub cpu: c_int,
    pub ____cacheline_aligned_in_smp: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_cache_stat {

    pub ___migrate_enable(void): extern void,
    pub rq: struct,
    pub runqueues): DECLARE_PER_CPU_SHARED_ALIGNED(struct rq,,
//
// The "struct rq" is not available here, so we can't access the
// "runqueues" with this_cpu_ptr(), as the compilation will fail in
// this_cpu_ptr() -> raw_cpu_ptr() -> __verify_pcpu_ptr():
// typeof((ptr) + 0)
//
// So use arch_raw_cpu_ptr()/PERCPU_PTR() directly here.
//

    pub current: *mut *mut task_p =,

//
// Check both overflow from migrate_disable() and superfluous
// migrate_enable().
//

//
// Ensure stop_task runs either before or after this, and that
// __set_cpus_allowed_ptr(SCA_MIGRATE_ENABLE) doesn't schedule().
//
// Mustn't clear migration_disabled() until cpus_ptr points back at the
// regular cpus_mask, otherwise things that race (eg.
// select_fallback_rq) get confused.
//
    pub 0: p->migration_disabled =,
    pub current: *mut *mut task_p =,

//
// Warn about overflow half-way through the range.
//
    pub 0): WARN_ON_ONCE((s16)p->migration_disabled <,

    pub 1: p->migration_disabled =,

//
// So that it is possible to not export the runqueues variable, define and
// export migrate_enable/migrate_disable in kernel/sched/core.c too, and use
// them for the modules. The macro "INSTANTIATE_EXPORTED_MIGRATE_DISABLE" will
// be defined in kernel/sched/core.c.
//

    pub migrate_disable(void): extern void,
    pub migrate_enable(void): extern void,

    pub migrate_disable(void): extern void,
    pub migrate_enable(void): extern void,

