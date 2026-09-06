//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-default-on.c
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
// LED Kernel Default ON Trigger
//
// Copyright 2008 Nick Forbes <nick.forbes@incepta.com>
//
// Based on Richard Purdie's ledtrig-timer.c.
//

#[no_mangle]
unsafe extern "C" fn defon_trig_activate(led_cdev: *mut led_classdev) -> c_int {
    static int defon_trig_activate(struct led_classdev *led_cdev)
    {
    led_set_brightness_nosleep(led_cdev, led_cdev.max_brightness);
    return 0;
    }
    static struct led_trigger defon_led_trigger = {
    .name     = "default-on",
    .activate = defon_trig_activate,
    };
    module_led_trigger(defon_led_trigger);
    MODULE_AUTHOR("Nick Forbes <nick.forbes@incepta.com>");
    MODULE_DESCRIPTION("Default-ON LED trigger");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("ledtrig:default-on");
