//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap2/omapfb/dss/dispc.h
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
// linux/drivers/video/omap2/dss/dispc.h
//
// Copyright (C) 2011 Texas Instruments
// Author: Archit Taneja <archit@ti.com>
//
// DISPC common registers
pub const DISPC_REVISION: c_uint = 0x0000;
pub const DISPC_SYSCONFIG: c_uint = 0x0010;
pub const DISPC_SYSSTATUS: c_uint = 0x0014;
pub const DISPC_IRQSTATUS: c_uint = 0x0018;
pub const DISPC_IRQENABLE: c_uint = 0x001C;
pub const DISPC_CONTROL: c_uint = 0x0040;
pub const DISPC_CONFIG: c_uint = 0x0044;
pub const DISPC_CAPABLE: c_uint = 0x0048;
pub const DISPC_LINE_STATUS: c_uint = 0x005C;
pub const DISPC_LINE_NUMBER: c_uint = 0x0060;
pub const DISPC_GLOBAL_ALPHA: c_uint = 0x0074;
pub const DISPC_CONTROL2: c_uint = 0x0238;
pub const DISPC_CONFIG2: c_uint = 0x0620;
pub const DISPC_DIVISOR: c_uint = 0x0804;
pub const DISPC_GLOBAL_BUFFER: c_uint = 0x0800;
pub const DISPC_CONTROL3: c_uint = 0x0848;
pub const DISPC_CONFIG3: c_uint = 0x084C;
pub const DISPC_MSTANDBY_CTRL: c_uint = 0x0858;
pub const DISPC_GLOBAL_MFLAG_ATTRIBUTE: c_uint = 0x085C;
// DISPC overlay registers

// DISPC up/downsampling FIR filter coefficient structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dispc_coef {
    pub hc4_vc22: i8,
    pub hc3_vc2: i8,
    pub hc2_vc1: u8,
    pub hc1_vc0: i8,
    pub hc0_vc00: i8,
}

// DISPC manager/channel specific registers
// Named as DISPC_SIZE_LCD, DISPC_SIZE_DIGIT and DISPC_SIZE_LCD2 in TRM
// DISPC overlay register base addresses
// DISPC overlay register offsets
// coef index i = {0, 1, 2, 3, 4, 5, 6, 7}
// coef index i = {0, 1, 2, 3, 4,}
// coef index i = {0, 1, 2, 3, 4, 5, 6, 7}
