//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cobalt/m00233_video_measure_memmap_package.h
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
// Copyright 2014-2015 Cisco Systems, Inc. and/or its affiliates.
// All rights reserved.
//
// Register Block
// M00233_VIDEO_MEASURE_MEMMAP_PACKAGE_VHD_REGMAP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m00233_video_measure_regmap {
    pub /: *mut *mut uint32_t irq_status; / Reg 0x0000,
// The vertical counter starts on rising edge of vsync
    pub /: *mut *mut uint32_t vsync_time; / Reg 0x0004,
    pub /: *mut *mut uint32_t vback_porch; / Reg 0x0008,
    pub /: *mut *mut uint32_t vactive_area; / Reg 0x000c,
    pub /: *mut *mut uint32_t vfront_porch; / Reg 0x0010,
// The horizontal counter starts on rising edge of hsync.
    pub /: *mut *mut uint32_t hsync_time; / Reg 0x0014,
    pub /: *mut *mut uint32_t hback_porch; / Reg 0x0018,
    pub /: *mut *mut uint32_t hactive_area; / Reg 0x001c,
    pub /: *mut *mut uint32_t hfront_porch; / Reg 0x0020,
    pub /: *mut *mut uint32_t control; / Reg 0x0024, Default=0x0,
    pub /: *mut *mut uint32_t irq_triggers; / Reg 0x0028, Default=0xff,
// Value is given in number of register bus clock periods between
// falling and rising edge of hsync. Must be non-zero.
    pub /: *mut *mut uint32_t hsync_timeout_val; / Reg 0x002c, Default=0x1fff,
    pub /: *mut *mut uint32_t status; / Reg 0x0030,
}

pub const M00233_VIDEO_MEASURE_REG_IRQ_STATUS_OFST: c_int = 0;
pub const M00233_VIDEO_MEASURE_REG_VSYNC_TIME_OFST: c_int = 4;
pub const M00233_VIDEO_MEASURE_REG_VBACK_PORCH_OFST: c_int = 8;
pub const M00233_VIDEO_MEASURE_REG_VACTIVE_AREA_OFST: c_int = 12;
pub const M00233_VIDEO_MEASURE_REG_VFRONT_PORCH_OFST: c_int = 16;
pub const M00233_VIDEO_MEASURE_REG_HSYNC_TIME_OFST: c_int = 20;
pub const M00233_VIDEO_MEASURE_REG_HBACK_PORCH_OFST: c_int = 24;
pub const M00233_VIDEO_MEASURE_REG_HACTIVE_AREA_OFST: c_int = 28;
pub const M00233_VIDEO_MEASURE_REG_HFRONT_PORCH_OFST: c_int = 32;
pub const M00233_VIDEO_MEASURE_REG_CONTROL_OFST: c_int = 36;
pub const M00233_VIDEO_MEASURE_REG_IRQ_TRIGGERS_OFST: c_int = 40;
pub const M00233_VIDEO_MEASURE_REG_HSYNC_TIMEOUT_VAL_OFST: c_int = 44;
pub const M00233_VIDEO_MEASURE_REG_STATUS_OFST: c_int = 48;
//
// Bit Mask for register
// M00233_VIDEO_MEASURE_MEMMAP_PACKAGE_VHD_BITMAP
//
// irq_status [7:0]

// control [4:0]

// irq_triggers [7:0]

// status [1:0]

