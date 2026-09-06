//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/ext.h
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
// BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
//
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2022 Tejun Heo <tj@kernel.org>
// Copyright (c) 2022 David Vernet <dvernet@meta.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_public_consts {
    SCX_OPS_NAME_LEN	= 128,

//
// %SCX_SLICE_DFL is used to refill slices when the BPF scheduler misses
// to set the slice for a task that is selected for execution.
// %SCX_EV_REFILL_SLICE_DFL counts the number of times the default slice
// refill has been triggered.
//
// %SCX_SLICE_BYPASS is used as the slice for all tasks in the bypass
// mode. As making forward progress for all tasks is the main goal of
// the bypass mode, a shorter slice is used.
//
    SCX_SLICE_DFL		= 20 * 1000000,	/* 20ms */
    SCX_SLICE_BYPASS	=  5 * 1000000, /*  5ms */
    SCX_SLICE_INF		= U64_MAX,	/* infinite, implies nohz */
}

//
// DSQ (dispatch queue) IDs are 64bit of the format:
//
// Bits: [63] [62 ..  0]
// [ B] [   ID   ]
//
// B: 1 for IDs for built-in DSQs, 0 for ops-created user DSQs
// ID: 63 bit ID
//
// Built-in IDs:
//
// Bits: [63] [62] [61..32] [31 ..  0]
// [ 1] [ L] [   R  ] [    V   ]
//
// 1: 1 for built-in DSQs.
// L: 1 for LOCAL_ON DSQ IDs, 0 for others
// V: For LOCAL_ON DSQ IDs, a CPU number. For others, a pre-defined value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_dsq_id_flags {
    SCX_DSQ_FLAG_BUILTIN	= 1LLU << 63,
    SCX_DSQ_FLAG_LOCAL_ON	= 1LLU << 62,

    SCX_DSQ_INVALID		= SCX_DSQ_FLAG_BUILTIN | 0,
    SCX_DSQ_GLOBAL		= SCX_DSQ_FLAG_BUILTIN | 1,
    SCX_DSQ_LOCAL		= SCX_DSQ_FLAG_BUILTIN | 2,
    SCX_DSQ_BYPASS		= SCX_DSQ_FLAG_BUILTIN | 3,
    SCX_DSQ_REJECT		= SCX_DSQ_FLAG_BUILTIN | 4,	/* internal - see find_dsq_for_dispatch() */
    SCX_DSQ_RESCUE		= SCX_DSQ_FLAG_BUILTIN | 5,	/* internal - see find_dsq_for_dispatch() */
    SCX_DSQ_LOCAL_ON	= SCX_DSQ_FLAG_BUILTIN | SCX_DSQ_FLAG_LOCAL_ON,
    SCX_DSQ_LOCAL_CPU_MASK	= 0xffffffffLLU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_deferred_reenq_user {
    pub node: list_head,
    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_dsq_pcpu {
    pub dsq: *mut scx_dispatch_q,
    pub deferred_reenq_user: scx_deferred_reenq_user,
}

//
// A dispatch queue (DSQ) can be either a FIFO or p->scx.dsq_vtime ordered
// queue. A built-in DSQ is always a FIFO. The built-in local DSQs are used to
// buffer between the scheduler core and the BPF scheduler. See the
// documentation for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_dispatch_q {
    pub lock: raw_spinlock_t,
    pub /: *mut *mut *mut task___rcu first_task; / lockless peek at head,
    pub /: *mut *mut list_head list; / tasks in dispatch order,
    pub /: *mut *mut rb_root priq; / used to order by p->scx.dsq_vtime,
    pub nr: u32,
    pub /: *mut *mut u32 seq; / used by BPF iter,
    pub id: u64,
    pub hash_node: rhash_head,
    pub free_node: llist_node,
    pub sched: *mut scx_sched,
    pub pcpu: *mut scx_dsq_pcpu __percpu,
    pub rcu: rcu_head,
}

// sched_ext_entity.flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_ent_flags {
    SCX_TASK_QUEUED		= 1 << 0, /* on ext runqueue */
    SCX_TASK_IN_CUSTODY	= 1 << 1, /* in custody, needs ops.dequeue() when leaving */
    SCX_TASK_RESET_RUNNABLE_AT = 1 << 2, /* runnable_at should be reset */
    SCX_TASK_DEQD_FOR_SLEEP	= 1 << 3, /* last dequeue was for SLEEP */
    SCX_TASK_SUB_INIT	= 1 << 4, /* task being initialized for a sub sched */
    SCX_TASK_IMMED		= 1 << 5, /* task is on local DSQ with %SCX_ENQ_IMMED */
    SCX_TASK_PROTECTED	= 1 << 6, /* slice and DSQ head position protected */

//
// Bits 8 to 10 are used to carry task state:
//
// NONE		ops.init_task() not called yet
// INIT_BEGIN	ops.init_task() in flight; see sched_ext_dead()
// INIT		ops.init_task() succeeded, but task can be cancelled
// READY	fully initialized, but not in sched_ext
// ENABLED	fully initialized and in sched_ext
// DEAD		terminal state set by sched_ext_dead()
//
    SCX_TASK_STATE_SHIFT	= 8,
    SCX_TASK_STATE_BITS	= 3,
    SCX_TASK_STATE_MASK	= ((1 << SCX_TASK_STATE_BITS) - 1) << SCX_TASK_STATE_SHIFT,

    SCX_TASK_NONE		= 0 << SCX_TASK_STATE_SHIFT,
    SCX_TASK_INIT_BEGIN	= 1 << SCX_TASK_STATE_SHIFT,
    SCX_TASK_INIT		= 2 << SCX_TASK_STATE_SHIFT,
    SCX_TASK_READY		= 3 << SCX_TASK_STATE_SHIFT,
    SCX_TASK_ENABLED	= 4 << SCX_TASK_STATE_SHIFT,
    SCX_TASK_DEAD		= 5 << SCX_TASK_STATE_SHIFT,

//
// Bits 12 to 14 are used to carry reenqueue reason. In addition to
// %SCX_ENQ_REENQ flag, ops.enqueue() can also test for
// %SCX_TASK_REENQ_REASON_NONE to distinguish reenqueues.
//
// NONE		not being reenqueued
// KFUNC	reenqueued by scx_bpf_dsq_reenq() and friends
// IMMED	reenqueued due to failed ENQ_IMMED
// PREEMPTED	preempted while running
// CAP		sub-sched cap miss, see p->scx.reenq_reason_
//
    SCX_TASK_REENQ_REASON_SHIFT = 12,
    SCX_TASK_REENQ_REASON_BITS = 3,
    SCX_TASK_REENQ_REASON_MASK = ((1 << SCX_TASK_REENQ_REASON_BITS) - 1) << SCX_TASK_REENQ_REASON_SHIFT,

    SCX_TASK_REENQ_NONE	= 0 << SCX_TASK_REENQ_REASON_SHIFT,
    SCX_TASK_REENQ_KFUNC	= 1 << SCX_TASK_REENQ_REASON_SHIFT,
    SCX_TASK_REENQ_IMMED	= 2 << SCX_TASK_REENQ_REASON_SHIFT,
    SCX_TASK_REENQ_PREEMPTED = 3 << SCX_TASK_REENQ_REASON_SHIFT,
    SCX_TASK_REENQ_CAP	= 4 << SCX_TASK_REENQ_REASON_SHIFT,

// iteration cursor, not a task
    SCX_TASK_CURSOR		= 1 << 31,
}

// scx_entity.dsq_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_ent_dsq_flags {
    SCX_TASK_DSQ_ON_PRIQ	= 1 << 0, /* task is queued on the priority queue of a dsq */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_dsq_lnode_flags {
    SCX_DSQ_LNODE_ITER_CURSOR = 1 << 0,

// high 16 bits can be for iter cursor flags
    __SCX_DSQ_LNODE_PRIV_SHIFT = 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_dsq_list_node {
    pub node: list_head,
    pub flags: u32,
    pub /: *mut *mut u32 priv; / can be used by iter cursor,
}

//
// The following is embedded in task_struct and contains all fields necessary
// for a task to be scheduled by SCX.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_ext_entity {

//
// Associated scx_sched. Updated either during fork or while holding
// both p->pi_lock and rq lock.
//
    pub sched: *mut scx_sched __rcu,

    pub dsq: *mut scx_dispatch_q,
    pub ops_state: atomic_long_t,
    pub ddsp_dsq_id: u64,
    pub ddsp_enq_flags: u64,
    pub ddsp_slice: u64,
    pub ddsp_vtime: u64,
    pub /: *mut *mut scx_dsq_list_node dsq_list; / dispatch order,
    pub /: *mut *mut rb_node dsq_priq; / p->scx.dsq_vtime order,
    pub dsq_seq: u32,
    pub /: *mut *mut u32 dsq_flags; / protected by DSQ lock,
    pub /: *mut *mut u32 flags; / protected by rq lock,
    pub weight: u32,
    pub /: *mut *mut u32 reenq_cnt; / reenqueues since last run,
    pub sticky_cpu: i32,
    pub holding_cpu: i32,
    pub selected_cpu: i32,
    pub /: *mut *mut s32 runnable_cpu; / cpu @p is runnable on, -1 if not,
    pub /: *mut *mut *mut task_kf_tasks[2]; / see SCX_CALL_OP_TASK(),
    pub /: *mut *mut list_head runnable_node; / rq->scx.runnable_list,
    pub runnable_at: c_ulong,

    pub /: *mut *mut unsigned long rescue_at; / queued on a rescue DSQ at, jiffies,

//
// Unique non-zero task ID assigned at fork. Persists across exec and
// is never reused. Lets BPF schedulers identify tasks without storing
// kernel pointers - arena-backed schedulers being one example. See
// scx_bpf_tid_to_task().
//
    pub tid: u64,
    pub /: *mut *mut rhash_head tid_hash_node; / see SCX_OPS_TID_TO_TASK,
// BPF scheduler modifiable fields
//
// Runtime budget in nsecs - how long the task may hold its cpu. Owned
// by the task's scheduler. Set it when enqueuing via
// scx_bpf_dsq_insert(), or otherwise via scx_bpf_task_set_slice().
// Automatically decreased as the task executes. On depletion a
// scheduling event is triggered.
//
// This value is cleared to zero if the task is preempted by
// %SCX_KICK_PREEMPT and shouldn't be used to determine how long the
// task ran. Use p->se.sum_exec_runtime instead.
//
    pub slice: u64,
//
// Used to order tasks when dispatching to the vtime-ordered priority
// queue of a dsq. This is usually set through
// scx_bpf_dsq_insert_vtime() but can also be modified directly by the
// BPF scheduler. Modifying it while a task is queued on a dsq may
// mangle the ordering and is not recommended.
//
    pub dsq_vtime: u64,
//
// Out-of-band slice request from scx_bpf_task_set_slice() when the
// caller does not hold the rq lock, applied under the rq lock at the
// next slice consideration. One atomic64 packs the pending flag, the
// issuing sch's id, and the requested slice. See scx_slice_oob_consts.
//
    pub slice_oob: core::sync::atomic::AtomicI64,
//
// Sub-sched cap rejected reenq context, valid only while
// %SCX_TASK_REENQ_CAP is set. @reenq_reason_caps is the SCX_CAP_* bits
// that were needed but missing. @reenq_reason_cid is the target cid.
//
    pub reenq_reason_caps: u64,
    pub reenq_reason_cid: i32,
//
// If set, reject future sched_setscheduler(2) calls updating the policy
// to %SCHED_EXT with -%EACCES.
//
// Can be set from ops.init_task() while the BPF scheduler is being
// loaded. If set and the task's policy is already %SCHED_EXT, the
// task's policy is rejected and forcefully reverted to %SCHED_NORMAL.
// The number of such events are reported through
// /sys/kernel/sched_ext/nr_rejected. Setting this flag from any other
// ops.init_task() invocation, such as during fork, fails the scheduler.
//
    pub /: *mut *mut bool disallow; / reject switching into SCX,
// cold fields

    pub cgrp_moving_from: *mut cgroup,

    pub tasks_node: list_head,
}

extern "C" {
    pub fn sched_ext_dead(p: *mut task_struct);
}
extern "C" {
    pub fn print_scx_info(log_lvl: *const c_char, p: *mut task_struct);
}
extern "C" {
    pub fn scx_softlockup(dur_s: u32);
}
extern "C" {
    pub fn scx_hardlockup(cpu: c_int) -> bool;
}
extern "C" {
    pub fn scx_rcu_cpu_stall(stalled_mask: *const cpumask) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_task_group {

//
// The sched this tg is on, NULL if none. SCX_TG_INITED tracks whether
// ops.cgroup_init() succeeded on it. When a child sched exits and its
// tgs move to the parent, a failed init leaves the tg on the parent
// with INITED clear (see scx_cgroup_return_subtree()).
//
// This is tracked separately from cgrp->scx_sched because the tg
// hierarchy can diverge from the cgroup2 hierarchy in both lifetime and
// shape. A tg stays online past its cgroup's removal while the
// cgrp->scx_sched rewrites visit only live cgroups, leaving a removed
// cgroup's pointer stale. The cpu controller can also be mounted on
// cgroup1.
//
    pub sched: *mut scx_sched,
    pub /: *mut *mut *mut u32 flags; / SCX_TG_,
    pub weight: u32,
    pub bw_period_us: u64,
    pub bw_quota_us: u64,
    pub bw_burst_us: u64,
    pub idle: bool,

}
