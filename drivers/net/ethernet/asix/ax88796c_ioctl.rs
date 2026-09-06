//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/asix/ax88796c_ioctl.h
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
// Copyright (c) 2010 ASIX Electronics Corporation
// Copyright (c) 2020 Samsung Electronics Co., Ltd.
//
// ASIX AX88796C SPI Fast Ethernet Linux driver
//

extern "C" {
    pub fn ax88796c_check_power(ax_local: *const ax88796c_device) -> bool;
}
extern "C" {
    pub fn ax88796c_check_power_and_wake(ax_local: *mut ax88796c_device) -> bool;
}
extern "C" {
    pub fn ax88796c_set_power_saving(ax_local: *mut ax88796c_device, ps_level: u8);
}
extern "C" {
    pub fn ax88796c_mdio_read(mdiobus: *mut mii_bus, phy_id: c_int, loc: c_int) -> c_int;
}
extern "C" {
    pub fn ax88796c_mdio_write(mdiobus: *mut mii_bus, phy_id: c_int, loc: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn ax88796c_ioctl(dev: *mut net_device, ifr: *mut ifreq, cmd: c_int) -> c_int;
}
