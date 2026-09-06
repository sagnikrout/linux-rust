//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43legacy/radio.h
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

// Macro flag: #define B43legacy_RADIO_H_

pub const B43legacy_RADIO_DEFAULT_CHANNEL_BG: c_int = 6;
// Force antenna 0.
pub const B43legacy_RADIO_TXANTENNA_0: c_int = 0;
// Force antenna 1.
pub const B43legacy_RADIO_TXANTENNA_1: c_int = 1;
// Use the RX antenna, that was selected for the most recently
// received good PLCP header.
//
pub const B43legacy_RADIO_TXANTENNA_LASTPLCP: c_int = 3;

pub const B43legacy_RADIO_INTERFMODE_NONE: c_int = 0;
pub const B43legacy_RADIO_INTERFMODE_NONWLAN: c_int = 1;
pub const B43legacy_RADIO_INTERFMODE_MANUALWLAN: c_int = 2;
pub const B43legacy_RADIO_INTERFMODE_AUTOWLAN: c_int = 3;
extern "C" {
    pub fn b43legacy_radio_lock(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_radio_unlock(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_radio_read16(dev: *mut b43legacy_wldev, offset: u16) -> u16;
}
extern "C" {
    pub fn b43legacy_radio_write16(dev: *mut b43legacy_wldev, offset: u16, val: u16);
}
extern "C" {
    pub fn b43legacy_radio_init2050(dev: *mut b43legacy_wldev) -> u16;
}
extern "C" {
    pub fn b43legacy_radio_turn_on(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_radio_turn_off(dev: *mut b43legacy_wldev, force: bool);
}
extern "C" {
    pub fn b43legacy_radio_set_txpower_a(dev: *mut b43legacy_wldev, txpower: u16);
}
extern "C" {
    pub fn b43legacy_default_baseband_attenuation(dev: *mut b43legacy_wldev) -> u16;
}
extern "C" {
    pub fn b43legacy_default_radio_attenuation(dev: *mut b43legacy_wldev) -> u16;
}
extern "C" {
    pub fn b43legacy_default_txctl1(dev: *mut b43legacy_wldev) -> u16;
}
extern "C" {
    pub fn b43legacy_radio_set_txantenna(dev: *mut b43legacy_wldev, val: u32);
}
extern "C" {
    pub fn b43legacy_radio_clear_tssi(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_radio_aci_detect(dev: *mut b43legacy_wldev, channel: u8) -> u8;
}
extern "C" {
    pub fn b43legacy_radio_aci_scan(dev: *mut b43legacy_wldev) -> u8;
}
extern "C" {
    pub fn b43legacy_calc_nrssi_slope(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_calc_nrssi_threshold(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_nrssi_hw_read(dev: *mut b43legacy_wldev, offset: u16) -> i16;
}
extern "C" {
    pub fn b43legacy_nrssi_hw_write(dev: *mut b43legacy_wldev, offset: u16, val: i16);
}
extern "C" {
    pub fn b43legacy_nrssi_hw_update(dev: *mut b43legacy_wldev, val: u16);
}
extern "C" {
    pub fn b43legacy_nrssi_mem_update(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_radio_calibrationvalue(dev: *mut b43legacy_wldev) -> u16;
}
