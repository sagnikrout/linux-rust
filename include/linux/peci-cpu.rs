//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/peci-cpu.h
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
// Copyright (c) 2021 Intel Corporation

// Copied from x86 <asm/processor.h>
pub const X86_VENDOR_INTEL: c_int = 0;
// Copied from x86 <asm/cpu_device_id.h>
pub const VFM_MODEL_BIT: c_int = 0;
pub const VFM_FAMILY_BIT: c_int = 8;
pub const VFM_VENDOR_BIT: c_int = 16;
pub const VFM_RSVD_BIT: c_int = 24;

// End of copied code

pub const PECI_PKG_ID_CPU_ID: c_uint = 0x0000  /* CPUID Info */;
pub const PECI_PKG_ID_PLATFORM_ID: c_uint = 0x0001  /* Platform ID */;
pub const PECI_PKG_ID_DEVICE_ID: c_uint = 0x0002  /* Uncore Device ID */;
pub const PECI_PKG_ID_MAX_THREAD_ID: c_uint = 0x0003  /* Max Thread ID */;
pub const PECI_PKG_ID_MICROCODE_REV: c_uint = 0x0004  /* CPU Microcode Update Revision */;
pub const PECI_PKG_ID_MCA_ERROR_LOG: c_uint = 0x0005  /* Machine Check Status */;

extern "C" {
    pub fn peci_temp_read(device: *mut peci_device, temp_raw: *mut i16) -> c_int;
}
