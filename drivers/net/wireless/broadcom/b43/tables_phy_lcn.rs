//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/tables_phy_lcn.h
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
// The LCN-PHY tables.
pub const B43_LCNTAB_TYPEMASK: c_uint = 0xF0000000;
pub const B43_LCNTAB_8BIT: c_uint = 0x10000000;
pub const B43_LCNTAB_16BIT: c_uint = 0x20000000;
pub const B43_LCNTAB_32BIT: c_uint = 0x30000000;

pub const B43_LCNTAB_TX_GAIN_SIZE: c_int = 128;
extern "C" {
    pub fn b43_lcntab_read(dev: *mut b43_wldev, offset: u32) -> u32;
}
extern "C" {
    pub fn b43_lcntab_write(dev: *mut b43_wldev, offset: u32, value: u32);
}
extern "C" {
    pub fn b43_phy_lcn_tables_init(dev: *mut b43_wldev);
}
