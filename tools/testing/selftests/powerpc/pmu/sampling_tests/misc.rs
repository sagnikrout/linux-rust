//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/pmu/sampling_tests/misc.h
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
// Copyright 2022, Athira Rajeev, IBM Corp.
// Copyright 2022, Madhavan Srinivasan, IBM Corp.
// Copyright 2022, Kajol Jain, IBM Corp.
//

pub const POWER11: c_uint = 0x82;
pub const POWER10: c_uint = 0x80;
pub const POWER9: c_uint = 0x4e;
pub const PERF_POWER9_MASK: c_uint = 0x7f8ffffffffffff;
pub const PERF_POWER10_MASK: c_uint = 0x7ffffffffffffff;

pub const MMCR0_FC56: c_uint = 0x00000010UL /* freeze counters 5 and 6 */;
pub const MMCR0_PMCCEXT: c_uint = 0x00000200UL /* PMCCEXT control */;
pub const MMCR1_RSQ: c_uint = 0x200000000000ULL /* radix scope qual field */;
pub const BHRB_DISABLE: c_uint = 0x2000000000ULL /* MMCRA BHRB DISABLE bit */;
extern "C" {
    pub fn check_pvr_for_sampling_tests() -> c_int;
}
extern "C" {
    pub fn platform_check_for_tests() -> c_int;
}
extern "C" {
    pub fn check_extended_regs_support() -> c_int;
}
extern "C" {
    pub fn perf_get_platform_reg_mask() -> u64;
}
//
// Event code field extraction macro.
// Raw event code is combination of multiple
// fields. Macro to extract individual fields
//
// x - Raw event code value
// y - Field to extract
//

extern "C" {
    pub fn collect_samples(sample_buff: *mut c_void) -> c_int;
}
extern "C" {
    pub fn get_reg_value(intr_regs: *mut u64, register_name: *mut c_char) -> u64;
}
extern "C" {
    pub fn get_thresh_cmp_val(event: event) -> c_int;
}
extern "C" {
    pub fn check_for_generic_compat_pmu() -> bool;
}
extern "C" {
    pub fn check_for_compat_mode() -> bool;
}
