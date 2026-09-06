//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/led-class-multicolor.h
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
// LED Multicolor class interface
// Copyright (C) 2019-20 Texas Instruments Incorporated - http://www.ti.com
//

//
// struct mc_subled - Color component description.
// @color_index: Color ID.
// @brightness: Scaled intensity.
// @intensity: Current intensity.
// @max_intensity: Maximum supported intensity value.
// @channel: Channel index.
//
// Describes a color component of a multicolor LED. Many multicolor LEDs
// do not support global brightness control in hardware, so they use
// the brightness field in connection with led_mc_calc_color_components()
// to perform the intensity scaling in software.
// Such drivers should set max_intensity to 0 to signal the multicolor LED core
// that the maximum global brightness of the LED class device should be used for
// limiting incoming intensity values.
//
// Multicolor LEDs that do support global brightness control in hardware
// should instead set max_intensity to the maximum intensity value supported
// by the hardware for a given color component.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_subled {
    pub color_index: c_uint,
    pub brightness: c_uint,
    pub intensity: c_uint,
    pub max_intensity: c_uint,
    pub channel: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_classdev_mc {
// led class device
    pub led_cdev: led_classdev,
    pub num_colors: c_uint,
    pub subled_info: *mut mc_subled,
}

extern "C" {
    pub fn container_of(_arg: led_cdev, led_classdev_mc: struct, _arg: led_cdev) -> return;
}
//
// led_classdev_multicolor_register_ext - register a new object of led_classdev
// class with support for multicolor LEDs
// @parent: the multicolor LED to register
// @mcled_cdev: the led_classdev_mc structure for this device
// @init_data: the LED class multicolor device initialization data
//
// Returns: 0 on success or negative error value on failure
//
// led_classdev_multicolor_unregister - unregisters an object of led_classdev
// class with support for multicolor LEDs
// @mcled_cdev: the multicolor LED to unregister
//
// Unregister a previously registered via led_classdev_multicolor_register
// object
//
extern "C" {
    pub fn led_classdev_multicolor_unregister(mcled_cdev: *mut led_classdev_mc);
}
//
// led_mc_calc_color_components() - Calculates component brightness values of a LED cluster.
// @mcled_cdev - Multicolor LED class device of the LED cluster.
// @brightness - Global brightness of the LED cluster.
//
// Calculates the brightness values for each color component of a monochrome LED cluster,
// see Documentation/leds/leds-class-multicolor.rst for details.
//
extern "C" {
    pub fn led_classdev_multicolor_register_ext(_arg: parent, _arg: mcled_cdev, _arg: NULL) -> return;
}
