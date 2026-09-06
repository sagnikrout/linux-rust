//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/itco_wdt.h
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
// Platform data for the Intel TCO Watchdog
//
// Watchdog resources
pub const ICH_RES_IO_TCO: c_int = 0;
pub const ICH_RES_IO_SMI: c_int = 1;
pub const ICH_RES_MEM_OFF: c_int = 2;
pub const ICH_RES_MEM_GCS_PMC: c_int = 0;
//
// struct itco_wdt_platform_data - iTCO_wdt platform data
// @name: Name of the platform
// @version: iTCO version
// @no_reboot_use_pmc: Use PMC BXT API to set and clear NO_REBOOT bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct itco_wdt_platform_data {
    pub name: [c_char; 32],
    pub version: c_uint,
    pub no_reboot_use_pmc: bool,
}
