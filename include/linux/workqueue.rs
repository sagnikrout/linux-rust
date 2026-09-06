//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/workqueue.h
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
// workqueue.h --- work queue handling for Linux.
//

//
// The first word is the work queue pointer and the flags rolled into
// one
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum work_bits {
    WORK_STRUCT_PENDING_BIT	= 0,	/* work item is pending execution */
    WORK_STRUCT_INACTIVE_BIT,	/* work item is inactive */
    WORK_STRUCT_PWQ_BIT,		/* data points to pwq */
    WORK_STRUCT_LINKED_BIT,		/* next work is linked to this one */

    WORK_STRUCT_STATIC_BIT,		/* static initializer (debugobjects) */

    WORK_STRUCT_FLAG_BITS,

// color for workqueue flushing
    WORK_STRUCT_COLOR_SHIFT	= WORK_STRUCT_FLAG_BITS,
    WORK_STRUCT_COLOR_BITS	= 4,

//
// When WORK_STRUCT_PWQ is set, reserve 8 bits off of pwq pointer w
// debugobjects turned off. This makes pwqs aligned to 256 bytes (512
// bytes w/ DEBUG_OBJECTS_WORK) and allows 16 workqueue flush colors.
//
// MSB
// [ pwq pointer ] [ flush color ] [ STRUCT flags ]
// 4 bits        4 or 5 bits
//
    WORK_STRUCT_PWQ_SHIFT	= WORK_STRUCT_COLOR_SHIFT + WORK_STRUCT_COLOR_BITS,

//
// data contains off-queue information when !WORK_STRUCT_PWQ.
//
// MSB
// [ pool ID ] [ disable depth ] [ OFFQ flags ] [ STRUCT flags ]
// 16 bits          1 bit        4 or 5 bits
//
    WORK_OFFQ_FLAG_SHIFT	= WORK_STRUCT_FLAG_BITS,
    WORK_OFFQ_BH_BIT	= WORK_OFFQ_FLAG_SHIFT,
    WORK_OFFQ_FLAG_END,
    WORK_OFFQ_FLAG_BITS	= WORK_OFFQ_FLAG_END - WORK_OFFQ_FLAG_SHIFT,

    WORK_OFFQ_DISABLE_SHIFT	= WORK_OFFQ_FLAG_SHIFT + WORK_OFFQ_FLAG_BITS,
    WORK_OFFQ_DISABLE_BITS	= 16,

//
// When a work item is off queue, the high bits encode off-queue flags
// and the last pool it was on. Cap pool ID to 31 bits and use the
// highest number to indicate that no pool is associated.
//
    WORK_OFFQ_POOL_SHIFT	= WORK_OFFQ_DISABLE_SHIFT + WORK_OFFQ_DISABLE_BITS,
    WORK_OFFQ_LEFT		= BITS_PER_LONG - WORK_OFFQ_POOL_SHIFT,
    WORK_OFFQ_POOL_BITS	= WORK_OFFQ_LEFT <= 31 ? WORK_OFFQ_LEFT : 31,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum work_flags {
    WORK_STRUCT_PENDING	= 1 << WORK_STRUCT_PENDING_BIT,
    WORK_STRUCT_INACTIVE	= 1 << WORK_STRUCT_INACTIVE_BIT,
    WORK_STRUCT_PWQ		= 1 << WORK_STRUCT_PWQ_BIT,
    WORK_STRUCT_LINKED	= 1 << WORK_STRUCT_LINKED_BIT,

    WORK_STRUCT_STATIC	= 1 << WORK_STRUCT_STATIC_BIT,

    WORK_STRUCT_STATIC	= 0,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wq_misc_consts {
    WORK_NR_COLORS		= (1 << WORK_STRUCT_COLOR_BITS),

// not bound to any CPU, prefer the local CPU
    WORK_CPU_UNBOUND	= NR_CPUS,

// bit mask for work_busy() return values
    WORK_BUSY_PENDING	= 1 << 0,
    WORK_BUSY_RUNNING	= 1 << 1,

// maximum string length for set_worker_desc()
    WORKER_DESC_LEN		= 32,
}

// Convenience constants - of type 'unsigned long', not 'enum'!

#[repr(C)]
#[derive(Copy, Clone)]
pub struct delayed_work {
    pub work: work_struct,
    pub timer: timer_list,
// target workqueue and CPU ->timer uses to queue ->work
    pub wq: *mut workqueue_struct,
    pub cpu: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_work {
    pub work: work_struct,
    pub rcu: rcu_head,
// target workqueue ->rcu uses to queue ->work
    pub wq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wq_affn_scope {
    WQ_AFFN_DFL,			/* use system default */
    WQ_AFFN_CPU,			/* one pod per CPU */
    WQ_AFFN_SMT,			/* one pod per SMT */
    WQ_AFFN_CACHE,			/* one pod per LLC */
    WQ_AFFN_CACHE_SHARD,		/* synthetic sub-LLC shards */
    WQ_AFFN_NUMA,			/* one pod per NUMA node */
    WQ_AFFN_SYSTEM,			/* one pod across the whole system */

    WQ_AFFN_NR_TYPES,
}

//
// struct workqueue_attrs - A struct for workqueue attributes.
//
// This can be used to change attributes of an unbound workqueue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct workqueue_attrs {
//
// @nice: nice level
//
    pub nice: c_int,
//
// @cpumask: allowed CPUs
//
// Work items in this workqueue are affine to these CPUs and not allowed
// to execute on other CPUs. A pool serving a workqueue must have the
// same @cpumask.
//
    pub cpumask: cpumask_var_t,
//
// @__pod_cpumask: internal attribute used to create per-pod pools
//
// Internal use only.
//
// Per-pod unbound worker pools are used to improve locality. Always a
// subset of ->cpumask. A workqueue can be associated with multiple
// worker pools with disjoint @__pod_cpumask's. Whether the enforcement
// of a pool's @__pod_cpumask is strict depends on @affn_strict.
//
    pub __pod_cpumask: cpumask_var_t,
//
// @affn_strict: affinity scope is strict
//
// If clear, workqueue will make a best-effort attempt at starting the
// worker inside @__pod_cpumask but the scheduler is free to migrate it
// outside.
//
// If set, workers are only allowed to run inside @__pod_cpumask.
//
    pub affn_strict: bool,
//
// Below fields aren't properties of a worker_pool. They only modify how
// :c:func:`apply_workqueue_attrs` select pools and thus don't
// participate in pool hash calculations or equality comparisons.
//
// If @affn_strict is set, @cpumask isn't a property of a worker_pool
// either.
//
// @affn_scope: unbound CPU affinity scope
//
// CPU pods are used to improve execution locality of unbound work
// items. There are multiple pod types, one for each wq_affn_scope, and
// every CPU in the system belongs to one pod in every pod type. CPUs
// that belong to the same pod share the worker pool. For example,
// selecting %WQ_AFFN_NUMA makes the workqueue use a separate worker
// pool for each NUMA node.
//
    pub affn_scope: wq_affn_scope,
//
// @ordered: work items must be executed one by one in queueing order
//
    pub ordered: bool,
}

extern "C" {
    pub fn container_of(_arg: work, delayed_work: struct, _arg: work) -> return;
}
extern "C" {
    pub fn container_of(_arg: work, rcu_work: struct, _arg: work) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct execute_work {
    pub work: work_struct,
}

//
// NB: because we have to copy the lockdep_map, setting _key
// here is required, otherwise it could get initialised to the
// copy of the lockdep_map!
//

extern "C" {
    pub fn __init_work(work: *mut work_struct, onstack: c_int);
}
extern "C" {
    pub fn destroy_work_on_stack(work: *mut work_struct);
}
extern "C" {
    pub fn destroy_delayed_work_on_stack(work: *mut delayed_work);
}

//
// initialize all of a work item in one go
//
// NOTE! No point in using "atomic_long_set()": using a direct
// assignment of the work data initializer allows the compiler
// to generate better code.
//

//
// work_pending - Find out whether a work item is currently pending
// @work: The work item in question
//

//
// delayed_work_pending - Find out whether a delayable work item is currently
// pending
// @w: The work item in question
//

//
// Workqueue flags and constants.  For details, please refer to
// Documentation/core-api/workqueue.rst.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wq_flags {
    WQ_BH			= 1 << 0, /* execute in bottom half (softirq) context */
    WQ_UNBOUND		= 1 << 1, /* not bound to any cpu */
    WQ_FREEZABLE		= 1 << 2, /* freeze during suspend */
    WQ_MEM_RECLAIM		= 1 << 3, /* may be used for memory reclaim */
    WQ_HIGHPRI		= 1 << 4, /* high priority */
    WQ_CPU_INTENSIVE	= 1 << 5, /* cpu intensive workqueue */
    WQ_SYSFS		= 1 << 6, /* visible in sysfs, see workqueue_sysfs_register() */

//
// Per-cpu workqueues are generally preferred because they tend to
// show better performance thanks to cache locality.  Per-cpu
// workqueues exclude the scheduler from choosing the CPU to
// execute the worker threads, which has an unfortunate side effect
// of increasing power consumption.
//
// The scheduler considers a CPU idle if it doesn't have any task
// to execute and tries to keep idle cores idle to conserve power;
// however, for example, a per-cpu work item scheduled from an
// interrupt handler on an idle CPU will force the scheduler to
// execute the work item on that CPU breaking the idleness, which in
// turn may lead to more scheduling choices which are sub-optimal
// in terms of power consumption.
//
// Workqueues marked with WQ_POWER_EFFICIENT are per-cpu by default
// but become unbound if workqueue.power_efficient kernel param is
// specified.  Per-cpu workqueues which are identified to
// contribute significantly to power-consumption are identified and
// marked with this flag and enabling the power_efficient mode
// leads to noticeable power saving at the cost of small
// performance disadvantage.
//
// http://thread.gmane.org/gmane.linux.kernel/1480396
//
    WQ_POWER_EFFICIENT	= 1 << 7,
    WQ_PERCPU		= 1 << 8, /* bound to a specific cpu */

    __WQ_DESTROYING		= 1 << 15, /* internal: workqueue is destroying */
    __WQ_DRAINING		= 1 << 16, /* internal: workqueue is draining */
    __WQ_ORDERED		= 1 << 17, /* internal: workqueue is ordered */
    __WQ_LEGACY		= 1 << 18, /* internal: create*_workqueue() */
    __WQ_DEPRECATED		= 1 << 19, /* internal: workqueue is deprecated */

// BH wq only allows the following flags
    __WQ_BH_ALLOWS		= WQ_BH | WQ_HIGHPRI | WQ_PERCPU,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wq_consts {
    WQ_MAX_ACTIVE		= 2048,	  /* I like 2048, better ideas? */
    WQ_UNBOUND_MAX_ACTIVE	= WQ_MAX_ACTIVE,
    WQ_DFL_ACTIVE		= WQ_MAX_ACTIVE / 2,

//
// Per-node default cap on min_active. Unless explicitly set, min_active
// is set to min(max_active, WQ_DFL_MIN_ACTIVE). For more details, see
// workqueue_struct->min_active definition.
//
    WQ_DFL_MIN_ACTIVE	= 8,
}

//
// System-wide workqueues which are always present.
//
// system_percpu_wq is the one used by schedule[_delayed]_work[_on]().
// Multi-CPU multi-threaded.  There are users which expect relatively
// short queue flush time.  Don't queue works which can run for too
// long.
//
// system_highpri_wq is similar to system_percpu_wq but for work items which
// require WQ_HIGHPRI.
//
// system_long_wq is similar to system_percpu_wq but may host long running
// works.  Queue flushing might take relatively long.
//
// system_dfl_long_wq is similar to system_dfl_wq but it may host long running
// works.
//
// system_dfl_wq is unbound workqueue.  Workers are not bound to
// any specific CPU, not concurrency managed, and all queued works are
// executed immediately as long as max_active limit is not reached and
// resources are available.
//
// system_freezable_wq is equivalent to system_percpu_wq except that it's
// freezable.
//
// *_power_efficient_wq are inclined towards saving power and converted
// into WQ_UNBOUND variants if 'wq_power_efficient' is enabled; otherwise,
// they are same as their non-power-efficient counterparts - e.g.
// system_power_efficient_wq is identical to system_percpu_wq if
// 'wq_power_efficient' is disabled.  See WQ_POWER_EFFICIENT for more info.
//
// system_bh[_highpri]_wq are convenience interface to softirq. BH work items
// are executed in the queueing CPU's BH context in the queueing order.
//
extern "C" {
    pub fn workqueue_softirq_action(highpri: bool);
}
extern "C" {
    pub fn workqueue_softirq_dead(cpu: c_uint);
}
//
// alloc_workqueue - allocate a workqueue
// @fmt: printf format for the name of the workqueue
// @flags: WQ_* flags
// @max_active: max in-flight work items, 0 for default
// @...: args for @fmt
//
// For a per-cpu workqueue, @max_active limits the number of in-flight work
// items for each CPU. e.g. @max_active of 1 indicates that each CPU can be
// executing at most one work item for the workqueue.
//
// For unbound workqueues, @max_active limits the number of in-flight work items
// for the whole system. e.g. @max_active of 16 indicates that there can be
// at most 16 work items executing for the workqueue in the whole system.
//
// As sharing the same active counter for an unbound workqueue across multiple
// NUMA nodes can be expensive, @max_active is distributed to each NUMA node
// according to the proportion of the number of online CPUs and enforced
// independently.
//
// Depending on online CPU distribution, a node may end up with per-node
// max_active which is significantly lower than @max_active, which can lead to
// deadlocks if the per-node concurrency limit is lower than the maximum number
// of interdependent work items for the workqueue.
//
// To guarantee forward progress regardless of online CPU distribution, the
// concurrency limit on every node is guaranteed to be equal to or greater than
// min_active which is set to min(@max_active, %WQ_DFL_MIN_ACTIVE). This means
// that the sum of per-node max_active's may be larger than @max_active.
//
// For detailed information on %WQ_\* flags, please refer to
// Documentation/core-api/workqueue.rst.
//
// RETURNS:
// Pointer to the allocated workqueue on success, %NULL on failure.
//

//
// devm_alloc_workqueue - Resource-managed allocate a workqueue
// @dev: Device to allocate workqueue for
// @fmt: printf format for the name of the workqueue
// @flags: WQ_* flags
// @max_active: max in-flight work items, 0 for default
// @...: args for @fmt
//
// Resource managed workqueue, see alloc_workqueue() for details.
//
// The workqueue will be automatically destroyed on driver detach.  Typically
// this should be used in drivers already relying on devm interafaces.
//
// RETURNS:
// Pointer to the allocated workqueue on success, %NULL on failure.
//

//
// alloc_workqueue_lockdep_map - allocate a workqueue with user-defined lockdep_map
// @fmt: printf format for the name of the workqueue
// @flags: WQ_* flags
// @max_active: max in-flight work items, 0 for default
// @lockdep_map: user-defined lockdep_map
// @...: args for @fmt
//
// Same as alloc_workqueue but with the a user-define lockdep_map. Useful for
// workqueues created with the same purpose and to avoid leaking a lockdep_map
// on each workqueue creation.
//
// RETURNS:
// Pointer to the allocated workqueue on success, %NULL on failure.
//
// alloc_ordered_workqueue_lockdep_map - allocate an ordered workqueue with
// user-defined lockdep_map
//
// @fmt: printf format for the name of the workqueue
// @flags: WQ_* flags (only WQ_FREEZABLE and WQ_MEM_RECLAIM are meaningful)
// @lockdep_map: user-defined lockdep_map
// @args: args for @fmt
//
// Same as alloc_ordered_workqueue but with the a user-define lockdep_map.
// Useful for workqueues created with the same purpose and to avoid leaking a
// lockdep_map on each workqueue creation.
//
// RETURNS:
// Pointer to the allocated workqueue on success, %NULL on failure.
//

//
// alloc_ordered_workqueue - allocate an ordered workqueue
// @fmt: printf format for the name of the workqueue
// @flags: WQ_* flags (only WQ_FREEZABLE and WQ_MEM_RECLAIM are meaningful)
// @args: args for @fmt
//
// Allocate an ordered workqueue.  An ordered workqueue executes at
// most one work item at any given time in the queued order.  They are
// implemented as unbound workqueues with @max_active of one.
//
// RETURNS:
// Pointer to the allocated workqueue on success, %NULL on failure.
//

extern "C" {
    pub fn destroy_workqueue(wq: *mut workqueue_struct);
}

extern "C" {
    pub fn free_workqueue_attrs(attrs: *mut workqueue_attrs);
}
extern "C" {
    pub fn workqueue_unbound_housekeeping_update(hk: *const cpumask) -> c_int;
}
extern "C" {
    pub fn queue_rcu_work(wq: *mut workqueue_struct, rwork: *mut rcu_work) -> bool;
}
extern "C" {
    pub fn __flush_workqueue(wq: *mut workqueue_struct);
}
extern "C" {
    pub fn drain_workqueue(wq: *mut workqueue_struct);
}
extern "C" {
    pub fn schedule_on_each_cpu(func: work_func_t) -> c_int;
}
extern "C" {
    pub fn execute_in_process_context(fn: work_func_t, : *mut execute_work) -> c_int;
}
extern "C" {
    pub fn flush_work(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn cancel_work(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn cancel_work_sync(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn flush_delayed_work(dwork: *mut delayed_work) -> bool;
}
extern "C" {
    pub fn cancel_delayed_work(dwork: *mut delayed_work) -> bool;
}
extern "C" {
    pub fn cancel_delayed_work_sync(dwork: *mut delayed_work) -> bool;
}
extern "C" {
    pub fn disable_work(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn disable_work_sync(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn enable_work(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn disable_delayed_work(dwork: *mut delayed_work) -> bool;
}
extern "C" {
    pub fn disable_delayed_work_sync(dwork: *mut delayed_work) -> bool;
}
extern "C" {
    pub fn enable_delayed_work(dwork: *mut delayed_work) -> bool;
}
extern "C" {
    pub fn flush_rcu_work(rwork: *mut rcu_work) -> bool;
}
extern "C" {
    pub fn current_is_workqueue_rescuer() -> bool;
}
extern "C" {
    pub fn current_is_workqueue_mem_reclaim() -> bool;
}
extern "C" {
    pub fn workqueue_congested(cpu: c_int, wq: *mut workqueue_struct) -> bool;
}
extern "C" {
    pub fn work_busy(work: *mut work_struct) -> c_uint;
}
extern "C" {
    pub fn __printf(_arg: 1, fmt: *const 2) void set_worker_desc(char, ...) -> extern;
}
extern "C" {
    pub fn print_worker_info(log_lvl: *const c_char, task: *mut task_struct);
}
extern "C" {
    pub fn show_all_workqueues();
}
extern "C" {
    pub fn show_freezable_workqueues();
}
extern "C" {
    pub fn show_one_workqueue(wq: *mut workqueue_struct);
}
extern "C" {
    pub fn wq_worker_comm(buf: *mut c_char, size: usize, task: *mut task_struct);
}
//
// queue_work - queue work on a workqueue
// @wq: workqueue to use
// @work: work to queue
//
// Returns %false if @work was already on a queue, %true otherwise.
//
// We queue the work to the CPU on which it was submitted, but if the CPU dies
// it can be processed by another CPU.
//
// Memory-ordering properties:  If it returns %true, guarantees that all stores
// preceding the call to queue_work() in the program order will be visible from
// the CPU which will execute @work by the time such work executes, e.g.,
//
// { x is initially 0 }
//
// CPU0				CPU1
//
// WRITE_ONCE(x, 1);			[ @work is being executed ]
// r0 = queue_work(wq, work);		  r1 = READ_ONCE(x);
//
// Forbids: r0 == true && r1 == 0
//
extern "C" {
    pub fn queue_work_on(_arg: WORK_CPU_UNBOUND, _arg: wq, _arg: work) -> return;
}
//
// queue_delayed_work - queue work on a workqueue after delay
// @wq: workqueue to use
// @dwork: delayable work to queue
// @delay: number of jiffies to wait before queueing
//
// Equivalent to queue_delayed_work_on() but tries to use the local CPU.
//
extern "C" {
    pub fn queue_delayed_work_on(_arg: WORK_CPU_UNBOUND, _arg: wq, _arg: dwork, _arg: delay) -> return;
}
//
// mod_delayed_work - modify delay of or queue a delayed work
// @wq: workqueue to use
// @dwork: work to queue
// @delay: number of jiffies to wait before queueing
//
// mod_delayed_work_on() on local CPU.
//
extern "C" {
    pub fn mod_delayed_work_on(_arg: WORK_CPU_UNBOUND, _arg: wq, _arg: dwork, _arg: delay) -> return;
}
//
// schedule_work_on - put work task on a specific cpu
// @cpu: cpu to put the work task on
// @work: job to be done
//
// This puts a job on a specific cpu
//
extern "C" {
    pub fn queue_work_on(_arg: cpu, _arg: system_percpu_wq, _arg: work) -> return;
}
//
// schedule_work - put work task in per-CPU workqueue
// @work: job to be done
//
// Returns %false if @work was already on the system per-CPU workqueue and
// %true otherwise.
//
// This puts a job in the system per-CPU workqueue if it was not already
// queued and leaves it in the same position on the system per-CPU
// workqueue otherwise.
//
// Shares the same memory-ordering properties of queue_work(), cf. the
// DocBook header of queue_work().
//
extern "C" {
    pub fn queue_work(_arg: system_percpu_wq, _arg: work) -> return;
}
//
// enable_and_queue_work - Enable and queue a work item on a specific workqueue
// @wq: The target workqueue
// @work: The work item to be enabled and queued
//
// This function combines the operations of enable_work() and queue_work(),
// providing a convenient way to enable and queue a work item in a single call.
// It invokes enable_work() on @work and then queues it if the disable depth
// reached 0. Returns %true if the disable depth reached 0 and @work is queued,
// and %false otherwise.
//
// Note that @work is always queued when disable depth reaches zero. If the
// desired behavior is queueing only if certain events took place while @work is
// disabled, the user should implement the necessary state tracking and perform
// explicit conditional queueing after enable_work().
//
// Detect attempt to flush system-wide workqueues at compile time when possible.
// Warn attempt to flush system-wide workqueues at runtime.
//
// See https://lkml.kernel.org/r/49925af7-78a8-a3dd-bce6-cfc02e1a9236@I-love.SAKURA.ne.jp
// for reasons and steps for converting system-wide workqueues into local workqueues.
//
// Please stop using this function, for this function will be removed in near future.

//
// schedule_delayed_work_on - queue work in per-CPU workqueue on CPU after delay
// @cpu: cpu to use
// @dwork: job to be done
// @delay: number of jiffies to wait
//
// After waiting for a given time this puts a job in the system per-CPU
// workqueue on the specified CPU.
//
extern "C" {
    pub fn queue_delayed_work_on(_arg: cpu, _arg: system_percpu_wq, _arg: dwork, _arg: delay) -> return;
}
//
// schedule_delayed_work - put work task in per-CPU workqueue after delay
// @dwork: job to be done
// @delay: number of jiffies to wait or 0 for immediate execution
//
// After waiting for a given time this puts a job in the system per-CPU
// workqueue.
//
extern "C" {
    pub fn queue_delayed_work(_arg: system_percpu_wq, _arg: dwork, _arg: delay) -> return;
}

extern "C" {
    pub fn fn(_arg: arg) -> return;
}
extern "C" {
    pub fn fn(_arg: arg) -> return;
}

//
// A new key is defined for each caller to make sure the work
// associated with the function doesn't share its locking class.
//

extern "C" {
    pub fn freeze_workqueues_begin();
}
extern "C" {
    pub fn freeze_workqueues_busy() -> bool;
}
extern "C" {
    pub fn thaw_workqueues();
}

extern "C" {
    pub fn workqueue_sysfs_register(wq: *mut workqueue_struct) -> c_int;
}

extern "C" {
    pub fn wq_watchdog_touch(cpu: c_int);
}

extern "C" {
    pub fn workqueue_prepare_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn workqueue_online_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn workqueue_offline_cpu(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn workqueue_init_early() -> void __init;
}
extern "C" {
    pub fn workqueue_init() -> void __init;
}
extern "C" {
    pub fn workqueue_init_topology() -> void __init;
}
