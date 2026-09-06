//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mv_usb.h
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
// Copyright (C) 2011 Marvell International Ltd. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_usb_addon_irq {
    pub irq: c_uint,
    pub (*poll)(void): *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_usb_platform_data {
    pub change*/: *mut *mut *mut mv_usb_addon_irq id; / Only valid for OTG. ID pin,
    pub change*/: *mut *mut *mut mv_usb_addon_irq vbus; / valid for OTG/UDC. VBUS,
// only valid for HCD. OTG or Host only
    pub mode: c_uint,
// This flag is used for that needs id pin checked by otg
    pub disable_otg_clock_gating:1: c_uint,
// Force a_bus_req to be asserted
    pub otg_force_a_bus_req:1: c_uint,
    pub regbase): *mut *mut int (phy_init)(void __iomem,
    pub regbase): *mut *mut void (phy_deinit)(void __iomem,
    pub vbus): *mut *mut int (set_vbus)(unsigned int,
}
