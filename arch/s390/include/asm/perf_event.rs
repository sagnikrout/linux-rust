//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/perf_event.h
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
// Performance event support - s390 specific definitions.
//
// Copyright IBM Corp. 2009, 2017
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
// Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
//

// Per-CPU flags for PMU states
pub const PMU_F_RESERVED: c_uint = 0x1000;
pub const PMU_F_ENABLED: c_uint = 0x2000;
pub const PMU_F_IN_USE: c_uint = 0x4000;
pub const PMU_F_ERR_IBE: c_uint = 0x0100;
pub const PMU_F_ERR_LSDA: c_uint = 0x0200;

// Perf definitions for PMU event attributes in sysfs

// Perf callbacks
extern "C" {
    pub fn perf_arch_instruction_pointer(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn perf_arch_misc_flags(regs: *mut pt_regs) -> c_ulong;
}

// Perf pt_regs extension for sample-data-entry indicators
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_sf_sde_regs {
    pub /: *mut *mut unsigned char in_guest:1; / guest sample,
    pub /: *mut *mut unsigned long reserved:63; / reserved,
}

