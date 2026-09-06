//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/leds.h
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

pub const B43_LED_MAX_NAME_LEN: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_led {
    pub wl: *mut b43_wl,
// The LED class device
    pub led_dev: led_classdev,
// The index number of the LED.
    pub index: u8,
// If activelow is true, the LED is ON if the
// bit is switched off.
    pub activelow: bool,
// The unique name string for this LED device.
    pub 1]: char name[B43_LED_MAX_NAME_LEN +,
// The current status of the LED. This is updated locklessly.
    pub state: core::sync::atomic::AtomicI32,
// The active state in hardware.
    pub hw_state: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_leds {
    pub led_tx: b43_led,
    pub led_rx: b43_led,
    pub led_radio: b43_led,
    pub led_assoc: b43_led,
    pub stop: bool,
    pub work: work_struct,
}

pub const B43_MAX_NR_LEDS: c_int = 4;
pub const B43_LED_BEHAVIOUR: c_uint = 0x7F;
pub const B43_LED_ACTIVELOW: c_uint = 0x80;
// LED behaviour values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_led_behaviour {
    B43_LED_OFF,
    B43_LED_ON,
    B43_LED_ACTIVITY,
    B43_LED_RADIO_ALL,
    B43_LED_RADIO_A,
    B43_LED_RADIO_B,
    B43_LED_MODE_BG,
    B43_LED_TRANSFER,
    B43_LED_APTRANSFER,
    B43_LED_WEIRD,		//FIXME
    B43_LED_ASSOC,
    B43_LED_INACTIVE,
}

extern "C" {
    pub fn b43_leds_register(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_leds_unregister(wl: *mut b43_wl);
}
extern "C" {
    pub fn b43_leds_init(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_leds_exit(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_leds_stop(dev: *mut b43_wldev);
}

// LED support disabled
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_leds {
// empty
}

