//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/pmu/ebb/ebb.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2014, Michael Ellerman, IBM Corp.
//

pub const NUM_PMC_VALUES: c_int = 128;
pub const COUNTER_OVERFLOW: c_uint = 0x80000000ull;
extern "C" {
    pub fn ebb_check_count(pmc: c_int, sample_period: u64, fudge: c_int) -> bool;
}
extern "C" {
    pub fn event_leader_ebb_init(e: *mut event);
}
extern "C" {
    pub fn event_ebb_init(e: *mut event);
}
extern "C" {
    pub fn event_bhrb_init(e: *mut event, ifm: unsigned);
}
extern "C" {
    pub fn setup_ebb_handler((*callee)(void): *mut c_void);
}
extern "C" {
    pub fn standard_ebb_callee();
}
extern "C" {
    pub fn ebb_event_enable(e: *mut event) -> c_int;
}
extern "C" {
    pub fn ebb_global_enable();
}
extern "C" {
    pub fn ebb_global_disable();
}
extern "C" {
    pub fn ebb_is_supported() -> bool;
}
extern "C" {
    pub fn ebb_freeze_pmcs();
}
extern "C" {
    pub fn ebb_unfreeze_pmcs();
}
extern "C" {
    pub fn count_pmc(pmc: c_int, sample_period: u32) -> c_int;
}
extern "C" {
    pub fn dump_ebb_state();
}
extern "C" {
    pub fn dump_summary_ebb_state();
}
extern "C" {
    pub fn dump_ebb_hw_state();
}
extern "C" {
    pub fn clear_ebb_stats();
}
extern "C" {
    pub fn write_pmc(pmc: c_int, value: u64);
}
extern "C" {
    pub fn read_pmc(pmc: c_int) -> u64;
}
extern "C" {
    pub fn reset_ebb_with_clear_mask(mmcr0_clear_mask: c_ulong);
}
extern "C" {
    pub fn reset_ebb();
}
extern "C" {
    pub fn ebb_check_mmcr0() -> c_int;
}
extern "C" {
    pub fn core_busy_loop() -> c_int;
}
extern "C" {
    pub fn ebb_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
}
extern "C" {
    pub fn catch_sigill((*func)(void): *mut c_void) -> c_int;
}
extern "C" {
    pub fn write_pmc1();
}
