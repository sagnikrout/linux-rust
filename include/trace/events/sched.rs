//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/sched.h
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
// Tracepoint for calling kthread_stop, performed to end a kthread:
//
// Tracepoint for the return value of the kthread stopping:
//
// sched_kthread_work_queue_work - called when a work gets queued
// @worker:	pointer to the kthread_worker
// @work:	pointer to struct kthread_work
//
// This event occurs when a work is queued immediately or once a
// delayed work is actually queued (ie: once the delay has been
// reached).
//
// sched_kthread_work_execute_start - called immediately before the work callback
// @work:	pointer to struct kthread_work
//
// Allows to track kthread work execution.
//
// sched_kthread_work_execute_end - called immediately after the work callback
// @work:	pointer to struct work_struct
// @function:   pointer to worker function
//
// Allows to track workqueue execution.
//
// Tracepoint for waking up a task:
//
// Tracepoint called when waking a task; this tracepoint is guaranteed to be
// called from the waking context.
//
// Tracepoint called when the task is actually woken; p->state == TASK_RUNNING.
// It is not always called from the waking context.
//
// Tracepoint for waking up a new task:
//

//
// Preemption ignores task state, therefore preempted tasks are always
// RUNNING (we will not have dequeued if state != RUNNING).
//
// task_state_index() uses fls() and returns a value from 0-8 range.
// Decrement it by 1 (except TASK_RUNNING state i.e 0) before using
// it for left shift operation to get the correct task->state
// mapping.
//

//
// Tracepoint for task switches, performed by the scheduler:
//
// XXX SCHED_DEADLINE
//
// Tracepoint for a task being migrated:
//
// Tracepoint for freeing a task:
//
// Tracepoint for a task exiting.
// Note, it's a superset of sched_process_template and should be kept
// compatible as much as possible. sched_process_exits has an extra
// `group_dead` argument, so sched_process_template can't be used,
// unfortunately, just like sched_migrate_task above.
//
// Tracepoint for waiting on task to unschedule:
//
// Tracepoint for a waiting task:
//
// Tracepoint for kernel_clone:
//
// Tracepoint for exec:
//
// sched_prepare_exec - called before setting up new exec
// @task:	pointer to the current task
// @bprm:	pointer to linux_binprm used for new exec
//
// Called before flushing the old exec, where @task is still unchanged, but at
// the point of no return during switching to the new exec. At the point it is
// called the exec will either succeed, or on failure terminate the task. Also
// see the "sched_process_exec" tracepoint, which is called right after @task
// has successfully switched to the new exec.
//

//
// XXX the below sched_stat tracepoints only apply to SCHED_OTHER/BATCH/IDLE
// adding sched_stat support to SCHED_FIFO/RR would be welcome.
//
// Tracepoint for accounting wait time (time the task is runnable
// but not actually running due to scheduler contention).
//
// Tracepoint for accounting sleep time (time the task is not runnable,
// including iowait, see below).
//
// Tracepoint for accounting iowait time (time the task is not runnable
// due to waiting on IO to complete).
//
// Tracepoint for accounting blocked time (time the task is in uninterruptible).
//
// Tracepoint for accounting runtime (time the task is executing
// on a CPU).
//
// Tracepoint for showing priority inheritance modifying a tasks
// priority.
//
// XXX SCHED_DEADLINE bits missing

//
// Tracks migration of tasks from one runqueue to another. Can be used to
// detect if automatic NUMA balancing is bouncing between nodes.
//

// Redefine for export.

// Redefine for symbolic printing.

//
// Tracepoint for waking a polling cpu without an IPI.
//
// Following tracepoints are not exported in tracefs and provide hooking
// mechanisms only for testing and debugging purposes.
//
pub const DL_OTHER: c_int = 0;
pub const DL_TASK: c_int = 1;
pub const DL_SERVER_FAIR: c_int = 2;
pub const DL_SERVER_EXT: c_int = 3;
// Call to update_curr_dl_se not involving throttle or replenish

// This part must be outside protection
