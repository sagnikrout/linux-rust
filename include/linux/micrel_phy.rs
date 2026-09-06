//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/micrel_phy.h
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
// include/linux/micrel_phy.h
//
// Micrel PHY IDs
//
pub const MICREL_OUI: c_uint = 0x0885;
pub const MICREL_PHY_ID_MASK: c_uint = 0x00fffff0;
pub const PHY_ID_KSZ8873MLL: c_uint = 0x000e7237;
pub const PHY_ID_KSZ9021: c_uint = 0x00221610;
pub const PHY_ID_KSZ9021RLRN: c_uint = 0x00221611;
pub const PHY_ID_KS8737: c_uint = 0x00221720;
pub const PHY_ID_KSZ8021: c_uint = 0x00221555;
pub const PHY_ID_KSZ8031: c_uint = 0x00221556;
pub const PHY_ID_KSZ8041: c_uint = 0x00221510;
// undocumented
pub const PHY_ID_KSZ8041RNLI: c_uint = 0x00221537;
pub const PHY_ID_KSZ8051: c_uint = 0x00221550;
// same id: ks8001 Rev. A/B, and ks8721 Rev 3.
pub const PHY_ID_KSZ8001: c_uint = 0x0022161A;
// same id: KS8081, KS8091
pub const PHY_ID_KSZ8081: c_uint = 0x00221560;
pub const PHY_ID_KSZ8061: c_uint = 0x00221570;
pub const PHY_ID_KSZ9031: c_uint = 0x00221620;
pub const PHY_ID_KSZ9131: c_uint = 0x00221640;
pub const PHY_ID_LAN8814: c_uint = 0x00221660;
pub const PHY_ID_LAN8804: c_uint = 0x00221670;
pub const PHY_ID_LAN8841: c_uint = 0x00221650;
pub const PHY_ID_LAN8842: c_uint = 0x002216C0;
pub const PHY_ID_LAN9645X: c_uint = 0x002216D0;
pub const PHY_ID_KSZ886X: c_uint = 0x00221430;
pub const PHY_ID_KSZ8863: c_uint = 0x00221435;
pub const PHY_ID_KSZ87XX: c_uint = 0x00221550;
pub const PHY_ID_KSZ9477: c_uint = 0x00221631;
// struct phy_device dev_flags definitions

pub const MICREL_KSZ9021_EXTREG_CTRL: c_uint = 0xB;
pub const MICREL_KSZ9021_EXTREG_DATA_WRITE: c_uint = 0xC;
pub const MICREL_KSZ9021_RGMII_CLK_CTRL_PAD_SCEW: c_uint = 0x104;
pub const MICREL_KSZ9021_RGMII_RX_DATA_PAD_SCEW: c_uint = 0x105;
// Device specific MII_BMCR (Reg 0) bits
// 1 = HP Auto MDI/MDI-X mode, 0 = Microchip Auto MDI/MDI-X mode

// 1 = Force MDI (transmit on RXP/RXM pins), 0 = Normal operation
// (transmit on TXP/TXM pins)
//

// 1 = Disable auto MDI-X

// PHY Special Control/Status Register (Reg 31)

