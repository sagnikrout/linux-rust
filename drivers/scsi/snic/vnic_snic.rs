//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/vnic_snic.h
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.
pub const VNIC_SNIC_WQ_DESCS_MIN: c_int = 64;
pub const VNIC_SNIC_WQ_DESCS_MAX: c_int = 1024;
pub const VNIC_SNIC_MAXDATAFIELDSIZE_MIN: c_int = 256;
pub const VNIC_SNIC_MAXDATAFIELDSIZE_MAX: c_int = 2112;
pub const VNIC_SNIC_IO_THROTTLE_COUNT_MIN: c_int = 1;
pub const VNIC_SNIC_IO_THROTTLE_COUNT_MAX: c_int = 1024;
pub const VNIC_SNIC_PORT_DOWN_TIMEOUT_MIN: c_int = 0;
pub const VNIC_SNIC_PORT_DOWN_TIMEOUT_MAX: c_int = 240000;
pub const VNIC_SNIC_PORT_DOWN_IO_RETRIES_MIN: c_int = 0;
pub const VNIC_SNIC_PORT_DOWN_IO_RETRIES_MAX: c_int = 255;
pub const VNIC_SNIC_LUNS_PER_TARGET_MIN: c_int = 1;
pub const VNIC_SNIC_LUNS_PER_TARGET_MAX: c_int = 1024;
// Device-specific region: scsi configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_snic_config {
    pub flags: u32,
    pub wq_enet_desc_count: u32,
    pub io_throttle_count: u32,
    pub port_down_timeout: u32,
    pub port_down_io_retries: u32,
    pub luns_per_tgt: u32,
    pub maxdatafieldsize: u16,
    pub intr_timer: u16,
    pub intr_timer_type: u8,
    pub _resvd2: u8,
    pub xpt_type: u8,
    pub hid: u8,
}
