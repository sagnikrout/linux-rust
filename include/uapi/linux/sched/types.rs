//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sched/types.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// Extended scheduling parameters data structure.
//
// This is needed because the original struct sched_param can not be
// altered without introducing ABI issues with legacy applications
// (e.g., in sched_getparam()).
//
// However, the possibility of specifying more than just a priority for
// the tasks may be useful for a wide variety of application fields, e.g.,
// multimedia, streaming, automation and control, and many others.
//
// This variant (sched_attr) allows to define additional attributes to
// improve the scheduler knowledge about task requirements.
//
// Scheduling Class Attributes
// ===========================
//
// A subset of sched_attr attributes specifies the
// scheduling policy and relative POSIX attributes:
//
// @size		size of the structure, for fwd/bwd compat.
//
// @sched_policy	task's scheduling policy
// @sched_nice		task's nice value      (SCHED_NORMAL/BATCH)
// @sched_priority	task's static priority (SCHED_FIFO/RR)
//
// Certain more advanced scheduling features can be controlled by a
// predefined set of flags via the attribute:
//
// @sched_flags	for customizing the scheduler behaviour
//
// Sporadic Time-Constrained Task Attributes
// =========================================
//
// A subset of sched_attr attributes allows to describe a so-called
// sporadic time-constrained task.
//
// In such a model a task is specified by:
// - the activation period or minimum instance inter-arrival time;
// - the maximum (or average, depending on the actual scheduling
// discipline) computation time of all instances, a.k.a. runtime;
// - the deadline (relative to the actual activation time) of each
// instance.
// Very briefly, a periodic (sporadic) task asks for the execution of
// some specific computation --which is typically called an instance--
// (at most) every period. Moreover, each instance typically lasts no more
// than the runtime and must be completed by time instant t equal to
// the instance activation time + the deadline.
//
// This is reflected by the following fields of the sched_attr structure:
//
// @sched_deadline	representative of the task's deadline in nanoseconds
// @sched_runtime	representative of the task's runtime in nanoseconds
// @sched_period	representative of the task's period in nanoseconds
//
// Given this task model, there are a multiplicity of scheduling algorithms
// and policies, that can be used to ensure all the tasks will make their
// timing constraints.
//
// As of now, the SCHED_DEADLINE policy (sched_dl scheduling class) is the
// only user of this new interface. More information about the algorithm
// available in the scheduling class file or in Documentation/.
//
// Task Utilization Attributes
// ===========================
//
// A subset of sched_attr attributes allows to specify the utilization
// expected for a task. These attributes allow to inform the scheduler about
// the utilization boundaries within which it should schedule the task. These
// boundaries are valuable hints to support scheduler decisions on both task
// placement and frequency selection.
//
// @sched_util_min	represents the minimum utilization
// @sched_util_max	represents the maximum utilization
//
// Utilization is a value in the range [0..SCHED_CAPACITY_SCALE]. It
// represents the percentage of CPU time used by a task when running at the
// maximum frequency on the highest capacity CPU of the system. For example, a
// 20% utilization task is a task running for 2ms every 10ms at maximum
// frequency.
//
// A task with a min utilization value bigger than 0 is more likely scheduled
// on a CPU with a capacity big enough to fit the specified value.
// A task with a max utilization value smaller than 1024 is more likely
// scheduled on a CPU with no more capacity than the specified value.
//
// A task utilization boundary can be reset by setting the attribute to -1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_attr {
    pub size: __u32,
    pub sched_policy: __u32,
    pub sched_flags: __u64,
// SCHED_NORMAL, SCHED_BATCH
    pub sched_nice: __s32,
// SCHED_FIFO, SCHED_RR
    pub sched_priority: __u32,
// SCHED_DEADLINE
    pub sched_runtime: __u64,
    pub sched_deadline: __u64,
    pub sched_period: __u64,
// Utilization hints
    pub sched_util_min: __u32,
    pub sched_util_max: __u32,
}
