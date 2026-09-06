//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/smscphy.h
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

pub const MII_LAN83C185_MODE_MASK: c_uint = 0xE0;
pub const MII_LAN83C185_MODE_POWERDOWN: c_uint = 0xC0 /* Power Down mode */;
pub const MII_LAN83C185_MODE_ALL: c_uint = 0xE0 /* All capable mode */;
extern "C" {
    pub fn smsc_phy_config_intr(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn smsc_phy_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t;
}
extern "C" {
    pub fn smsc_phy_config_init(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn lan87xx_read_status(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn smsc_phy_probe(phydev: *mut phy_device) -> c_int;
}
pub const MII_LAN874X_PHY_MMD_WOL_WUCSR: c_uint = 0x8010;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_CFGA: c_uint = 0x8011;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_CFGB: c_uint = 0x8012;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK0: c_uint = 0x8021;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK1: c_uint = 0x8022;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK2: c_uint = 0x8023;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK3: c_uint = 0x8024;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK4: c_uint = 0x8025;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK5: c_uint = 0x8026;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK6: c_uint = 0x8027;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK7: c_uint = 0x8028;
pub const MII_LAN874X_PHY_MMD_WOL_RX_ADDRA: c_uint = 0x8061;
pub const MII_LAN874X_PHY_MMD_WOL_RX_ADDRB: c_uint = 0x8062;
pub const MII_LAN874X_PHY_MMD_WOL_RX_ADDRC: c_uint = 0x8063;
pub const MII_LAN874X_PHY_MMD_MCFGR: c_uint = 0x8064;

pub const MII_LAN874X_PHY_PME_SELF_CLEAR_DELAY: c_uint = 0x1000 /* 81 milliseconds */;
