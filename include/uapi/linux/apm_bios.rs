//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/apm_bios.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Include file for the interface to an APM BIOS
// Copyright 1994-2001 Stephen Rothwell (sfr@canb.auug.org.au)
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//

pub type apm_event_t = c_ushort;
pub type apm_eventinfo_t = c_ushort;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_bios_info {
    pub version: __u16,
    pub cseg: __u16,
    pub offset: __u32,
    pub cseg_16: __u16,
    pub dseg: __u16,
    pub flags: __u16,
    pub cseg_len: __u16,
    pub cseg_16_len: __u16,
    pub dseg_len: __u16,
}

//
// Power states
//
pub const APM_STATE_READY: c_uint = 0x0000;
pub const APM_STATE_STANDBY: c_uint = 0x0001;
pub const APM_STATE_SUSPEND: c_uint = 0x0002;
pub const APM_STATE_OFF: c_uint = 0x0003;
pub const APM_STATE_BUSY: c_uint = 0x0004;
pub const APM_STATE_REJECT: c_uint = 0x0005;
pub const APM_STATE_OEM_SYS: c_uint = 0x0020;
pub const APM_STATE_OEM_DEV: c_uint = 0x0040;
pub const APM_STATE_DISABLE: c_uint = 0x0000;
pub const APM_STATE_ENABLE: c_uint = 0x0001;
pub const APM_STATE_DISENGAGE: c_uint = 0x0000;
pub const APM_STATE_ENGAGE: c_uint = 0x0001;
//
// Events (results of Get PM Event)
//
pub const APM_SYS_STANDBY: c_uint = 0x0001;
pub const APM_SYS_SUSPEND: c_uint = 0x0002;
pub const APM_NORMAL_RESUME: c_uint = 0x0003;
pub const APM_CRITICAL_RESUME: c_uint = 0x0004;
pub const APM_LOW_BATTERY: c_uint = 0x0005;
pub const APM_POWER_STATUS_CHANGE: c_uint = 0x0006;
pub const APM_UPDATE_TIME: c_uint = 0x0007;
pub const APM_CRITICAL_SUSPEND: c_uint = 0x0008;
pub const APM_USER_STANDBY: c_uint = 0x0009;
pub const APM_USER_SUSPEND: c_uint = 0x000a;
pub const APM_STANDBY_RESUME: c_uint = 0x000b;
pub const APM_CAPABILITY_CHANGE: c_uint = 0x000c;
pub const APM_USER_HIBERNATION: c_uint = 0x000d;
pub const APM_HIBERNATION_RESUME: c_uint = 0x000e;
//
// Error codes
//
pub const APM_SUCCESS: c_uint = 0x00;
pub const APM_DISABLED: c_uint = 0x01;
pub const APM_CONNECTED: c_uint = 0x02;
pub const APM_NOT_CONNECTED: c_uint = 0x03;
pub const APM_16_CONNECTED: c_uint = 0x05;
pub const APM_16_UNSUPPORTED: c_uint = 0x06;
pub const APM_32_CONNECTED: c_uint = 0x07;
pub const APM_32_UNSUPPORTED: c_uint = 0x08;
pub const APM_BAD_DEVICE: c_uint = 0x09;
pub const APM_BAD_PARAM: c_uint = 0x0a;
pub const APM_NOT_ENGAGED: c_uint = 0x0b;
pub const APM_BAD_FUNCTION: c_uint = 0x0c;
pub const APM_RESUME_DISABLED: c_uint = 0x0d;
pub const APM_NO_ERROR: c_uint = 0x53;
pub const APM_BAD_STATE: c_uint = 0x60;
pub const APM_NO_EVENTS: c_uint = 0x80;
pub const APM_NOT_PRESENT: c_uint = 0x86;
//
// APM Device IDs
//
pub const APM_DEVICE_BIOS: c_uint = 0x0000;
pub const APM_DEVICE_ALL: c_uint = 0x0001;
pub const APM_DEVICE_DISPLAY: c_uint = 0x0100;
pub const APM_DEVICE_STORAGE: c_uint = 0x0200;
pub const APM_DEVICE_PARALLEL: c_uint = 0x0300;
pub const APM_DEVICE_SERIAL: c_uint = 0x0400;
pub const APM_DEVICE_NETWORK: c_uint = 0x0500;
pub const APM_DEVICE_PCMCIA: c_uint = 0x0600;
pub const APM_DEVICE_BATTERY: c_uint = 0x8000;
pub const APM_DEVICE_OEM: c_uint = 0xe000;
pub const APM_DEVICE_OLD_ALL: c_uint = 0xffff;
pub const APM_DEVICE_CLASS: c_uint = 0x00ff;
pub const APM_DEVICE_MASK: c_uint = 0xff00;
//
// Battery status
//
pub const APM_MAX_BATTERIES: c_int = 2;
//
// APM defined capability bit flags
//
pub const APM_CAP_GLOBAL_STANDBY: c_uint = 0x0001;
pub const APM_CAP_GLOBAL_SUSPEND: c_uint = 0x0002;
pub const APM_CAP_RESUME_STANDBY_TIMER: c_uint = 0x0004 /* Timer resume from standby */;
pub const APM_CAP_RESUME_SUSPEND_TIMER: c_uint = 0x0008 /* Timer resume from suspend */;
pub const APM_CAP_RESUME_STANDBY_RING: c_uint = 0x0010 /* Resume on Ring fr standby */;
pub const APM_CAP_RESUME_SUSPEND_RING: c_uint = 0x0020 /* Resume on Ring fr suspend */;
pub const APM_CAP_RESUME_STANDBY_PCMCIA: c_uint = 0x0040 /* Resume on PCMCIA Ring	*/;
pub const APM_CAP_RESUME_SUSPEND_PCMCIA: c_uint = 0x0080 /* Resume on PCMCIA Ring	*/;
//
// ioctl operations
//

