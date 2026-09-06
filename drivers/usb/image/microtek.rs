//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/image/microtek.h
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
// Driver for Microtek Scanmaker X6 USB scanner and possibly others.
//
// (C) Copyright 2000 John Fremlin <vii@penguinpowered.com>
// (C) Copyright 2000 Oliver Neukum <Oliver.Neukum@lrz.uni-muenchen.de>
//
// See microtek.c for history
//
extern "C" {
    pub fn void(: *mut *mut mts_scsi_cmnd_callback)(struct scsi_cmnd) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mts_desc {
    pub next: *mut mts_desc,
    pub prev: *mut mts_desc,
    pub usb_dev: *mut usb_device,
    pub usb_intf: *mut usb_interface,
// Endpoint addresses
    pub ep_out: u8,
    pub ep_response: u8,
    pub ep_image: u8,
    pub host: *mut Scsi_Host,
    pub urb: *mut urb,
    pub context: mts_transfer_context,
}

pub const MTS_EP_OUT: c_uint = 0x1;
pub const MTS_EP_RESPONSE: c_uint = 0x2;
pub const MTS_EP_IMAGE: c_uint = 0x3;
pub const MTS_EP_TOTAL: c_uint = 0x3;
