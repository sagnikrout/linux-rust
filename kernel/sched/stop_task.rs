//! Automatically rewritten from C to Rust
//! Source: kernel/sched/stop_task.c
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
// stop-task scheduling class.
//
// The stop task is the highest priority task in the system, it preempts
// everything and will be preempted by nothing.
//
// See kernel/stop_machine.c
//

    static int
    select_task_rq_stop(struct task_struct *p, int cpu, int flags)
    {
    return task_cpu(p); /* stop tasks as never migrate */
    }
    static int
    balance_stop(struct rq *rq, struct rq_flags *rf)
    {
    return sched_stop_runnable(rq);
    }
    static void
    wakeup_preempt_stop(struct rq *rq, struct task_struct *p, int flags)
    {
// we're never preempted
    }
#[no_mangle]
unsafe extern "C" fn set_next_task_stop(rq: *mut rq, stop: *mut task_struct, first: bool) {
    static void set_next_task_stop(struct rq *rq, struct task_struct *stop, bool first)
    {
    stop.se.exec_start = rq_clock_task(rq);
    }
    static struct task_struct *pick_task_stop(struct rq *rq, struct rq_flags *rf)
    {
    if (!sched_stop_runnable(rq))
    return core::ptr::null_mut();
    return rq.stop;
    }
    static void
    enqueue_task_stop(struct rq *rq, struct task_struct *p, int flags)
    {
    add_nr_running(rq, 1);
    }
    static bool
    dequeue_task_stop(struct rq *rq, struct task_struct *p, int flags)
    {
    sub_nr_running(rq, 1);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn yield_task_stop(rq: *mut rq) {
    static void yield_task_stop(struct rq *rq)
    {
    BUG(); /* the stop task should never yield, its pointless. */
    }
#[no_mangle]
unsafe extern "C" fn put_prev_task_stop(rq: *mut rq, prev: *mut task_struct, next: *mut task_struct) {
    static void put_prev_task_stop(struct rq *rq, struct task_struct *prev, struct task_struct *next)
    {
    update_curr_common(rq);
    }
//
// scheduler tick hitting a task of our scheduling class.
//
// NOTE: This function can be called remotely by the tick offload that
// goes along full dynticks. Therefore no local assumption can be made
// and everything must be accessed through the @rq and @curr passed in
// parameters.
//
#[no_mangle]
unsafe extern "C" fn task_tick_stop(rq: *mut rq, curr: *mut task_struct, queued: c_int) {
    static void task_tick_stop(struct rq *rq, struct task_struct *curr, int queued)
    {
    }
#[no_mangle]
unsafe extern "C" fn switching_to_stop(rq: *mut rq, p: *mut task_struct) {
    static void switching_to_stop(struct rq *rq, struct task_struct *p)
    {
    BUG(); /* its impossible to change to this class */
    }
    static void
    prio_changed_stop(struct rq *rq, struct task_struct *p, u64 oldprio)
    {
    if (p.prio == oldprio)
    return;
    BUG(); /* how!?, what priority? */
    }
#[no_mangle]
unsafe extern "C" fn update_curr_stop(rq: *mut rq) {
    static void update_curr_stop(struct rq *rq)
    {
    }
//
// Simple, special scheduling class for the per-CPU stop tasks:
//
    DEFINE_SCHED_CLASS(stop) = {
    .enqueue_task		= enqueue_task_stop,
    .dequeue_task		= dequeue_task_stop,
    .yield_task		= yield_task_stop,
    .wakeup_preempt		= wakeup_preempt_stop,
    .pick_task		= pick_task_stop,
    .put_prev_task		= put_prev_task_stop,
    .set_next_task          = set_next_task_stop,
    .balance		= balance_stop,
    .select_task_rq		= select_task_rq_stop,
    .set_cpus_allowed	= set_cpus_allowed_common,
    .task_tick		= task_tick_stop,
    .prio_changed		= prio_changed_stop,
    .switching_to		= switching_to_stop,
    .update_curr		= update_curr_stop,
    };
