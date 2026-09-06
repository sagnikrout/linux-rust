//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpio_keys.h
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

//
// struct gpio_keys_button - configuration parameters
// @code:		input event code (KEY_*, SW_*)
// @gpio:		%-1 if this key does not support gpio
// @active_low:		%true indicates that button is considered
// depressed when gpio is low
// @desc:		label that will be attached to button's gpio
// @type:		input event type (%EV_KEY, %EV_SW, %EV_ABS)
// @wakeup:		configure the button as a wake-up source
// @wakeup_event_action:	event action to trigger wakeup
// @debounce_interval:	debounce ticks interval in msecs
// @can_disable:	%true indicates that userspace is allowed to
// disable button via sysfs
// @value:		axis value for %EV_ABS
// @irq:		Irq number in case of interrupt keys
// @wakeirq:		Optional dedicated wake-up interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_keys_button {
    pub code: c_uint,

    pub gpio: c_int,

    pub active_low: c_int,
    pub desc: *const c_char,
    pub type: c_uint,
    pub wakeup: c_int,
    pub wakeup_event_action: c_int,
    pub debounce_interval: c_int,
    pub can_disable: bool,
    pub value: c_int,
    pub irq: c_uint,
    pub wakeirq: c_uint,
}

//
// struct gpio_keys_platform_data - platform data for gpio_keys driver
// @buttons:		pointer to array of &gpio_keys_button structures
// describing buttons attached to the device
// @nbuttons:		number of elements in @buttons array
// @poll_interval:	polling interval in msecs - for polling driver only
// @rep:		enable input subsystem auto repeat
// @enable:		platform hook for enabling the device
// @disable:		platform hook for disabling the device
// @name:		input device name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_keys_platform_data {
    pub buttons: *const gpio_keys_button,
    pub nbuttons: c_int,
    pub poll_interval: c_uint,
    pub rep:1: c_uint,
    pub dev): *mut *mut int (enable)(struct device,
    pub dev): *mut *mut void (disable)(struct device,
    pub name: *const c_char,
}
