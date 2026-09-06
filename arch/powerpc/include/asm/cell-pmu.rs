//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cell-pmu.h
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
// Cell Broadband Engine Performance Monitor
//
// (C) Copyright IBM Corporation 2006
//
// Author:
// David Erb (djerb@us.ibm.com)
// Kevin Corry (kevcorry@us.ibm.com)
//
// The Cell PMU has four hardware performance counters, which can be
// configured as four 32-bit counters or eight 16-bit counters.
//
pub const NR_PHYS_CTRS: c_int = 4;

// Macros for the pm_control register.

// Macros for the trace_address register.
pub const CBE_PM_TRACE_BUF_EMPTY: c_uint = 0x00000400;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_reg_name {
    group_control,
    debug_bus_control,
    trace_address,
    ext_tr_timer,
    pm_status,
    pm_control,
    pm_interval,
    pm_start_stop,
}
