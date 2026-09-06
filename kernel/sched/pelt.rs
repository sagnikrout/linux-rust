//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/pelt.h
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

extern "C" {
    pub fn __update_load_avg_blocked_se(now: u64, se: *mut sched_entity) -> c_int;
}
extern "C" {
    pub fn __update_load_avg_se(now: u64, cfs_rq: *mut cfs_rq, se: *mut sched_entity) -> c_int;
}
extern "C" {
    pub fn __update_load_avg_cfs_rq(now: u64, cfs_rq: *mut cfs_rq) -> c_int;
}
extern "C" {
    pub fn update_rt_rq_load_avg(now: u64, rq: *mut rq, running: c_int) -> c_int;
}
extern "C" {
    pub fn update_dl_rq_load_avg(now: u64, rq: *mut rq, running: c_int) -> c_int;
}
extern "C" {
    pub fn update_other_load_avgs(rq: *mut rq) -> bool;
}

extern "C" {
    pub fn update_hw_load_avg(now: u64, rq: *mut rq, capacity: u64) -> c_int;
}
extern "C" {
    pub fn READ_ONCE(_arg: rq->avg_hw.load_avg) -> return;
}

extern "C" {
    pub fn update_irq_load_avg(rq: *mut rq, running: u64) -> c_int;
}

// Avoid store if the flag has been already reset
// Reset flag to report util_avg has been updated
// The rq is idle, we can sync to clock_task
// Paired with smp_rmb in migrate_se_pelt_lag()
//
// The clock_pelt scales the time to reflect the effective amount of
// computation done during the running delta time but then sync back to
// clock_task when rq is idle.
//
// absolute time   | 1| 2| 3| 4| 5| 6| 7| 8| 9|10|11|12|13|14|15|16
// @ max capacity  ------******---------------******---------------
// @ half capacity ------************---------************---------
// clock pelt      | 1| 2|    3|    4| 7| 8| 9|   10|   11|14|15|16
//
// When a rq runs at a lower compute capacity, it will need
// more time to do the same amount of work than at max
// capacity. In order to be invariant, we scale the delta to
// reflect how much work has been really done.
// Running longer results in stealing idle time that will
// disturb the load signal compared to max capacity. This
// stolen idle time will be automatically reflected when the
// rq will be idle and the clock will be synced with
// rq_clock_task.
//
// Scale the elapsed time to reflect the real amount of
// computation
//
// When rq becomes idle, we have to check if it has lost idle time
// because it was fully busy. A rq is fully used when the /Sum util_sum
// is greater or equal to:
// (LOAD_AVG_MAX - 1024 + rq->cfs.avg.period_contrib) << SCHED_CAPACITY_SHIFT;
// For optimization and computing rounding purpose, we don't take into account
// the position in the current window (period_contrib) and we use the higher
// bound of util_sum to decide.
//
// Reflecting stolen time makes sense only if the idle
// phase would be present at max capacity. As soon as the
// utilization of a rq has reached the maximum value, it is
// considered as an always running rq without idle time to
// steal. This potential idle time is considered as lost in
// this case. We keep track of this lost idle time compare to
// rq's clock_task.
//

// rq->task_clock normalized against any time this cfs_rq has spent throttled

extern "C" {
    pub fn rq_clock_pelt(_arg: rq_of(cfs_rq)) -> return;
}

