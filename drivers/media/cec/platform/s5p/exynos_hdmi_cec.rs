//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/cec/platform/s5p/exynos_hdmi_cec.h
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
// drivers/media/platform/s5p-cec/exynos_hdmi_cec.h
//
// Copyright (c) 2010, 2014 Samsung Electronics
// http://www.samsung.com
//
// Header file for interface of Samsung Exynos hdmi cec hardware
//

extern "C" {
    pub fn s5p_cec_set_divider(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_enable_rx(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_mask_rx_interrupts(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_unmask_rx_interrupts(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_mask_tx_interrupts(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_unmask_tx_interrupts(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_reset(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_tx_reset(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_rx_reset(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_threshold(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_set_addr(cec: *mut s5p_cec_dev, addr: u32);
}
extern "C" {
    pub fn s5p_cec_get_status(cec: *mut s5p_cec_dev) -> u32;
}
extern "C" {
    pub fn s5p_clr_pending_tx(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_clr_pending_rx(cec: *mut s5p_cec_dev);
}
extern "C" {
    pub fn s5p_cec_get_rx_buf(cec: *mut s5p_cec_dev, size: u32, buffer: *mut u8);
}
