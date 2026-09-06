//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/media/video-interfaces.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2022 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//
pub const MEDIA_BUS_TYPE_CSI2_CPHY: c_int = 1;
pub const MEDIA_BUS_TYPE_CSI1: c_int = 2;
pub const MEDIA_BUS_TYPE_CCP2: c_int = 3;
pub const MEDIA_BUS_TYPE_CSI2_DPHY: c_int = 4;
pub const MEDIA_BUS_TYPE_PARALLEL: c_int = 5;
pub const MEDIA_BUS_TYPE_BT656: c_int = 6;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_ABC: c_int = 0;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_ACB: c_int = 1;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_BAC: c_int = 2;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_BCA: c_int = 3;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_CAB: c_int = 4;
pub const MEDIA_BUS_CSI2_CPHY_LINE_ORDER_CBA: c_int = 5;
pub const MEDIA_PCLK_SAMPLE_FALLING_EDGE: c_int = 0;
pub const MEDIA_PCLK_SAMPLE_RISING_EDGE: c_int = 1;
pub const MEDIA_PCLK_SAMPLE_DUAL_EDGE: c_int = 2;
