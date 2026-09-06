//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/device.h
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
// KUnit basic device implementation
//
// Helpers for creating and managing fake devices for KUnit tests.
//
// Copyright (C) 2023, Google LLC.
// Author: David Gow <davidgow@google.com>
//

//
// kunit_driver_create() - Create a struct device_driver attached to the kunit_bus
// @test: The test context object.
// @name: The name to give the created driver.
//
// Creates a struct device_driver attached to the kunit_bus, with the name @name.
// This driver will automatically be cleaned up on test exit.
//
// Return: a stub struct device_driver, managed by KUnit, with the name @name.
//
// kunit_device_register() - Create a struct device for use in KUnit tests
// @test: The test context object.
// @name: The name to give the created device.
//
// Creates a struct kunit_device (which is a struct device) with the given name,
// and a corresponding driver. The device and driver will be cleaned up on test
// exit, or when kunit_device_unregister is called. See also
// kunit_device_register_with_driver, if you wish to provide your own
// struct device_driver.
//
// Return: a pointer to a struct device which will be cleaned up when the test
// exits, or an error pointer if the device could not be allocated or registered.
//
// kunit_device_register_with_driver() - Create a struct device for use in KUnit tests
// @test: The test context object.
// @name: The name to give the created device.
// @drv: The struct device_driver to associate with the device.
//
// Creates a struct kunit_device (which is a struct device) with the given
// name, and driver. The device will be cleaned up on test exit, or when
// kunit_device_unregister is called. See also kunit_device_register, if you
// wish KUnit to create and manage a driver for you.
//
// Return: a pointer to a struct device which will be cleaned up when the test
// exits, or an error pointer if the device could not be allocated or registered.
//
// kunit_device_unregister() - Unregister a KUnit-managed device
// @test: The test context object which created the device
// @dev: The device.
//
// Unregisters and destroys a struct device which was created with
// kunit_device_register or kunit_device_register_with_driver. If KUnit created
// a driver, cleans it up as well.
//
extern "C" {
    pub fn kunit_device_unregister(test: *mut kunit, dev: *mut device);
}

