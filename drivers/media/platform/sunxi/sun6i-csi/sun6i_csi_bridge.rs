//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun6i-csi/sun6i_csi_bridge.h
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
// Copyright 2021-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sun6i_csi_bridge_pad {
    SUN6I_CSI_BRIDGE_PAD_SINK	= 0,
    SUN6I_CSI_BRIDGE_PAD_SOURCE	= 1,
    SUN6I_CSI_BRIDGE_PAD_COUNT	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_bridge_format {
    pub mbus_code: u32,
    pub input_format: u8,
    pub input_yuv_seq: u8,
    pub input_yuv_seq_invert: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_bridge_source {
    pub subdev: *mut v4l2_subdev,
    pub endpoint: v4l2_fwnode_endpoint,
    pub expected: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_bridge_async_subdev {
    pub async_subdev: v4l2_async_connection,
    pub source: *mut sun6i_csi_bridge_source,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_bridge {
    pub subdev: v4l2_subdev,
    pub notifier: v4l2_async_notifier,
    pub pads: [media_pad; 2],
    pub mbus_format: v4l2_mbus_framefmt,
    pub /: *mut *mut mutex lock; / Mbus format lock.,
    pub source_parallel: sun6i_csi_bridge_source,
    pub source_mipi_csi2: sun6i_csi_bridge_source,
}

// Helpers
// Format
// Bridge
extern "C" {
    pub fn sun6i_csi_bridge_setup(csi_dev: *mut sun6i_csi_device) -> c_int;
}
extern "C" {
    pub fn sun6i_csi_bridge_cleanup(csi_dev: *mut sun6i_csi_device);
}
