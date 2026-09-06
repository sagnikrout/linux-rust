//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkcif/rkcif-interface.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Rockchip Camera Interface (CIF) Driver
//
// Abstraction for the INTERFACE and CROP parts of the different CIF variants.
// They shall be represented as V4L2 subdevice with one sink pad and one
// source pad. The sink pad is connected to a subdevice: either the subdevice
// provided by the driver of the companion chip connected to the DVP, or the
// subdevice provided by the MIPI CSI-2 receiver driver. The source pad is
// to V4l2 device(s) provided by one or many instance(s) of the DMA
// abstraction.
//
// Copyright (C) 2025 Michael Riesch <michael.riesch@wolfvision.net>
// Copyright (C) 2025 Collabora, Ltd.
//

extern "C" {
    pub fn rkcif_interface_unregister(interface: *mut rkcif_interface);
}
