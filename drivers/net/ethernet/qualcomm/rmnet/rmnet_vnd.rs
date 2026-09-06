//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/rmnet/rmnet_vnd.h
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
// Copyright (c) 2013-2017, The Linux Foundation. All rights reserved.
//
// RMNET Data Virtual Network Device APIs
//
extern "C" {
    pub fn rmnet_vnd_do_flow_control(dev: *mut net_device, enable: c_int) -> c_int;
}
extern "C" {
    pub fn rmnet_vnd_rx_fixup(skb: *mut sk_buff, dev: *mut net_device);
}
extern "C" {
    pub fn rmnet_vnd_tx_fixup_len(len: c_uint, dev: *mut net_device);
}
extern "C" {
    pub fn rmnet_vnd_tx_fixup(skb: *mut sk_buff, dev: *mut net_device);
}
extern "C" {
    pub fn rmnet_vnd_setup(dev: *mut net_device);
}
extern "C" {
    pub fn rmnet_vnd_validate_real_dev_mtu(real_dev: *mut net_device) -> c_int;
}
