//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/oki-semi/pch_gbe/pch_gbe_phy.h
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
// Copyright (C) 1999 - 2010 Intel Corporation.
// Copyright (C) 2010 OKI SEMICONDUCTOR Co., LTD.
//
// This code was derived from the Intel e1000e Linux driver.
//
pub const PCH_GBE_PHY_REGS_LEN: c_int = 32;
pub const PCH_GBE_PHY_RESET_DELAY_US: c_int = 10;
extern "C" {
    pub fn pch_gbe_phy_get_id(hw: *mut pch_gbe_hw) -> i32;
}
extern "C" {
    pub fn pch_gbe_phy_read_reg_miic(hw: *mut pch_gbe_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn pch_gbe_phy_write_reg_miic(hw: *mut pch_gbe_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn pch_gbe_phy_hw_reset(hw: *mut pch_gbe_hw);
}
extern "C" {
    pub fn pch_gbe_phy_power_up(hw: *mut pch_gbe_hw);
}
extern "C" {
    pub fn pch_gbe_phy_power_down(hw: *mut pch_gbe_hw);
}
extern "C" {
    pub fn pch_gbe_phy_set_rgmii(hw: *mut pch_gbe_hw);
}
extern "C" {
    pub fn pch_gbe_phy_init_setting(hw: *mut pch_gbe_hw);
}
extern "C" {
    pub fn pch_gbe_phy_disable_hibernate(hw: *mut pch_gbe_hw) -> c_int;
}
