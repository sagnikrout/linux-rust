//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43legacy/leds.h
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

// Macro flag: #define B43legacy_LEDS_H_

pub const B43legacy_LED_MAX_NAME_LEN: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_led {
    pub dev: *mut b43legacy_wldev,
// The LED class device
    pub led_dev: led_classdev,
// The index number of the LED.
    pub index: u8,
// If activelow is true, the LED is ON if the
// bit is switched off.
    pub activelow: bool,
// The unique name string for this LED device.
    pub 1]: char name[B43legacy_LED_MAX_NAME_LEN +,
}

pub const B43legacy_LED_BEHAVIOUR: c_uint = 0x7F;
pub const B43legacy_LED_ACTIVELOW: c_uint = 0x80;
// LED behaviour values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43legacy_led_behaviour {
    B43legacy_LED_OFF,
    B43legacy_LED_ON,
    B43legacy_LED_ACTIVITY,
    B43legacy_LED_RADIO_ALL,
    B43legacy_LED_RADIO_A,
    B43legacy_LED_RADIO_B,
    B43legacy_LED_MODE_BG,
    B43legacy_LED_TRANSFER,
    B43legacy_LED_APTRANSFER,
    B43legacy_LED_WEIRD,
    B43legacy_LED_ASSOC,
    B43legacy_LED_INACTIVE,
}

extern "C" {
    pub fn b43legacy_leds_init(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_leds_exit(dev: *mut b43legacy_wldev);
}

// LED support disabled
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_led {
// empty
}

