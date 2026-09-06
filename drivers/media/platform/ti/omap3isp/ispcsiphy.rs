//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/ispcsiphy.h
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
// ispcsiphy.h
//
// TI OMAP3 ISP - CSI PHY module
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc.
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_csiphy {
    pub isp: *mut isp_device,
    pub /: *mut *mut mutex mutex; / serialize csiphy configuration,
    pub csi2: *mut isp_csi2_device,
    pub vdd: *mut regulator,
// the entity that acquired the phy
    pub entity: *mut media_entity,
// mem resources - enums as defined in enum isp_mem_resources
    pub cfg_regs: c_uint,
    pub phy_regs: c_uint,
    pub /: *mut *mut u8 num_data_lanes; / number of CSI2 Data Lanes supported,
}

extern "C" {
    pub fn omap3isp_csiphy_release(phy: *mut isp_csiphy);
}
extern "C" {
    pub fn omap3isp_csiphy_init(isp: *mut isp_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_csiphy_cleanup(isp: *mut isp_device);
}
