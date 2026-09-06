//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/usb-ohci-s3c2410.h
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


// SPDX-License-Identifier: GPL-2.0-only
// arch/arm/plat-samsung/include/plat/usb-control.h
//
// Copyright (c) 2004 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// S3C - USB host port information
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c2410_hcd_port {
    pub flags: c_uchar,
    pub power: c_uchar,
    pub oc_status: c_uchar,
    pub oc_changed: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c2410_hcd_info {
    pub hcd: *mut usb_hcd,
    pub port: [s3c2410_hcd_port; 2],
    pub to): *mut *mut void (power_control)(int port, int,
    pub on): *mut *mut *mut void (enable_oc)(struct s3c2410_hcd_info , int,
    pub ports): *mut *mut *mut void (report_oc)(struct s3c2410_hcd_info , int,
}

extern "C" {
    pub fn s3c_ohci_set_platdata(info: *mut s3c2410_hcd_info);
}
