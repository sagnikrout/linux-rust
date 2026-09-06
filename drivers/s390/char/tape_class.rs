//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/tape_class.h
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
// Copyright IBM Corp. 2004   All Rights Reserved.
//
// Tape class device support
//
// Author: Stefan Bader <shbader@de.ibm.com>
// Based on simple class device code by Greg K-H
//

pub const TAPECLASS_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tape_class_device {
    pub char_device: *mut cdev,
    pub class_device: *mut device,
    pub device_name: [c_char; TAPECLASS_NAME_LEN],
    pub mode_name: [c_char; TAPECLASS_NAME_LEN],
}

//
// Register a tape device and return a pointer to the tape class device
// created by the call.
//
// device
// The pointer to the struct device of the physical (base) device.
// dev
// The intended major/minor number. The major number may be 0 to
// get a dynamic major number.
// fops
// The pointer to the drivers file operations for the tape device.
// device_name
// Pointer to the logical device name (will also be used as kobject name
// of the cdev). This can also be called the name of the tape class
// device.
// mode_name
// Points to the name of the tape mode. This creates a link with that
// name from the physical device to the logical device (class).
//
extern "C" {
    pub fn unregister_tape_dev(device: *mut device, tcd: *mut tape_class_device);
}
extern "C" {
    pub fn tape_class_init() -> c_int;
}
extern "C" {
    pub fn tape_class_exit();
}
