//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/s3fwrn5/phy_common.h
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
// Link Layer for Samsung S3FWRN5 NCI based Driver
//
// Copyright (C) 2015 Samsung Electronics
// Robert Baldyga <r.baldyga@samsung.com>
// Copyright (C) 2020 Samsung Electronics
// Bongsu Jeon <bongsu.jeon@samsung.com>
//

pub const S3FWRN5_EN_WAIT_TIME: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_common {
    pub ndev: *mut nci_dev,
    pub gpio_en: *mut gpio_desc,
    pub gpio_fw_wake: *mut gpio_desc,
    pub mutex: mutex,
    pub mode: s3fwrn5_mode,
}

extern "C" {
    pub fn s3fwrn5_phy_set_wake(phy_id: *mut c_void, wake: bool);
}
extern "C" {
    pub fn s3fwrn5_phy_power_ctrl(phy: *mut phy_common, mode: s3fwrn5_mode) -> bool;
}
extern "C" {
    pub fn s3fwrn5_phy_set_mode(phy_id: *mut c_void, mode: s3fwrn5_mode);
}
extern "C" {
    pub fn s3fwrn5_phy_get_mode(phy_id: *mut c_void) -> s3fwrn5_mode;
}
