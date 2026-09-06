//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/lo.h
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
// G-PHY Local Oscillator

// Local Oscillator control value-pair.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_loctl {
// Control values.
    pub i: i8,
    pub q: i8,
}

// Debugging: Poison value for i and q values.
pub const B43_LOCTL_POISON: c_int = 111;
// This struct holds calibrated LO settings for a set of
// Baseband and RF attenuation settings.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_lo_calib {
// The set of attenuation values this set of LO
// control values is calibrated for.
    pub bbatt: b43_bbatt,
    pub rfatt: b43_rfatt,
// The set of control values for the LO.
    pub ctl: b43_loctl,
// The time when these settings were calibrated (in jiffies)
    pub calib_time: c_ulong,
// List.
    pub list: list_head,
}

// Size of the DC Lookup Table in 16bit words.
pub const B43_DC_LT_SIZE: c_int = 32;
// Local Oscillator calibration information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_txpower_lo_control {
// Lists of RF and BB attenuation values for this device.
// Used for building hardware power control tables.
    pub rfatt_list: b43_rfatt_list,
    pub bbatt_list: b43_bbatt_list,
// The DC Lookup Table is cached in memory here.
// Note that this is only used for Hardware Power Control.
    pub dc_lt: [u16; B43_DC_LT_SIZE],
// List of calibrated control values (struct b43_lo_calib).
    pub calib_list: list_head,
// Last time the power vector was read (jiffies).
    pub pwr_vec_read_time: c_ulong,
// Last time the txctl values were measured (jiffies).
    pub txctl_measured_time: c_ulong,
// Current TX Bias value
    pub tx_bias: u8,
// Current TX Magnification Value (if used by the device)
    pub tx_magn: u8,
// Saved device PowerVector
    pub power_vector: u64,
}

// Calibration expire timeouts.
// Timeouts must be multiple of 15 seconds. To make sure
// the item really expired when the 15 second timer hits, we
// subtract two additional seconds from the timeout.

// Adjust the Local Oscillator to the saved attenuation
// and txctl values.
//
extern "C" {
    pub fn b43_lo_g_adjust(dev: *mut b43_wldev);
}
// Adjust to specific values.
extern "C" {
    pub fn b43_gphy_dc_lt_init(dev: *mut b43_wldev, update_all: bool);
}
extern "C" {
    pub fn b43_lo_g_maintenance_work(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_lo_g_cleanup(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_lo_g_init(dev: *mut b43_wldev);
}
