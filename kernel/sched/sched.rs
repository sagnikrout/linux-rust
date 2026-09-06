//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/sched.h
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
// Scheduler internal types and methods:
//

// task_struct::on_rq states:
pub const TASK_ON_RQ_QUEUED: c_int = 1;
pub const TASK_ON_RQ_MIGRATING: c_int = 2;
extern "C" {
    pub fn calc_global_load_tick(this_rq: *mut rq);
}
extern "C" {
    pub fn calc_load_fold_active(this_rq: *mut rq, adjust: c_long) -> c_long;
}
extern "C" {
    pub fn call_trace_sched_update_nr_running(rq: *mut rq, count: c_int);
}
//
// Asymmetric CPU capacity bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asym_cap_data {
    pub link: list_head,
    pub rcu: rcu_head,
    pub capacity: c_ulong,
    pub cpus: [c_ulong; ],
}

//
// Helpers for converting nanosecond timing to jiffy resolution
//

//
// Increase resolution of nice-level calculations for 64-bit architectures.
// The extra resolution improves shares distribution and load balancing of
// low-weight task groups (eg. nice +19 on an autogroup), deeper task-group
// hierarchies, especially on larger systems. This is not a user-visible change
// and does not change the user-interface for setting shares/weights.
//
// We increase resolution only if we have enough bits to allow this increased
// resolution (i.e. 64-bit). The costs for increasing resolution when 32-bit
// are pretty high and the returns do not justify the increased costs.
//
// Really only required when CONFIG_FAIR_GROUP_SCHED=y is also set, but to
// increase coverage and consistency always enable it on 64-bit platforms.
//

//
// Task weight (visible to users) and its load (invisible to users) have
// independent resolution, but they should be well calibrated. We use
// scale_load() and scale_load_down(w) to convert between them. The
// following must be true:
//
// scale_load(sched_prio_to_weight[NICE_TO_PRIO(0)-MAX_RT_PRIO]) == NICE_0_LOAD
//

//
// Single value that decides SCHED_DEADLINE internal math precision.
// 10 -> just above 1us
// 9  -> just above 0.5us
//
pub const DL_SCALE: c_int = 10;
//
// Single value that denotes runtime == period, ie unlimited time.
//

extern "C" {
    pub fn idle_policy(_arg: p->policy) -> return;
}
extern "C" {
    pub fn rt_policy(_arg: p->policy) -> return;
}
extern "C" {
    pub fn dl_policy(_arg: p->policy) -> return;
}

// avg += diff / 8;
//
// Shifting a value by an exponent greater *or equal* to the size of said value
// is UB; cap at size-1.
//

//
// cgroup weight knobs should use the common MIN, DFL and MAX values which are
// 1, 100 and 10000 respectively. While it loses a bit of range on both ends, it
// maps pretty well onto the shares value used by scheduler and the round-trip
// conversions preserve the original value over the entire range.
//
extern "C" {
    pub fn DIV_ROUND_CLOSEST_ULL(1024: *mut *mut cgrp_weight, _arg: CGROUP_WEIGHT_DFL) -> return;
}
//
// !! For sched_setattr_nocheck() (kernel) only !!
//
// This is actually gross. :(
//
// It is used to make schedutil kworker(s) higher priority than SCHED_DEADLINE
// tasks, but still be able to sleep. We need this on platforms that cannot
// atomically change clock frequency. Remove once fast switching will be
// available on such platforms.
//
// SUGOV stands for SchedUtil GOVernor.
//
pub const SCHED_FLAG_SUGOV: c_uint = 0x10000000;

extern "C" {
    pub fn unlikely(SCHED_FLAG_SUGOV: dl_se->flags &) -> return;
}

//
// Tells if entity @a should preempt entity @b.
//
// This is the priority-queue data structure of the RT scheduling class:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_prio_array {
    pub /: *mut *mut DECLARE_BITMAP(bitmap, MAX_RT_PRIO+1); / include 1 bit for delimiter,
    pub queue: [list_head; MAX_RT_PRIO],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_bandwidth {
// nests inside the rq lock:
    pub rt_runtime_lock: raw_spinlock_t,
    pub rt_period: ktime_t,
    pub rt_runtime: u64,
    pub rt_period_timer: hrtimer,
    pub rt_period_active: c_uint,
}

//
// To keep the bandwidth of -deadline tasks under control
// we need some place where:
// - store the maximum -deadline bandwidth of each cpu;
// - cache the fraction of bandwidth that is currently allocated in
// each root domain;
//
// This is all done in the data structure below. It is similar to the
// one used for RT-throttling (rt_bandwidth), with the main difference
// that, since here we are only interested in admission control, we
// do not decrease any runtime while the group "executes", neither we
// need a timer to replenish it.
//
// With respect to SMP, bandwidth is given on a per root domain basis,
// meaning that:
// - bw (< 100%) is the deadline bandwidth of each CPU;
// - total_bw is the currently allocated bandwidth in each root domain;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dl_bw {
    pub lock: raw_spinlock_t,
    pub bw: u64,
    pub total_bw: u64,
}

extern "C" {
    pub fn init_dl_bw(dl_b: *mut dl_bw);
}
extern "C" {
    pub fn sched_dl_global_validate() -> c_int;
}
extern "C" {
    pub fn sched_dl_do_global();
}
extern "C" {
    pub fn sched_dl_overflow(p: *mut task_struct, policy: c_int, attr: *const sched_attr) -> c_int;
}
extern "C" {
    pub fn __setparam_dl(p: *mut task_struct, attr: *const sched_attr);
}
extern "C" {
    pub fn __getparam_dl(p: *mut task_struct, attr: *mut sched_attr, flags: c_uint);
}
extern "C" {
    pub fn __checkparam_dl(attr: *const sched_attr) -> bool;
}
extern "C" {
    pub fn dl_param_changed(p: *mut task_struct, attr: *const sched_attr) -> bool;
}
extern "C" {
    pub fn dl_cpuset_cpumask_can_shrink(cur: *const cpumask, trial: *const cpumask) -> c_int;
}
extern "C" {
    pub fn dl_bw_deactivate(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn dl_scaled_delta_exec(rq: *mut rq, dl_se: *mut sched_dl_entity, delta_exec: i64) -> i64;
}
//
// SCHED_DEADLINE supports servers (nested scheduling) with the following
// interface:
//
// dl_se::rq -- runqueue we belong to.
//
// dl_se::server_pick() -- nested pick_next_task(); we yield the period if this
// returns NULL.
//
// dl_server_update() -- called from update_curr_common(), propagates runtime
// to the server.
//
// dl_server_start() -- start the server when it has tasks; it will stop
// automatically when there are no more tasks, per
// dl_se::server_pick() returning NULL.
//
// dl_server_stop() -- (force) stop the server; use when updating
// parameters.
//
// dl_server_init() -- initializes the server.
//
// When started the dl_server will (per dl_defer) schedule a timer for its
// zero-laxity point -- that is, unlike regular EDF tasks which run ASAP, a
// server will run at the very end of its period.
//
// This is done such that any runtime from the target class can be accounted
// against the server -- through dl_server_update() above -- such that when it
// becomes time to run, it might already be out of runtime and get deferred
// until the next period. In this case dl_server_timer() will alternate
// between defer and replenish but never actually enqueue the server.
//
// Only when the target class does not manage to exhaust the server's runtime
// (there's actualy starvation in the given period), will the dl_server get on
// the runqueue. Once queued it will pick tasks from the target class and run
// them until either its runtime is exhaused, at which point its back to
// dl_server_timer, or until there are no more tasks to run, at which point
// the dl_server stops itself.
//
// By stopping at this point the dl_server retains bandwidth, which, if a new
// task wakes up imminently (starting the server again), can be used --
// subject to CBS wakeup rules -- without having to wait for the next period.
//
// Additionally, because of the dl_defer behaviour the start/stop behaviour is
// naturally thottled to once per period, avoiding high context switch
// workloads from spamming the hrtimer program/cancel paths.
//
extern "C" {
    pub fn dl_server_update_idle(dl_se: *mut sched_dl_entity, delta_exec: i64);
}
extern "C" {
    pub fn dl_server_update(dl_se: *mut sched_dl_entity, delta_exec: i64);
}
extern "C" {
    pub fn dl_server_start(dl_se: *mut sched_dl_entity);
}
extern "C" {
    pub fn dl_server_stop(dl_se: *mut sched_dl_entity);
}
extern "C" {
    pub fn sched_init_dl_servers();
}
extern "C" {
    pub fn fair_server_init(rq: *mut rq);
}
extern "C" {
    pub fn ext_server_init(rq: *mut rq);
}
extern "C" {
    pub fn __dl_server_attach_root(dl_se: *mut sched_dl_entity, rq: *mut rq);
}
extern "C" {
    pub fn dl_server_attach_bw(dl_se: *mut sched_dl_entity) -> c_int;
}
extern "C" {
    pub fn dl_server_detach_bw(dl_se: *mut sched_dl_entity);
}

//
// default period for group bandwidth.
// default: 0.1s, units: microseconds
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfs_bandwidth {

    pub lock: raw_spinlock_t,
    pub period: ktime_t,
    pub quota: u64,
    pub runtime: u64,
    pub burst: u64,
    pub runtime_snap: u64,
    pub hierarchical_quota: i64,
    pub idle: u8,
    pub period_active: u8,
    pub slack_started: u8,
    pub period_timer: hrtimer,
    pub slack_timer: hrtimer,
    pub throttled_cfs_rq: list_head,
// Statistics:
    pub nr_periods: c_int,
    pub nr_throttled: c_int,
    pub nr_burst: c_int,
    pub throttled_time: u64,
    pub burst_time: u64,

}

// Task group related information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_group {
    pub css: cgroup_subsys_state,

// A positive value indicates that this is a SCHED_IDLE group.
    pub idle: c_int,

// runqueue "owned" by this group on each CPU
    pub cfs_rq: *mut cfs_rq __percpu,
    pub shares: c_ulong,
//
// load_avg can be heavily contended at clock tick time, so put
// it in its own cache-line separated from the fields above which
// will also be accessed at each tick.
//
    pub ____cacheline_aligned: atomic_long_t load_avg,
    pub runnable_avg: atomic_long_t,

    pub rt_se: *mut sched_rt_entity,
    pub rt_rq: *mut rt_rq,
    pub rt_bandwidth: rt_bandwidth,

    pub scx: scx_task_group,
    pub rcu: rcu_head,
    pub list: list_head,
    pub parent: *mut task_group,
    pub siblings: list_head,
    pub children: list_head,

    pub autogroup: *mut autogroup,

    pub cfs_bandwidth: cfs_bandwidth,

// The two decimal precision [%] value requested from user-space
    pub uclamp_pct: [c_uint; UCLAMP_CNT],
// Clamp values requested for a task group
    pub uclamp_req: [uclamp_se; UCLAMP_CNT],
// Effective clamp values used for a task group
    pub uclamp: [uclamp_se; UCLAMP_CNT],
}

extern "C" {
    pub fn int(: *mut *mut tg_visitor)(struct task_group, : *mut c_void) -> typedef;
}
//
// Iterate the full tree, calling @down when first entering a node and @up when
// leaving it for the final time.
//
// Caller must hold rcu_lock or sufficient equivalent.
//
extern "C" {
    pub fn walk_tg_tree_from(_arg: &root_task_group, _arg: down, _arg: up, _arg: data) -> return;
}
extern "C" {
    pub fn tg_nop(tg: *mut task_group, data: *mut c_void) -> c_int;
}

extern "C" {
    pub fn free_fair_sched_group(tg: *mut task_group);
}
extern "C" {
    pub fn alloc_fair_sched_group(tg: *mut task_group, parent: *mut task_group) -> c_int;
}
extern "C" {
    pub fn online_fair_sched_group(tg: *mut task_group);
}
extern "C" {
    pub fn unregister_fair_sched_group(tg: *mut task_group);
}
extern "C" {
    pub fn __sched_cgroup_mode_update(mode: c_int);
}

extern "C" {
    pub fn init_cfs_bandwidth(cfs_b: *mut cfs_bandwidth, parent: *mut cfs_bandwidth);
}
extern "C" {
    pub fn __refill_cfs_bandwidth_runtime(cfs_b: *mut cfs_bandwidth);
}
extern "C" {
    pub fn start_cfs_bandwidth(cfs_b: *mut cfs_bandwidth);
}
extern "C" {
    pub fn unthrottle_cfs_rq(cfs_rq: *mut cfs_rq);
}
extern "C" {
    pub fn cfs_task_bw_constrained(p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn sched_group_set_rt_runtime(tg: *mut task_group, rt_runtime_us: c_long) -> c_int;
}
extern "C" {
    pub fn sched_group_set_rt_period(tg: *mut task_group, rt_period_us: u64) -> c_int;
}
extern "C" {
    pub fn sched_group_rt_runtime(tg: *mut task_group) -> c_long;
}
extern "C" {
    pub fn sched_group_rt_period(tg: *mut task_group) -> c_long;
}
extern "C" {
    pub fn sched_rt_can_attach(tg: *mut task_group, tsk: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn sched_destroy_group(tg: *mut task_group);
}
extern "C" {
    pub fn sched_release_group(tg: *mut task_group);
}
extern "C" {
    pub fn sched_move_task(tsk: *mut task_struct, for_autogroup: bool);
}

extern "C" {
    pub fn sched_group_set_shares(tg: *mut task_group, shares: c_ulong) -> c_int;
}
extern "C" {
    pub fn sched_group_set_idle(tg: *mut task_group, idle: c_long) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfs_bandwidth {
    pub }: *mut *mut static inline bool cfs_task_bw_constrained(struct task_struct p) { return false;,

//
// A weight of 0 or 1 can cause arithmetics problems.
// A weight of a cfs_rq is the sum of weights of which entities
// are queued on this cfs_rq, so a weight of a entity should not be
// too large, so as the shares value of a task group.
// (The default weight is 1024 - so there's no practical
// limitation from this.)
//

    pub tg): *mut extern void unregister_rt_sched_group(struct task_group,
    pub tg): *mut extern void free_rt_sched_group(struct task_group,
    pub parent): *mut *mut extern int alloc_rt_sched_group(struct task_group tg, struct task_group,
//
// u64_u32_load/u64_u32_store
//
// Use a copy of a u64 value to protect against data race. This is only
// applicable for 32-bits architectures.
//

    pub \: u64 __val, __val_copy;,
    pub \: __val_copy = copy;,
// \
// paired with u64_u32_store_copy(), ordering access	\
// to var and copy.					\
// \
    pub \: smp_rmb();,
    pub \: __val = var;,
    pub \: } while (__val != __val_copy);,
    pub \: __val;,

    pub \: typeof(val) __val = (val);,
    pub \: var = __val;,
// \
// paired with u64_u32_load_copy(), ordering access to var and	\
// copy.							\
// \
    pub \: smp_wmb();,
    pub \: copy = __val;,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct balance_callback {
    pub next: *mut balance_callback,
    pub func: Option<unsafe extern "C" fn()>,
}

// Fair scheduling SCHED_{NORMAL,BATCH,IDLE} related fields in a runqueue:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfs_rq {
    pub load: load_weight,
    pub nr_queued: c_uint,
    pub /: *mut *mut unsigned int h_nr_queued; / SCHED_{NORMAL,BATCH,IDLE},
    pub /: *mut *mut unsigned int h_nr_runnable; / SCHED_{NORMAL,BATCH,IDLE},
    pub /: *mut *mut unsigned int h_nr_idle; / SCHED_IDLE,
    pub sum_w_vruntime: i64,
    pub sum_weight: u64,
    pub zero_vruntime: u64,
    pub sum_shift: c_uint,

    pub forceidle_seq: c_uint,
    pub zero_vruntime_fi: u64,

    pub tasks_timeline: rb_root_cached,
//
// 'curr' points to the currently running entity on this cfs_rq.
// It is set to NULL otherwise (i.e when none are currently running).
//
    pub curr: *mut sched_entity,
    pub next: *mut sched_entity,
//
// CFS load tracking
//
    pub h_curr: *mut sched_entity,
    pub avg: sched_avg,

    pub last_update_time_copy: u64,

    pub ____cacheline_aligned: raw_spinlock_t lock,
    pub nr: c_int,
    pub load_avg: c_ulong,
    pub util_avg: c_ulong,
    pub runnable_avg: c_ulong,
    pub removed: },

    pub last_update_tg_load_avg: u64,
    pub tg_load_avg_contrib: c_ulong,
    pub tg_runnable_avg_contrib: c_ulong,
    pub propagate: c_long,
    pub prop_runnable_sum: c_long,
//
// h_load = weight * f(tg)
//
// Where f(tg) is the recursive weight fraction assigned to
// this group.
//
    pub h_load: c_ulong,
    pub last_h_load_update: u64,
    pub h_load_next: *mut sched_entity,
    pub /: *mut *mut *mut rq rq; / CPU runqueue to which this cfs_rq is attached,
//
// leaf cfs_rqs are those that hold tasks (lowest schedulable entity in
// a hierarchy). Non-leaf lrqs hold other higher schedulable entities
// (like users, containers etc.)
//
// leaf_cfs_rq_list ties together list of leaf cfs_rq's in a CPU.
// This list is used during load balance.
//
    pub on_list: c_int,
    pub leaf_cfs_rq_list: list_head,
    pub /: *mut *mut *mut task_group tg; / Group that "owns" this runqueue,
// Locally cached copy of our task_group's idle value
    pub idle: c_int,

    pub runtime_enabled: c_int,
    pub runtime_remaining: i64,
    pub throttled_pelt_idle: u64,

    pub throttled_pelt_idle_copy: u64,

    pub throttled_clock: u64,
    pub throttled_clock_pelt: u64,
    pub throttled_clock_pelt_time: u64,
    pub throttled_clock_self: u64,
    pub throttled_clock_self_time: u64,
    pub throttled:1: bool,
    pub pelt_clock_throttled:1: bool,
    pub throttle_count: c_int,
    pub throttled_list: list_head,
    pub throttled_csd_list: list_head,
    pub throttled_limbo_list: list_head,

}

// scx_rq->flags, protected by the rq lock
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_rq_flags {
//
// A hotplugged CPU starts scheduling before rq_online_scx(). Track
// ops.cpu_on/offline() state so that ops.enqueue/dispatch() are called
// only while the BPF scheduler considers the CPU to be online.
//
    SCX_RQ_ONLINE		= 1 << 0,
    SCX_RQ_CAN_STOP_TICK	= 1 << 1,
    SCX_RQ_CLK_VALID	= 1 << 5, /* RQ clock is fresh and valid */
    SCX_RQ_BAL_CB_PENDING	= 1 << 6, /* must queue a cb after dispatching */
    SCX_RQ_SUB_IDLE_RENOTIFY	= 1 << 7, /* sub-scheds are owed update_idle() */
    SCX_RQ_ROOT_IDLE_RENOTIFY	= 1 << 8, /* the root is owed update_idle() */

    SCX_RQ_IN_WAKEUP	= 1 << 16,
    SCX_RQ_IN_DISPATCH	= 1 << 17,
}

// per-rq rescue execution state, see scx_rescue_timerfn()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_rq_rescue {
    pub /: *mut *mut scx_dispatch_q dsq; / stranded tasks awaiting rescue,
    pub /: *mut *mut s64 budget; / execution token bucket, ns,
    pub /: *mut *mut u64 clock; / last budget accrual timestamp,
    pub /: *mut *mut *mut task_curr; / task being rescued, one at a time,
    pub /: *mut *mut s64 slice; / curr's admitted slice,
    pub /: *mut *mut u64 exec_snap; / sum_exec_runtime at admission,
    pub /: *mut *mut timer_list timer; / paces admission and escalation,
    pub /: *mut *mut u64 kill_at; / last ejection, init before any,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_rq {
    pub local_dsq: scx_dispatch_q,

    pub /: *mut *mut scx_dispatch_q reject_dsq; / staging for cap-rejected tasks,
    pub rescue: scx_rq_rescue,

    pub /: *mut *mut list_head runnable_list; / runnable tasks on this rq,
    pub /: *mut *mut list_head ddsp_deferred_locals; / deferred ddsps from enq,
    pub ops_qseq: c_ulong,
// both stashed across the activate_task() in move_remote_task_to_local_dsq()
    pub remote_activate_enq_flags: u64,
    pub remote_activate_sch: *mut scx_sched,
    pub nr_running: u32,
    pub /: *mut *mut u32 cpuperf_target; / [0, SCHED_CAPACITY_SCALE],
    pub in_select_cpu: bool,
    pub cpu_released: bool,
    pub flags: u32,
    pub /: *mut *mut u32 nr_immed; / ENQ_IMMED tasks on local_dsq,

    pub /: *mut *mut u32 lock_drop_seq; / nr dispatch lock releases,

    pub /: *mut *mut u64 clock; / current per-rq clock -- see scx_bpf_now(),

    pub /: *mut *mut llist_head ecaps_to_sync; / pending ecaps syncs,
    pub sub_dispatch_prev: *mut task_struct,

    pub cpus_to_sync: cpumask_var_t,
    pub kick_sync_pending: bool,
    pub kick_sync: c_ulong,
    pub /: *mut *mut list_head sched_pcpus_to_kick; / see kick_cpus_irq_workfn(),
    pub deferred_reenq_lock: raw_spinlock_t,
    pub /: *mut *mut list_head deferred_reenq_locals; / scheds requesting reenq of local DSQ,
    pub /: *mut *mut list_head deferred_reenq_users; / user DSQs requesting reenq,
    pub deferred_bal_cb: balance_callback,
    pub kick_sync_bal_cb: balance_callback,
    pub deferred_irq_work: irq_work,
    pub kick_cpus_irq_work: irq_work,
}

// RT IPI pull logic requires IRQ_WORK

// Real-Time classes' related field in a runqueue:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_rq {
    pub active: rt_prio_array,
    pub rt_nr_running: c_uint,
    pub rr_nr_running: c_uint,
    pub /: *mut *mut int curr; / highest queued rt task prio,
    pub /: *mut *mut int next; / next highest,
    pub highest_prio: },
    pub overloaded: bool,
    pub pushable_tasks: plist_head,
    pub rt_queued: c_int,

    pub rt_throttled: c_int,
    pub /: *mut *mut u64 rt_time; / consumed RT time, goes up in update_curr_rt,
    pub /: *mut *mut u64 rt_runtime; / allotted RT time, "slice" from rt_bandwidth, RT sharing/balancing,
// Nests inside the rq lock:
    pub rt_runtime_lock: raw_spinlock_t,
    pub rt_nr_boosted: c_uint,
    pub /: *mut *mut *mut rq rq; / this is always top-level rq, cache?,

    pub /: *mut *mut *mut task_group tg; / this tg has "this" rt_rq on given CPU for runnable entities,

}

// Deadline class' related fields in a runqueue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dl_rq {
// runqueue is an rbtree, ordered by deadline
    pub root: rb_root_cached,
    pub dl_nr_running: c_uint,
//
// Deadline values of the currently executing and the
// earliest ready task on this rq. Caching these facilitates
// the decision whether or not a ready but not running task
// should migrate somewhere else.
//
    pub curr: u64,
    pub next: u64,
    pub earliest_dl: },
    pub overloaded: bool,
    pub curr: *mut sched_dl_entity,
//
// Tasks on this rq that can be pushed away. They are kept in
// an rb-tree, ordered by tasks' deadlines, with caching
// of the leftmost (earliest deadline) element.
//
    pub pushable_dl_tasks_root: rb_root_cached,
//
// "Active utilization" for this runqueue: increased when a
// task wakes up (becomes TASK_RUNNING) and decreased when a
// task blocks
//
    pub running_bw: u64,
//
// Utilization of the tasks "assigned" to this runqueue (including
// the tasks that are in runqueue and the tasks that executed on this
// CPU and blocked). Increased when a task moves to this runqueue, and
// decreased when the task moves away (migrates, changes scheduling
// policy, or terminates).
// This is needed to compute the "inactive utilization" for the
// runqueue (inactive utilization = this_bw - running_bw).
//
    pub this_bw: u64,
    pub extra_bw: u64,
//
// Maximum available bandwidth for reclaiming by SCHED_FLAG_RECLAIM
// tasks of this rq. Used in calculation of reclaimable bandwidth(GRUB).
//
    pub max_bw: u64,
//
// Inverse of the fraction of CPU utilization that can be reclaimed
// by the GRUB algorithm.
//
    pub bw_ratio: u64,
}

// Check whether a task group is root tg

// An entity is a task if it doesn't "own" a runqueue

pub const entity_is_task(se): c_int = 1;

//
// XXX we want to get rid of these helpers and use the full load resolution.
//
extern "C" {
    pub fn scale_load_down(_arg: se->load.weight) -> return;
}
extern "C" {
    pub fn arch_asym_cpu_priority(arch_asym_cpu_priority(b: a) >) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_domain {
    pub em_pd: *mut em_perf_domain,
    pub next: *mut perf_domain,
    pub rcu: rcu_head,
}

//
// We add the notion of a root-domain which will be used to define per-domain
// variables. Each exclusive cpuset essentially defines an island domain by
// fully partitioning the member CPUs from any other cpuset. Whenever a new
// exclusive cpuset is created, we also create and attach a new root-domain
// object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct root_domain {
    pub refcount: core::sync::atomic::AtomicI32,
    pub rto_count: core::sync::atomic::AtomicI32,
    pub rcu: rcu_head,
    pub span: cpumask_var_t,
    pub online: cpumask_var_t,
//
// Indicate pullable load on at least one CPU, e.g:
// - More than one runnable task
// - Running task is misfit
//
    pub overloaded: bool,
// Indicate one or more CPUs over-utilized (tipping point)
    pub overutilized: bool,
//
// The bit corresponding to a CPU gets set here if such CPU has more
// than one runnable -deadline task (as it is below for RT tasks).
//
    pub dlo_mask: cpumask_var_t,
    pub dlo_count: core::sync::atomic::AtomicI32,
    pub dl_bw: dl_bw,
    pub cpudl: cpudl,
//
// Indicate whether a root_domain's dl_bw has been checked or
// updated. It's monotonously increasing value.
//
// Also, some corner cases, like 'wrap around' is dangerous, but given
// that u64 is 'big enough'. So that shouldn't be a concern.
//
    pub visit_cookie: u64,

//
// For IPI pull requests, loop across the rto_mask.
//
    pub rto_push_work: irq_work,
    pub rto_lock: raw_spinlock_t,
// These are only updated and read within rto_lock
    pub rto_loop: c_int,
    pub rto_cpu: c_int,
// These atomics are updated outside of a lock
    pub rto_loop_next: core::sync::atomic::AtomicI32,
    pub rto_loop_start: core::sync::atomic::AtomicI32,

//
// The "RT overload" flag: it gets set if a CPU has more than
// one runnable RT task.
//
    pub rto_mask: cpumask_var_t,
    pub cpupri: cpupri,
//
// NULL-terminated list of performance domains intersecting with the
// CPUs of the rd. Protected by RCU.
//
    pub pd: *mut perf_domain __rcu,
}

extern "C" {
    pub fn init_defrootdomain();
}
extern "C" {
    pub fn sched_init_domains(cpu_map: *const cpumask) -> c_int;
}
extern "C" {
    pub fn rq_attach_root(rq: *mut rq, rd: *mut root_domain);
}
extern "C" {
    pub fn sched_get_rd(rd: *mut root_domain);
}
extern "C" {
    pub fn sched_put_rd(rd: *mut root_domain);
}
extern "C" {
    pub fn READ_ONCE(_arg: rd->overloaded) -> return;
}

extern "C" {
    pub fn rto_push_irq_work_func(work: *mut irq_work);
}

//
// struct uclamp_bucket - Utilization clamp bucket
// @value: utilization clamp value for tasks on this clamp bucket
// @tasks: number of RUNNABLE tasks on this clamp bucket
//
// Keep track of how many tasks are RUNNABLE for a given utilization
// clamp value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclamp_bucket {
    pub bits_per(SCHED_CAPACITY_SCALE): unsigned long value :,
    pub bits_per(SCHED_CAPACITY_SCALE): unsigned long tasks : BITS_PER_LONG -,
}

//
// struct uclamp_rq - rq's utilization clamp
// @value: currently active clamp values for a rq
// @bucket: utilization clamp buckets affecting a rq
//
// Keep track of RUNNABLE tasks on a rq to aggregate their clamp values.
// A clamp value is affecting a rq when there is at least one task RUNNABLE
// (or actually running) with that value.
//
// There are up to UCLAMP_CNT possible different clamp values, currently there
// are only two: minimum utilization and maximum utilization.
//
// All utilization clamping values are MAX aggregated, since:
// - for util_min: we want to run the CPU at least at the max of the minimum
// utilization required by its currently RUNNABLE tasks.
// - for util_max: we want to allow the CPU to run up to the max of the
// maximum utilization allowed by its currently RUNNABLE tasks.
//
// Since on each system we expect only a limited number of different
// utilization clamp values (UCLAMP_BUCKETS), use a simple array to track
// the metrics required to compute all the per-rq utilization clamp values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclamp_rq {
    pub value: c_uint,
    pub bucket: [uclamp_bucket; UCLAMP_BUCKETS],
}

//
// This is the main, per-CPU runqueue data structure.
//
// Locking rule: those places that want to lock multiple runqueues
// (such as the load balancing or the thread migration code), lock
// acquire operations must be ordered by ascending &runqueue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq {
//
// The following members are loaded together, without holding the
// rq->lock, in an extremely hot loop in update_sg_lb_stats()
// (called from pick_next_task()). To reduce cache pollution from
// this operation, they are placed together on this dedicated cache
// line. Even though some of them are frequently modified, they are
// loaded much more frequently than they are stored.
//
    pub nr_running: c_uint,

    pub nr_numa_running: c_uint,
    pub nr_preferred_running: c_uint,

    pub ttwu_pending: c_uint,
    pub cpu_capacity: c_ulong,

    pub /: *mut *mut *mut task___rcu donor; / Scheduling context,
    pub /: *mut *mut *mut task___rcu curr; / Execution context,

    pub /: *mut *mut *mut task___rcu donor; / Scheduler context,
    pub /: *mut *mut *mut task___rcu curr; / Execution context,
}

// padding left here deliberately
//
// The next cacheline holds the (hot) runqueue lock, as well as
// some other less performance-critical fields.
//
// runqueue lock:

// Utilization clamp values based on CPU's RUNNABLE tasks
pub const UCLAMP_FLAG_IDLE: c_uint = 0x01;

// list of leaf cfs_rq on this CPU:

//
// This is part of a global counter where only the total sum
// over all CPUs matters. A task can increase this counter on
// one CPU and if it got migrated afterwards it may decrease
// it on another CPU. Always updated under the runqueue lock:
//
// The following fields of clock data are frequently referenced
// and updated together, and should go on their own cache line.
//

// For active balancing
// CPU of this runqueue:

// This is used to determine avg_idle's max value

// calc_load related fields

// latency stats
// sys_sched_yield() stats
// schedule() stats
// try_to_wake_up() stats

// Must be inspected within a RCU lock section

// per rq
// shared state -- careful with sched_core_cpu_deactivate()

// Scratch cpumask to be temporarily used under rq_lock

// CPU runqueue to which this cfs_rq is attached

extern "C" {
    pub fn container_of(_arg: cfs_rq, rq: struct, _arg: cfs) -> return;
}

pub const MDF_PUSH: c_uint = 0x01;
extern "C" {
    pub fn prandom_u32_state(_arg: this_cpu_ptr(&sched_rnd_state)) -> return;
}
extern "C" {
    pub fn this_cpu_ptr(_arg: &runqueues) -> return;
}

//
// available_idle_cpu - is a given CPU idle for enqueuing work.
// @cpu: the CPU in question.
//
// Return: 1 if the CPU is currently idle. 0 otherwise.
//

// Do nothing

//
// Be careful with this function; not for general use. The return value isn't
// stable unless you actually hold a relevant rq->__lock.
//
extern "C" {
    pub fn task_vruntime_update(rq: *mut rq, p: *mut task_struct, in_fi: bool);
}
//
// Helpers to check if the CPU's core cookie matches with the task's cookie
// when core scheduling is enabled.
// A special case is that the task's cookie always matches with CPU's core
// cookie if the CPU is in an idle core.
//
// Ignore cookie match if core scheduler is not enabled on the CPU.
//
// A CPU in an idle core is always the best choice for tasks with
// cookies.
//
// Ignore cookie match if core scheduler is not enabled on the CPU.
extern "C" {
    pub fn sched_core_enqueue(rq: *mut rq, p: *mut task_struct);
}
extern "C" {
    pub fn sched_core_dequeue(rq: *mut rq, p: *mut task_struct, flags: c_int);
}
extern "C" {
    pub fn sched_core_get();
}
extern "C" {
    pub fn sched_core_put();
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &rt_group_sched) -> return;
}

extern "C" {
    pub fn static_branch_likely(_arg: &rt_group_sched) -> return;
}

extern "C" {
    pub fn __update_idle_core(rq: *mut rq);
}

extern "C" {
    pub fn container_of(_arg: se, task_struct: struct, _arg: se) -> return;
}
// runqueue on which this entity is (to be) queued
// runqueue "owned" by this group

// runqueue "owned" by this group

extern "C" {
    pub fn update_rq_avg_idle(rq: *mut rq);
}
extern "C" {
    pub fn update_rq_clock(rq: *mut rq);
}
//
// rq::clock_update_flags bits
//
// %RQCF_REQ_SKIP - will request skipping of clock update on the next
// call to __schedule(). This is an optimisation to avoid
// neighbouring rq clock updates.
//
// %RQCF_ACT_SKIP - is set from inside of __schedule() when skipping is
// in effect and calls to update_rq_clock() are being ignored.
//
// %RQCF_UPDATED - is a debug flag that indicates whether a call has been
// made to update_rq_clock() since the last time rq::lock was pinned.
//
// If inside of __schedule(), clock_update_flags will have been
// shifted left (a left shift is a cheap operation for the fast path
// to promote %RQCF_REQ_SKIP to %RQCF_ACT_SKIP), so you must use,
//
// if (rq-clock_update_flags >= RQCF_UPDATED)
//
// to check if %RQCF_UPDATED is set. It'll never be shifted more than
// one position though, because the next rq_unpin_lock() will shift it
// back.
//
pub const RQCF_REQ_SKIP: c_uint = 0x01;
pub const RQCF_ACT_SKIP: c_uint = 0x02;
pub const RQCF_UPDATED: c_uint = 0x04;
//
// The only reason for not seeing a clock update since the
// last rq_pin_lock() is if we're currently skipping updates.
//
// See rt task throttling, which is the only time a skip
// request is canceled.
//
// During cpu offlining and rq wide unthrottling, we can trigger
// an update_rq_clock() for several cfs and rt runqueues (Typically
// when using list_for_each_entry_*)
// rq_clock_start_loop_update() can be called after updating the clock
// once and before iterating over the list to prevent multiple update.
// After the iterative traversal, we need to call rq_clock_stop_loop_update()
// to clear RQCF_ACT_SKIP of rq->clock_update_flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_flags {
    pub flags: c_ulong,
    pub cookie: pin_cookie,
//
// A copy of (rq::clock_update_flags & RQCF_UPDATED) for the
// current pin context is stashed here in case it needs to be
// restored in rq_repin_lock().
//
    pub clock_update_flags: c_uint,
}

//
// Lockdep annotation that avoids accidental unlocks; it's like a
// sticky/continuous lockdep_assert_held().
//
// This avoids code that has access to 'struct rq *rq' (basically everything in
// the scheduler) from accidentally unlocking the rq if they do not also have a
// copy of the (on-stack) 'struct rq_flags rf'.
//
// Also see Documentation/locking/lockdep-design.rst.
//
// Restore the value we stashed in @rf for this pin context.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum numa_topology_type {
    NUMA_DIRECT,
    NUMA_GLUELESS_MESH,
    NUMA_BACKPLANE,
}

extern "C" {
    pub fn find_numa_distance(distance: c_int) -> bool;
}
extern "C" {
    pub fn sched_init_numa(offline_node: c_int);
}
extern "C" {
    pub fn sched_update_numa(cpu: c_int, online: bool);
}
extern "C" {
    pub fn sched_domains_numa_masks_set(cpu: c_uint);
}
extern "C" {
    pub fn sched_domains_numa_masks_clear(cpu: c_uint);
}
extern "C" {
    pub fn sched_numa_find_closest(cpus: *const cpumask, cpu: c_int) -> c_int;
}

// The regions in numa_faults array from task_struct
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum numa_faults_stats {
    NUMA_MEM = 0,
    NUMA_CPU,
    NUMA_MEMBUF,
    NUMA_CPUBUF
}

extern "C" {
    pub fn sched_setnuma(p: *mut task_struct, node: c_int);
}
extern "C" {
    pub fn migrate_task_to(p: *mut task_struct, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn init_numa_balancing(clone_flags: u64, p: *mut task_struct);
}

extern "C" {
    pub fn task_llc(p: *const task_struct) -> c_int;
}
//
// Don't (re)queue an already queued item; nor queue anything when
// balance_push() is active, see the comment with
// balance_push_callback.
//

//
// The domain tree (rq->sd) is protected by RCU's quiescent state transition.
// See destroy_sched_domains: call_rcu for details.
//
// The domain tree of any CPU may only be accessed from within
// preempt-disabled sections.
//

// A mask of all the SD flags that have the SDF_SHARED_CHILD metaflag

//
// highest_flag_domain - Return highest sched_domain containing flag.
// @cpu:	The CPU whose highest level of sched domain is to
// be returned.
// @flag:	The flag to check for the highest sched_domain
// for the given CPU.
//
// Returns the highest sched_domain of a CPU which contains @flag. If @flag has
// the SDF_SHARED_CHILD metaflag, all the children domains also have @flag.
//
// Stop the search if @flag is known to be shared at lower
// levels. It will not be found further up.
//
extern "C" {
    pub fn static_branch_unlikely(_arg: &sched_asym_cpucapacity) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_group_capacity {
    pub ref: core::sync::atomic::AtomicI32,
//
// CPU capacity of this group, SCHED_CAPACITY_SCALE being max capacity
// for a single CPU.
//
    pub capacity: c_ulong,
    pub /: *mut *mut unsigned long min_capacity; / Min per-CPU capacity in group,
    pub /: *mut *mut unsigned long max_capacity; / Max per-CPU capacity in group,
    pub next_update: c_ulong,
    pub /: *mut *mut int imbalance; / XXX unrelated to capacity but shared group state,
    pub id: c_int,
    pub /: *mut *mut unsigned long cpumask[]; / Balance mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_group {
    pub /: *mut *mut *mut sched_group next; / Must be a circular list,
    pub ref: core::sync::atomic::AtomicI32,
    pub group_weight: c_uint,
    pub cores: c_uint,
    pub sgc: *mut sched_group_capacity,
    pub /: *mut *mut int asym_prefer_cpu; / CPU of highest priority in group,
    pub flags: c_int,
//
// The CPUs this group covers.
//
// NOTE: this field is variable length. (Allocated dynamically
// by attaching extra space to the end of the structure,
// depending on how many CPUs the kernel has booted up with)
//
    pub cpumask: [c_ulong; ],
}

extern "C" {
    pub fn to_cpumask(_arg: sg->cpumask) -> return;
}
//
// See build_balance_mask().
//
extern "C" {
    pub fn to_cpumask(_arg: sg->sgc->cpumask) -> return;
}
extern "C" {
    pub fn group_balance_cpu(sg: *mut sched_group) -> c_int;
}
extern "C" {
    pub fn update_sched_domain_debugfs();
}
extern "C" {
    pub fn dirty_sched_domain_sysctl(cpu: c_int);
}
extern "C" {
    pub fn sched_update_scaling() -> c_int;
}

//
// Return the group to which this tasks belongs.
//
// We cannot use task_css() and friends because the cgroup subsystem
// changes that value before the cgroup_subsys::attach() method is called,
// therefore we cannot pin it and might observe the wrong value.
//
// The same is true for autogroup's p->signal->autogroup->tg, the autogroup
// core changes this before calling sched_move_task().
//
// Instead we use a 'copy' which is updated from sched_move_task() while
// holding both task_struct::pi_lock and rq::lock.
//

//
// Defined here to be available before stats.h is included, since
// stats.h has dependencies on things defined later in this file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfs_tg_state {
    pub cfs_rq: cfs_rq,
    pub se: sched_entity,
    pub stats: sched_statistics,
    pub __no_randomize_layout: },
// Access a specific CPU's cfs_rq from a task group
    pub cpu): return per_cpu_ptr(tg->cfs_rq,,
    pub state: *mut cfs_tg_state,
    pub NULL: return,
    pub cfs_rq): state = container_of(tg_cfs_rq(tg, cpu), struct cfs_tg_state,,
    pub &state->se: return,
    pub state: *mut cfs_tg_state,
    pub NULL: return,
    pub cfs_rq): state = container_of(cfs_rq, struct cfs_tg_state,,
    pub &state->se: return,

// Change a task's cfs_rq and parent entity if it moves across CPUs/groups

    pub task_group(p): *mut *mut task_group tg =,

    pub cpu)): set_task_rq_fair(&p->se, p->se.cfs_rq, tg_cfs_rq(tg,,
    pub cpu): p->se.cfs_rq = tg_cfs_rq(tg,,
    pub cpu): p->se.parent = tg_se(tg,,
    pub 0: p->se.depth = p->se.parent ? p->se.parent->depth + 1 :,

//
// p->rt.rt_rq is NULL initially and it is easier to assign
// root_task_group's rt_rq than switching in rt_rq_of_se()
// Clobbers tg(!)
//
    pub &root_task_group: tg =,
    pub tg->rt_rq[cpu]: p->rt.rt_rq =,
    pub tg->rt_se[cpu]: p->rt.parent =,

    pub NULL: return,

    pub cpu): set_task_rq(p,,

//
// After ->cpu is set up to a new value, task_rq_lock(p, ...) can be
// successfully executed on another CPU. We must ensure that updates of
// per-task data have been completed by this moment.
//
    pub cpu): WRITE_ONCE(task_thread_info(p)->cpu,,
    pub cpu: p->wake_cpu =,

//
// Tunables:
//

}

//
// To support run-time toggling of sched features, all the translation units
// (but core.c) reference the sysctl_sched_features defined in core.c.
//

//
// Is p the current execution context?
//
// Is p the current scheduling context?
//
// Note that it might be the current execution context at the same time if
// rq->curr == rq->donor == p.
//
// Wake flags. The first three directly map to some SD flag value
pub const WF_EXEC: c_uint = 0x02 /* Wakeup after exec; maps to SD_BALANCE_EXEC */;
pub const WF_FORK: c_uint = 0x04 /* Wakeup after fork; maps to SD_BALANCE_FORK */;
pub const WF_TTWU: c_uint = 0x08 /* Wakeup;            maps to SD_BALANCE_WAKE */;
pub const WF_SYNC: c_uint = 0x10 /* Waker goes to sleep after wakeup */;
pub const WF_MIGRATED: c_uint = 0x20 /* Internal use, task got migrated */;
pub const WF_CURRENT_CPU: c_uint = 0x40 /* Prefer to move the wakee to the current CPU. */;
pub const WF_RQ_SELECTED: c_uint = 0x80 /* ->select_task_rq() was called */;
//
// To aid in avoiding the subversion of "niceness" due to uneven distribution
// of tasks with abnormal "nice" values across CPUs the contribution that
// each task makes to its run queue's load is weighted according to its
// scheduling class and "nice" value. For SCHED_NORMAL tasks this is just a
// scaled version of the new time slice allocation that they receive on time
// slice expiry etc.
//
pub const WEIGHT_IDLEPRIO: c_int = 3;
pub const WMULT_IDLEPRIO: c_int = 1431655765;
//
// {de,en}queue flags:
//
// SLEEP/WAKEUP - task is no-longer/just-became runnable
//
// SAVE/RESTORE - an otherwise spurious dequeue/enqueue, done to ensure tasks
// are in a known state which allows modification. Such pairs
// should preserve as much state as possible.
//
// MOVE - paired with SAVE/RESTORE, explicitly does not preserve the location
// in the runqueue. IOW the priority is allowed to change. Callers
// must expect to deal with balance callbacks.
//
// NOCLOCK - skip the update_rq_clock() (avoids double updates)
//
// MIGRATION - p->on_rq == TASK_ON_RQ_MIGRATING (used for DEADLINE)
//
// DELAYED - de/re-queue a sched_delayed task
//
// CLASS - going to update p->sched_class; makes sched_change call the
// various switch methods.
//
// ENQUEUE_HEAD      - place at front of runqueue (tail if not specified)
// ENQUEUE_REPLENISH - CBS (replenish runtime and postpone deadline)
// ENQUEUE_MIGRATED  - the task was migrated during wakeup
// ENQUEUE_RQ_SELECTED - ->select_task_rq() was called
//
// XXX SAVE/RESTORE in combination with CLASS doesn't really make sense, but
// SCHED_DEADLINE seems to rely on this for now.
//
pub const DEQUEUE_SLEEP: c_uint = 0x0001 /* Matches ENQUEUE_WAKEUP */;
pub const DEQUEUE_SAVE: c_uint = 0x0002 /* Matches ENQUEUE_RESTORE */;
pub const DEQUEUE_MOVE: c_uint = 0x0004 /* Matches ENQUEUE_MOVE */;
pub const DEQUEUE_NOCLOCK: c_uint = 0x0008 /* Matches ENQUEUE_NOCLOCK */;
pub const DEQUEUE_MIGRATING: c_uint = 0x0010 /* Matches ENQUEUE_MIGRATING */;
pub const DEQUEUE_DELAYED: c_uint = 0x0020 /* Matches ENQUEUE_DELAYED */;
pub const DEQUEUE_CLASS: c_uint = 0x0040 /* Matches ENQUEUE_CLASS */;
pub const DEQUEUE_SPECIAL: c_uint = 0x00010000;
pub const DEQUEUE_THROTTLE: c_uint = 0x00020000;
pub const ENQUEUE_WAKEUP: c_uint = 0x0001;
pub const ENQUEUE_RESTORE: c_uint = 0x0002;
pub const ENQUEUE_MOVE: c_uint = 0x0004;
pub const ENQUEUE_NOCLOCK: c_uint = 0x0008;
pub const ENQUEUE_MIGRATING: c_uint = 0x0010;
pub const ENQUEUE_DELAYED: c_uint = 0x0020;
pub const ENQUEUE_CLASS: c_uint = 0x0040;
pub const ENQUEUE_HEAD: c_uint = 0x00010000;
pub const ENQUEUE_REPLENISH: c_uint = 0x00020000;
pub const ENQUEUE_MIGRATED: c_uint = 0x00040000;
pub const ENQUEUE_INITIAL: c_uint = 0x00080000;
pub const ENQUEUE_RQ_SELECTED: c_uint = 0x00100000;
pub const ENQUEUE_QUEUED: c_uint = 0x00200000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affinity_context {
    pub new_mask: *const cpumask,
    pub user_mask: *mut cpumask,
    pub flags: c_uint,
}

extern "C" {
    pub fn update_curr_common(rq: *mut rq) -> i64;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_class {

    pub uclamp_enabled: c_int,

//
// move_queued_task/activate_task/enqueue_task: rq->lock
// ttwu_do_activate/activate_task/enqueue_task: rq->lock
// wake_up_new_task/activate_task/enqueue_task: task_rq_lock
// ttwu_runnable/enqueue_task: task_rq_lock
// proxy_task_current: rq->lock
// sched_change_end
//
    pub flags): *mut *mut *mut *mut void (enqueue_task) (struct rq rq, struct task_struct p, int,
//
// move_queued_task/deactivate_task/dequeue_task: rq->lock
// __schedule/block_task/dequeue_task: rq->lock
// proxy_task_current: rq->lock
// wait_task_inactive: task_rq_lock
// sched_change_begin
//
    pub flags): *mut *mut *mut *mut bool (dequeue_task) (struct rq rq, struct task_struct p, int,
//
// do_sched_yield: rq->lock
//
    pub rq): *mut *mut void (yield_task) (struct rq,
//
// yield_to: rq->lock (double)
//
    pub p): *mut *mut *mut bool (yield_to_task)(struct rq rq, struct task_struct,
//
// move_queued_task: rq->lock
// __migrate_swap_task: rq->lock
// ttwu_do_activate: rq->lock
// ttwu_runnable: task_rq_lock
// wake_up_new_task: task_rq_lock
//
    pub flags): *mut *mut *mut *mut void (wakeup_preempt)(struct rq rq, struct task_struct p, int,
//
// schedule/pick_next_task/prev_balance: rq->lock
//
    pub rf): *mut *mut *mut int (balance)(struct rq rq, struct rq_flags,
//
// schedule/pick_next_task: rq->lock
//
    pub rf): *mut *mut *mut *mut task_(pick_task)(rq rq, rq_flags,
//
// sched_change:
// __schedule: rq->lock
//
    pub next): *mut *mut *mut *mut void (put_prev_task)(struct rq rq, struct task_struct p, struct task_struct,
    pub first): *mut *mut *mut *mut void (set_next_task)(struct rq rq, struct task_struct p, bool,
//
// select_task_rq: p->pi_lock
// sched_exec: p->pi_lock
//
    pub flags): *mut *mut *mut int (select_task_rq)(struct task_struct p, int task_cpu, int,
//
// set_task_cpu: p->pi_lock || rq->lock (ttwu like)
//
    pub new_cpu): *mut *mut *mut void (migrate_task_rq)(struct task_struct p, int,
//
// ttwu_do_activate: rq->lock
// wake_up_new_task: task_rq_lock
//
    pub task): *mut *mut *mut void (task_woken)(struct rq this_rq, struct task_struct,
//
// do_set_cpus_allowed: task_rq_lock + sched_change
//
    pub ctx): *mut *mut *mut void (set_cpus_allowed)(struct task_struct p, struct affinity_context,
//
// sched_set_rq_{on,off}line: rq->lock
//
    pub rq): *mut *mut void (rq_online)(struct rq,
    pub rq): *mut *mut void (rq_offline)(struct rq,
//
// push_cpu_stop: p->pi_lock && rq->lock
//
    pub rq): *mut *mut *mut *mut rq (find_lock_rq)(task_p, rq,
//
// hrtick: rq->lock
// sched_tick: rq->lock
// sched_tick_remote: rq->lock
//
    pub queued): *mut *mut *mut *mut void (task_tick)(struct rq rq, struct task_struct p, int,
//
// sched_cgroup_fork: p->pi_lock
//
    pub p): *mut *mut void (task_fork)(struct task_struct,
//
// finish_task_switch: no locks
//
    pub p): *mut *mut void (task_dead)(struct task_struct,
//
// sched_change
//
    pub task): *mut *mut *mut void (switching_from)(struct rq this_rq, struct task_struct,
    pub task): *mut *mut *mut void (switched_from) (struct rq this_rq, struct task_struct,
    pub task): *mut *mut *mut void (switching_to) (struct rq this_rq, struct task_struct,
    pub task): *mut *mut *mut void (switched_to) (struct rq this_rq, struct task_struct,
    pub task): *mut *mut *mut u64 (get_prio) (struct rq this_rq, struct task_struct,
    pub oldprio): u64,
//
// set_load_weight: task_rq_lock + sched_change
// __setscheduler_parms: task_rq_lock + sched_change
//
    pub lw): *const load_weight,
//
// sched_rr_get_interval: task_rq_lock
//
    pub task): *mut task_struct,
//
// task_sched_runtime: task_rq_lock
//
    pub rq): *mut *mut void (update_curr)(struct rq,

//
// sched_change_group: task_rq_lock + sched_change
//
    pub p): *mut *mut void (task_change_group)(struct task_struct,

//
// pick_next_task: rq->lock
// try_steal_cookie: rq->lock (double)
//
    pub cpu): *mut *mut *mut int (task_is_throttled)(struct task_struct p, int,

}

//
// Helper to define a sched_class instance; each one is placed in a separate
// section which is ordered by the linker script:
//
// include/asm-generic/vmlinux.lds.h
//
// *CAREFUL* they are laid out in *REVERSE* order!!!
//
// Also enforce alignment on the instance, not the type, to guarantee layout.
//

// Defined in include/asm-generic/vmlinux.lds.h
//
// Iterate only active classes. SCX can take over all fair tasks or be
// completely disabled. If the former, skip fair. If the latter, skip SCX.
//

extern "C" {
    pub fn sched_class_above(_arg: rq->next_class, _arg: class) -> return;
}
pub const SCA_CHECK: c_uint = 0x01;
pub const SCA_MIGRATE_DISABLE: c_uint = 0x02;
pub const SCA_MIGRATE_ENABLE: c_uint = 0x04;
pub const SCA_USER: c_uint = 0x08;
extern "C" {
    pub fn update_group_capacity(sd: *mut sched_domain, cpu: c_int);
}
extern "C" {
    pub fn sched_balance_trigger(rq: *mut rq);
}
extern "C" {
    pub fn __set_cpus_allowed_ptr(p: *mut task_struct, ctx: *mut affinity_context) -> c_int;
}
extern "C" {
    pub fn set_cpus_allowed_common(p: *mut task_struct, ctx: *mut affinity_context);
}
// When not in the task's cpumask, no point in looking further.
// Can @cpu run a user thread?
//
// See set_cpus_allowed_force() above for the rcu_head usage.
//
extern "C" {
    pub fn kmalloc_node(_arg: size, _arg: GFP_KERNEL, _arg: node) -> return;
}
extern "C" {
    pub fn get_task_struct(_arg: p) -> return;
}
extern "C" {
    pub fn push_cpu_stop(arg: *mut c_void) -> c_int;
}

extern "C" {
    pub fn schedule_idle();
}
extern "C" {
    pub fn schedule_user() -> asmlinkage void;
}
extern "C" {
    pub fn sysrq_sched_debug_show();
}
extern "C" {
    pub fn sched_init_granularity();
}
extern "C" {
    pub fn update_max_interval();
}
extern "C" {
    pub fn init_sched_dl_class();
}
extern "C" {
    pub fn init_sched_rt_class();
}
extern "C" {
    pub fn init_sched_fair_class();
}
extern "C" {
    pub fn resched_curr(rq: *mut rq);
}
extern "C" {
    pub fn resched_curr_lazy(rq: *mut rq);
}
extern "C" {
    pub fn resched_cpu(cpu: c_int);
}
extern "C" {
    pub fn init_rt_bandwidth(rt_b: *mut rt_bandwidth, period: u64, runtime: u64);
}
extern "C" {
    pub fn sched_rt_bandwidth_account(rt_rq: *mut rt_rq) -> bool;
}
extern "C" {
    pub fn init_dl_entity(dl_se: *mut sched_dl_entity);
}
extern "C" {
    pub fn init_cfs_throttle_work(p: *mut task_struct);
}
pub const BW_SHIFT: c_int = 20;

pub const RATIO_SHIFT: c_int = 8;

extern "C" {
    pub fn to_ratio(period: u64, runtime: u64) -> u64;
}
extern "C" {
    pub fn init_entity_runnable_average(se: *mut sched_entity);
}
extern "C" {
    pub fn post_init_entity_util_avg(p: *mut task_struct);
}

extern "C" {
    pub fn sched_can_stop_tick(rq: *mut rq) -> bool;
}
extern "C" {
    pub fn sched_tick_offload_init() -> int __init;
}
//
// Tick may be needed by tasks in the runqueue depending on their policy and
// requirements. If tick is needed, lets send the target an IPI to kick it out of
// nohz mode if necessary.
//

// Check if we still need preemption
//
// The moment this write goes through, ttwu() can swoop in and migrate
// this task, rendering our rq->__lock ineffective.
//
// __schedule()				try_to_wake_up()
// LOCK rq->__lock			  LOCK p->pi_lock
// pick_next_task()
// pick_next_task_fair()
// pick_next_entity()
// dequeue_entities()
// __block_task()
// RELEASE p->on_rq = 0	  if (p->on_rq && ...)
// break;
//
// ACQUIRE (after ctrl-dep)
//
// cpu = select_task_rq();
// set_task_cpu(p, cpu);
// ttwu_queue()
// ttwu_do_activate()
// LOCK rq->__lock
// activate_task()
// STORE p->on_rq = 1
// UNLOCK rq->__lock
//
// Callers must ensure to not reference @p after this -- we no longer
// own it.
//
extern "C" {
    pub fn activate_task(rq: *mut rq, p: *mut task_struct, flags: c_int);
}
extern "C" {
    pub fn deactivate_task(rq: *mut rq, p: *mut task_struct, flags: c_int);
}
extern "C" {
    pub fn wakeup_preempt(rq: *mut rq, p: *mut task_struct, flags: c_int);
}
//
// attach_task() -- attach the task detached by detach_task() to its new rq.
//
// attach_one_task() -- attaches the task returned from detach_one_task() to
// its new rq.
//

//
// Use hrtick when:
// - enabled by features
// - hrtimer is actually high res
//
extern "C" {
    pub fn cpu_active(hrtimer_highres_enabled(: cpu_of(rq)) &&) -> return;
}
extern "C" {
    pub fn sched_feat(hrtick_enabled(rq: HRTICK) &&) -> return;
}
extern "C" {
    pub fn sched_feat(hrtick_enabled(rq: HRTICK_DL) &&) -> return;
}
extern "C" {
    pub fn hrtick_start(rq: *mut rq, delay: u64);
}
extern "C" {
    pub fn hrtimer_active(_arg: &rq->hrtick_timer) -> return;
}

//
// arch_scale_freq_capacity - get the frequency scale factor of a given CPU.
// @cpu: the CPU in question.
//
// Return: the frequency scale factor normalized against SCHED_CAPACITY_SCALE, i.e.
//
// f_curr
// ------ * SCHED_CAPACITY_SCALE
// f_max
//

//
// In double_lock_balance()/double_rq_lock(), we use raw_spin_rq_lock() to
// acquire rq lock instead of rq_lock(). So at the end of these two functions
// we need to call double_rq_clock_clear_update() to clear RQCF_UPDATED of
// rq->clock_update_flags to avoid the WARN_DOUBLE_CLOCK warning.
//

// __UNIQUE_ID(unlock1) __cleanup(__class_##_name##_cleanup_ctx1) = (void *)(_T1),\
// __UNIQUE_ID(unlock2) __cleanup(__class_##_name##_cleanup_ctx2) = (void *)(_T2)

//
// In order to not have {0,2},{1,3} turn into into an AB-BA,
// order by core-id first and cpu-id second.
//
// Notably:
//
// double_rq_lock(0,3); will take core-0, core-1 lock
// double_rq_lock(1,2); will take core-1, core-0 lock
//
// when only cpu-id is considered.
//
// __sched_core_flip() relies on SMT having cpu-id lock order.
//

//
// fair double_lock_balance: Safely acquires both rq->locks in a fair
// way at the expense of forcing extra atomic operations in all
// invocations.  This assures that the double_lock is acquired using the
// same underlying policy as the spinlock_t on this architecture, which
// reduces latency compared to the unfair variant below.  However, it
// also adds more overhead and therefore may reduce throughput.
//

//
// Unfair double_lock_balance: Optimizes throughput at the expense of
// latency by eliminating extra atomic operations when the locks are
// already in proper order on entry.  This favors lower CPU-ids and will
// grant the double lock to lower CPUs over higher ids under contention,
// regardless of entry order into the function.
//

//
// double_lock_balance - lock the busiest runqueue, this_rq is locked already.
//
extern "C" {
    pub fn _double_lock_balance(_arg: this_rq, _arg: busiest) -> return;
}

//
// double_rq_unlock - safely unlock two runqueues
//
// Note this does not restore interrupts like task_rq_unlock,
// you need to do so manually after calling.
//
extern "C" {
    pub fn set_rq_online(rq: *mut rq);
}
extern "C" {
    pub fn set_rq_offline(rq: *mut rq);
}
extern "C" {
    pub fn print_cfs_stats(m: *mut seq_file, cpu: c_int);
}
extern "C" {
    pub fn print_rt_stats(m: *mut seq_file, cpu: c_int);
}
extern "C" {
    pub fn print_dl_stats(m: *mut seq_file, cpu: c_int);
}
extern "C" {
    pub fn print_cfs_rq(m: *mut seq_file, cpu: c_int, cfs_rq: *mut cfs_rq);
}
extern "C" {
    pub fn print_rt_rq(m: *mut seq_file, cpu: c_int, rt_rq: *mut rt_rq);
}
extern "C" {
    pub fn print_dl_rq(m: *mut seq_file, cpu: c_int, dl_rq: *mut dl_rq);
}
extern "C" {
    pub fn resched_latency_warn(cpu: c_int, latency: u64);
}

extern "C" {
    pub fn show_numa_stats(p: *mut task_struct, m: *mut seq_file);
}

extern "C" {
    pub fn init_cfs_rq(cfs_rq: *mut cfs_rq);
}
extern "C" {
    pub fn init_rt_rq(rt_rq: *mut rt_rq);
}
extern "C" {
    pub fn init_dl_rq(dl_rq: *mut dl_rq);
}
extern "C" {
    pub fn cfs_bandwidth_usage_inc();
}
extern "C" {
    pub fn cfs_bandwidth_usage_dec();
}

pub const NOHZ_BALANCE_KICK_BIT: c_int = 0;
pub const NOHZ_STATS_KICK_BIT: c_int = 1;
pub const NOHZ_NEWILB_KICK_BIT: c_int = 2;
pub const NOHZ_NEXT_KICK_BIT: c_int = 3;
// Run sched_balance_domains()

// Update blocked load

// Update blocked load when entering idle

// Update nohz.next_balance

extern "C" {
    pub fn nohz_balance_exit_idle(rq: *mut rq);
}

extern "C" {
    pub fn nohz_run_idle_balance(cpu: c_int);
}

extern "C" {
    pub fn __sched_core_account_forceidle(rq: *mut rq);
}
extern "C" {
    pub fn __sched_core_tick(rq: *mut rq);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irqtime {
    pub total: u64,
    pub tick_delta: u64,
    pub irq_start_time: u64,
    pub sync: u64_stats_sync,
}

extern "C" {
    pub fn static_branch_likely(_arg: &sched_clock_irqtime) -> return;
}
//
// Returns the irqtime minus the softirq time computed by ksoftirqd.
// Otherwise ksoftirqd's sum_exec_runtime is subtracted its own runtime
// and never move forward.
//

//
// cpufreq_update_util - Take a note about CPU utilization changes.
// @rq: Runqueue to carry out the update for.
// @flags: Update reason flags.
//
// This function is called by the scheduler on the CPU whose utilization is
// being updated.
//
// It can only be called from RCU-sched read-side critical sections.
//
// The way cpufreq is currently arranged requires it to evaluate the CPU
// performance state (frequency/voltage) on a regular basis to prevent it from
// being stuck in a completely inadequate performance level for too long.
// That is not guaranteed to happen if the updates are only triggered from CFS
// and DL, though, because they may not be coming in if only RT tasks are
// active all the time (or there are RT tasks only).
//
// As a workaround for that issue, this function is called periodically by the
// RT sched class to trigger extra cpufreq updates to prevent it from stalling,
// but that really is a band-aid.  Going forward it should be replaced with
// solutions targeted more specifically at RT tasks.
//

//
// Verify the fitness of task @p to run on @cpu taking into account the
// CPU original capacity and the runtime/deadline ratio of the task.
//
// The function will return true if the original capacity of @cpu is
// greater than or equal to task's deadline density right shifted by
// (BW_SHIFT - SCHED_CAPACITY_SHIFT) and false otherwise.
//
extern "C" {
    pub fn READ_ONCE(_arg: rq->avg_dl.util_avg) -> return;
}
extern "C" {
    pub fn cpu_util_cfs(cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn cpu_util_cfs_boost(cpu: c_int) -> c_ulong;
}
extern "C" {
    pub fn READ_ONCE(_arg: rq->avg_rt.util_avg) -> return;
}

extern "C" {
    pub fn uclamp_eff_value(p: *mut task_struct, clamp_id: uclamp_id) -> c_ulong;
}
//
// When uclamp is compiled in, the aggregation at rq level is 'turned off'
// by default in the fast path and only gets turned on once userspace performs
// an operation that requires it.
//
// Returns true if userspace opted-in to use uclamp and aggregation at rq level
// hence is active.
//
extern "C" {
    pub fn static_branch_likely(_arg: &sched_uclamp_used) -> return;
}
//
// Enabling static branches would get the cpus_read_lock(),
// check whether uclamp_is_used before enable it to avoid always
// calling cpus_read_lock(). Because we never disable this
// static key once enable it.
//
extern "C" {
    pub fn READ_ONCE(_arg: rq->uclamp[clamp_id].value) -> return;
}
// Is the rq being capped/throttled by uclamp_max?

// Integer rounded range for each bucket

extern "C" {
    pub fn min_t(int: unsigned, UCLAMP_BUCKET_DELTA: clamp_value /, 1: UCLAMP_BUCKETS -) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: rq->avg_irq.util_avg) -> return;
}

extern "C" {
    pub fn __setparam_fair(p: *mut task_struct, attr: *const sched_attr);
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &sched_energy_present) -> return;
}

//
// The scheduler provides memory barriers required by membarrier between:
// - prior user-space memory accesses and store to rq->membarrier_state,
// - store to rq->membarrier_state and following user-space memory accesses.
// In the same way it provides those guarantees around store to rq->curr.
//

extern "C" {
    pub fn swake_up_all_locked(q: *mut swait_queue_head);
}
extern "C" {
    pub fn __prepare_to_swait(q: *mut swait_queue_head, wait: *mut swait_queue);
}
extern "C" {
    pub fn try_to_wake_up(tsk: *mut task_struct, state: c_uint, wake_flags: c_int) -> c_int;
}

extern "C" {
    pub fn sched_dynamic_mode(str: *const c_char) -> c_int;
}
extern "C" {
    pub fn sched_dynamic_update(mode: c_int);
}

// True if none of the MM_CID_ONCPU, MM_CID_TRANSIT, MM_CID_UNSET bits is set
// Clear the ONCPU bit, but do not set UNSET in the per CPU storage
// Is it in the optimal CID space?
// Try to find one in the optimal space. Otherwise keep the provided.
// Preserve the ONCPU mode of the original CID
// Optimize for the common case where both have the ONCPU bit set
// Try to converge into the optimal CID space
// Hand over or drop the task owned CID
// Still nothing, allocate a new one
// Handle the transition mode flag if required
// Optimize for the common case, where both have the ONCPU bit clear
// Try to converge into the optimal CID space
// Hand over or drop the CPU owned CID
// Still nothing, allocate a new one
// Set the transition mode flag if required
// During mode transitions CIDs are temporary and need to be dropped
//
// If transition mode is done, transfer ownership when the CID is
// within the convergence range to optimize the next schedule in.
//
// Update both so that the next schedule in goes into the fast path

extern "C" {
    pub fn static_branch_unlikely(_arg: &sched_cache_active) -> return;
}
extern "C" {
    pub fn sched_cache_active_set();
}

extern "C" {
    pub fn sched_domains_free_llc_id(cpu: c_int);
}
extern "C" {
    pub fn init_sched_mm(p: *mut task_struct);
}
extern "C" {
    pub fn avg_vruntime(cfs_rq: *mut cfs_rq) -> u64;
}
extern "C" {
    pub fn entity_eligible(cfs_rq: *mut cfs_rq, se: *mut sched_entity) -> c_int;
}

extern "C" {
    pub fn __rt_effective_prio(_arg: pi_task, _arg: prio) -> return;
}

extern "C" {
    pub fn __sched_setscheduler(p: *mut task_struct, attr: *const sched_attr, user: bool, pi: bool) -> c_int;
}
extern "C" {
    pub fn __sched_setaffinity(p: *mut task_struct, ctx: *mut affinity_context) -> c_int;
}
extern "C" {
    pub fn set_load_weight(p: *mut task_struct, update_load: bool);
}
extern "C" {
    pub fn enqueue_task(rq: *mut rq, p: *mut task_struct, flags: c_int);
}
extern "C" {
    pub fn dequeue_task(rq: *mut rq, p: *mut task_struct, flags: c_int) -> bool;
}
extern "C" {
    pub fn __balance_callbacks(rq: *mut rq, rf: *mut rq_flags);
}
extern "C" {
    pub fn balance_callbacks(rq: *mut rq, head: *mut balance_callback);
}
//
// The 'sched_change' pattern is the safe, easy and slow way of changing a
// task's scheduling properties. It dequeues a task, such that the scheduler
// is fully unaware of it; at which point its properties can be modified;
// after which it is enqueued again.
//
// Typically this must be called while holding task_rq_lock, since most/all
// properties are serialized under those locks. There is currently one
// exception to this rule in sched/ext which only holds rq->lock.
//
// This structure is a temporary, used to preserve/convey the queueing state
// of the task between sched_change_begin() and sched_change_end(). Ensuring
// the task's queueing state is idempotent across the operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_change_ctx {
    pub prio: u64,
    pub p: *mut task_struct,
    pub class: *const sched_class,
    pub flags: c_int,
    pub queued: bool,
    pub running: bool,
}

extern "C" {
    pub fn sched_change_end(ctx: *mut sched_change_ctx);
}

