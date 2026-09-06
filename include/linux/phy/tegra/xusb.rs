//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy/tegra/xusb.h
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
// Copyright (c) 2016-2022, NVIDIA CORPORATION.  All rights reserved.
//
extern "C" {
    pub fn tegra_xusb_padctl_put(padctl: *mut tegra_xusb_padctl);
}
extern "C" {
    pub fn tegra_phy_xusb_utmi_pad_power_on(phy: *mut phy);
}
extern "C" {
    pub fn tegra_phy_xusb_utmi_pad_power_down(phy: *mut phy);
}
extern "C" {
    pub fn tegra_phy_xusb_utmi_port_reset(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn tegra_xusb_padctl_get_port_number(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn tegra_xusb_padctl_disable_phy_sleepwalk(padctl: *mut tegra_xusb_padctl, phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn tegra_xusb_padctl_enable_phy_wake(padctl: *mut tegra_xusb_padctl, phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn tegra_xusb_padctl_disable_phy_wake(padctl: *mut tegra_xusb_padctl, phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn tegra_xusb_padctl_remote_wake_detected(padctl: *mut tegra_xusb_padctl, phy: *mut phy) -> bool;
}
