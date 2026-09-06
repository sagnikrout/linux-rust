//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ti/omap1-usb.h
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
// Constants in this file are used all over the place, in platform
// code, as well as the udc, phy and ohci drivers.
// This is not a great design, but unlikely to get fixed after
// such a long time. Don't do this elsewhere.
//
pub const OMAP1_OTG_BASE: c_uint = 0xfffb0400;
pub const OMAP1_UDC_BASE: c_uint = 0xfffb4000;
pub const OMAP2_UDC_BASE: c_uint = 0x4805e200;
pub const OMAP2_OTG_BASE: c_uint = 0x4805e300;

//
// OTG and transceiver registers, for OMAPs starting with ARM926
//

// same bits as in IRQ_EN

// -------------------------------------------------------------------------
// OMAP1

