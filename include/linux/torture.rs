//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/torture.h
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
// Common functions for in-kernel torture tests.
//
// Copyright IBM Corporation, 2014
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
//

// Definitions for a non-string torture-test module parameter.

extern "C" {
    pub fn verbose_torout_sleep();
}

// Definitions for online/offline exerciser.

extern "C" {
    pub fn torture_num_online_cpus() -> c_int;
}

extern "C" {
    pub fn torture_ofl_func() -> typedef void;
}
extern "C" {
    pub fn torture_onoff_init(ooholdoff: c_long, oointerval: c_long, f: *mut torture_ofl_func) -> c_int;
}
extern "C" {
    pub fn torture_onoff_stats();
}
extern "C" {
    pub fn torture_onoff_failures() -> bool;
}
// Low-rider random number generator.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct torture_random_state {
    pub trs_state: c_ulong,
    pub trs_count: c_long,
}

extern "C" {
    pub fn torture_random(trsp: *mut torture_random_state) -> c_ulong;
}
// Definitions for high-resolution-timer sleeps.
extern "C" {
    pub fn torture_hrtimeout_us(baset_us: u32, fuzzt_ns: u32, trsp: *mut torture_random_state) -> c_int;
}
extern "C" {
    pub fn torture_hrtimeout_ms(baset_ms: u32, fuzzt_us: u32, trsp: *mut torture_random_state) -> c_int;
}
extern "C" {
    pub fn torture_hrtimeout_jiffies(baset_j: u32, trsp: *mut torture_random_state) -> c_int;
}
extern "C" {
    pub fn torture_hrtimeout_s(baset_s: u32, fuzzt_ms: u32, trsp: *mut torture_random_state) -> c_int;
}
// Task shuffler, which causes CPUs to occasionally go idle.
extern "C" {
    pub fn torture_shuffle_task_register(tp: *mut task_struct);
}
extern "C" {
    pub fn torture_shuffle_init(shuffint: c_long) -> c_int;
}
// Test auto-shutdown handling.
extern "C" {
    pub fn torture_shutdown_absorb(title: *const c_char);
}
extern "C" {
    pub fn torture_shutdown_init(ssecs: c_int, (*cleanup)(void): *mut c_void) -> c_int;
}
// Task stuttering, which forces load/no-load transitions.
extern "C" {
    pub fn stutter_wait(title: *const c_char) -> bool;
}
extern "C" {
    pub fn torture_stutter_init(s: c_int, sgap: c_int) -> c_int;
}
// Initialization and cleanup.
extern "C" {
    pub fn torture_init_begin(ttype: *mut c_char, v: c_int) -> bool;
}
extern "C" {
    pub fn torture_init_end();
}
extern "C" {
    pub fn get_torture_init_jiffies() -> c_ulong;
}
extern "C" {
    pub fn torture_cleanup_begin() -> bool;
}
extern "C" {
    pub fn torture_cleanup_end();
}
extern "C" {
    pub fn torture_must_stop() -> bool;
}
extern "C" {
    pub fn torture_must_stop_irq() -> bool;
}
extern "C" {
    pub fn torture_kthread_stopping(title: *mut c_char);
}
extern "C" {
    pub fn _torture_stop_kthread(m: *mut c_char, tp: *mut task_struct);
}

// Scheduler-related definitions.

extern "C" {
    pub fn torture_sched_set_normal(t: *mut task_struct, nice: c_int);
}

extern "C" {
    pub fn torture_sched_setaffinity(pid: pid_t, in_mask: *const cpumask, dowarn: bool) -> c_long;
}

