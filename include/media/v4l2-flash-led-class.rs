//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-flash-led-class.h
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
// V4L2 flash LED sub-device registration helpers.
//
// Copyright (C) 2015 Samsung Electronics Co., Ltd
// Author: Jacek Anaszewski <j.anaszewski@samsung.com>
//

//
// struct v4l2_flash_ctrl_data - flash control initialization data, filled
// basing on the features declared by the LED flash
// class driver in the v4l2_flash_config
// @config:	initialization data for a control
// @cid:	contains v4l2 flash control id if the config
// field was initialized, 0 otherwise
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_flash_ctrl_data {
    pub config: v4l2_ctrl_config,
    pub cid: u32,
}

//
// struct v4l2_flash_ops - V4L2 flash operations
//
// @external_strobe_set: Setup strobing the flash by hardware pin state
// assertion.
// @intensity_to_led_brightness: Convert intensity to brightness in a device
// specific manner
// @led_brightness_to_intensity: convert brightness to intensity in a device
// specific manner.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_flash_ops {
    pub enable): bool,
    pub intensity): *mut *mut (struct v4l2_flash v4l2_flash, s32,
    pub led_brightness): *mut *mut (struct v4l2_flash v4l2_flash, enum,
}

//
// struct v4l2_flash_config - V4L2 Flash sub-device initialization data
// @dev_name:			the name of the media entity,
// unique in the system
// @intensity:			non-flash strobe constraints for the LED
// @flash_faults:		bitmask of flash faults that the LED flash class
// device can report; corresponding LED_FAULT* bit
// definitions are available in the header file
// <linux/led-class-flash.h>
// @has_external_strobe:	external strobe capability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_flash_config {
    pub dev_name: [c_char; 32],
    pub intensity: led_flash_setting,
    pub flash_faults: u32,
    pub has_external_strobe:1: c_uint,
}

//
// struct v4l2_flash - Flash sub-device context
// @fled_cdev:		LED flash class device controlled by this sub-device
// @iled_cdev:		LED class device representing indicator LED associated
// with the LED flash class device
// @ops:		V4L2 specific flash ops
// @sd:			V4L2 sub-device
// @hdl:		flash controls handler
// @ctrls:		array of pointers to controls, whose values define
// the sub-device state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_flash {
    pub fled_cdev: *mut led_classdev_flash,
    pub iled_cdev: *mut led_classdev,
    pub ops: *const v4l2_flash_ops,
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub ctrls: *mut v4l2_ctrl,
}

//
// v4l2_subdev_to_v4l2_flash - Returns a &struct v4l2_flash from the
// &struct v4l2_subdev embedded on it.
//
// @sd: pointer to &struct v4l2_subdev
//
extern "C" {
    pub fn container_of(_arg: sd, v4l2_flash: struct, _arg: sd) -> return;
}
//
// v4l2_ctrl_to_v4l2_flash - Returns a &struct v4l2_flash from the
// &struct v4l2_ctrl embedded on it.
//
// @c: pointer to &struct v4l2_ctrl
//
extern "C" {
    pub fn container_of(_arg: c->handler, v4l2_flash: struct, _arg: hdl) -> return;
}

//
// v4l2_flash_init - initialize V4L2 flash led sub-device
// @dev:	flash device, e.g. an I2C device
// @fwn:	fwnode_handle of the LED, may be NULL if the same as device's
// @fled_cdev:	LED flash class device to wrap
// @ops:	V4L2 Flash device ops
// @config:	initialization data for V4L2 Flash sub-device
//
// Create V4L2 Flash sub-device wrapping given LED subsystem device.
// The ops pointer is stored by the V4L2 flash framework. No
// references are held to config nor its contents once this function
// has returned.
//
// Returns: A valid pointer, or, when an error occurs, the return
// value is encoded using ERR_PTR(). Use IS_ERR() to check and
// PTR_ERR() to obtain the numeric return value.
//
// v4l2_flash_indicator_init - initialize V4L2 indicator sub-device
// @dev:	flash device, e.g. an I2C device
// @fwn:	fwnode_handle of the LED, may be NULL if the same as device's
// @iled_cdev:	LED flash class device representing the indicator LED
// @config:	initialization data for V4L2 Flash sub-device
//
// Create V4L2 Flash sub-device wrapping given LED subsystem device.
// The ops pointer is stored by the V4L2 flash framework. No
// references are held to config nor its contents once this function
// has returned.
//
// Returns: A valid pointer, or, when an error occurs, the return
// value is encoded using ERR_PTR(). Use IS_ERR() to check and
// PTR_ERR() to obtain the numeric return value.
//
// v4l2_flash_release - release V4L2 Flash sub-device
// @v4l2_flash: the V4L2 Flash sub-device to release
//
// Release V4L2 Flash sub-device.
//
extern "C" {
    pub fn v4l2_flash_release(v4l2_flash: *mut v4l2_flash);
}

