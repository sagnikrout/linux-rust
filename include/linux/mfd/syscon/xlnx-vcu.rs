//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/syscon/xlnx-vcu.h
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
// Copyright (C) 2020 Pengutronix, Michael Tretter <kernel@pengutronix.de>
//
pub const VCU_ECODER_ENABLE: c_uint = 0x00;
pub const VCU_DECODER_ENABLE: c_uint = 0x04;
pub const VCU_MEMORY_DEPTH: c_uint = 0x08;
pub const VCU_ENC_COLOR_DEPTH: c_uint = 0x0c;
pub const VCU_ENC_VERTICAL_RANGE: c_uint = 0x10;
pub const VCU_ENC_FRAME_SIZE_X: c_uint = 0x14;
pub const VCU_ENC_FRAME_SIZE_Y: c_uint = 0x18;
pub const VCU_ENC_COLOR_FORMAT: c_uint = 0x1c;
pub const VCU_ENC_FPS: c_uint = 0x20;
pub const VCU_MCU_CLK: c_uint = 0x24;
pub const VCU_CORE_CLK: c_uint = 0x28;
pub const VCU_PLL_BYPASS: c_uint = 0x2c;
pub const VCU_ENC_CLK: c_uint = 0x30;
pub const VCU_PLL_CLK: c_uint = 0x34;
pub const VCU_ENC_VIDEO_STANDARD: c_uint = 0x38;
pub const VCU_STATUS: c_uint = 0x3c;
pub const VCU_AXI_ENC_CLK: c_uint = 0x40;
pub const VCU_AXI_DEC_CLK: c_uint = 0x44;
pub const VCU_AXI_MCU_CLK: c_uint = 0x48;
pub const VCU_DEC_VIDEO_STANDARD: c_uint = 0x4c;
pub const VCU_DEC_FRAME_SIZE_X: c_uint = 0x50;
pub const VCU_DEC_FRAME_SIZE_Y: c_uint = 0x54;
pub const VCU_DEC_FPS: c_uint = 0x58;
pub const VCU_BUFFER_B_FRAME: c_uint = 0x5c;
pub const VCU_WPP_EN: c_uint = 0x60;
pub const VCU_PLL_CLK_DEC: c_uint = 0x64;
pub const VCU_NUM_CORE: c_uint = 0x6c;
pub const VCU_GASKET_INIT: c_uint = 0x74;
pub const VCU_GASKET_VALUE: c_uint = 0x03;
