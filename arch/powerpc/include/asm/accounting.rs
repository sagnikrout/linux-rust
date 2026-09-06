//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/accounting.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Common time accounting prototypes and such for all ppc machines.
//
// Stuff for accurate time accounting
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_accounting_data {
// Accumulated cputime values to flush on ticks
    pub utime: c_ulong,
    pub stime: c_ulong,

    pub utime_scaled: c_ulong,
    pub stime_scaled: c_ulong,

    pub gtime: c_ulong,
    pub hardirq_time: c_ulong,
    pub softirq_time: c_ulong,
    pub steal_time: c_ulong,
    pub idle_time: c_ulong,
// Internal counters
    pub /: *mut *mut unsigned long starttime; / TB value snapshot,
    pub /: *mut *mut unsigned long starttime_user; / TB value on exit to usermode,

    pub /: *mut *mut unsigned long startspurr; / SPURR value snapshot,
    pub /: *mut *mut unsigned long utime_sspurr; / ->user_time when ->startspurr set,

}
