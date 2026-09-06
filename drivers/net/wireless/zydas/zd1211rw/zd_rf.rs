//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/zydas/zd1211rw/zd_rf.h
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
// ZD1211 USB-WLAN driver for Linux
//
// Copyright (C) 2005-2007 Ulrich Kunitz <kune@deine-taler.de>
// Copyright (C) 2006-2007 Daniel Drake <dsd@gentoo.org>
//
pub const UW2451_RF: c_uint = 0x2;
pub const UCHIP_RF: c_uint = 0x3;
pub const AL2230_RF: c_uint = 0x4;
pub const AL7230B_RF: c_uint = 0x5	/* a,b,g */;
pub const THETA_RF: c_uint = 0x6;
pub const AL2210_RF: c_uint = 0x7;
pub const MAXIM_NEW_RF: c_uint = 0x8;
pub const UW2453_RF: c_uint = 0x9;
pub const AL2230S_RF: c_uint = 0xa;
pub const RALINK_RF: c_uint = 0xb;
pub const INTERSIL_RF: c_uint = 0xc;
pub const RF2959_RF: c_uint = 0xd;
pub const MAXIM_NEW2_RF: c_uint = 0xe;
pub const PHILIPS_RF: c_uint = 0xf;

// Provides functions of the RF transceiver.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_rf {
    pub type: u8,
    pub channel: u8,
// whether channel integration and calibration should be updated
// defaults to 1 (yes)
    pub update_channel_int:1: u8,
// whether ZD_CR47 should be patched from the EEPROM, if the appropriate
// flag is set in the POD. The vendor driver suggests that this should
// be done for all RF's, but a bug in their code prevents but their
// HW_OverWritePhyRegFromE2P() routine from ever taking effect.
    pub patch_cck_gain:1: u8,
// private RF driver data
    pub priv: *mut c_void,
// RF-specific functions
    pub rf): *mut *mut int (init_hw)(struct zd_rf,
    pub channel): *mut *mut *mut int (set_channel)(struct zd_rf rf, u8,
    pub rf): *mut *mut int (switch_radio_on)(struct zd_rf,
    pub rf): *mut *mut int (switch_radio_off)(struct zd_rf,
    pub channel): *mut *mut *mut int (patch_6m_band_edge)(struct zd_rf rf, u8,
    pub rf): *mut *mut void (clear)(struct zd_rf,
}

extern "C" {
    pub fn zd_rf_init(rf: *mut zd_rf);
}
extern "C" {
    pub fn zd_rf_clear(rf: *mut zd_rf);
}
extern "C" {
    pub fn zd_rf_init_hw(rf: *mut zd_rf, type: u8) -> c_int;
}
extern "C" {
    pub fn zd_rf_scnprint_id(rf: *mut zd_rf, buffer: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn zd_rf_set_channel(rf: *mut zd_rf, channel: u8) -> c_int;
}
extern "C" {
    pub fn zd_switch_radio_on(rf: *mut zd_rf) -> c_int;
}
extern "C" {
    pub fn zd_switch_radio_off(rf: *mut zd_rf) -> c_int;
}
extern "C" {
    pub fn zd_rf_patch_6m_band_edge(rf: *mut zd_rf, channel: u8) -> c_int;
}
extern "C" {
    pub fn zd_rf_generic_patch_6m(rf: *mut zd_rf, channel: u8) -> c_int;
}
// Functions for individual RF chips
extern "C" {
    pub fn zd_rf_init_rf2959(rf: *mut zd_rf) -> c_int;
}
extern "C" {
    pub fn zd_rf_init_al2230(rf: *mut zd_rf) -> c_int;
}
extern "C" {
    pub fn zd_rf_init_al7230b(rf: *mut zd_rf) -> c_int;
}
extern "C" {
    pub fn zd_rf_init_uw2453(rf: *mut zd_rf) -> c_int;
}
