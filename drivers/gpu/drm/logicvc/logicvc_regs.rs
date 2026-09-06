//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/logicvc/logicvc_regs.h
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
// Copyright (C) 2019-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//
// Copyright (C) 2014 Xylon d.o.o.
// Author: Davor Joja <davor.joja@logicbricks.com>
//

pub const LOGICVC_HSYNC_FRONT_PORCH_REG: c_uint = 0x00;
pub const LOGICVC_HSYNC_REG: c_uint = 0x08;
pub const LOGICVC_HSYNC_BACK_PORCH_REG: c_uint = 0x10;
pub const LOGICVC_HRES_REG: c_uint = 0x18;
pub const LOGICVC_VSYNC_FRONT_PORCH_REG: c_uint = 0x20;
pub const LOGICVC_VSYNC_REG: c_uint = 0x28;
pub const LOGICVC_VSYNC_BACK_PORCH_REG: c_uint = 0x30;
pub const LOGICVC_VRES_REG: c_uint = 0x38;
pub const LOGICVC_CTRL_REG: c_uint = 0x40;

pub const LOGICVC_DTYPE_REG: c_uint = 0x48;
pub const LOGICVC_BACKGROUND_COLOR_REG: c_uint = 0x50;
pub const LOGICVC_BUFFER_SEL_REG: c_uint = 0x58;

pub const LOGICVC_BUFFER_SEL_MAX: c_int = 2;
pub const LOGICVC_DOUBLE_CLUT_REG: c_uint = 0x60;
pub const LOGICVC_INT_STAT_REG: c_uint = 0x68;

pub const LOGICVC_INT_MASK_REG: c_uint = 0x70;

pub const LOGICVC_POWER_CTRL_REG: c_uint = 0x78;

pub const LOGICVC_IP_VERSION_REG: c_uint = 0xf8;

pub const LOGICVC_LAYER_VOFFSET_MAX: c_int = 4095;

