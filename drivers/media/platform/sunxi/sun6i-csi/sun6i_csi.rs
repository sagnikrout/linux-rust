//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun6i-csi/sun6i_csi.h
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
// Copyright (c) 2011-2018 Magewell Electronics Co., Ltd. (Nanjing)
// Author: Yong Deng <yong.deng@magewell.com>
// Copyright 2021-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sun6i_csi_port {
    SUN6I_CSI_PORT_PARALLEL		= 0,
    SUN6I_CSI_PORT_MIPI_CSI2	= 1,
    SUN6I_CSI_PORT_ISP		= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_buffer {
    pub v4l2_buffer: vb2_v4l2_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_v4l2 {
    pub v4l2_dev: v4l2_device,
    pub media_dev: media_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_device {
    pub dev: *mut device,
    pub v4l2_dev: *mut v4l2_device,
    pub media_dev: *mut media_device,
    pub v4l2: sun6i_csi_v4l2,
    pub bridge: sun6i_csi_bridge,
    pub capture: sun6i_csi_capture,
    pub regmap: *mut regmap,
    pub clock_mod: *mut clk,
    pub clock_ram: *mut clk,
    pub reset: *mut reset_control,
    pub isp_available: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_variant {
    pub clock_mod_rate: c_ulong,
}

// ISP
