//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/comedi/comedilib.h
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
// comedilib.h
// Header file for kcomedilib
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1998-2001 David A. Schleef <ds@schleef.org>
//
// comedi_open() - Open a COMEDI device from the kernel
// @filename: Fake pathname of the form "/dev/comediN".
//
// Converts @filename to a COMEDI device number and "opens" it if it exists
// and is attached to a low-level COMEDI driver.
//
// Return: A pointer to the COMEDI device on success.
// Return %NULL on failure.
//
extern "C" {
    pub fn comedi_open_from(_arg: path, _arg: -1) -> return;
}
extern "C" {
    pub fn comedi_close_from(dev: *mut comedi_device, from: c_int) -> c_int;
}
//
// comedi_close() - Close a COMEDI device from the kernel
// @dev: COMEDI device.
//
// Closes a COMEDI device previously opened by comedi_open().
//
// Returns: 0
//
extern "C" {
    pub fn comedi_close_from(_arg: dev, _arg: -1) -> return;
}
extern "C" {
    pub fn comedi_get_n_channels(dev: *mut comedi_device, subdevice: c_uint) -> c_int;
}
