//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/clock.h
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
// Do not use outside of architecture code which knows its limitations.
//
// sched_clock() has no promise of monotonicity or bounded drift between
// CPUs, use (which you should not) requires disabling IRQs.
//
// Please use one of the three interfaces below.
//
extern "C" {
    pub fn sched_clock() -> u64;
}

extern "C" {
    pub fn sched_clock_noinstr() -> u64;
}

extern "C" {
    pub fn sched_clock() -> return;
}

//
// See the comment in kernel/sched/clock.c
//
extern "C" {
    pub fn running_clock() -> u64;
}
extern "C" {
    pub fn sched_clock_cpu(cpu: c_int) -> u64;
}
extern "C" {
    pub fn sched_clock_init();
}

extern "C" {
    pub fn sched_clock() -> return;
}
extern "C" {
    pub fn sched_clock_noinstr() -> return;
}
extern "C" {
    pub fn sched_clock() -> return;
}

extern "C" {
    pub fn sched_clock_stable() -> c_int;
}
extern "C" {
    pub fn clear_sched_clock_stable();
}
//
// When sched_clock_stable(), __sched_clock_offset provides the offset
// between local_clock() and sched_clock().
//
extern "C" {
    pub fn sched_clock_tick();
}
extern "C" {
    pub fn sched_clock_tick_stable();
}
extern "C" {
    pub fn sched_clock_idle_sleep_event();
}
extern "C" {
    pub fn sched_clock_idle_wakeup_event();
}
//
// As outlined in clock.c, provides a fast, high resolution, nanosecond
// time source that is monotonic per cpu argument and has bounded drift
// between cpus.
//
// ######################### BIG FAT WARNING ##########################
// # when comparing cpu_clock(i) to cpu_clock(j) for i != j, time can #
// # go backwards !!                                                  #
// ####################################################################
//
extern "C" {
    pub fn sched_clock_cpu(_arg: cpu) -> return;
}
extern "C" {
    pub fn local_clock_noinstr() -> u64;
}
extern "C" {
    pub fn local_clock() -> u64;
}

//
// An i/f to runtime opt-in for irq time accounting based off of sched_clock.
// The reason for this explicit opt-in is not to have perf penalty with
// slow sched_clocks.
//
extern "C" {
    pub fn enable_sched_clock_irqtime();
}
extern "C" {
    pub fn disable_sched_clock_irqtime();
}

