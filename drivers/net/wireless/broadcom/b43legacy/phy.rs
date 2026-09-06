//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43legacy/phy.h
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

// Macro flag: #define B43legacy_PHY_H_

// PHY Registers
// Routing
pub const B43legacy_PHYROUTE_OFDM_GPHY: c_uint = 0x400;
pub const B43legacy_PHYROUTE_EXT_GPHY: c_uint = 0x800;
// Base registers.

// OFDM (A) registers of a G-PHY

// Extended G-PHY registers

// Extended G-PHY Registers

pub const B43legacy_PHY_GTABOFF: c_uint = 0x03FF			/* G-PHY table offset (see below) */;
pub const B43legacy_PHY_GTABNR: c_uint = 0xFC00			/* G-PHY table number (see below) */;
pub const B43legacy_PHY_GTABNR_SHIFT: c_int = 10;

// OFDM table numbers

extern "C" {
    pub fn b43legacy_put_attenuation_into_ranges(_bbatt: *mut c_int, _rfatt: *mut c_int);
}
// OFDM (A) PHY Registers

pub const B43legacy_PHY_BBANDCFG_RXANT: c_uint = 0x180			/* RX Antenna selection */;
pub const B43legacy_PHY_BBANDCFG_RXANT_SHIFT: c_int = 7;

pub const B43legacy_PHY_ANTDWELL_AUTODIV1: c_uint = 0x0100			/* Automatic RX diversity start antenna */;

pub const B43legacy_PHY_ENCORE_EN: c_uint = 0x0200				/* Encore enable */;

pub const B43legacy_PHY_OFDM61_10: c_uint = 0x0010				/* FIXME rename */;

pub const B43legacy_PHY_OTABLEOFF: c_uint = 0x03FF				/* OFDM table offset (see below) */;
pub const B43legacy_PHY_OTABLENR: c_uint = 0xFC00				/* OFDM table number (see below) */;
pub const B43legacy_PHY_OTABLENR_SHIFT: c_int = 10;

pub const B43legacy_PHY_ANTWRSETT_ARXDIV: c_uint = 0x2000				/* Automatic RX diversity enabled */;

extern "C" {
    pub fn b43legacy_put_attenuation_into_ranges(_bbatt: *mut c_int, _rfatt: *mut c_int);
}
// Masks for the different PHY versioning registers.
pub const B43legacy_PHYVER_ANALOG: c_uint = 0xF000;
pub const B43legacy_PHYVER_ANALOG_SHIFT: c_int = 12;
pub const B43legacy_PHYVER_TYPE: c_uint = 0x0F00;
pub const B43legacy_PHYVER_TYPE_SHIFT: c_int = 8;
pub const B43legacy_PHYVER_VERSION: c_uint = 0x00FF;
extern "C" {
    pub fn b43legacy_phy_lock(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_phy_unlock(dev: *mut b43legacy_wldev);
}
// Card uses the loopback gain stuff

extern "C" {
    pub fn b43legacy_phy_read(dev: *mut b43legacy_wldev, offset: u16) -> u16;
}
extern "C" {
    pub fn b43legacy_phy_write(dev: *mut b43legacy_wldev, offset: u16, val: u16);
}
extern "C" {
    pub fn b43legacy_phy_init_tssi2dbm_table(dev: *mut b43legacy_wldev) -> c_int;
}
extern "C" {
    pub fn b43legacy_phy_init(dev: *mut b43legacy_wldev) -> c_int;
}
extern "C" {
    pub fn b43legacy_set_rx_antenna(dev: *mut b43legacy_wldev, antenna: c_int);
}
extern "C" {
    pub fn b43legacy_phy_set_antenna_diversity(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_phy_calibrate(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_phy_connect(dev: *mut b43legacy_wldev, connect: c_int) -> c_int;
}
extern "C" {
    pub fn b43legacy_phy_lo_b_measure(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_phy_lo_g_measure(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_phy_xmitpower(dev: *mut b43legacy_wldev);
}
// Adjust the LocalOscillator to the saved values.
// "fixed" is only set to 1 once in initialization. Set to 0 otherwise.
//
extern "C" {
    pub fn b43legacy_phy_lo_adjust(dev: *mut b43legacy_wldev, fixed: c_int);
}
extern "C" {
    pub fn b43legacy_phy_lo_mark_all_unused(dev: *mut b43legacy_wldev);
}
