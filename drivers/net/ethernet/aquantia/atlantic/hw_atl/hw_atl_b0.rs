//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl/hw_atl_b0.h
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
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File hw_atl_b0.h: Declaration of abstract interface for Atlantic hardware
// specific functions.
//

extern "C" {
    pub fn hw_atl_b0_hw_ring_tx_start(self: *mut aq_hw_s, ring: *mut aq_ring_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_ring_rx_start(self: *mut aq_hw_s, ring: *mut aq_ring_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_ring_rx_receive(self: *mut aq_hw_s, ring: *mut aq_ring_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_ring_tx_stop(self: *mut aq_hw_s, ring: *mut aq_ring_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_ring_rx_stop(self: *mut aq_hw_s, ring: *mut aq_ring_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_init_rx_rss_ctrl1(self: *mut aq_hw_s);
}
extern "C" {
    pub fn hw_atl_b0_hw_mac_addr_set(self: *mut aq_hw_s, mac_addr: *const u8) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_set_fc(self: *mut aq_hw_s, fc: u32, tc: u32) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_set_loopback(self: *mut aq_hw_s, mode: u32, enable: bool) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_start(self: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_irq_enable(self: *mut aq_hw_s, mask: u64) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_irq_disable(self: *mut aq_hw_s, mask: u64) -> c_int;
}
extern "C" {
    pub fn hw_atl_b0_hw_irq_read(self: *mut aq_hw_s, mask: *mut u64) -> c_int;
}
