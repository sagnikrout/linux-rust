//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/media-dev-allocator.h
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
// media-dev-allocator.h - Media Controller Device Allocator API
//
// Copyright (c) 2019 Shuah Khan <shuah@kernel.org>
//
// Credits: Suggested by Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//
// This file adds a global ref-counted Media Controller Device Instance API.
// A system wide global media device list is managed and each media device
// includes a kref count. The last put on the media device releases the media
// device instance.
//

//
// media_device_usb_allocate() - Allocate and return struct &media device
//
// @udev:		struct &usb_device pointer
// @module_name:	should be filled with %KBUILD_MODNAME
// @owner:		struct module pointer %THIS_MODULE for the driver.
// %THIS_MODULE is null for a built-in driver.
// It is safe even when %THIS_MODULE is null.
//
// This interface should be called to allocate a Media Device when multiple
// drivers share usb_device and the media device. This interface allocates
// &media_device structure and calls media_device_usb_init() to initialize
// it.
//
// media_device_delete() - Release media device. Calls kref_put().
//
// @mdev:		struct &media_device pointer
// @module_name:	should be filled with %KBUILD_MODNAME
// @owner:		struct module pointer %THIS_MODULE for the driver.
// %THIS_MODULE is null for a built-in driver.
// It is safe even when %THIS_MODULE is null.
//
// This interface should be called to put Media Device Instance kref.
//

