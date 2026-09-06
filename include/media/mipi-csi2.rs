//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/mipi-csi2.h
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
// MIPI CSI-2 Data Types
//
// Copyright (C) 2022 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//
// Short packet data types
pub const MIPI_CSI2_DT_FS: c_uint = 0x00;
pub const MIPI_CSI2_DT_FE: c_uint = 0x01;
pub const MIPI_CSI2_DT_LS: c_uint = 0x02;
pub const MIPI_CSI2_DT_LE: c_uint = 0x03;

// Long packet data types
pub const MIPI_CSI2_DT_NULL: c_uint = 0x10;
pub const MIPI_CSI2_DT_BLANKING: c_uint = 0x11;
pub const MIPI_CSI2_DT_EMBEDDED_8B: c_uint = 0x12;

pub const MIPI_CSI2_DT_YUV420_8B: c_uint = 0x18;
pub const MIPI_CSI2_DT_YUV420_10B: c_uint = 0x19;
pub const MIPI_CSI2_DT_YUV420_8B_LEGACY: c_uint = 0x1a;
pub const MIPI_CSI2_DT_YUV420_8B_CS: c_uint = 0x1c;
pub const MIPI_CSI2_DT_YUV420_10B_CS: c_uint = 0x1d;
pub const MIPI_CSI2_DT_YUV422_8B: c_uint = 0x1e;
pub const MIPI_CSI2_DT_YUV422_10B: c_uint = 0x1f;
pub const MIPI_CSI2_DT_RGB444: c_uint = 0x20;
pub const MIPI_CSI2_DT_RGB555: c_uint = 0x21;
pub const MIPI_CSI2_DT_RGB565: c_uint = 0x22;
pub const MIPI_CSI2_DT_RGB666: c_uint = 0x23;
pub const MIPI_CSI2_DT_RGB888: c_uint = 0x24;
pub const MIPI_CSI2_DT_RAW28: c_uint = 0x26;
pub const MIPI_CSI2_DT_RAW24: c_uint = 0x27;
pub const MIPI_CSI2_DT_RAW6: c_uint = 0x28;
pub const MIPI_CSI2_DT_RAW7: c_uint = 0x29;
pub const MIPI_CSI2_DT_RAW8: c_uint = 0x2a;
pub const MIPI_CSI2_DT_RAW10: c_uint = 0x2b;
pub const MIPI_CSI2_DT_RAW12: c_uint = 0x2c;
pub const MIPI_CSI2_DT_RAW14: c_uint = 0x2d;
pub const MIPI_CSI2_DT_RAW16: c_uint = 0x2e;
pub const MIPI_CSI2_DT_RAW20: c_uint = 0x2f;

