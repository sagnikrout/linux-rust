//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lochnagar.h
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
// Lochnagar internals
//
// Copyright (c) 2013-2018 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// Author: Charles Keepax <ckeepax@opensource.cirrus.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lochnagar_type {
    LOCHNAGAR1,
    LOCHNAGAR2,
}

//
// struct lochnagar - Core data for the Lochnagar audio board driver.
//
// @type: The type of Lochnagar device connected.
// @dev: A pointer to the struct device for the main MFD.
// @regmap: The devices main register map.
// @analogue_config_lock: Lock used to protect updates in the analogue
// configuration as these must not be changed whilst the hardware is processing
// the last update.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lochnagar {
    pub type: lochnagar_type,
    pub dev: *mut device,
    pub regmap: *mut regmap,
// Lock to protect updates to the analogue configuration
    pub analogue_config_lock: mutex,
}

// Register Addresses
pub const LOCHNAGAR_SOFTWARE_RESET: c_uint = 0x00;
pub const LOCHNAGAR_FIRMWARE_ID1: c_uint = 0x01;
pub const LOCHNAGAR_FIRMWARE_ID2: c_uint = 0x02;
// (0x0000)  Software Reset
pub const LOCHNAGAR_DEVICE_ID_MASK: c_uint = 0xFFFC;
pub const LOCHNAGAR_DEVICE_ID_SHIFT: c_int = 2;
pub const LOCHNAGAR_REV_ID_MASK: c_uint = 0x0003;
pub const LOCHNAGAR_REV_ID_SHIFT: c_int = 0;
extern "C" {
    pub fn lochnagar_update_config(lochnagar: *mut lochnagar) -> c_int;
}
