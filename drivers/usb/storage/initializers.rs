//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/storage/initializers.h
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
// Header file for Special Initializers for certain USB Mass Storage devices
//
// Current development and maintenance by:
// (c) 1999, 2000 Matthew Dharm (mdharm-usb@one-eyed-alien.net)
//
// This driver is based on the 'USB Mass Storage Class' document. This
// describes in detail the protocol used to communicate with such
// devices.  Clearly, the designers had SCSI and ATAPI commands in
// mind when they created this document.  The commands are all very
// similar to commands in the SCSI-II and ATAPI specifications.
//
// It is important to note that in a number of cases this class
// exhibits class-specific exemptions from the USB specification.
// Notably the usage of NAK, STALL and ACK differs from the norm, in
// that they are used to communicate wait, failed and OK on commands.
//
// Also, for certain devices, the interrupt endpoint is used to convey
// status of a command.
//

//
// This places the Shuttle/SCM USB<->SCSI bridge devices in multi-target
// mode
//
extern "C" {
    pub fn usb_stor_euscsi_init(us: *mut us_data) -> c_int;
}
//
// This function is required to activate all four slots on the UCR-61S2B
// flash reader
//
extern "C" {
    pub fn usb_stor_ucr61s2b_init(us: *mut us_data) -> c_int;
}
// This places the HUAWEI E220 devices in multi-port mode
extern "C" {
    pub fn usb_stor_huawei_e220_init(us: *mut us_data) -> c_int;
}
