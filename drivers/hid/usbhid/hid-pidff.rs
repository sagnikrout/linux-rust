//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/usbhid/hid-pidff.h
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

// HID PIDFF quirks
// Delay field (0xA7) missing. Skip it during set effect report upload

// Missing Paramter block offset (0x23). Skip it during SET_CONDITION upload

// Initialise device control field even if logical_minimum != 1

// Use fixed 0x4000 direction during SET_EFFECT report upload

// Force all periodic effects to be uploaded as SINE

// Allow devices with missing negative coefficient in the set condition usage

// Allow devices with missing negative saturation in the set condition usage

// Allow devices with missing deadband in the set condition usage

extern "C" {
    pub fn hid_pidff_init(hid: *mut hid_device) -> c_int;
}
extern "C" {
    pub fn hid_pidff_init_with_quirks(hid: *mut hid_device, initial_quirks: u32) -> c_int;
}

