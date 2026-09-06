//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/voltage-omap.h
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
// OMAP Voltage Management Routines
//
// Copyright (C) 2011, Texas Instruments, Inc.
//
// struct omap_volt_data - Omap voltage specific data.
// @volt_nominal:	The possible voltage value in uV
// @sr_efuse_offs:	The offset of the efuse register(from system
// control module base address) from where to read
// the n-target value for the smartreflex module.
// @sr_errminlimit:	Error min limit value for smartreflex. This value
// differs at differnet opp and thus is linked
// with voltage.
// @vp_errgain:		Error gain value for the voltage processor. This
// field also differs according to the voltage/opp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_volt_data {
    pub volt_nominal: u32,
    pub sr_efuse_offs: u32,
    pub sr_errminlimit: u8,
    pub vp_errgain: u8,
}

extern "C" {
    pub fn voltdm_get_voltage(voltdm: *mut voltagedomain) -> c_ulong;
}
