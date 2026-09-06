//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci-plat.h
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
// xhci-plat.h - xHCI host controller driver platform Bus Glue.
//
// Copyright (C) 2015 Renesas Electronics Corporation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_plat_priv {
    pub firmware_name: *const c_char,
    pub quirks: c_ulonglong,
    pub power_lost: bool,
    pub sideband_at_suspend:1: unsigned,
    pub ): *mut *mut void (plat_start)(struct usb_hcd,
    pub ): *mut *mut int (init_quirk)(struct usb_hcd,
    pub ): *mut *mut int (suspend_quirk)(struct usb_hcd,
    pub ): *mut *mut int (resume_quirk)(struct usb_hcd,
    pub ): *mut *mut int (post_resume_quirk)(struct usb_hcd,
}

extern "C" {
    pub fn xhci_plat_remove(dev: *mut platform_device);
}
