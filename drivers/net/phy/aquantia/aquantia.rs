//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/aquantia/aquantia.h
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
// HWMON driver for Aquantia PHY
//
// Author: Nikita Yushchenko <nikita.yoush@cogentembedded.com>
// Author: Andrew Lunn <andrew@lunn.ch>
// Author: Heiner Kallweit <hkallweit1@gmail.com>
//

// Vendor specific 1, MDIO_MMD_VEND1
pub const VEND1_GLOBAL_SC: c_uint = 0x0;

pub const VEND1_GLOBAL_FW_ID: c_uint = 0x0020;

pub const VEND1_GLOBAL_MAILBOX_INTERFACE1: c_uint = 0x0200;

pub const VEND1_GLOBAL_MAILBOX_INTERFACE2: c_uint = 0x0201;
pub const VEND1_GLOBAL_MAILBOX_INTERFACE3: c_uint = 0x0202;

pub const VEND1_GLOBAL_MAILBOX_INTERFACE4: c_uint = 0x0203;

pub const VEND1_GLOBAL_MAILBOX_INTERFACE5: c_uint = 0x0204;

pub const VEND1_GLOBAL_MAILBOX_INTERFACE6: c_uint = 0x0205;

// The following registers all have similar layouts; first the registers...
pub const VEND1_GLOBAL_CFG_10M: c_uint = 0x0310;
pub const VEND1_GLOBAL_CFG_100M: c_uint = 0x031b;
pub const VEND1_GLOBAL_CFG_1G: c_uint = 0x031c;
pub const VEND1_GLOBAL_CFG_2_5G: c_uint = 0x031d;
pub const VEND1_GLOBAL_CFG_5G: c_uint = 0x031e;
pub const VEND1_GLOBAL_CFG_10G: c_uint = 0x031f;
// ...and now the fields

pub const VEND1_GLOBAL_CFG_SERDES_MODE_XFI: c_int = 0;
pub const VEND1_GLOBAL_CFG_SERDES_MODE_SGMII: c_int = 3;
pub const VEND1_GLOBAL_CFG_SERDES_MODE_OCSGMII: c_int = 4;
pub const VEND1_GLOBAL_CFG_SERDES_MODE_XFI5G: c_int = 6;

pub const VEND1_GLOBAL_CFG_RATE_ADAPT_NONE: c_int = 0;
pub const VEND1_GLOBAL_CFG_RATE_ADAPT_USX: c_int = 1;
pub const VEND1_GLOBAL_CFG_RATE_ADAPT_PAUSE: c_int = 2;
// Vendor specific 1, MDIO_MMD_VEND2
pub const VEND1_GLOBAL_CONTROL2: c_uint = 0xc001;

pub const VEND1_GLOBAL_LED_PROV: c_uint = 0xc430;

pub const VEND1_GLOBAL_LED_DRIVE: c_uint = 0xc438;

pub const VEND1_THERMAL_PROV_HIGH_TEMP_FAIL: c_uint = 0xc421;
pub const VEND1_THERMAL_PROV_LOW_TEMP_FAIL: c_uint = 0xc422;
pub const VEND1_THERMAL_PROV_HIGH_TEMP_WARN: c_uint = 0xc423;
pub const VEND1_THERMAL_PROV_LOW_TEMP_WARN: c_uint = 0xc424;
pub const VEND1_THERMAL_STAT1: c_uint = 0xc820;
pub const VEND1_THERMAL_STAT2: c_uint = 0xc821;

pub const VEND1_GENERAL_STAT1: c_uint = 0xc830;

pub const VEND1_GLOBAL_GEN_STAT2: c_uint = 0xc831;

pub const VEND1_GLOBAL_RSVD_STAT1: c_uint = 0xc885;

pub const VEND1_GLOBAL_RSVD_STAT9: c_uint = 0xc88d;

pub const VEND1_GLOBAL_RSVD_STAT9_1000BT2: c_uint = 0x23;
// MDIO_MMD_C22EXT
pub const MDIO_C22EXT_STAT_SGMII_RX_GOOD_FRAMES: c_uint = 0xd292;
pub const MDIO_C22EXT_STAT_SGMII_RX_BAD_FRAMES: c_uint = 0xd294;
pub const MDIO_C22EXT_STAT_SGMII_RX_FALSE_CARRIER: c_uint = 0xd297;
pub const MDIO_C22EXT_STAT_SGMII_TX_GOOD_FRAMES: c_uint = 0xd313;
pub const MDIO_C22EXT_STAT_SGMII_TX_BAD_FRAMES: c_uint = 0xd315;
pub const MDIO_C22EXT_STAT_SGMII_TX_FALSE_CARRIER: c_uint = 0xd317;
pub const MDIO_C22EXT_STAT_SGMII_TX_COLLISIONS: c_uint = 0xd318;
pub const MDIO_C22EXT_STAT_SGMII_TX_LINE_COLLISIONS: c_uint = 0xd319;
pub const MDIO_C22EXT_STAT_SGMII_TX_FRAME_ALIGN_ERR: c_uint = 0xd31a;
pub const MDIO_C22EXT_STAT_SGMII_TX_RUNT_FRAMES: c_uint = 0xd31b;
pub const VEND1_GLOBAL_INT_STD_STATUS: c_uint = 0xfc00;
pub const VEND1_GLOBAL_INT_VEND_STATUS: c_uint = 0xfc01;
pub const VEND1_GLOBAL_INT_STD_MASK: c_uint = 0xff00;

pub const VEND1_GLOBAL_INT_VEND_MASK: c_uint = 0xff01;

pub const AQR_MAX_LEDS: c_int = 3;
// Custom driver definitions for constructing a single variable out of
// aggregate firmware build information. These do not represent hardware
// fields.
//

// 10G-QXGMII firmware for NXP SPF-30841 riser board (AQR412C)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aqr107_hw_stat {
    pub name: *const c_char,
    pub reg: c_int,
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aqr_rate_adaptation {
    AQR_RATE_ADAPT_NONE,
    AQR_RATE_ADAPT_USX,
    AQR_RATE_ADAPT_PAUSE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aqr_global_syscfg {
    pub speed: c_int,
    pub interface: phy_interface_t,
    pub rate_adapt: aqr_rate_adaptation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aqr107_priv {
    pub sgmii_stats: [u64; AQR107_SGMII_STAT_SZ],
    pub fingerprint: u64,
    pub leds_active_low: c_ulong,
    pub leds_active_high: c_ulong,
    pub wait_on_global_cfg: bool,
    pub global_cfg: [aqr_global_syscfg; AQR_NUM_GLOBAL_CFG],
}

extern "C" {
    pub fn aqr_hwmon_probe(phydev: *mut phy_device) -> c_int;
}

extern "C" {
    pub fn aqr_firmware_load(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn aqr_phy_led_active_low_set(phydev: *mut phy_device, index: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn aqr_wait_reset_complete(phydev: *mut phy_device) -> c_int;
}
