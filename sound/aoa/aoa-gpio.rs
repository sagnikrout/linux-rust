//! Automatically rewritten from C Header to Rust Module
//! Source: sound/aoa/aoa-gpio.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Apple Onboard Audio GPIO definitions
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//

extern "C" {
    pub fn void(data: *mut *mut notify_func_t)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum notify_type {
    AOA_NOTIFY_HEADPHONE,
    AOA_NOTIFY_LINE_IN,
    AOA_NOTIFY_LINE_OUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_methods {
// for initialisation/de-initialisation of the GPIO layer
    pub rt): *mut *mut void (init)(struct gpio_runtime,
    pub rt): *mut *mut void (exit)(struct gpio_runtime,
// turn off headphone, speakers, lineout
    pub rt): *mut *mut void (all_amps_off)(struct gpio_runtime,
// turn headphone, speakers, lineout back to previous setting
    pub rt): *mut *mut void (all_amps_restore)(struct gpio_runtime,
    pub on): *mut *mut *mut void (set_headphone)(struct gpio_runtime rt, int,
    pub on): *mut *mut *mut void (set_speakers)(struct gpio_runtime rt, int,
    pub on): *mut *mut *mut void (set_lineout)(struct gpio_runtime rt, int,
    pub on): *mut *mut *mut void (set_master)(struct gpio_runtime rt, int,
    pub rt): *mut *mut int (get_headphone)(struct gpio_runtime,
    pub rt): *mut *mut int (get_speakers)(struct gpio_runtime,
    pub rt): *mut *mut int (get_lineout)(struct gpio_runtime,
    pub rt): *mut *mut int (get_master)(struct gpio_runtime,
    pub on): *mut *mut *mut void (set_hw_reset)(struct gpio_runtime rt, int,
// use this to be notified of any events. The notification
// function is passed the data, and is called in process
// context by the use of schedule_work.
// The interface for it is that setting a function to NULL
// removes it, and they return 0 if the operation succeeded,
// and -EBUSY if the notification is already assigned by
// someone else.
    pub data): *mut c_void,
// returns 0 if not plugged in, 1 if plugged in
// or a negative error code
    pub type): notify_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_notification {
    pub work: delayed_work,
    pub notify: notify_func_t,
    pub data: *mut c_void,
    pub gpio_private: *mut c_void,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_runtime {
// to be assigned by fabric
    pub node: *mut device_node,
// since everyone needs this pointer anyway...
    pub methods: *mut gpio_methods,
// to be used by the gpio implementation
    pub implementation_private: c_int,
    pub headphone_notify: gpio_notification,
    pub line_in_notify: gpio_notification,
    pub line_out_notify: gpio_notification,
}
