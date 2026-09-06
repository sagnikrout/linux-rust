//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/air_phy_lib.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2026 Airoha Technology Corp.
// Copyright (C) 2026 Collabora Ltd.
// Louis-Alexis Eyraud <louisalexis.eyraud@collabora.com>
//

pub const AIR_EXT_PAGE_ACCESS: c_uint = 0x1f;
pub const AIR_PHY_PAGE_STANDARD: c_uint = 0x0000;
pub const AIR_PHY_PAGE_EXTENDED_1: c_uint = 0x0001;
pub const AIR_PHY_PAGE_EXTENDED_4: c_uint = 0x0004;
// MII Registers Page 4
pub const AIR_BPBUS_MODE: c_uint = 0x10;
pub const AIR_BPBUS_MODE_ADDR_FIXED: c_uint = 0x0000;

pub const AIR_BPBUS_WR_ADDR_HIGH: c_uint = 0x11;
pub const AIR_BPBUS_WR_ADDR_LOW: c_uint = 0x12;
pub const AIR_BPBUS_WR_DATA_HIGH: c_uint = 0x13;
pub const AIR_BPBUS_WR_DATA_LOW: c_uint = 0x14;
pub const AIR_BPBUS_RD_ADDR_HIGH: c_uint = 0x15;
pub const AIR_BPBUS_RD_ADDR_LOW: c_uint = 0x16;
pub const AIR_BPBUS_RD_DATA_HIGH: c_uint = 0x17;
pub const AIR_BPBUS_RD_DATA_LOW: c_uint = 0x18;
extern "C" {
    pub fn air_phy_read_page(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn air_phy_write_page(phydev: *mut phy_device, page: c_int) -> c_int;
}
