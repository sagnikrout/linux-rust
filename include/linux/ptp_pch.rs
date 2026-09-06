//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ptp_pch.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PTP PCH
//
// Copyright 2019 Linaro Ltd.
//
// Author Lee Jones <lee.jones@linaro.org>
//

extern "C" {
    pub fn pch_ch_control_write(pdev: *mut pci_dev, val: u32);
}
extern "C" {
    pub fn pch_ch_event_read(pdev: *mut pci_dev) -> u32;
}
extern "C" {
    pub fn pch_ch_event_write(pdev: *mut pci_dev, val: u32);
}
extern "C" {
    pub fn pch_src_uuid_lo_read(pdev: *mut pci_dev) -> u32;
}
extern "C" {
    pub fn pch_src_uuid_hi_read(pdev: *mut pci_dev) -> u32;
}
extern "C" {
    pub fn pch_rx_snap_read(pdev: *mut pci_dev) -> u64;
}
extern "C" {
    pub fn pch_tx_snap_read(pdev: *mut pci_dev) -> u64;
}
extern "C" {
    pub fn pch_set_station_address(addr: *mut u8, pdev: *mut pci_dev) -> c_int;
}
