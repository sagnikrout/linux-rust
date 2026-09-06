//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/6fire/comm.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Linux driver for TerraTec DMX 6Fire USB
//
// Author:	Torsten Schenk <torsten.schenk@zoho.com>
// Created:	Jan 01, 2011
// Copyright:	(C) Torsten Schenk
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_runtime {
    pub chip: *mut sfire_chip,
    pub receiver: urb,
    pub receiver_buffer: *mut u8,
    pub /: *mut *mut u8 serial; / urb serial,
    pub urb)): *mut *mut *mut void context, void(handler)(struct urb,
// writes control data to the device
    pub value): *mut *mut *mut int (write8)(struct comm_runtime rt, u8 request, u8 reg, u8,
    pub vl): u8 vh, u8,
}

extern "C" {
    pub fn usb6fire_comm_init(chip: *mut sfire_chip) -> c_int;
}
extern "C" {
    pub fn usb6fire_comm_abort(chip: *mut sfire_chip);
}
extern "C" {
    pub fn usb6fire_comm_destroy(chip: *mut sfire_chip);
}
