//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/geode/display_gx1.h
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
// drivers/video/geode/display_gx1.h
// -- Geode GX1 display controller
//
// Copyright (C) 2005 Arcom Control Systems Ltd.
//
// Based on AMD's original 2.4 driver:
// Copyright (C) 2004 Advanced Micro Devices, Inc.
//
extern "C" {
    pub fn gx1_gx_base() -> unsigned;
}
extern "C" {
    pub fn gx1_frame_buffer_size() -> c_int;
}
// GX1 configuration I/O registers
pub const CONFIG_CCR3: c_uint = 0xc3;

pub const CONFIG_GCR: c_uint = 0xb8;
// Memory controller registers
pub const MC_BANK_CFG: c_uint = 0x08;

pub const MC_GBASE_ADD: c_uint = 0x14;

// Display controller registers
pub const DC_PAL_ADDRESS: c_uint = 0x70;
pub const DC_PAL_DATA: c_uint = 0x74;
pub const DC_UNLOCK: c_uint = 0x00;

pub const DC_GENERAL_CFG: c_uint = 0x04;

pub const DC_TIMING_CFG: c_uint = 0x08;

pub const DC_OUTPUT_CFG: c_uint = 0x0C;

pub const DC_FB_ST_OFFSET: c_uint = 0x10;
pub const DC_CB_ST_OFFSET: c_uint = 0x14;
pub const DC_CURS_ST_OFFSET: c_uint = 0x18;
pub const DC_ICON_ST_OFFSET: c_uint = 0x1C;
pub const DC_VID_ST_OFFSET: c_uint = 0x20;
pub const DC_LINE_DELTA: c_uint = 0x24;
pub const DC_BUF_SIZE: c_uint = 0x28;
pub const DC_H_TIMING_1: c_uint = 0x30;
pub const DC_H_TIMING_2: c_uint = 0x34;
pub const DC_H_TIMING_3: c_uint = 0x38;
pub const DC_FP_H_TIMING: c_uint = 0x3C;
pub const DC_V_TIMING_1: c_uint = 0x40;
pub const DC_V_TIMING_2: c_uint = 0x44;
pub const DC_V_TIMING_3: c_uint = 0x48;
pub const DC_FP_V_TIMING: c_uint = 0x4C;
pub const DC_CURSOR_X: c_uint = 0x50;
pub const DC_ICON_X: c_uint = 0x54;
pub const DC_V_LINE_CNT: c_uint = 0x54;
pub const DC_CURSOR_Y: c_uint = 0x58;
pub const DC_ICON_Y: c_uint = 0x5C;
pub const DC_SS_LINE_CMP: c_uint = 0x5C;
pub const DC_CURSOR_COLOR: c_uint = 0x60;
pub const DC_ICON_COLOR: c_uint = 0x64;
pub const DC_BORDER_COLOR: c_uint = 0x68;
pub const DC_PAL_ADDRESS: c_uint = 0x70;
pub const DC_PAL_DATA: c_uint = 0x74;
pub const DC_DFIFO_DIAG: c_uint = 0x78;
pub const DC_CFIFO_DIAG: c_uint = 0x7C;
