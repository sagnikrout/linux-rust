//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/mediatek/mtk.h
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
// Common definition for Mediatek Ethernet PHYs
// Author: SkyLake Huang <SkyLake.Huang@mediatek.com>
// Copyright (c) 2024 MediaTek Inc.
//
pub const MTK_PHY_AUX_CTRL_AND_STATUS: c_uint = 0x14;

pub const MTK_EXT_PAGE_ACCESS: c_uint = 0x1f;
pub const MTK_PHY_PAGE_EXTENDED_1: c_uint = 0x0001;
pub const MTK_PHY_PAGE_STANDARD: c_uint = 0x0000;
pub const MTK_PHY_PAGE_EXTENDED_52B5: c_uint = 0x52b5;
// Registers on MDIO_MMD_VEND2
pub const MTK_PHY_LED0_ON_CTRL: c_uint = 0x24;
pub const MTK_PHY_LED1_ON_CTRL: c_uint = 0x26;

pub const MTK_PHY_LED0_BLINK_CTRL: c_uint = 0x25;
pub const MTK_PHY_LED1_BLINK_CTRL: c_uint = 0x27;

pub const MTK_PHY_LED_STATE_FORCE_ON: c_int = 0;
pub const MTK_PHY_LED_STATE_FORCE_BLINK: c_int = 1;
pub const MTK_PHY_LED_STATE_NETDEV: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_socphy_priv {
    pub led_state: c_ulong,
}

extern "C" {
    pub fn mtk_phy_read_page(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn mtk_phy_write_page(phydev: *mut phy_device, page: c_int) -> c_int;
}
extern "C" {
    pub fn mtk_phy_leds_state_init(phydev: *mut phy_device);
}
