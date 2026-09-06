//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/apm_bios.h
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
// Include file for the interface to an APM BIOS
// Copyright 1994-2001 Stephen Rothwell (sfr@canb.auug.org.au)
//

// Results of APM Installation Check
pub const APM_16_BIT_SUPPORT: c_uint = 0x0001;
pub const APM_32_BIT_SUPPORT: c_uint = 0x0002;
pub const APM_IDLE_SLOWS_CLOCK: c_uint = 0x0004;
pub const APM_BIOS_DISABLED: c_uint = 0x0008;
pub const APM_BIOS_DISENGAGED: c_uint = 0x0010;
//
// Data for APM that is persistent across module unload/load
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_info {
    pub bios: apm_bios_info,
    pub connection_version: c_ushort,
    pub get_power_status_broken: c_int,
    pub get_power_status_swabinminutes: c_int,
    pub allow_ints: c_int,
    pub forbid_idle: c_int,
    pub realmode_power_off: c_int,
    pub disabled: c_int,
}

//
// The APM function codes
//
pub const APM_FUNC_INST_CHECK: c_uint = 0x5300;
pub const APM_FUNC_REAL_CONN: c_uint = 0x5301;
pub const APM_FUNC_16BIT_CONN: c_uint = 0x5302;
pub const APM_FUNC_32BIT_CONN: c_uint = 0x5303;
pub const APM_FUNC_DISCONN: c_uint = 0x5304;
pub const APM_FUNC_IDLE: c_uint = 0x5305;
pub const APM_FUNC_BUSY: c_uint = 0x5306;
pub const APM_FUNC_SET_STATE: c_uint = 0x5307;
pub const APM_FUNC_ENABLE_PM: c_uint = 0x5308;
pub const APM_FUNC_RESTORE_BIOS: c_uint = 0x5309;
pub const APM_FUNC_GET_STATUS: c_uint = 0x530a;
pub const APM_FUNC_GET_EVENT: c_uint = 0x530b;
pub const APM_FUNC_GET_STATE: c_uint = 0x530c;
pub const APM_FUNC_ENABLE_DEV_PM: c_uint = 0x530d;
pub const APM_FUNC_VERSION: c_uint = 0x530e;
pub const APM_FUNC_ENGAGE_PM: c_uint = 0x530f;
pub const APM_FUNC_GET_CAP: c_uint = 0x5310;
pub const APM_FUNC_RESUME_TIMER: c_uint = 0x5311;
pub const APM_FUNC_RESUME_ON_RING: c_uint = 0x5312;
pub const APM_FUNC_TIMER: c_uint = 0x5313;
//
// Function code for APM_FUNC_RESUME_TIMER
//
pub const APM_FUNC_DISABLE_TIMER: c_int = 0;
pub const APM_FUNC_GET_TIMER: c_int = 1;
pub const APM_FUNC_SET_TIMER: c_int = 2;
//
// Function code for APM_FUNC_RESUME_ON_RING
//
pub const APM_FUNC_DISABLE_RING: c_int = 0;
pub const APM_FUNC_ENABLE_RING: c_int = 1;
pub const APM_FUNC_GET_RING: c_int = 2;
//
// Function code for APM_FUNC_TIMER_STATUS
//
pub const APM_FUNC_TIMER_DISABLE: c_int = 0;
pub const APM_FUNC_TIMER_ENABLE: c_int = 1;
pub const APM_FUNC_TIMER_GET: c_int = 2;
//
// in arch/i386/kernel/setup.c
//
// This is the "All Devices" ID communicated to the BIOS
//

