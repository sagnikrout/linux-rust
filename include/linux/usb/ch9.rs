//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/ch9.h
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
// This file holds USB constants and structures that are needed for
// USB device APIs.  These are used by the USB device model, which is
// defined in chapter 9 of the USB 2.0 specification and in the
// Wireless USB 1.0 spec (now defunct).  Linux has several APIs in C that
// need these:
//
// - the host side Linux-USB kernel driver API;
// - the "usbfs" user space API; and
// - the Linux "gadget" device/peripheral side driver API.
//
// USB 2.0 adds an additional "On The Go" (OTG) mode, which lets systems
// act either as a USB host or as a USB device.  That means the host and
// device side APIs benefit from working well together.
//
// Note all descriptors are declared '__attribute__((packed))' so that:
//
// [a] they never get padded, either internally (USB spec writers
// probably handled that) or externally;
//
// [b] so that accessing bigger-than-a-bytes fields will never
// generate bus errors on any platform, even when the location of
// its descriptor inside a bundle isn't "naturally aligned", and
//
// [c] for consistency, removing all doubt even when it appears to
// someone that the two other points are non-issues for that
// particular descriptor type.
//

// USB 3.2 SuperSpeed Plus phy signaling rate generation and lane count
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_ssp_rate {
    USB_SSP_GEN_UNKNOWN = 0,
    USB_SSP_GEN_2x1,
    USB_SSP_GEN_1x2,
    USB_SSP_GEN_2x2,
}

extern "C" {
    pub fn usb_get_maximum_speed(dev: *mut device) -> usb_device_speed;
}
extern "C" {
    pub fn usb_get_maximum_ssp_rate(dev: *mut device) -> usb_ssp_rate;
}

