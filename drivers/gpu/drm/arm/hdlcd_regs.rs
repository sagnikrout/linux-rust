//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/hdlcd_regs.h
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


//
// Copyright (C) 2013,2014 ARM Limited
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
// ARM HDLCD Controller register definition
//
// register offsets
pub const HDLCD_REG_VERSION: c_uint = 0x0000	/* ro */;
pub const HDLCD_REG_INT_RAWSTAT: c_uint = 0x0010	/* rw */;
pub const HDLCD_REG_INT_CLEAR: c_uint = 0x0014	/* wo */;
pub const HDLCD_REG_INT_MASK: c_uint = 0x0018	/* rw */;
pub const HDLCD_REG_INT_STATUS: c_uint = 0x001c	/* ro */;
pub const HDLCD_REG_FB_BASE: c_uint = 0x0100	/* rw */;
pub const HDLCD_REG_FB_LINE_LENGTH: c_uint = 0x0104	/* rw */;
pub const HDLCD_REG_FB_LINE_COUNT: c_uint = 0x0108	/* rw */;
pub const HDLCD_REG_FB_LINE_PITCH: c_uint = 0x010c	/* rw */;
pub const HDLCD_REG_BUS_OPTIONS: c_uint = 0x0110	/* rw */;
pub const HDLCD_REG_V_SYNC: c_uint = 0x0200	/* rw */;
pub const HDLCD_REG_V_BACK_PORCH: c_uint = 0x0204	/* rw */;
pub const HDLCD_REG_V_DATA: c_uint = 0x0208	/* rw */;
pub const HDLCD_REG_V_FRONT_PORCH: c_uint = 0x020c	/* rw */;
pub const HDLCD_REG_H_SYNC: c_uint = 0x0210	/* rw */;
pub const HDLCD_REG_H_BACK_PORCH: c_uint = 0x0214	/* rw */;
pub const HDLCD_REG_H_DATA: c_uint = 0x0218	/* rw */;
pub const HDLCD_REG_H_FRONT_PORCH: c_uint = 0x021c	/* rw */;
pub const HDLCD_REG_POLARITIES: c_uint = 0x0220	/* rw */;
pub const HDLCD_REG_COMMAND: c_uint = 0x0230	/* rw */;
pub const HDLCD_REG_PIXEL_FORMAT: c_uint = 0x0240	/* rw */;
pub const HDLCD_REG_RED_SELECT: c_uint = 0x0244	/* rw */;
pub const HDLCD_REG_GREEN_SELECT: c_uint = 0x0248	/* rw */;
pub const HDLCD_REG_BLUE_SELECT: c_uint = 0x024c	/* rw */;
// version
pub const HDLCD_PRODUCT_ID: c_uint = 0x1CDC0000;
pub const HDLCD_PRODUCT_MASK: c_uint = 0xFFFF0000;
pub const HDLCD_VERSION_MAJOR_MASK: c_uint = 0x0000FF00;
pub const HDLCD_VERSION_MINOR_MASK: c_uint = 0x000000FF;
// interrupts

// polarities

// commands

// pixel format

// bus options
pub const HDLCD_BUS_BURST_MASK: c_uint = 0x01f;
pub const HDLCD_BUS_MAX_OUTSTAND: c_uint = 0xf00;

// Max resolution supported is 4096x4096, 32bpp
pub const HDLCD_MAX_XRES: c_int = 4096;
pub const HDLCD_MAX_YRES: c_int = 4096;
pub const NR_PALETTE: c_int = 256;
