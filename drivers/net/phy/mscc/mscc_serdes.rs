//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/mscc/mscc_serdes.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Driver for Microsemi VSC85xx PHYs
//
// Copyright (c) 2021 Microsemi Corporation
//

pub const PHY_S6G_PLL5G_CFG2_ENA_GAIN: c_int = 1;
pub const PHY_S6G_DES_PHY_CTRL_POS: c_int = 13;
pub const PHY_S6G_DES_MBTR_CTRL_POS: c_int = 10;
pub const PHY_S6G_DES_CPMD_SEL_POS: c_int = 8;
pub const PHY_S6G_DES_BW_HYST_POS: c_int = 5;
pub const PHY_S6G_DES_BW_ANA_POS: c_int = 1;
pub const PHY_S6G_DES_CFG: c_uint = 0x21;
pub const PHY_S6G_IB_CFG0: c_uint = 0x22;
pub const PHY_S6G_IB_CFG1: c_uint = 0x23;
pub const PHY_S6G_IB_CFG2: c_uint = 0x24;
pub const PHY_S6G_IB_CFG3: c_uint = 0x25;
pub const PHY_S6G_IB_CFG4: c_uint = 0x26;
pub const PHY_S6G_GP_CFG: c_uint = 0x2E;
pub const PHY_S6G_DFT_CFG0: c_uint = 0x35;
pub const PHY_S6G_IB_DFT_CFG2: c_uint = 0x37;
extern "C" {
    pub fn vsc85xx_sd6g_config_v2(phydev: *mut phy_device) -> c_int;
}
