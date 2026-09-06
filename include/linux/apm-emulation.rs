//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/apm-emulation.h
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
// -*- linux-c -*-
//
// (C) 2003 zecke@handhelds.org
//
// based on arch/arm/kernel/apm.c
// factor out the information needed by architectures to provide
// apm status
//

//
// This structure gets filled in by the machine specific 'get_power_status'
// implementation.  Any fields which are not set default to a safe value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_power_info {
    pub ac_line_status: c_uchar,
pub const APM_AC_OFFLINE: c_int = 0;
pub const APM_AC_ONLINE: c_int = 1;
pub const APM_AC_BACKUP: c_int = 2;
pub const APM_AC_UNKNOWN: c_uint = 0xff;
    pub battery_status: c_uchar,
pub const APM_BATTERY_STATUS_HIGH: c_int = 0;
pub const APM_BATTERY_STATUS_LOW: c_int = 1;
pub const APM_BATTERY_STATUS_CRITICAL: c_int = 2;
pub const APM_BATTERY_STATUS_CHARGING: c_int = 3;
pub const APM_BATTERY_STATUS_NOT_PRESENT: c_int = 4;
pub const APM_BATTERY_STATUS_UNKNOWN: c_uint = 0xff;
    pub battery_flag: c_uchar,

pub const APM_BATTERY_FLAG_UNKNOWN: c_uint = 0xff;
    pub battery_life: c_int,
    pub time: c_int,
    pub units: c_int,
pub const APM_UNITS_MINS: c_int = 0;
pub const APM_UNITS_SECS: c_int = 1;

}

//
// This allows machines to provide their own "apm get power status" function.
//
extern "C" {
    pub fn void(: *mut *mut apm_get_power_status)(struct apm_power_info) -> extern;
}
//
// Queue an event (APM_SYS_SUSPEND or APM_CRITICAL_SUSPEND)
//
extern "C" {
    pub fn apm_queue_event(event: apm_event_t);
}
