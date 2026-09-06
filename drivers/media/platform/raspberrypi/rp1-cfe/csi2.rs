//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/raspberrypi/rp1-cfe/csi2.h
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
// RP1 CSI-2 Driver
//
// Copyright (c) 2021-2024 Raspberry Pi Ltd.
// Copyright (c) 2023-2024 Ideas on Board Oy
//

pub const CSI2_NUM_CHANNELS: c_int = 4;
pub const CSI2_PAD_SINK: c_int = 0;
pub const CSI2_PAD_FIRST_SOURCE: c_int = 1;
pub const CSI2_PAD_NUM_SOURCES: c_int = 4;
pub const CSI2_NUM_PADS: c_int = 5;
pub const DISCARDS_TABLE_NUM_VCS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi2_mode {
    CSI2_MODE_NORMAL = 0,
    CSI2_MODE_REMAP = 1,
    CSI2_MODE_COMPRESSED = 2,
    CSI2_MODE_FE_STREAMING = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi2_compression_mode {
    CSI2_COMPRESSION_DELTA = 1,
    CSI2_COMPRESSION_SIMPLE = 2,
    CSI2_COMPRESSION_COMBINED = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum discards_table_index {
    DISCARDS_TABLE_OVERFLOW = 0,
    DISCARDS_TABLE_LENGTH_LIMIT,
    DISCARDS_TABLE_UNMATCHED,
    DISCARDS_TABLE_INACTIVE,
    DISCARDS_TABLE_NUM_ENTRIES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csi2_device {
// Parent V4l2 device
    pub v4l2_dev: *mut v4l2_device,
    pub base: *mut void __iomem,
    pub dphy: dphy_data,
    pub bus_type: v4l2_mbus_type,
    pub bus_flags: c_uint,
    pub num_lines: [c_uint; CSI2_NUM_CHANNELS],
    pub pad: [media_pad; CSI2_NUM_PADS],
    pub sd: v4l2_subdev,
// lock for csi2 errors counters
    pub errors_lock: spinlock_t,
    pub overflows: u32,
    pub discards_table: [u32; DISCARDS_TABLE_NUM_VCS][DISCARDS_TABLE_NUM_ENTRIES],
    pub discards_dt_table: [u32; DISCARDS_TABLE_NUM_ENTRIES],
}

extern "C" {
    pub fn csi2_isr(csi2: *mut csi2_device, sof: *mut bool, eof: *mut bool);
}
extern "C" {
    pub fn csi2_stop_channel(csi2: *mut csi2_device, channel: c_uint);
}
extern "C" {
    pub fn csi2_open_rx(csi2: *mut csi2_device);
}
extern "C" {
    pub fn csi2_close_rx(csi2: *mut csi2_device);
}
extern "C" {
    pub fn csi2_init(csi2: *mut csi2_device, debugfs: *mut dentry) -> c_int;
}
extern "C" {
    pub fn csi2_uninit(csi2: *mut csi2_device);
}
