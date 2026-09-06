//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/s390-cpumcf-kernel.h
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
// Support for s390 CPU measurement counter set diagnostic facility
//
// Copyright IBM Corp. 2019
// Thomas Richter <tmricht@linux.ibm.com>
//
pub const S390_CPUMCF_DIAG_DEF: c_uint = 0xfeef	/* Counter diagnostic entry ID */;
pub const PERF_EVENT_CPUM_CF_DIAG: c_uint = 0xBC000	/* Event: Counter sets */;
pub const PERF_EVENT_CPUM_SF_DIAG: c_uint = 0xBD000 /* Event: Combined-sampling */;
pub const PERF_EVENT_PAI_CRYPTO_ALL: c_uint = 0x1000 /* Event: CRYPTO_ALL */;
pub const PERF_EVENT_PAI_NNPA_ALL: c_uint = 0x1800 /* Event: NNPA_ALL */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cf_ctrset_entry {
    pub /: *mut *mut unsigned int def:16; / 0-15 Data Entry Format,
    pub /: *mut *mut unsigned int set:16; / 16-23 Counter set identifier,
    pub /: *mut *mut unsigned int ctr:16; / 24-39 Number of stored counters,
    pub /: *mut *mut unsigned int res1:16; / 40-63 Reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cf_trailer_entry {
// 0 - 7
    pub /: *mut *mut unsigned int clock_base:1; / TOD clock base,
    pub /: *mut *mut unsigned int speed:1; / CPU speed,
// Measurement alerts
    pub /: *mut *mut unsigned int mtda:1; / Loss of MT ctr. data alert,
    pub /: *mut *mut unsigned int caca:1; / Counter auth. change alert,
    pub /: *mut *mut unsigned int lcda:1; / Loss of counter data alert,
}

// 8 - 15
// 16 - 23
// 24 - 55
// 56 - 63

