//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/phy_companion.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// phy-companion.h -- phy companion to indicate the comparator part of PHY
//
// Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
//
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

// phy_companion to take care of VBUS, ID and srp capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_companion {
// effective for A-peripheral, ignored for B devices
    pub enabled): *mut *mut *mut int (set_vbus)(struct phy_companion x, bool,
// for B devices only:  start session with A-Host
    pub x): *mut *mut int (start_srp)(struct phy_companion,
}
