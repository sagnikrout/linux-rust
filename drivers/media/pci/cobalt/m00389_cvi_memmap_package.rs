//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cobalt/m00389_cvi_memmap_package.h
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
// M00389_CVI_MEMMAP_PACKAGE_VHD_REGMAP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m00389_cvi_regmap {
    pub /: *mut *mut uint32_t control; / Reg 0x0000, Default=0x0,
    pub /: *mut *mut uint32_t frame_width; / Reg 0x0004, Default=0x10,
    pub /: *mut *mut uint32_t frame_height; / Reg 0x0008, Default=0xc,
    pub /: *mut *mut uint32_t freewheel_period; / Reg 0x000c, Default=0x0,
    pub /: *mut *mut uint32_t error_color; / Reg 0x0010, Default=0x0,
    pub /: *mut *mut uint32_t status; / Reg 0x0014,
}

pub const M00389_CVI_REG_CONTROL_OFST: c_int = 0;
pub const M00389_CVI_REG_FRAME_WIDTH_OFST: c_int = 4;
pub const M00389_CVI_REG_FRAME_HEIGHT_OFST: c_int = 8;
pub const M00389_CVI_REG_FREEWHEEL_PERIOD_OFST: c_int = 12;
pub const M00389_CVI_REG_ERROR_COLOR_OFST: c_int = 16;
pub const M00389_CVI_REG_STATUS_OFST: c_int = 20;
//
// Bit Mask for register
// M00389_CVI_MEMMAP_PACKAGE_VHD_BITMAP
//
// control [2:0]

// status [1:0]

