//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/kcsan/kcsan.h
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
// The Kernel Concurrency Sanitizer (KCSAN) infrastructure. For more info please
// see Documentation/dev-tools/kcsan.rst.
//
// Copyright (C) 2019, Google LLC.
//

// The number of adjacent watchpoints to check.
pub const KCSAN_CHECK_ADJACENT: c_int = 1;

//
// Globally enable and disable KCSAN.
//
// Save/restore IRQ flags state trace dirtied by KCSAN.
//
extern "C" {
    pub fn kcsan_save_irqtrace(task: *mut task_struct);
}
extern "C" {
    pub fn kcsan_restore_irqtrace(task: *mut task_struct);
}
//
// Statistics counters displayed via debugfs; should only be modified in
// slow-paths.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kcsan_counter_id {
//
// Number of watchpoints currently in use.
//
    KCSAN_COUNTER_USED_WATCHPOINTS,

//
// Total number of watchpoints set up.
//
    KCSAN_COUNTER_SETUP_WATCHPOINTS,

//
// Total number of data races.
//
    KCSAN_COUNTER_DATA_RACES,

//
// Total number of ASSERT failures due to races. If the observed race is
// due to two conflicting ASSERT type accesses, then both will be
// counted.
//
    KCSAN_COUNTER_ASSERT_FAILURES,

//
// Number of times no watchpoints were available.
//
    KCSAN_COUNTER_NO_CAPACITY,

//
// A thread checking a watchpoint raced with another checking thread;
// only one will be reported.
//
    KCSAN_COUNTER_REPORT_RACES,

//
// Observed data value change, but writer thread unknown.
//
    KCSAN_COUNTER_RACES_UNKNOWN_ORIGIN,

//
// The access cannot be encoded to a valid watchpoint.
//
    KCSAN_COUNTER_UNENCODABLE_ACCESSES,

//
// Watchpoint encoding caused a watchpoint to fire on mismatching
// accesses.
//
    KCSAN_COUNTER_ENCODING_FALSE_POSITIVES,

    KCSAN_COUNTER_COUNT, /* number of counters */
}

//
// Returns true if data races in the function symbol that maps to func_addr
// (offsets are ignored) should *not* be reported.
//
extern "C" {
    pub fn kcsan_skip_report_debugfs(func_addr: c_ulong) -> bool;
}
//
// Value-change states.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kcsan_value_change {
//
// Did not observe a value-change, however, it is valid to report the
// race, depending on preferences.
//
    KCSAN_VALUE_CHANGE_MAYBE,

//
// Did not observe a value-change, and it is invalid to report the race.
//
    KCSAN_VALUE_CHANGE_FALSE,

//
// The value was observed to change, and the race should be reported.
//
    KCSAN_VALUE_CHANGE_TRUE,
}

//
// The calling thread hit and consumed a watchpoint: set the access information
// to be consumed by the reporting thread. No report is printed yet.
//
// The calling thread observed that the watchpoint it set up was hit and
// consumed: print the full report based on information set by the racing
// thread.
//
// No other thread was observed to race with the access, but the data value
// before and after the stall differs. Reports a race of "unknown origin".
//
