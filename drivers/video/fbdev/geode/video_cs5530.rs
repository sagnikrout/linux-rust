//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/geode/video_cs5530.h
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
// drivers/video/geode/video_cs5530.h
// -- CS5530 video device
//
// Copyright (C) 2005 Arcom Control Systems Ltd.
//
// Based on AMD's original 2.4 driver:
// Copyright (C) 2004 Advanced Micro Devices, Inc.
//
// CS5530 Video device registers
pub const CS5530_VIDEO_CONFIG: c_uint = 0x0000;

pub const CS5530_DISPLAY_CONFIG: c_uint = 0x0004;

pub const CS5530_VIDEO_X_POS: c_uint = 0x0008;
pub const CS5530_VIDEO_Y_POS: c_uint = 0x000C;
pub const CS5530_VIDEO_SCALE: c_uint = 0x0010;
pub const CS5530_VIDEO_COLOR_KEY: c_uint = 0x0014;
pub const CS5530_VIDEO_COLOR_MASK: c_uint = 0x0018;
pub const CS5530_PALETTE_ADDRESS: c_uint = 0x001C;
pub const CS5530_PALETTE_DATA: c_uint = 0x0020;
pub const CS5530_DOT_CLK_CONFIG: c_uint = 0x0024;
pub const CS5530_CRCSIG_TFT_TV: c_uint = 0x0028;
