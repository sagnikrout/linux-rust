//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/isph3a.h
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
// isph3a.h
//
// TI OMAP3 ISP - H3A AF module
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc.
//
// Contacts: David Cohen <dacohen@gmail.com>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

//
// ----------
// -H3A AEWB-
// ----------
//
pub const AEWB_PACKET_SIZE: c_int = 16;
pub const AEWB_SATURATION_LIMIT: c_uint = 0x3ff;
// Flags for changed registers

// ISPH3A REGISTERS bits

//
// --------
// -H3A AF-
// --------
//
// Peripheral Revision
pub const AFPID: c_uint = 0x0;
pub const AFCOEF_OFFSET: c_uint = 0x00000004	/* COEF base address */;
// PCR fields

// AFPAX1 fields

pub const AF_PAXH: c_uint = 0x7F;
// AFPAX2 fields

pub const AF_PAXHC: c_uint = 0x3F;
// AFPAXSTART fields

pub const AF_PAXSV: c_uint = 0xFFF;
// COEFFICIENT MASK
pub const AF_COEF_MASK0: c_uint = 0xFFF;

// BIT SHIFTS
pub const AF_RGBPOS_SHIFT: c_int = 11;
pub const AF_MED_TH_SHIFT: c_int = 3;
pub const AF_PAXW_SHIFT: c_int = 16;
pub const AF_LINE_INCR_SHIFT: c_int = 13;
pub const AF_VT_COUNT_SHIFT: c_int = 6;
pub const AF_HZ_START_SHIFT: c_int = 16;
pub const AF_COEF_SHIFT: c_int = 16;
// Init and cleanup functions
extern "C" {
    pub fn omap3isp_h3a_aewb_init(isp: *mut isp_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_h3a_af_init(isp: *mut isp_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_h3a_aewb_cleanup(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_h3a_af_cleanup(isp: *mut isp_device);
}
