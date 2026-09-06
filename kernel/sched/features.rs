//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/features.h
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
// Using the avg_vruntime, do the right thing and preserve lag across
// sleep+wake cycles. EEVDF placement strategy #1, #2 if disabled.
//
// Give new tasks half a slice to ease into the competition.
//
// Preserve relative virtual deadline on 'migration'.
//
// Inhibit (wakeup) preemption until the current task has either matched the
// 0-lag point or until is has exhausted it's slice.
//
// Allow wakeup of tasks with a shorter slice to cancel RUN_TO_PARITY for
// current.
//
// Prefer to schedule the task we woke last (assuming it failed
// wakeup-preemption), since its likely going to consume data we
// touched, increases cache locality.
//
// Allow completely ignoring cfs_rq->next; which can be set from various
// places:
// - NEXT_BUDDY (wakeup preemption)
// - yield_to_task()
// - cgroup dequeue / pick
//
// Consider buddies to be cache hot, decreases the likeliness of a
// cache buddy being migrated away, increases cache locality.
//
// Delay dequeueing tasks until they get selected or woken.
//
// By delaying the dequeue for non-eligible tasks, they remain in the
// competition and can burn off their negative lag. When they get selected
// they'll have positive lag by definition.
//
// DELAY_ZERO clips the lag on dequeue (or wakeup) to 0.
//
// Allow wakeup-time preemption of the current task:
//

//
// Decrement CPU capacity based on time not spent running tasks
//

//
// Queue remote wakeups on the target CPU and process them
// using the scheduler IPI. Reduces rq->lock contention/bounces.
//

//
// When doing wakeups, attempt to limit superfluous scans of the LLC domain.
//
// Issue a WARN when we do multiple update_rq_clock() calls
// in a single rq->lock section. Default disabled because the
// annotations are not complete.
//

//
// In order to avoid a thundering herd attack of CPUs that are
// lowering their priorities at the same time, and there being
// a single CPU that has an RT task that can migrate and is waiting
// to run, where the other CPUs will try to take that CPUs
// rq lock and possibly create a large contention, sending an
// IPI to that CPU and let that CPU push the RT task to where
// it should go may be a better scenario.
//
// This is best for PREEMPT_RT, but for non-RT it can cause issues
// when preemption is disabled for long periods of time. Have
// it only default enabled for PREEMPT_RT.
//

//
// UtilEstimation. Use estimated CPU utilization.
//
// Do newidle balancing proportional to its success rate using randomization.
//
