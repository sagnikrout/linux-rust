//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lcd.h
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
// LCD Lowlevel Control Abstraction
//
// Copyright (C) 2003,2004 Hewlett-Packard Company
//

// Notes on locking:
//
// lcd_device->ops_lock is an internal backlight lock protecting the ops
// field and no code outside the core should need to touch it.
//
// Access to set_power() is serialised by the update_lock mutex since
// most drivers seem to need this and historically get it wrong.
//
// Most drivers don't need locking on their get_power() method.
// If yours does, you need to implement it in the driver. You can use the
// update_lock mutex if appropriate.
//
// Any other use of the locks below is probably wrong.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_properties {
// The maximum value for contrast (read-only)
    pub max_contrast: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_ops {
// Get the LCD panel power status (0: full on, 1..3: controller
    pub ): *mut *mut int (get_power)(struct lcd_device,
// Enable or disable power to the LCD (0: on; 4: off, see FB_BLANK_XXX)
    pub power): *mut *mut *mut int (set_power)(struct lcd_device , int,
// Get the current contrast setting (0-max_contrast)
    pub ): *mut *mut int (get_contrast)(struct lcd_device,
// Set LCD panel contrast
    pub contrast): *mut *mut *mut int (set_contrast)(struct lcd_device , int,
//
// Set LCD panel mode (resolutions ...)
//
    pub yres): *mut *mut *mut int (set_mode)(struct lcd_device lcd, u32 xres, u32,
//
// Check if the LCD controls the given display device. This
// operation is optional and if not implemented it is assumed that
// the display is always the one controlled by the LCD.
//
// RETURNS:
//
// If display_dev is NULL or display_dev matches the device controlled by
// the LCD, return true. Otherwise return false.
//
    pub display_device): *mut *mut *mut bool (controls_device)(struct lcd_device lcd, struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_device {
    pub props: lcd_properties,
// This protects the 'ops' field. If 'ops' is NULL, the driver that
    pub ops_lock: mutex,
// If this is NULL, the backing module is unloaded
    pub ops: *const lcd_ops,
// Serialise access to set_power method
    pub update_lock: mutex,
//
// @entry: List entry of all registered lcd devices
//
    pub entry: list_head,
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_platform_data {
// reset lcd panel device.
    pub ld): *mut *mut int (reset)(struct lcd_device,
// on or off to lcd panel. if 'enable' is 0 then
    pub enable): *mut *mut *mut int (power_on)(struct lcd_device ld, int,
// it indicates whether lcd panel was enabled
    pub lcd_enabled: c_int,
// it means delay for stable time when it becomes low to high
    pub reset_delay: c_uint,
// stable time needing to become lcd power on.
    pub power_on_delay: c_uint,
// stable time needing to become lcd power off.
    pub power_off_delay: c_uint,
// it could be used for any purpose.
    pub pdata: *mut c_void,
}

extern "C" {
    pub fn lcd_device_unregister(ld: *mut lcd_device);
}

extern "C" {
    pub fn lcd_notify_blank_all(display_dev: *mut device, power: c_int);
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &ld_dev->dev) -> return;
}
