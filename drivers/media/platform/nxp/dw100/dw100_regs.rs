//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/nxp/dw100/dw100_regs.h
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
// DW100 Hardware dewarper
//
// Copyright 2022 NXP
// Author: Xavier Roumegue (xavier.roumegue@oss.nxp.com)
//
// AHB register offset
pub const DW100_DEWARP_ID: c_uint = 0x00;
pub const DW100_DEWARP_CTRL: c_uint = 0x04;

pub const DW100_MAP_LUT_ADDR: c_uint = 0x08;

pub const DW100_MAP_LUT_SIZE: c_uint = 0x0c;

pub const DW100_SRC_IMG_Y_BASE: c_uint = 0x10;

pub const DW100_SRC_IMG_UV_BASE: c_uint = 0x14;

pub const DW100_SRC_IMG_SIZE: c_uint = 0x18;

pub const DW100_SRC_IMG_STRIDE: c_uint = 0x1c;
pub const DW100_MAP_LUT_ADDR2: c_uint = 0x20;
pub const DW100_MAP_LUT_SIZE2: c_uint = 0x24;
pub const DW100_SRC_IMG_Y_BASE2: c_uint = 0x28;
pub const DW100_SRC_IMG_UV_BASE2: c_uint = 0x2c;
pub const DW100_SRC_IMG_SIZE2: c_uint = 0x30;
pub const DW100_SRC_IMG_STRIDE2: c_uint = 0x34;
pub const DW100_DST_IMG_Y_BASE: c_uint = 0x38;
pub const DW100_DST_IMG_UV_BASE: c_uint = 0x3c;
pub const DW100_DST_IMG_SIZE: c_uint = 0x40;
pub const DW100_DST_IMG_STRIDE: c_uint = 0x44;
pub const DW100_DST_IMG_Y_BASE2: c_uint = 0x48;
pub const DW100_DST_IMG_UV_BASE2: c_uint = 0x4c;
pub const DW100_DST_IMG_SIZE2: c_uint = 0x50;
pub const DW100_DST_IMG_STRIDE2: c_uint = 0x54;
pub const DW100_SWAP_CONTROL: c_uint = 0x58;

pub const DW100_VERTICAL_SPLIT_LINE: c_uint = 0x5c;
pub const DW100_HORIZON_SPLIT_LINE: c_uint = 0x60;
pub const DW100_SCALE_FACTOR: c_uint = 0x64;
pub const DW100_ROI_START: c_uint = 0x68;

pub const DW100_BOUNDARY_PIXEL: c_uint = 0x6c;

pub const DW100_INTERRUPT_STATUS: c_uint = 0x70;

pub const DW100_BUS_CTRL: c_uint = 0x74;

pub const DW100_BUS_CTRL1: c_uint = 0x78;
pub const DW100_BUS_TIME_OUT_CYCLE: c_uint = 0x7c;
pub const DW100_DST_IMG_Y_SIZE1: c_uint = 0x80;

pub const DW100_DST_IMG_UV_SIZE1: c_uint = 0x84;
pub const DW100_DST_IMG_Y_SIZE2: c_uint = 0x88;
pub const DW100_DST_IMG_UV_SIZE2: c_uint = 0x8c;
