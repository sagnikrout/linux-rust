//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/max9271.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2017-2020 Jacopo Mondi
// Copyright (C) 2017-2020 Kieran Bingham
// Copyright (C) 2017-2020 Laurent Pinchart
// Copyright (C) 2017-2020 Niklas Söderlund
// Copyright (C) 2016 Renesas Electronics Corporation
// Copyright (C) 2015 Cogent Embedded, Inc.
//

pub const MAX9271_DEFAULT_ADDR: c_uint = 0x40;
// Register 0x02

// Register 0x04

// Register 0x07

// Register 0x08

// Register 0x09
pub const MAX9271_ID: c_uint = 0x09;
// Register 0x0d

// Register 0x0f

// Register 0x15

//
// struct max9271_device - max9271 device
// @client: The i2c client for the max9271 instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max9271_device {
    pub client: *mut i2c_client,
}

//
// max9271_wake_up() - Wake up the serializer by issuing an i2c transaction
// @dev: The max9271 device
//
// This function shall be called before any other interaction with the
// serializer.
//
extern "C" {
    pub fn max9271_wake_up(dev: *mut max9271_device);
}
//
// max9271_set_serial_link() - Enable/disable serial link
// @dev: The max9271 device
// @enable: Serial link enable/disable flag
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_set_serial_link(dev: *mut max9271_device, enable: bool) -> c_int;
}
//
// max9271_configure_i2c() - Configure I2C bus parameters
// @dev: The max9271 device
// @i2c_config: The I2C bus configuration bit mask
//
// Configure MAX9271 I2C interface. The bus configuration provided in the
// @i2c_config parameter shall be assembled using bit values defined by the
// MAX9271_I2C* macros.
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_configure_i2c(dev: *mut max9271_device, i2c_config: u8) -> c_int;
}
//
// max9271_set_high_threshold() - Enable or disable reverse channel high
// threshold
// @dev: The max9271 device
// @enable: High threshold enable/disable flag
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_set_high_threshold(dev: *mut max9271_device, enable: bool) -> c_int;
}
//
// max9271_configure_gmsl_link() - Configure the GMSL link
// @dev: The max9271 device
//
// FIXME: the GMSL link configuration is currently hardcoded and performed
// by programming registers 0x04, 0x07 and 0x02.
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_configure_gmsl_link(dev: *mut max9271_device) -> c_int;
}
//
// max9271_set_gpios() - Set gpio lines to physical high value
// @dev: The max9271 device
// @gpio_mask: The mask of gpio lines to set to high value
//
// The @gpio_mask parameter shall be assembled using the MAX9271_GP[IO|O]
// bit values.
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_set_gpios(dev: *mut max9271_device, gpio_mask: u8) -> c_int;
}
//
// max9271_clear_gpios() - Set gpio lines to physical low value
// @dev: The max9271 device
// @gpio_mask: The mask of gpio lines to set to low value
//
// The @gpio_mask parameter shall be assembled using the MAX9271_GP[IO|O]
// bit values.
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_clear_gpios(dev: *mut max9271_device, gpio_mask: u8) -> c_int;
}
//
// max9271_enable_gpios() - Enable gpio lines
// @dev: The max9271 device
// @gpio_mask: The mask of gpio lines to enable
//
// The @gpio_mask parameter shall be assembled using the MAX9271_GPIO
// bit values. GPO line is always enabled by default.
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_enable_gpios(dev: *mut max9271_device, gpio_mask: u8) -> c_int;
}
//
// max9271_disable_gpios() - Disable gpio lines
// @dev: The max9271 device
// @gpio_mask: The mask of gpio lines to disable
//
// The @gpio_mask parameter shall be assembled using the MAX9271_GPIO
// bit values. GPO line is always enabled by default and cannot be disabled.
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_disable_gpios(dev: *mut max9271_device, gpio_mask: u8) -> c_int;
}
//
// max9271_verify_id() - Read and verify MAX9271 id
// @dev: The max9271 device
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_verify_id(dev: *mut max9271_device) -> c_int;
}
//
// max9271_set_address() - Program a new I2C address
// @dev: The max9271 device
// @addr: The new I2C address in 7-bit format
//
// This function only takes care of programming the new I2C address @addr to
// in the MAX9271 chip registers, it is responsiblity of the caller to set
// the i2c address client to the @addr value to be able to communicate with
// the MAX9271 chip using the I2C framework APIs after this function returns.
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_set_address(dev: *mut max9271_device, addr: u8) -> c_int;
}
//
// max9271_set_deserializer_address() - Program the remote deserializer address
// @dev: The max9271 device
// @addr: The deserializer I2C address in 7-bit format
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_set_deserializer_address(dev: *mut max9271_device, addr: u8) -> c_int;
}
//
// max9271_set_translation() - Program I2C address translation
// @dev: The max9271 device
// @source: The I2C source address
// @dest: The I2C destination address
//
// Program address translation from @source to @dest. This is required to
// communicate with local devices that do not support address reprogramming.
//
// TODO: The device supports translation of two address, this function currently
// supports a single one.
//
// Return 0 on success or a negative error code on failure
//
extern "C" {
    pub fn max9271_set_translation(dev: *mut max9271_device, source: u8, dest: u8) -> c_int;
}
