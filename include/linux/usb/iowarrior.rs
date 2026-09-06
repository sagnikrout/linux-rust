//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/iowarrior.h
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
pub const CODEMERCS_MAGIC_NUMBER: c_uint = 0xC0	/* like COde Mercenaries */;
// Define the ioctl commands for reading and writing data

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iowarrior_info {
// vendor id : supposed to be USB_VENDOR_ID_CODEMERCS in all cases
    pub vendor: __u32,
// product id : depends on type of chip (USB_DEVICE_ID_CODEMERCS_X)
    pub product: __u32,
// the serial number of our chip (if a serial-number is not available
// this is empty string)
    pub serial: [__u8; 9],
// revision number of the chip
    pub revision: __u32,
// USB-speed of the device (0=UNKNOWN, 1=LOW, 2=FULL 3=HIGH)
    pub speed: __u32,
// power consumption of the device in mA
    pub power: __u32,
// the number of the endpoint
    pub if_num: __u32,
// size of the data-packets on this interface
    pub report_size: __u32,
}

//

