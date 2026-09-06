//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-isys-csi2.h
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
// Copyright (C) 2013--2024 Intel Corporation

pub const NR_OF_CSI2_VC: c_int = 16;

pub const NR_OF_CSI2_SINK_PADS: c_int = 1;
pub const CSI2_PAD_SINK: c_int = 0;
pub const NR_OF_CSI2_SRC_PADS: c_int = 8;
pub const CSI2_PAD_SRC: c_int = 1;

pub const CSI2_CSI_RX_DLY_CNT_TERMEN_CLANE_A: c_int = 0;
pub const CSI2_CSI_RX_DLY_CNT_TERMEN_CLANE_B: c_int = 0;
pub const CSI2_CSI_RX_DLY_CNT_SETTLE_CLANE_A: c_int = 95;

pub const CSI2_CSI_RX_DLY_CNT_TERMEN_DLANE_A: c_int = 0;
pub const CSI2_CSI_RX_DLY_CNT_TERMEN_DLANE_B: c_int = 0;
pub const CSI2_CSI_RX_DLY_CNT_SETTLE_DLANE_A: c_int = 85;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_csi2 {
    pub asd: ipu6_isys_subdev,
    pub isys: *mut ipu6_isys,
    pub av: [ipu6_isys_video; NR_OF_CSI2_SRC_PADS],
    pub base: *mut void __iomem,
    pub receiver_errors: u32,
    pub nlanes: c_uint,
    pub port: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_csi2_timing {
    pub ctermen: u32,
    pub csettle: u32,
    pub dtermen: u32,
    pub dsettle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_csi2_error {
    pub error_string: *const c_char,
    pub is_info_only: bool,
}

extern "C" {
    pub fn ipu6_isys_csi2_get_link_freq(csi2: *mut ipu6_isys_csi2) -> i64;
}
extern "C" {
    pub fn ipu6_isys_csi2_cleanup(csi2: *mut ipu6_isys_csi2);
}
extern "C" {
    pub fn ipu6_isys_csi2_sof_event_by_stream(stream: *mut ipu6_isys_stream);
}
extern "C" {
    pub fn ipu6_isys_csi2_eof_event_by_stream(stream: *mut ipu6_isys_stream);
}
extern "C" {
    pub fn ipu6_isys_register_errors(csi2: *mut ipu6_isys_csi2);
}
extern "C" {
    pub fn ipu6_isys_csi2_error(csi2: *mut ipu6_isys_csi2);
}
