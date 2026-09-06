//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun6i-mipi-csi2/sun6i_mipi_csi2.h
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
// Copyright 2020-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sun6i_mipi_csi2_pad {
    SUN6I_MIPI_CSI2_PAD_SINK	= 0,
    SUN6I_MIPI_CSI2_PAD_SOURCE	= 1,
    SUN6I_MIPI_CSI2_PAD_COUNT	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_mipi_csi2_format {
    pub mbus_code: u32,
    pub data_type: u8,
    pub bpp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_mipi_csi2_bridge {
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; SUN6I_MIPI_CSI2_PAD_COUNT],
    pub endpoint: v4l2_fwnode_endpoint,
    pub notifier: v4l2_async_notifier,
    pub mbus_format: v4l2_mbus_framefmt,
    pub /: *mut *mut mutex lock; / Mbus format lock.,
    pub source_subdev: *mut v4l2_subdev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_mipi_csi2_device {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub clock_mod: *mut clk,
    pub reset: *mut reset_control,
    pub dphy: *mut phy,
    pub bridge: sun6i_mipi_csi2_bridge,
}
