//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/synopsys/hdmirx/snps_hdmirx_cec.h
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
// Copyright (c) 2021 Rockchip Electronics Co. Ltd.
//
// Author: Shunqing Chen <csq@rock-chips.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmirx_cec_ops {
    pub val): *mut *mut *mut void (write)(struct snps_hdmirx_dev hdmirx_dev, int reg, u32,
    pub reg): *mut *mut *mut u32 (read)(struct snps_hdmirx_dev hdmirx_dev, int,
    pub hdmirx): *mut *mut void (enable)(struct snps_hdmirx_dev,
    pub hdmirx): *mut *mut void (disable)(struct snps_hdmirx_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmirx_cec_data {
    pub hdmirx: *mut snps_hdmirx_dev,
    pub ops: *const hdmirx_cec_ops,
    pub dev: *mut device,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmirx_cec {
    pub hdmirx: *mut snps_hdmirx_dev,
    pub dev: *mut device,
    pub ops: *const hdmirx_cec_ops,
    pub addresses: u32,
    pub adap: *mut cec_adapter,
    pub rx_msg: cec_msg,
    pub tx_status: c_uint,
    pub tx_done: bool,
    pub rx_done: bool,
    pub irq: c_int,
}

extern "C" {
    pub fn snps_hdmirx_cec_unregister(cec: *mut hdmirx_cec);
}
