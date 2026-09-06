//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/qcom/hidma_mgmt.h
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
// Qualcomm Technologies HIDMA Management common header
//
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidma_mgmt_dev {
    pub hw_version_major: u8,
    pub hw_version_minor: u8,
    pub max_wr_xactions: u32,
    pub max_rd_xactions: u32,
    pub max_write_request: u32,
    pub max_read_request: u32,
    pub dma_channels: u32,
    pub chreset_timeout_cycles: u32,
    pub hw_version: u32,
    pub priority: *mut u32,
    pub weight: *mut u32,
// Hardware device constants
    pub virtaddr: *mut void __iomem,
    pub addrsize: resource_size_t,
    pub chroots: *mut kobject,
    pub pdev: *mut platform_device,
}

extern "C" {
    pub fn hidma_mgmt_init_sys(dev: *mut hidma_mgmt_dev) -> c_int;
}
extern "C" {
    pub fn hidma_mgmt_setup(mgmtdev: *mut hidma_mgmt_dev) -> c_int;
}
