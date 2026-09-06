//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sungem_phy.h
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

// Operations supported by any kind of PHY
// Structure used to statically define an mii/gii based PHY
// An instance of a PHY, partially borrowed from mii_if_info
// 1: autoneg enabled, 0: disabled
// forced speed & duplex (no autoneg)
// partner speed & duplex & pause (autoneg)
//
// Provided by host chip
// Pass in a struct mii_phy with dev, mdio_read and mdio_write
// filled, the remaining fields will be filled on return
//
extern "C" {
    pub fn sungem_phy_probe(phy: *mut mii_phy, mii_id: c_int) -> c_int;
}
// MII definitions missing from mii.h
pub const BMCR_SPD2: c_uint = 0x0040		/* Gigabit enable (bcm54xx)	*/;
pub const LPA_PAUSE: c_uint = 0x0400;
// More PHY registers (model specific)
// MII BCM5201 MULTIPHY interrupt register
pub const MII_BCM5201_INTERRUPT: c_uint = 0x1A;
pub const MII_BCM5201_INTERRUPT_INTENABLE: c_uint = 0x4000;
pub const MII_BCM5201_AUXMODE2: c_uint = 0x1B;
pub const MII_BCM5201_AUXMODE2_LOWPOWER: c_uint = 0x0008;
pub const MII_BCM5201_MULTIPHY: c_uint = 0x1E;
// MII BCM5201 MULTIPHY register bits
pub const MII_BCM5201_MULTIPHY_SERIALMODE: c_uint = 0x0002;
pub const MII_BCM5201_MULTIPHY_SUPERISOLATE: c_uint = 0x0008;
// MII BCM5221 Additional registers
pub const MII_BCM5221_TEST: c_uint = 0x1f;
pub const MII_BCM5221_TEST_ENABLE_SHADOWS: c_uint = 0x0080;
pub const MII_BCM5221_SHDOW_AUX_STAT2: c_uint = 0x1b;
pub const MII_BCM5221_SHDOW_AUX_STAT2_APD: c_uint = 0x0020;
pub const MII_BCM5221_SHDOW_AUX_MODE4: c_uint = 0x1a;
pub const MII_BCM5221_SHDOW_AUX_MODE4_IDDQMODE: c_uint = 0x0001;
pub const MII_BCM5221_SHDOW_AUX_MODE4_CLKLOPWR: c_uint = 0x0004;
// MII BCM5241 Additional registers
pub const MII_BCM5241_SHDOW_AUX_MODE4_STANDBYPWR: c_uint = 0x0008;
// MII BCM5400 1000-BASET Control register
pub const MII_BCM5400_GB_CONTROL: c_uint = 0x09;
pub const MII_BCM5400_GB_CONTROL_FULLDUPLEXCAP: c_uint = 0x0200;
// MII BCM5400 AUXCONTROL register
pub const MII_BCM5400_AUXCONTROL: c_uint = 0x18;
pub const MII_BCM5400_AUXCONTROL_PWR10BASET: c_uint = 0x0004;
// MII BCM5400 AUXSTATUS register
pub const MII_BCM5400_AUXSTATUS: c_uint = 0x19;
pub const MII_BCM5400_AUXSTATUS_LINKMODE_MASK: c_uint = 0x0700;
pub const MII_BCM5400_AUXSTATUS_LINKMODE_SHIFT: c_int = 8;
// 1000BT control (Marvell & BCM54xx at least)
pub const MII_1000BASETCONTROL: c_uint = 0x09;
pub const MII_1000BASETCONTROL_FULLDUPLEXCAP: c_uint = 0x0200;
pub const MII_1000BASETCONTROL_HALFDUPLEXCAP: c_uint = 0x0100;
// Marvell 88E1011 PHY control
pub const MII_M1011_PHY_SPEC_CONTROL: c_uint = 0x10;
pub const MII_M1011_PHY_SPEC_CONTROL_MANUAL_MDIX: c_uint = 0x20;
pub const MII_M1011_PHY_SPEC_CONTROL_AUTO_MDIX: c_uint = 0x40;
// Marvell 88E1011 PHY status
pub const MII_M1011_PHY_SPEC_STATUS: c_uint = 0x11;
pub const MII_M1011_PHY_SPEC_STATUS_1000: c_uint = 0x8000;
pub const MII_M1011_PHY_SPEC_STATUS_100: c_uint = 0x4000;
pub const MII_M1011_PHY_SPEC_STATUS_SPD_MASK: c_uint = 0xc000;
pub const MII_M1011_PHY_SPEC_STATUS_FULLDUPLEX: c_uint = 0x2000;
pub const MII_M1011_PHY_SPEC_STATUS_RESOLVED: c_uint = 0x0800;
pub const MII_M1011_PHY_SPEC_STATUS_TX_PAUSE: c_uint = 0x0008;
pub const MII_M1011_PHY_SPEC_STATUS_RX_PAUSE: c_uint = 0x0004;
