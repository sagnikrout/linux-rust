//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/rcu/tasks.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Task-based RCU implementations.
//
// Copyright (C) 2020 Paul E. McKenney
//

//
// Generic data structures.
extern "C" {
    pub fn void(rtp: *mut *mut rcu_tasks_gp_func_t)(struct rcu_tasks) -> typedef;
}
extern "C" {
    pub fn void(hop: *mut *mut pregp_func_t)(struct list_head) -> typedef;
}
extern "C" {
    pub fn void(t: *mut *mut pertask_func_t)(struct task_struct, hop: *mut list_head) -> typedef;
}
extern "C" {
    pub fn void(hop: *mut *mut postscan_func_t)(struct list_head) -> typedef;
}
extern "C" {
    pub fn void(hop: *mut *mut holdouts_func_t)(struct list_head, ndrpt: bool, frptp: *mut bool) -> typedef;
}
extern "C" {
    pub fn void(rtp: *mut *mut postgp_func_t)(struct rcu_tasks) -> typedef;
}
//
// struct rcu_tasks_percpu - Per-CPU component of definition for a Tasks-RCU-like mechanism.
// @cblist: Callback list.
// @lock: Lock protecting per-CPU callback list.
// @rtp_jiffies: Jiffies counter value for statistics.
// @lazy_timer: Timer to unlazify callbacks.
// @urgent_gp: Number of additional non-lazy grace periods.
// @rtp_n_lock_retries: Rough lock-contention statistic.
// @rtp_work: Work queue for invoking callbacks.
// @rtp_irq_work: IRQ work queue for deferred wakeups.
// @barrier_q_head: RCU callback for barrier operation.
// @rtp_blkd_tasks: List of tasks blocked as readers.
// @rtp_exit_list: List of tasks in the latter portion of do_exit().
// @cpu: CPU number corresponding to this entry.
// @index: Index of this CPU in rtpcp_array of the rcu_tasks structure.
// @rtpp: Pointer to the rcu_tasks structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_tasks_percpu {
    pub cblist: rcu_segcblist,
    pub lock: raw_spinlock_t __private,
    pub rtp_jiffies: c_ulong,
    pub rtp_n_lock_retries: c_ulong,
    pub lazy_timer: timer_list,
    pub urgent_gp: c_uint,
    pub rtp_work: work_struct,
    pub rtp_irq_work: irq_work,
    pub barrier_q_head: rcu_head,
    pub rtp_blkd_tasks: list_head,
    pub rtp_exit_list: list_head,
    pub cpu: c_int,
    pub index: c_int,
    pub rtpp: *mut rcu_tasks,
}

//
// struct rcu_tasks - Definition for a Tasks-RCU-like mechanism.
// @cbs_wait: RCU wait allowing a new callback to get kthread's attention.
// @cbs_gbl_lock: Lock protecting callback list.
// @tasks_gp_mutex: Mutex protecting grace period, needed during mid-boot dead zone.
// @gp_func: This flavor's grace-period-wait function.
// @gp_state: Grace period's most recent state transition (debugging).
// @gp_sleep: Per-grace-period sleep to prevent CPU-bound looping.
// @init_fract: Initial backoff sleep interval.
// @gp_jiffies: Time of last @gp_state transition.
// @gp_start: Most recent grace-period start in jiffies.
// @tasks_gp_seq: Number of grace periods completed since boot in upper bits.
// @n_ipis: Number of IPIs sent to encourage grace periods to end.
// @kthread_ptr: This flavor's grace-period/callback-invocation kthread.
// @lazy_jiffies: Number of jiffies to allow callbacks to be lazy.
// @pregp_func: This flavor's pre-grace-period function (optional).
// @pertask_func: This flavor's per-task scan function (optional).
// @postscan_func: This flavor's post-task scan function (optional).
// @holdouts_func: This flavor's holdout-list scan function (optional).
// @postgp_func: This flavor's post-grace-period function (optional).
// @call_func: This flavor's call_rcu()-equivalent function.
// @wait_state: Task state for synchronous grace-period waits (default TASK_UNINTERRUPTIBLE).
// @rtpcpu: This flavor's rcu_tasks_percpu structure.
// @rtpcp_array: Array of pointers to rcu_tasks_percpu structure of CPUs in cpu_possible_mask.
// @percpu_enqueue_shift: Shift down CPU ID this much when enqueuing callbacks.
// @percpu_enqueue_lim: Number of per-CPU callback queues in use for enqueuing.
// @percpu_dequeue_lim: Number of per-CPU callback queues in use for dequeuing.
// @percpu_dequeue_gpseq: RCU grace-period number to propagate enqueue limit to dequeuers.
// @barrier_q_mutex: Serialize barrier operations.
// @barrier_q_count: Number of queues being waited on.
// @barrier_q_completion: Barrier wait/wakeup mechanism.
// @barrier_q_seq: Sequence number for barrier operations.
// @barrier_q_start: Most recent barrier start in jiffies.
// @name: This flavor's textual name.
// @kname: This flavor's kthread name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_tasks {
    pub cbs_wait: rcuwait,
    pub cbs_gbl_lock: raw_spinlock_t,
    pub tasks_gp_mutex: mutex,
    pub gp_state: c_int,
    pub gp_sleep: c_int,
    pub init_fract: c_int,
    pub gp_jiffies: c_ulong,
    pub gp_start: c_ulong,
    pub tasks_gp_seq: c_ulong,
    pub n_ipis: c_ulong,
    pub kthread_ptr: *mut task_struct,
    pub lazy_jiffies: c_ulong,
    pub gp_func: rcu_tasks_gp_func_t,
    pub pregp_func: pregp_func_t,
    pub pertask_func: pertask_func_t,
    pub postscan_func: postscan_func_t,
    pub holdouts_func: holdouts_func_t,
    pub postgp_func: postgp_func_t,
    pub call_func: call_rcu_func_t,
    pub wait_state: c_uint,
    pub rtpcpu: *mut rcu_tasks_percpu __percpu,
    pub rtpcp_array: *mut rcu_tasks_percpu,
    pub percpu_enqueue_shift: c_int,
    pub percpu_enqueue_lim: c_int,
    pub percpu_dequeue_lim: c_int,
    pub percpu_dequeue_gpseq: c_ulong,
    pub barrier_q_mutex: mutex,
    pub barrier_q_count: core::sync::atomic::AtomicI32,
    pub barrier_q_completion: completion,
    pub barrier_q_seq: c_ulong,
    pub barrier_q_start: c_ulong,
    pub name: *mut c_char,
    pub kname: *mut c_char,
}

extern "C" {
    pub fn call_rcu_tasks_iw_wakeup(iwp: *mut irq_work) -> static void;
}

// Report delay of scan exiting tasklist in rcu_tasks_postscan().
extern "C" {
    pub fn tasks_rcu_exit_stall(unused: *mut timer_list) -> static void;
}
extern "C" {
    pub fn DEFINE_TIMER(_arg: tasks_rcu_exit_stall_timer, _arg: tasks_rcu_exit_stall) -> static;
}

// Control stall timeouts.  Disable with <= 0, otherwise jiffies till stall.

// RCU tasks grace-period state for debugging.
pub const RTGS_INIT: c_int = 0;
pub const RTGS_WAIT_WAIT_CBS: c_int = 1;
pub const RTGS_WAIT_GP: c_int = 2;
pub const RTGS_PRE_WAIT_GP: c_int = 3;
pub const RTGS_SCAN_TASKLIST: c_int = 4;
pub const RTGS_POST_SCAN_TASKLIST: c_int = 5;
pub const RTGS_WAIT_SCAN_HOLDOUTS: c_int = 6;
pub const RTGS_SCAN_HOLDOUTS: c_int = 7;
pub const RTGS_POST_GP: c_int = 8;
pub const RTGS_WAIT_READERS: c_int = 9;
pub const RTGS_INVOKE_CBS: c_int = 10;
pub const RTGS_WAIT_CBS: c_int = 11;

//
// Generic code.
extern "C" {
    pub fn rcu_tasks_invoke_cbs_wq(wp: *mut work_struct) -> static void;
}
// Record grace-period phase and time.

// Return state name.

// Initialize per-CPU callback lists for the specified flavor of
// Tasks RCU.  Do not enqueue callbacks before this function is invoked.
// Compute wakeup time for lazy callback timer.
// Timer handler that unlazifies lazy callbacks.
// IRQ-work handler that does deferred wakeup for call_rcu_tasks_generic().
// Enqueue a callback for the specified flavor of Tasks RCU.
// Queuing callbacks before initialization not yet supported.
// We can't create the kthread with interrupts disabled because a
// scheduler spinlock might be held, so kthread creation is deferred
// until core_initcall() time.  Similarly, wakeups are deferred using
// irq_work in order to avoid potential scheduler-lock-deadlock
// lockdep splats.
// RCU callback function for rcu_barrier_tasks_generic().
// Wait for all in-flight callbacks for the specified RCU Tasks flavor.
// Operates in a manner similar to rcu_barrier().
// Advance callbacks and indicate whether either a grace period or
// callback invocation is needed.
// Advance and accelerate any new callbacks.
// Should we shrink down to a single callback queue?
// Shrink down to a single callback queue if appropriate.
// This is done in two stages: (1) If there are no more than
// rcu_task_collapse_lim callbacks on CPU 0 and none on any other
// CPU, limit enqueueing to CPU 0.  (2) After an RCU grace period,
// if there has not been an increase in callbacks, limit dequeuing
// to CPU 0.  Note the matching RCU read-side critical section in
// call_rcu_tasks_generic().
// Advance callbacks and invoke any that are ready.
// Workqueue flood to advance callbacks and invoke any that are ready.
// Wait for one grace period.
// If there were none, wait a bit and start over.
// Wait for one grace period.
// Invoke callbacks.
// RCU-tasks kthread that detects grace periods and invokes callbacks.
// Run on housekeeping CPUs by default.  Sysadm can move if desired.
//
// Each pass through the following loop makes one check for
// newly arrived callbacks, and, if there are some, waits for
// one RCU-tasks grace period and then invokes the callbacks.
// This loop is terminated by the system going down.  ;-)
//
// Wait for one grace period and invoke any callbacks
// that are ready.
// Paranoid sleep to keep this from entering a tight loop.
// Wait for a grace period for the specified flavor of Tasks RCU.
// Complain if the scheduler has not started.
// If the grace-period kthread is running, use it.
// Spawn RCU-tasks grace-period kthread.

//
// Print any non-default Tasks RCU settings.
//

// Dump out rcutorture-relevant state common to all RCU-tasks flavors.
// Dump out more rcutorture-relevant state common to all RCU-tasks flavors.

//
// Shared code between task-list-scanning variants of Tasks RCU.
// Wait for one RCU-tasks grace period.
//
// There were callbacks, so we need to wait for an RCU-tasks
// grace period.  Start off by scanning the task list for tasks
// that are not already voluntarily blocked.  Mark these tasks
// and make a list of them in holdouts.
//
// Each pass through the following loop scans the list of holdout
// tasks, removing any that are no longer holdouts.  When the list
// is empty, we are done.
//
// Start off with initial wait and slowly back off to 1 HZ wait.
// Slowly back off waiting for holdouts
// Print pre-stall informational messages if needed.

//
// Simple variant of RCU whose quiescent states are voluntary context
// switch, cond_resched_tasks_rcu_qs(), user-space execution, and idle.
// As such, grace periods can take one good long time.  There are no
// read-side primitives similar to rcu_read_lock() and rcu_read_unlock()
// because this implementation is intended to get the system into a safe
// state for some of the manipulations involved in tracing and the like.
// Finally, this implementation does not support high call_rcu_tasks()
// rates from multiple CPUs.  If this is required, per-CPU callback lists
// will be needed.
//
// The implementation uses rcu_tasks_wait_gp(), which relies on function
// pointers in the rcu_tasks structure.  The rcu_spawn_tasks_kthread()
// function sets these function pointers up so that rcu_tasks_wait_gp()
// invokes these functions in this order:
//
// rcu_tasks_pregp_step():
// Invokes synchronize_rcu() in order to wait for all in-flight
// t->on_rq and t->nvcsw transitions to complete.	This works because
// all such transitions are carried out with interrupts disabled.
// rcu_tasks_pertask(), invoked on every non-idle task:
// For every runnable non-idle task other than the current one, use
// get_task_struct() to pin down that task, snapshot that task's
// number of voluntary context switches, and add that task to the
// holdout list.
// rcu_tasks_postscan():
// Gather per-CPU lists of tasks in do_exit() to ensure that all
// tasks that were in the process of exiting (and which thus might
// not know to synchronize with this RCU Tasks grace period) have
// completed exiting.  The synchronize_rcu() in rcu_tasks_postgp()
// will take care of any tasks stuck in the non-preemptible region
// of do_exit() following its call to exit_tasks_rcu_finish().
// check_all_holdout_tasks(), repeatedly until holdout list is empty:
// Scans the holdout list, attempting to identify a quiescent state
// for each task on the list.  If there is a quiescent state, the
// corresponding task is removed from the holdout list.
// rcu_tasks_postgp():
// Invokes synchronize_rcu() in order to ensure that all prior
// t->on_rq and t->nvcsw transitions are seen by all CPUs and tasks
// to have happened before the end of this RCU Tasks grace period.
// Again, this works because all such transitions are carried out
// with interrupts disabled.
//
// For each exiting task, the exit_tasks_rcu_start() and
// exit_tasks_rcu_finish() functions add and remove, respectively, the
// current task to a per-CPU list of tasks that rcu_tasks_postscan() must
// wait on.  This is necessary because rcu_tasks_postscan() must wait on
// tasks that have already been removed from the global list of tasks.
//
// Pre-grace-period update-side code is ordered before the grace
// via the raw_spin_lock.*rcu_node().  Pre-grace-period read-side code
// is ordered before the grace period via synchronize_rcu() call in
// rcu_tasks_pregp_step() and by the scheduler's locks and interrupt
// disabling.
// Pre-grace-period preparation.
//
// Wait for all pre-existing t->on_rq and t->nvcsw transitions
// to complete.  Invoking synchronize_rcu() suffices because all
// these transitions occur with interrupts disabled.  Without this
// synchronize_rcu(), a read-side critical section that started
// before the grace period might be incorrectly seen as having
// started after the grace period.
//
// This synchronize_rcu() also dispenses with the need for a
// memory barrier on the first store to t->rcu_tasks_holdout,
// as it forces the store to happen after the beginning of the
// grace period.
//
// Check for quiescent states since the pregp's synchronize_rcu()
// Has the task been seen voluntarily sleeping?
//
// t->on_rq && !t->se.sched_delayed *could* be considered sleeping but
// since it is a spurious state (it will transition into the
// traditional blocked state or get woken up without outside
// dependencies), not considering it such should only affect timing.
//
// Be conservative for now and not include it.
//
// Idle tasks (or idle injection) within the idle loop are RCU-tasks
// quiescent states. But CPU boot code performed by the idle task
// isn't a quiescent state.
//
// Idle tasks on offline CPUs are RCU-tasks quiescent states.
// Per-task initial processing.
extern "C" {
    pub fn call_rcu_tasks(rhp: *mut rcu_head, func: rcu_callback_t);
}
// Processing between scanning taskslist and draining the holdout list.
//
// Exiting tasks may escape the tasklist scan. Those are vulnerable
// until their final schedule() with TASK_DEAD state. To cope with
// this, divide the fragile exit path part in two intersecting
// read side critical sections:
//
// 1) A task_struct list addition before calling exit_notify(),
// which may remove the task from the tasklist, with the
// removal after the final preempt_disable() call in do_exit().
//
// 2) An _RCU_ read side starting with the final preempt_disable()
// call in do_exit() and ending with the final call to schedule()
// with TASK_DEAD state.
//
// This handles the part 1). And postgp will handle part 2) with a
// call to synchronize_rcu().
//
// RT kernels need frequent pauses, otherwise
// pause at least once per pair of jiffies.
// Keep our place in the list while pausing.
// Nothing else traverses this list, so adding a
// bare list_head is OK.
// See if tasks are still holding out, complain if so.
// firstreport = false;
// Scan the holdout lists for tasks no longer holding out.
// Finish off the Tasks-RCU grace period.
//
// Because ->on_rq and ->nvcsw are not guaranteed to have a full
// memory barriers prior to them in the schedule() path, memory
// reordering on other CPUs could cause their RCU-tasks read-side
// critical sections to extend past the end of the grace period.
// However, because these ->nvcsw updates are carried out with
// interrupts disabled, we can use synchronize_rcu() to force the
// needed ordering on all such CPUs.
//
// This synchronize_rcu() also confines all ->rcu_tasks_holdout
// accesses to be within the grace period, avoiding the need for
// memory barriers for ->rcu_tasks_holdout accesses.
//
// In addition, this synchronize_rcu() waits for exiting tasks
// to complete their final preempt_disable() region of execution,
// enforcing the whole region before tasklist removal until
// the final schedule() with TASK_DEAD state to be an RCU TASKS
// read side critical section.
//

//
// call_rcu_tasks() - Queue an RCU for invocation task-based grace period
// @rhp: structure to be used for queueing the RCU updates.
// @func: actual callback function to be invoked after the grace period
//
// The callback function will be invoked some time after a full grace
// period elapses, in other words after all currently executing rcu-tasks
// read-side critical sections have completed. call_rcu_tasks() assumes
// that the read-side critical sections end at a voluntary context
// switch (not a preemption!), cond_resched_tasks_rcu_qs(), entry into idle,
// or transition to usermode execution.  As such, there are no read-side
// primitives analogous to rcu_read_lock() and rcu_read_unlock() because
// this primitive is intended to determine that all tasks have passed
// through a safe state, not so much for data-structure synchronization.
//
// See the description of call_rcu() for more detailed information on
// memory ordering guarantees.
//
// synchronize_rcu_tasks - wait until an rcu-tasks grace period has elapsed.
//
// Control will return to the caller some time after a full rcu-tasks
// grace period has elapsed, in other words after all currently
// executing rcu-tasks read-side critical sections have elapsed.  These
// read-side critical sections are delimited by calls to schedule(),
// cond_resched_tasks_rcu_qs(), idle execution, userspace execution, calls
// to synchronize_rcu_tasks(), and (in theory, anyway) cond_resched().
//
// This is a very specialized primitive, intended only for a few uses in
// tracing and other situations requiring manipulation of function
// preambles and profiling hooks.  The synchronize_rcu_tasks() function
// is not (yet) intended for heavy use from multiple CPUs.
//
// See the description of synchronize_rcu() for more detailed information
// on memory ordering guarantees.
//
// rcu_barrier_tasks - Wait for in-flight call_rcu_tasks() callbacks.
//
// Although the current implementation is guaranteed to wait, it is not
// obligated to, for example, if there are no pending callbacks.
//

// flags = 0;
// gp_seq = rcu_seq_current(&rcu_tasks.tasks_gp_seq);
//
// Protect against tasklist scan blind spot while the task is exiting and
// may be removed from the tasklist.  Do this by adding the task to yet
// another list.
//
// Note that the task will remove itself from this list, so there is no
// need for get_task_struct(), except in the case where rcu_tasks_pertask()
// adds it to the holdout list, in which case rcu_tasks_pertask() supplies
// the needed get_task_struct().
//
// Remove the task from the "yet another list" because do_exit() is now
// non-preemptible, allowing synchronize_rcu() to wait beyond this point.
//

//
// "Rude" variant of Tasks RCU, inspired by Steve Rostedt's
// trick of passing an empty function to schedule_on_each_cpu().
// This approach provides batching of concurrent calls to the synchronous
// synchronize_rcu_tasks_rude() API.  This invokes schedule_on_each_cpu()
// in order to send IPIs far and wide and induces otherwise unnecessary
// context switches on all online CPUs, whether idle or not.
//
// Callback handling is provided by the rcu_tasks_kthread() function.
//
// Ordering is provided by the scheduler's context-switch code.
// Empty function to allow workqueues to force a context switch.
// Wait for one rude RCU-tasks grace period.
extern "C" {
    pub fn call_rcu_tasks_rude(rhp: *mut rcu_head, func: rcu_callback_t) -> static void;
}
//
// call_rcu_tasks_rude() - Queue a callback rude task-based grace period
// @rhp: structure to be used for queueing the RCU updates.
// @func: actual callback function to be invoked after the grace period
//
// The callback function will be invoked some time after a full grace
// period elapses, in other words after all currently executing rude
// rcu-tasks read-side critical sections have completed. call_rcu_tasks_rude()
// assumes that the read-side critical sections end at context switch,
// cond_resched_tasks_rcu_qs(), or transition to usermode execution (as
// usermode execution is schedulable). As such, there are no read-side
// primitives analogous to rcu_read_lock() and rcu_read_unlock() because
// this primitive is intended to determine that all tasks have passed
// through a safe state, not so much for data-structure synchronization.
//
// See the description of call_rcu() for more detailed information on
// memory ordering guarantees.
//
// This is no longer exported, and is instead reserved for use by
// synchronize_rcu_tasks_rude().
//
// synchronize_rcu_tasks_rude - wait for a rude rcu-tasks grace period
//
// Control will return to the caller some time after a rude rcu-tasks
// grace period has elapsed, in other words after all currently
// executing rude rcu-tasks read-side critical sections have elapsed. These
// read-side critical sections are delimited by calls to schedule(),
// cond_resched_tasks_rcu_qs(), userspace execution (which is a schedulable
// context), and (in theory, anyway) cond_resched().
//
// This is a very specialized primitive, intended only for a few uses in
// tracing and other situations requiring manipulation of function preambles
// and profiling hooks.  The synchronize_rcu_tasks_rude() function is not
// (yet) intended for heavy use from multiple CPUs.
//
// See the description of synchronize_rcu() for more detailed information
// on memory ordering guarantees.
//

// flags = 0;
// gp_seq = rcu_seq_current(&rcu_tasks_rude.tasks_gp_seq);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_tasks_test_desc {
    pub rh: rcu_head,
    pub name: *const c_char,
    pub notrun: bool,
    pub runstart: c_ulong,
    pub (*gp_dbg)(void): *mut c_void,
}

// If not defined, the test is skipped.
// Dump rcu tasks status, if test failed.
// If not defined, the test is skipped.

//
// Return:  0 - test passed
// 1 - test failed, but have not timed out yet
// -1 - test failed and timed out
//
// Repeat the rcu_tasks_verify_self_tests() call once every second until the
// test passes or has timed out.
//
// Test fails but not timed out yet, reschedule another check

// Run the self-tests.

//
// Tracing variant of Tasks RCU.  This variant is designed to be used
// to protect tracing hooks, including those of BPF.  This variant
// is implemented via a straightforward mapping onto SRCU-fast.
extern "C" {
    pub fn srcu_batches_completed(_arg: &rcu_tasks_trace_srcu_struct) -> return;
}
