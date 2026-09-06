//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/digitv.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct digitv_state {
    pub is_nxt6000: c_int,
    pub sndbuf: [c_uchar; 7],
    pub rcvbuf: [c_uchar; 7],
}

// protocol (from usblogging and the SDK:
//
// Always 7 bytes bulk message(s) for controlling
//
// First byte describes the command. Reads are 2 consecutive transfer (as always).
//
// General structure:
//
// write or first message of a read:
// <cmdbyte> VV <len> B0 B1 B2 B3
//
// second message of a read
// <cmdbyte> VV <len> R0 R1 R2 R3
//
// whereas 0 < len <= 4
//
// I2C address is stored somewhere inside the device.
//
// 0x01 read from EEPROM
// VV = offset; B* = 0; R* = value(s)
//
// 0x02 read register of the COFDM
// VV = register; B* = 0; R* = value(s)
//
// 0x05 write register of the COFDM
// VV = register; B* = value(s);
//
// 0x06 write to the tuner (only for NXT6000)
// VV = 0; B* = PLL data; len = 4;
//
// 0x03 read remote control
// VV = 0; B* = 0; len = 4; R* = key
//
// 0x07 write to the remote (don't know why one should this, resetting ?)
// VV = 0; B* = key; len = 4;
//
// 0x08 write remote type
// VV = 0; B[0] = 0x01, len = 4
//
// 0x09 write device init
// TODO
//
pub const USB_READ_EEPROM: c_int = 1;
pub const USB_READ_COFDM: c_int = 2;
pub const USB_WRITE_COFDM: c_int = 5;
pub const USB_WRITE_TUNER: c_int = 6;
pub const USB_READ_REMOTE: c_int = 3;
pub const USB_WRITE_REMOTE: c_int = 7;
pub const USB_WRITE_REMOTE_TYPE: c_int = 8;
pub const USB_DEV_INIT: c_int = 9;
