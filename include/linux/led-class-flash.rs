//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/led-class-flash.h
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
// LED Flash class interface
//
// Copyright (C) 2015 Samsung Electronics Co., Ltd.
// Author: Jacek Anaszewski <j.anaszewski@samsung.com>
//

//
// Supported led fault bits - must be kept in synch
// with V4L2_FLASH_FAULT bits.
//

pub const LED_NUM_FLASH_FAULTS: c_int = 9;
pub const LED_FLASH_SYSFS_GROUPS_SIZE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_flash_ops {
// set flash brightness
    pub brightness): u32,
// get flash brightness
    pub brightness): *mut u32,
// set flash strobe state
    pub state): *mut *mut *mut int (strobe_set)(struct led_classdev_flash fled_cdev, bool,
// get flash strobe state
    pub state): *mut *mut *mut int (strobe_get)(struct led_classdev_flash fled_cdev, bool,
// set flash timeout
    pub timeout): *mut *mut *mut int (timeout_set)(struct led_classdev_flash fled_cdev, u32,
// get the flash LED fault
    pub fault): *mut *mut *mut int (fault_get)(struct led_classdev_flash fled_cdev, u32,
// set flash duration
    pub duration): *mut *mut *mut int (duration_set)(struct led_classdev_flash fled_cdev, u32,
}

//
// Current value of a flash setting along
// with its constraints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_flash_setting {
// maximum allowed value
    pub min: u32,
// maximum allowed value
    pub max: u32,
// step value
    pub step: u32,
// current value
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_classdev_flash {
// led class device
    pub led_cdev: led_classdev,
// flash led specific ops
    pub ops: *const led_flash_ops,
// flash brightness value in microamperes along with its constraints
    pub brightness: led_flash_setting,
// flash timeout value in microseconds along with its constraints
    pub timeout: led_flash_setting,
// flash timeout value in microseconds along with its constraints
    pub duration: led_flash_setting,
// LED Flash class sysfs groups
    pub sysfs_groups: [*const attribute_group; LED_FLASH_SYSFS_GROUPS_SIZE],
}

extern "C" {
    pub fn container_of(_arg: lcdev, led_classdev_flash: struct, _arg: led_cdev) -> return;
}
//
// led_classdev_flash_register_ext - register a new object of LED class with
// init data and with support for flash LEDs
// @parent: LED flash controller device this flash LED is driven by
// @fled_cdev: the led_classdev_flash structure for this device
// @init_data: the LED class flash device initialization data
//
// Returns: 0 on success or negative error value on failure
//
// led_classdev_flash_unregister - unregisters an object of led_classdev class
// with support for flash LEDs
// @fled_cdev: the flash LED to unregister
//
// Unregister a previously registered via led_classdev_flash_register object
//
extern "C" {
    pub fn led_classdev_flash_unregister(fled_cdev: *mut led_classdev_flash);
}
extern "C" {
    pub fn led_classdev_flash_register_ext(_arg: parent, _arg: fled_cdev, _arg: NULL) -> return;
}
extern "C" {
    pub fn devm_led_classdev_flash_register_ext(_arg: parent, _arg: fled_cdev, _arg: NULL) -> return;
}
//
// led_set_flash_strobe - setup flash strobe
// @fled_cdev: the flash LED to set strobe on
// @state: 1 - strobe flash, 0 - stop flash strobe
//
// Strobe the flash LED.
//
// Returns: 0 on success or negative error value on failure
//
// led_get_flash_strobe - get flash strobe status
// @fled_cdev: the flash LED to query
// @state: 1 - flash is strobing, 0 - flash is off
//
// Check whether the flash is strobing at the moment.
//
// Returns: 0 on success or negative error value on failure
//
// led_set_flash_brightness - set flash LED brightness
// @fled_cdev: the flash LED to set
// @brightness: the brightness to set it to
//
// Set a flash LED's brightness.
//
// Returns: 0 on success or negative error value on failure
//
// led_update_flash_brightness - update flash LED brightness
// @fled_cdev: the flash LED to query
//
// Get a flash LED's current brightness and update led_flash->brightness
// member with the obtained value.
//
// Returns: 0 on success or negative error value on failure
//
extern "C" {
    pub fn led_update_flash_brightness(fled_cdev: *mut led_classdev_flash) -> c_int;
}
//
// led_set_flash_timeout - set flash LED timeout
// @fled_cdev: the flash LED to set
// @timeout: the flash timeout to set it to
//
// Set the flash strobe timeout.
//
// Returns: 0 on success or negative error value on failure
//
extern "C" {
    pub fn led_set_flash_timeout(fled_cdev: *mut led_classdev_flash, timeout: u32) -> c_int;
}
//
// led_get_flash_fault - get the flash LED fault
// @fled_cdev: the flash LED to query
// @fault: bitmask containing flash faults
//
// Get the flash LED fault.
//
// Returns: 0 on success or negative error value on failure
//
extern "C" {
    pub fn led_get_flash_fault(fled_cdev: *mut led_classdev_flash, fault: *mut u32) -> c_int;
}
//
// led_set_flash_duration - set flash LED duration
// @fled_cdev: the flash LED to set
// @timeout: the flash duration to set it to
//
// Set the flash strobe duration.
//
// Returns: 0 on success or negative error value on failure
//
extern "C" {
    pub fn led_set_flash_duration(fled_cdev: *mut led_classdev_flash, duration: u32) -> c_int;
}
